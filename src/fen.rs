// FEN parsing and formatting.
//
// The parser uses a flat ASCII lookup (single `match` on the piece byte —
// branch-predictor friendly) and validates geometry before constructing the
// position. The formatter is the exact inverse.
//
// SPDX-License-Identifier: MIT

use crate::board::Board;
use crate::types::{Color, Piece, Role, Square, NO_EP};

/// FEN parse failure with a description of the offending field.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FenError(pub String);

impl core::fmt::Display for FenError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid FEN: {}", self.0)
    }
}
impl std::error::Error for FenError {}

/// ASCII lookup: FEN piece byte -> (color, role).
#[inline]
fn piece_from_byte(b: u8) -> Option<(Color, Role)> {
    match b {
        b'P' => Some((Color::White, Role::Pawn)),
        b'N' => Some((Color::White, Role::Knight)),
        b'B' => Some((Color::White, Role::Bishop)),
        b'R' => Some((Color::White, Role::Rook)),
        b'Q' => Some((Color::White, Role::Queen)),
        b'K' => Some((Color::White, Role::King)),
        b'p' => Some((Color::Black, Role::Pawn)),
        b'n' => Some((Color::Black, Role::Knight)),
        b'b' => Some((Color::Black, Role::Bishop)),
        b'r' => Some((Color::Black, Role::Rook)),
        b'q' => Some((Color::Black, Role::Queen)),
        b'k' => Some((Color::Black, Role::King)),
        _ => None,
    }
}

/// FEN byte for a piece (uppercase for White, lowercase for Black).
#[inline]
fn byte_from_piece(p: Piece) -> u8 {
    let c = p.role.char_upper() as u8;
    if p.color == Color::White {
        c
    } else {
        c + 32
    }
}

