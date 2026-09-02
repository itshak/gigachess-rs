// Core chess type definitions: Square, Color, Role, Piece, castling rights.
//
// SPDX-License-Identifier: MIT

/// En-passant square sentinel: "no en-passant square".
pub const NO_EP: u8 = u8::MAX;

/// Castling-right bit for White kingside (O-O).
pub const CASTLE_WK: u8 = 1 << 0;
/// Castling-right bit for White queenside (O-O-O).
pub const CASTLE_WQ: u8 = 1 << 1;
/// Castling-right bit for Black kingside (O-O).
pub const CASTLE_BK: u8 = 1 << 2;
/// Castling-right bit for Black queenside (O-O-O).
pub const CASTLE_BQ: u8 = 1 << 3;

/// The castling-right bit for `(color, kingside)`:
/// White kingside = 0, White queenside = 1, Black kingside = 2,
/// Black queenside = 3.
#[inline]
pub const fn castle_right_bit(color: Color, kingside: bool) -> u8 {
    (1 - color as u8) * 2 + (kingside as u8 ^ 1)
}

/// A square on the board, `0..=63`, with `a1 = 0` and `h8 = 63`
/// (little-endian rank-file mapping).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Square(pub u8);

impl Square {
    /// Constructs a square from its raw index (`0..=63`).
    #[inline]
    pub const fn new(i: u8) -> Self {
        Square(i)
    }

    /// Constructs a square from file (0..8, 'a'..'h') and rank (0..8, '1'..'8').
    #[inline]
    pub const fn from_coords(file: u8, rank: u8) -> Self {
        Square(rank * 8 + file)
    }

    /// Raw square index.
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// File index, 0 = 'a' .. 7 = 'h'.
    #[inline]
    pub const fn file(self) -> u8 {
        self.0 & 7
    }

    /// Rank index, 0 = '1' .. 7 = '8'.
    #[inline]
    pub const fn rank(self) -> u8 {
        self.0 >> 3
    }

    /// Single-bit bitboard for this square.
    #[inline]
    pub const fn bit(self) -> u64 {
        1u64 << self.0
    }

    /// Parses a square from algebraic coordinates such as `"e4"`.
    pub fn from_alg(s: &str) -> Option<Square> {
        let b = s.as_bytes();
        if b.len() != 2 {
            return None;
        }
        let file = b[0].wrapping_sub(b'a');
        let rank = b[1].wrapping_sub(b'1');
        if file > 7 || rank > 7 {
            return None;
        }
        Some(Square::from_coords(file, rank))
    }

    /// Algebraic coordinates, e.g. `"e4"`.
    pub const fn to_alg(self) -> [u8; 2] {
        [b'a' + self.file(), b'1' + self.rank()]
    }

    /// Chebyshev distance between two squares.
    pub const fn distance(self, other: Square) -> u8 {
        let df = (self.file() as i32 - other.file() as i32).abs();
        let dr = (self.rank() as i32 - other.rank() as i32).abs();
        (if df > dr { df } else { dr }) as u8
    }
}

impl core::fmt::Display for Square {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let [f0, r0] = self.to_alg();
        write!(f, "{}{}", f0 as char, r0 as char)
    }
}

/// Side to move / piece color. `Black` maps to index 0 and `White` to index 1,
/// matching the Polyglot zobrist piece-key interleaving.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    /// Array index for this color (Black = 0, White = 1).
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// The opposite color.
    #[inline]
    pub const fn other(self) -> Color {
        if matches!(self, Color::White) {
            Color::Black
        } else {
            Color::White
        }
    }

    /// Parses `'w'` / `'b'`.
    pub fn from_char(c: char) -> Option<Color> {
        match c {
            'w' => Some(Color::White),
            'b' => Some(Color::Black),
            _ => None,
        }
    }
}
/// A piece type. Discriminants match the `moves2` promotion encoding:
/// `0 = none/pawn, 1 = knight, 2 = bishop, 3 = rook, 4 = queen` (king = 5).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Role {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Role {
    /// Raw discriminant (also the `moves2` promotion nibble).
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Decodes a `moves2` promotion nibble (0 = no promotion, 1..4 = N/B/R/Q).
    #[inline]
    pub const fn from_promo_bits(bits: u16) -> Option<Role> {
        match bits {
            1 => Some(Role::Knight),
            2 => Some(Role::Bishop),
            3 => Some(Role::Rook),
            4 => Some(Role::Queen),
            _ => None,
        }
    }

    /// Uppercase FEN/SAN letter.
    pub const fn char_upper(self) -> char {
        match self {
            Role::Pawn => 'P',
            Role::Knight => 'N',
            Role::Bishop => 'B',
            Role::Rook => 'R',
            Role::Queen => 'Q',
            Role::King => 'K',
        }
    }
}

