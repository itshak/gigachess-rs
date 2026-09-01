// 64-bit Zobrist hashing, Polyglot book-format compatible.
//
// Key layout (Polyglot specification):
//   [0..768)   piece-square keys, index = 64 * (2 * role + color) + square
//              with Black = 0, White = 1 and role: P=0, N=1, B=2, R=3, Q=4, K=5
//   [768..772) castling keys: White O-O, White O-O-O, Black O-O, Black O-O-O
//   [772..780) en-passant file keys (a..h)
//   [780]      turn key (XORed when White is to move)
//
// The en-passant file key is XORed only when a pawn of the side to move is
// positioned to capture en passant (pseudo-legality), exactly as specified.
//
// SPDX-License-Identifier: MIT

use crate::polyglot_keys::POLYGLOT_RANDOM_ARRAY;
use crate::types::{Color, Role, Square, NO_EP};

const PIECE_BASE: usize = 0;
const CASTLE_BASE: usize = 768;
const EP_BASE: usize = 772;
const TURN_KEY: usize = 780;

/// Zobrist key for a piece of `color`/`role` standing on `sq`.
#[inline]
pub fn piece_key(color: Color, role: Role, sq: Square) -> u64 {
    POLYGLOT_RANDOM_ARRAY[PIECE_BASE + 64 * (2 * role.index() + color.index()) + sq.index()]
}

/// Zobrist key for a single castling right (bit 0..3 = WK, WQ, BK, BQ).
#[inline]
pub fn castle_key(right_bit: u8) -> u64 {
    POLYGLOT_RANDOM_ARRAY[CASTLE_BASE + right_bit as usize]
}

/// Zobrist key for the en-passant file of `sq`.
#[inline]
pub fn ep_key(sq: Square) -> u64 {
    POLYGLOT_RANDOM_ARRAY[EP_BASE + sq.file() as usize]
}

/// Turn key (XORed when White is to move).
#[inline]
pub fn turn_key() -> u64 {
    POLYGLOT_RANDOM_ARRAY[TURN_KEY]
}

/// XOR-fold of the castle keys for a rights bitmask.
#[inline]
pub fn castle_keys_xor(rights: u8) -> u64 {
    let mut h = 0u64;
    let mut b = 0u8;
    while b < 4 {
        if rights & (1 << b) != 0 {
            h ^= castle_key(b);
        }
        b += 1;
    }
    h
}

/// En-passant key contribution for a position, or 0 when no capture is
/// pseudo-legally possible (mirrors the Polyglot hashing rule).
///
/// `ep` is the en-passant target square (or [`NO_EP`]); `pawns_of_side` is the
/// pawn bitboard of the side that would capture; `pawn_attacks_of_other` is
/// [`crate::bitboard::PAWN_ATT`] for the *opposite* color (i.e. the set of
/// squares from which a pawn of `side` attacks `sq`).
#[inline]
pub fn ep_contribution(ep: u8, pawns_of_side: u64, pawn_attacks_of_other: u64) -> u64 {
    if ep == NO_EP {
        return 0;
    }
    let capturers = pawn_attacks_of_other & pawns_of_side;
    if capturers != 0 {
        ep_key(Square(ep))
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Piece;

    #[test]
    fn polyglot_key_layout() {
        // Turn key must be the last entry used.
        assert_eq!(TURN_KEY, POLYGLOT_RANDOM_ARRAY.len() - 1);
        assert_eq!(POLYGLOT_RANDOM_ARRAY.len(), 781);
        // Piece key indexing: black pawn a1 = index 0.
        let bp_a1 = piece_key(Color::Black, Role::Pawn, Square(0));
        assert_eq!(bp_a1, POLYGLOT_RANDOM_ARRAY[0]);
        let wk_h8 = piece_key(Color::White, Role::King, Square(63));
        assert_eq!(wk_h8, POLYGLOT_RANDOM_ARRAY[64 * (2 * 5 + 1) + 63]);
    }

    #[test]
    fn castle_keys_order() {
        assert_eq!(castle_key(0), POLYGLOT_RANDOM_ARRAY[768]);
        assert_eq!(castle_key(3), POLYGLOT_RANDOM_ARRAY[771]);
        assert_eq!(castle_keys_xor(0x0F), {
            let mut h = 0u64;
            for b in 0..4u8 {
                h ^= POLYGLOT_RANDOM_ARRAY[768 + b as usize];
            }
            h
        });
    }

    #[test]
    fn ep_contribution_rules() {
        let e3: u8 = 20; // en-passant target after e2e4
        // White to move: white pawns that attack e3 stand on d2/f2 (squares 11, 13).
        let white_pawns = (1u64 << 11) | (1u64 << 13);
        assert_ne!(
            ep_contribution(e3, white_pawns, crate::bitboard::PAWN_ATT[Color::Black as usize][e3 as usize]),
            0
        );
        // No adjacent white pawn -> no contribution.
        assert_eq!(
            ep_contribution(e3, 1u64 << 8, crate::bitboard::PAWN_ATT[Color::Black as usize][e3 as usize]),
            0
        );
        let _ = Piece::new(Color::White, Role::Pawn);
    }
}
