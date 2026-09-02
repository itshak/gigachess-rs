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

/// Parses a FEN string into a validated [`Board`].
pub fn parse_fen(fen: &str) -> Result<Board, FenError> {
    let fields: Vec<&str> = fen.split_whitespace().collect();
    if fields.len() < 4 {
        return Err(FenError("expected at least 4 fields".into()));
    }

    let mut board = Board::empty();

    // 1) Piece placement (rank 8 first).
    let rows: Vec<&str> = fields[0].split('/').collect();
    if rows.len() != 8 {
        return Err(FenError("piece placement must have 8 ranks".into()));
    }
    for (i, row) in rows.iter().enumerate() {
        let rank = 7 - i as u8;
        let mut file = 0u8;
        for b in row.bytes() {
            if b.is_ascii_digit() {
                file += b - b'0';
            } else {
                let (color, role) = piece_from_byte(b)
                    .ok_or_else(|| FenError(format!("bad piece char {}", b as char)))?;
                if file > 7 {
                    return Err(FenError("rank overflow".into()));
                }
                let sq = Square::from_coords(file, rank);
                board.put_piece(sq.0, Piece::new(color, role).code());
                file += 1;
            }
        }
        if file != 8 {
            return Err(FenError("rank does not describe 8 files".into()));
        }
    }

    // 2) Side to move.
    let turn = Color::from_char(
        fields[1]
            .chars()
            .next()
            .ok_or_else(|| FenError("empty side field".into()))?,
    )
    .ok_or_else(|| FenError("side must be 'w' or 'b'".into()))?;

    // 3) Castling rights. Standard K/Q/k/q letters select the outermost rook
    //    of the color on its back rank on that side of the king (X-FEN);
    //    file letters (Shredder-FEN, uppercase = White) name the rook file
    //    directly. Rights are stored with the rook square backing each.
    let mut castling = 0u8;
    let mut castle_rook_sq = [0u8; 4];
    if fields[2] != "-" {
        for b in fields[2].bytes() {
            let (right, rook_sq) = match b {
                b'K' | b'Q' | b'k' | b'q' => {
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
    let ep = if fields[3] == "-" {
        NO_EP
    } else {
        let sq = Square::from_alg(fields[3]).ok_or_else(|| FenError("bad ep square".into()))?;
        let expected_rank = if turn == Color::White { 5 } else { 2 };
        if sq.rank() != expected_rank {
            return Err(FenError("ep square on wrong rank".into()));
        }
        sq.0
    };

    // 5) Clocks (optional in relaxed mode).
    let halfmove: u16 = fields
        .get(4)
        .map(|s| s.parse::<u16>().map_err(|_| FenError("bad halfmove clock".into())))
        .transpose()?
        .unwrap_or(0);
    let fullmove: u16 = fields
        .get(5)
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

        for rank in (0..8).rev() {
            let mut empty = 0u8;
            for file in 0..8 {
                if let Some(p) = self.piece_at(Square::from_coords(file, rank)) {
                    if empty > 0 {
                        out.push(b'0' + empty);
                        empty = 0;
                    }
                    out.push(PIECE_CHAR[p.code() as usize]);
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