/// A colored piece (`color` + `role`).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    /// Constructs a piece from color and role.
    #[inline]
    pub const fn new(color: Color, role: Role) -> Piece {
        Piece { color, role }
    }

    /// Mailbox code (`color_index * 6 + role`).
    #[inline]
    pub const fn code(self) -> u8 {
        (self.color as u8) * 6 + (self.role as u8)
    }

    /// Decodes a mailbox code; returns `None` for the empty sentinel.
    pub const fn from_code(code: u8) -> Option<Piece> {
        if code > 11 {
            None
        } else {
            Some(Piece {
                color: if code >= 6 { Color::White } else { Color::Black },
                role: match code % 6 {
                    0 => Role::Pawn,
                    1 => Role::Knight,
                    2 => Role::Bishop,
                    3 => Role::Rook,
                    4 => Role::Queen,
                    _ => Role::King,
                },
            })
        }
    }
}

/// Bitmask of squares for each file (a..h).
pub const FILE_BB: [u64; 8] = {
    let mut t = [0u64; 8];
    let mut f = 0;
    while f < 8 {
        let mut r = 0;
        while r < 8 {
            t[f] |= 1u64 << (r * 8 + f);
            r += 1;
        }
        f += 1;
    }
    t
};

/// Bitmask of squares for each rank (1..8).
pub const RANK_BB: [u64; 8] = {
    let mut t = [0u64; 8];
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            t[r] |= 1u64 << (r * 8 + f);
            f += 1;
        }
        r += 1;
    }
    t
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_coord_roundtrip() {
        for sq in 0..64u8 {
            let s = Square(sq);
            assert_eq!(Square::from_coords(s.file(), s.rank()), s);
            assert_eq!(Square::from_alg(&s.to_string()), Some(s));
        }
        assert_eq!(Square::from_alg("a1"), Some(Square(0)));
        assert_eq!(Square::from_alg("h8"), Some(Square(63)));
        assert_eq!(Square::from_alg("e4"), Some(Square(28)));
        assert_eq!(Square::from_alg("i1"), None);
        assert_eq!(Square::from_alg("e9"), None);
        assert_eq!(Square::from_alg("e"), None);
    }

    #[test]
    fn piece_code_roundtrip() {
        for color in [Color::Black, Color::White] {
            for role in [
                Role::Pawn,
                Role::Knight,
                Role::Bishop,
                Role::Rook,
                Role::Queen,
                Role::King,
            ] {
                let p = Piece::new(color, role);
                assert_eq!(Piece::from_code(p.code()), Some(p));
            }
        }
        assert_eq!(Piece::from_code(255), None);
    }

    #[test]
    fn promo_bit_mapping() {
        assert_eq!(Role::from_promo_bits(0), None);
        assert_eq!(Role::from_promo_bits(1), Some(Role::Knight));
        assert_eq!(Role::from_promo_bits(4), Some(Role::Queen));
        assert_eq!(Role::from_promo_bits(5), None);
    }

    #[test]
    fn file_rank_masks() {
        assert_eq!(FILE_BB[0], 0x0101_0101_0101_0101);
        assert_eq!(RANK_BB[0], 0x0000_0000_0000_00FF);
        assert_eq!(RANK_BB[7], 0xFF00_0000_0000_0000);
    }
}
