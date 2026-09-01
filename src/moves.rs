// 16-bit packed `Move` (moves2 wire format):
//   word = from | (to << 6) | (promo << 12)
// where promo: 0 = none, 1 = knight, 2 = bishop, 3 = rook, 4 = queen.
//
// SPDX-License-Identifier: MIT

use crate::types::{Role, Square};

/// A chess move packed into 16 bits (the `moves2` binary database format).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Move(pub u16);

impl Move {
    /// Packs `from`, `to` and an optional promotion role into 16 bits.
    #[inline]
    pub const fn new(from: Square, to: Square, promo: Option<Role>) -> Move {
        let p = match promo {
            None => 0u16,
            Some(r) => r as u16,
        };
        Move((from.0 as u16) | ((to.0 as u16) << 6) | (p << 12))
    }

    /// Origin square.
    #[inline]
    pub const fn from(self) -> Square {
        Square((self.0 & 0x3f) as u8)
    }

    /// Destination square.
    #[inline]
    pub const fn to(self) -> Square {
        Square(((self.0 >> 6) & 0x3f) as u8)
    }

    /// Promotion role, if any.
    #[inline]
    pub const fn promotion(self) -> Option<Role> {
        Role::from_promo_bits(self.0 >> 12)
    }

    /// Raw 16-bit word for the `moves2` wire format.
    #[inline]
    pub const fn word(self) -> u16 {
        self.0
    }

    /// Reconstructs a move from its raw 16-bit `moves2` word.
    #[inline]
    pub const fn from_word(w: u16) -> Move {
        Move(w)
    }
}

impl core::fmt::Display for Move {
    /// Renders the move in UCI notation (e.g. `e2e4`, `e7e8q`).
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let [ff, fr] = self.from().to_alg();
        let [tf, tr] = self.to().to_alg();
        write!(f, "{}{}{}{}", ff as char, fr as char, tf as char, tr as char)?;
        if let Some(p) = self.promotion() {
            write!(f, "{}", p.char_upper().to_ascii_lowercase())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Color, Piece};

    #[test]
    fn round_trip_packing() {
        for from in 0..64u8 {
            for to in 0..64u8 {
                let promos: [Option<Role>; 6] = [
                    None,
                    Some(Role::Knight),
                    Some(Role::Bishop),
                    Some(Role::Rook),
                    Some(Role::Queen),
                    Some(Role::King), // king "promo" bit = 5: encodes losslessly too
                ];
                for promo in promos {
                    let mv = Move::new(Square(from), Square(to), promo);
                    assert_eq!(mv.from(), Square(from));
                    assert_eq!(mv.to(), Square(to));
                    assert_eq!(mv.promotion(), promo.filter(|r| *r != Role::King));
                    assert_eq!(Move::from_word(mv.word()), mv);
                    // Bit layout exactly per spec.
                    assert_eq!(mv.0 & 0x3f, from as u16);
                    assert_eq!((mv.0 >> 6) & 0x3f, to as u16);
                    assert_eq!(mv.0 >> 12, promo.map(|r| r as u16).unwrap_or(0));
                }
            }
        }
    }

    #[test]
    fn display_uci() {
        assert_eq!(
            Move::new(Square::from_alg("e2").unwrap(), Square::from_alg("e4").unwrap(), None)
                .to_string(),
            "e2e4"
        );
        assert_eq!(
            Move::new(
                Square::from_alg("e7").unwrap(),
                Square::from_alg("e8").unwrap(),
                Some(Role::Queen)
            )
            .to_string(),
            "e7e8q"
        );
    }

    #[test]
    fn piece_code_layout_matches_color_index() {
        // Mailbox code = color_index * 6 + role (White = 1, Black = 0).
        let wq = Piece::new(Color::White, Role::Queen);
        assert_eq!(wq.code(), 10);
        let bq = Piece::new(Color::Black, Role::Queen);
        assert_eq!(bq.code(), 4);
    }
}