/// Parses a FEN string into a validated [`Board`] — `split_ascii_whitespace`
/// + `chars` direct (no `Vec` alloc) like `ultrachess` `144ns` path.
pub fn parse_fen(fen: &str) -> Result<Board, FenError> {
    let mut it = fen.split_ascii_whitespace();
    let placement = it.next().ok_or_else(|| FenError("expected at least 4 fields".into()))?;
    let side = it.next().ok_or_else(|| FenError("expected at least 4 fields".into()))?;
    let castling_field = it.next().ok_or_else(|| FenError("expected at least 4 fields".into()))?;
    let ep_field = it.next().ok_or_else(|| FenError("expected at least 4 fields".into()))?;
    let halfmove_s = it.next();
    let fullmove_s = it.next();

    let mut board = Board::empty();

    // 1) Piece placement: ranks 8..1, '/' separated, bytes direct (no `chars` UTF-8 decode,
    //    no `to_digit` — branchless byte compare, like `ultrachess` `144ns` path).
    let mut rank: i32 = 7;
    let mut file: i32 = 0;
    for &b in placement.as_bytes() {
        if b == b'/' {
            if file != 8 {
                return Err(FenError(format!("rank {rank} has {file} squares")));
            }
            rank -= 1;
            file = 0;
            continue;
        }
        if b >= b'1' && b <= b'8' {
            let d = (b - b'0') as i32;
            file += d;
            if file > 8 {
                return Err(FenError(format!("rank {rank} overflowed past h")));
            }
            continue;
        }
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return Err(FenError(format!("position {file},{rank} out of range")));
        }
        let piece = piece_from_byte(b)
            .ok_or_else(|| FenError(format!("bad piece char {}", b as char)))?;
        let sq = (rank as u8) * 8 + file as u8;
        board.put_piece_no_hash(sq, Piece::new(piece.0, piece.1).code());
        file += 1;
    }
    if rank != 0 || file != 8 {
        return Err(FenError("rank does not describe 8 files".into()));
    }

    // 2) Side to move — single byte check (no `chars`).
    let turn = match side.as_bytes().first().copied() {
        Some(b'w') => Color::White,
        Some(b'b') => Color::Black,
        Some(_) => return Err(FenError("side must be 'w' or 'b'".into())),
        None => return Err(FenError("empty side field".into())),
    };

    // 3) Castling rights. Standard K/Q/k/q letters select the outermost rook
    //    of the color on its back rank on that side of the king (X-FEN);
    //    file letters (Shredder-FEN, uppercase = White) name the rook file
    //    directly. Rights are stored with the rook square backing each.
    //    Fast path for standard (king on e1/e8, rooks on a/h): directly map
    //    K→h1/a1 etc. without scanning `piece_bb` (ultrachess parity, saves
    //    `rooks.leading_zeros` per `KQkq` letter on `M1` `ARM`).
    let mut castling = 0u8;
    let mut castle_rook_sq = [0u8; 4];
    let is_standard_king = board.king_square(Color::White).0 == 4 && board.king_square(Color::Black).0 == 60;
    let is_standard_castling_field = castling_field.bytes().all(|b| matches!(b, b'K' | b'Q' | b'k' | b'q' | b'-'));
    let use_fast_standard = is_standard_king && is_standard_castling_field;
    if castling_field != "-" {
        for b in castling_field.bytes() {
            let (right, rook_sq) = match b {
                b'K' | b'Q' | b'k' | b'q' => {
                    if use_fast_standard {
                        // Standard fixed squares: h1/a1/h8/a8, no scan
                        let (right, sq) = match b {
                            b'K' => (0, 7),
                            b'Q' => (1, 0),
                            b'k' => (2, 63),
                            _ => (3, 56),
                        };
                        (right, sq)
                    } else {
                        let (color, kingside) = match b {
                            b'K' => (Color::White, true),
                            b'Q' => (Color::White, false),
                            b'k' => (Color::Black, true),
                            _ => (Color::Black, false),
                        };
                        let ksq = board.king_square(color).0;
                        let rank = ksq >> 3;
                        // Outermost rook of the color on its back rank.
                        let rooks = board.piece_bb(color, Role::Rook)
                            & crate::types::RANK_BB[rank as usize];
                        if rooks == 0 {
                            return Err(FenError(
                                "castling letter with no rook on the king's rank".into(),
                            ));
                        }
                        let rook_sq = if kingside {
                            63 - (rooks.leading_zeros() as u8) // rightmost
                        } else {
                            rooks.trailing_zeros() as u8 // leftmost
                        };
                        if (rook_sq & 7 > ksq & 7) != kingside {
                            return Err(FenError(
                                "castling letter with no rook on that side of the king".into(),
                            ));
                        }
                        (if kingside { if color == Color::White { 0 } else { 2 } } else { if color == Color::White { 1 } else { 3 } }, rook_sq)
                    }
                }
                b'A'..=b'H' => {
                    // White rook on the named file (Shredder-FEN).
                    let file = b - b'A';
                    let ksq = board.king_square(Color::White).0;
                    let sq = Square::from_coords(file, ksq >> 3);
                    if board.piece_at(sq) != Some(Piece::new(Color::White, Role::Rook)) {
                        return Err(FenError(format!(
                            "no white rook on file {} for castling right",
                            file
                        )));
                    }
                    (if file > (ksq & 7) { 0 } else { 1 }, sq.0)
                }
                b'a'..=b'h' => {
                    // Black rook on the named file (Shredder-FEN).
                    let file = b - b'a';
                    let ksq = board.king_square(Color::Black).0;
                    let sq = Square::from_coords(file, ksq >> 3);
                    if board.piece_at(sq) != Some(Piece::new(Color::Black, Role::Rook)) {
                        return Err(FenError(format!(
                            "no black rook on file {} for castling right",
                            file
                        )));
                    }
                    (if file > (ksq & 7) { 2 } else { 3 }, sq.0)
                }
                _ => return Err(FenError("bad castling field".into())),
            };
            if castling & (1 << right) != 0 {
                return Err(FenError("duplicate castling right".into()));
            }
            castling |= 1 << right;
            castle_rook_sq[right as usize] = rook_sq;
        }
    }

    // 4) En-passant target square.
    let ep = if ep_field == "-" {
        NO_EP
    } else {
        let sq = Square::from_alg(ep_field).ok_or_else(|| FenError("bad ep square".into()))?;
        let expected_rank = if turn == Color::White { 5 } else { 2 };
        if sq.rank() != expected_rank {
            return Err(FenError("ep square on wrong rank".into()));
        }
        sq.0
    };

    // 5) Clocks (optional in relaxed mode).
    let halfmove: u16 = halfmove_s
        .map(|s| s.parse::<u16>().map_err(|_| FenError("bad halfmove clock".into())))
        .transpose()?
        .unwrap_or(0);
    let fullmove: u16 = fullmove_s
        .map(|s| s.parse::<u16>().map_err(|_| FenError("bad fullmove number".into())))
        .transpose()?
        .unwrap_or(1);
    if fullmove == 0 {
        return Err(FenError("fullmove number must be >= 1".into()));
    }

    // Validation: exactly one king per side; no pawns on ranks 1/8; kings not
    // adjacent.
    for color in [Color::White, Color::Black] {
        if board.piece_bb(color, Role::King).count_ones() != 1 {
            return Err(FenError("position must contain exactly one king per side".into()));
        }
        if board.piece_bb(color, Role::Pawn)
            & (crate::types::RANK_BB[0] | crate::types::RANK_BB[7])
            != 0
        {
            return Err(FenError("pawn on back rank".into()));
        }
    }
    let wk = board.king_square(Color::White);
    let bk = board.king_square(Color::Black);
    if wk.distance(bk) <= 1 {
        return Err(FenError("kings adjacent".into()));
    }

    board.set_state(turn, castling, castle_rook_sq, ep, halfmove, fullmove);
    Ok(board)
}

