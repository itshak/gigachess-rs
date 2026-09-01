// FEN parsing and formatting.
//
// The parser uses a flat ASCII lookup (single `match` on the piece byte —
// branch-predictor friendly) and validates geometry before constructing the
// position. The formatter is the exact inverse.
//
// SPDX-License-Identifier: MIT

use crate::board::Board;
use crate::types::{
    Color, Piece, Role, Square, NO_EP, CASTLE_BK, CASTLE_BQ, CASTLE_WK, CASTLE_WQ,
};

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

    // 3) Castling rights.
    let mut castling = 0u8;
    if fields[2] != "-" {
        for b in fields[2].bytes() {
            castling |= match b {
                b'K' => CASTLE_WK,
                b'Q' => CASTLE_WQ,
                b'k' => CASTLE_BK,
                b'q' => CASTLE_BQ,
                _ => return Err(FenError("bad castling field".into())),
            };
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

    board.set_state(turn, castling, ep, halfmove, fullmove);
    Ok(board)
}

impl Board {
    /// Renders the position as a FEN string.
    pub fn to_fen(&self) -> String {
        let mut s = String::with_capacity(90);
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                match self.piece_at(Square::from_coords(file, rank)) {
                    None => empty += 1,
                    Some(p) => {
                        if empty > 0 {
                            s.push((b'0' + empty) as char);
                            empty = 0;
                        }
                        s.push(byte_from_piece(p) as char);
                    }
                }
            }
            if empty > 0 {
                s.push((b'0' + empty) as char);
            }
            if rank > 0 {
                s.push('/');
            }
        }
        s.push(' ');
        s.push(if self.turn() == Color::White { 'w' } else { 'b' });
        s.push(' ');
        let c = self.castling_rights();
        if c == 0 {
            s.push('-');
        } else {
            if c & CASTLE_WK != 0 {
                s.push('K');
            }
            if c & CASTLE_WQ != 0 {
                s.push('Q');
            }
            if c & CASTLE_BK != 0 {
                s.push('k');
            }
            if c & CASTLE_BQ != 0 {
                s.push('q');
            }
        }
        s.push(' ');
        match self.en_passant() {
            Some(sq) => s.push_str(&sq.to_string()),
            None => s.push('-'),
        }
        s.push(' ');
        s.push_str(&self.halfmove_clock().to_string());
        s.push(' ');
        s.push_str(&self.fullmove_number().to_string());
        s
    }
}