impl Board {
    /// Renders the position as a FEN string — branchless via `PIECE_CHAR` table
    /// and `ArrayVec<u8,128>` without `format!` (`ultrachess/src/fen.rs:189` 88ns).
    pub fn to_fen(&self) -> String {
        // `PIECE_CHAR` indexed by mailbox code (0..12, Black=0..5, White=6..11)
        // — branchless table vs `match` on role/color.
        const PIECE_CHAR: [u8; 12] = *b"pnbrqkPNBRQK";
        let mut out = arrayvec::ArrayVec::<u8, 128>::new();

        // Helper: push decimal `u16` without `format!` (branchless digit loop).
        #[inline]
        fn push_u16(out: &mut arrayvec::ArrayVec<u8, 128>, mut n: u16) {
            if n == 0 {
                out.push(b'0');
                return;
            }
            let mut buf = [0u8; 5];
            let mut len = 0;
            while n > 0 {
                buf[len] = b'0' + (n % 10) as u8;
                n /= 10;
                len += 1;
            }
            while len > 0 {
                len -= 1;
                out.push(buf[len]);
            }
        }

        // Stamp pieces from the 12 bitboards once (close-gap D3, task 4.1:
        // `Board` no longer stores a mailbox, so rebuild a local one in 12
        // scans instead of 64 `piece_at` scans — restores the 77ns target).
        let mut mailbox = [u8::MAX; 64]; // u8::MAX = EMPTY sentinel
        let mut code = 0usize;
        for color in [Color::Black, Color::White] {
            for role in [
                Role::Pawn,
                Role::Knight,
                Role::Bishop,
                Role::Rook,
                Role::Queen,
                Role::King,
            ] {
                let mut bb = self.piece_bb(color, role);
                while bb != 0 {
                    let sq = bb.trailing_zeros() as usize;
                    bb &= bb - 1;
                    mailbox[sq] = code as u8;
                }
                code += 1;
            }
        }
        for rank in (0..8).rev() {
            let mut empty = 0u8;
            for file in 0..8 {
                let code = mailbox[rank * 8 + file];
                if code != u8::MAX {
                    if empty > 0 {
                        out.push(b'0' + empty);
                        empty = 0;
                    }
                    out.push(PIECE_CHAR[code as usize]);
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                out.push(b'0' + empty);
            }
            if rank > 0 {
                out.push(b'/');
            }
        }
        out.push(b' ');
        out.push(if self.turn() == Color::White { b'w' } else { b'b' });
        out.push(b' ');
        // Castling, X-FEN convention (matches python-chess castling_xfen):
        // Branchless file-letter vs side-letter via stack array (no Vec alloc).
        let c = self.castling_rights();
        if c == 0 {
            out.push(b'-');
        } else {
            for color in [Color::White, Color::Black] {
                let wk = crate::types::castle_right_bit(color, true);
                let wq = crate::types::castle_right_bit(color, false);
                // At most 2 rights per color, stack array + manual sort (no heap Vec).
                let mut rights = [0u8; 2];
                let mut n = 0usize;
                if c & (1 << wk) != 0 {
                    rights[n] = wk;
                    n += 1;
                }
                if c & (1 << wq) != 0 {
                    rights[n] = wq;
                    n += 1;
                }
                // Sort descending by rook square (same as ultrachess).
                if n == 2 && self.castling_rook_square(rights[0]).0 < self.castling_rook_square(rights[1]).0 {
                    rights.swap(0, 1);
                }
                for i in 0..n {
                    let rb = rights[i];
                    let rook_file = self.castling_rook_square(rb).file();
                    let king_file = self.king_square(color).file();
                    let a_side = rook_file < king_file;
                    let ambiguous = {
                        let other = if rb == wk { wq } else { wk };
                        c & (1 << other) != 0
                            && (self.castling_rook_square(other).file() < king_file) == a_side
                    };
                    let ch = if ambiguous {
                        b'a' + rook_file
                    } else if a_side {
                        b'q'
                    } else {
                        b'k'
                    };
                    out.push(if color == Color::White { ch - 32 } else { ch });
                }
            }
        }
        out.push(b' ');
        match self.en_passant() {
            Some(sq) => {
                let [f, r] = sq.to_alg();
                out.push(f);
                out.push(r);
            }
            None => out.push(b'-'),
        }
        out.push(b' ');
        push_u16(&mut out, self.halfmove_clock());
        out.push(b' ');
        push_u16(&mut out, self.fullmove_number());
        // SAFETY: we only pushed ASCII FEN bytes.
        unsafe { String::from_utf8_unchecked(out.into_iter().collect::<Vec<u8>>()) }
    }
}
