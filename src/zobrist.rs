// 64-bit Zobrist hashing, Polyglot book-format compatible.
//
// Key layout:
//   [0..768)   piece-square keys (Polyglot), index = 64 * (2 * role + color)
//              with Black = 0, White = 1 and role: P=0, N=1, B=2, R=3, Q=4, K=5
//   [768..772) Polyglot castling keys: White O-O, White O-O-O, Black O-O,
//              Black O-O-O. These are reused verbatim as the (color, rook
//              file) keys for files a/h so that every standard-chess position
//              hashes bit-identically to the Polyglot specification.
//   [772..780) en-passant file keys (a..h)
//   [780]      turn key (XORed when White is to move)
//
// Castling rights are hashed with **16 keys indexed by (color, rook file)**
// (ADR-003, decision D2). The 12 keys for files b..g are compile-time
// constants derived from the documented splitmix64 PRNG (seed below); the
// file-a/h keys are pinned to the Polyglot castling keys 768..771.
//
// The en-passant file key is XORed only when a pawn of the side to move is
// positioned to capture en passant (pseudo-legal / geometric adjacency),
// exactly as specified by Polyglot and implemented by python-chess.
//
// SPDX-License-Identifier: MIT

use crate::polyglot_keys::POLYGLOT_RANDOM_ARRAY;
use crate::types::{Color, Role, Square, NO_EP};

const PIECE_BASE: usize = 0;
const CASTLE_BASE: usize = 768;
const EP_BASE: usize = 772;
const TURN_KEY: usize = 780;

/// Zobrist key for a piece of `color`/`role` standing on `sq`.
#[inline(always)]
pub fn piece_key(color: Color, role: Role, sq: Square) -> u64 {
    unsafe {
        *POLYGLOT_RANDOM_ARRAY.get_unchecked(PIECE_BASE + 64 * (2 * role.index() + color.index()) + sq.index())
    }
}

/// Raw Polyglot castling key (bit 0..3 = WK, WQ, BK, BQ), i.e. entries
/// `768..772` of the Polyglot table. Kept for reference/migration tooling;
/// live hashing uses [`castle_file_key`], keyed by rook file (ADR-003).
#[inline(always)]
pub fn castle_key(right_bit: u8) -> u64 {
    unsafe { *POLYGLOT_RANDOM_ARRAY.get_unchecked(CASTLE_BASE + right_bit as usize) }
}

/// Zobrist key for the en-passant file of `sq`.
#[inline(always)]
pub fn ep_key(sq: Square) -> u64 {
    unsafe { *POLYGLOT_RANDOM_ARRAY.get_unchecked(EP_BASE + sq.file() as usize) }
}

/// Turn key (XORed when White is to move).
#[inline(always)]
pub fn turn_key() -> u64 {
    unsafe { *POLYGLOT_RANDOM_ARRAY.get_unchecked(TURN_KEY) }
}

// -- rook-file castling keys (ADR-003, decision D2) -------------------------

/// splitmix64 state seed for the 12 derived castling keys. Published so the
/// key set is fully reproducible (cozy-chess precedent: compile-time key
/// generation from a documented seed).
pub const CASTLE_KEY_SEED: u64 = 0x00C0_FFEE_DABA_D00D;

const fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Castling keys indexed by `color.index() * 8 + file` (Black = 0, White = 1,
/// file 0 = a .. 7 = h).
///
/// Files a/h are pinned to the Polyglot castling keys (`768..771`: White
/// O-O = h-file, White O-O-O = a-file, Black O-O = h-file, Black O-O-O =
/// a-file) so standard-chess positions hash bit-identically to Polyglot.
/// The remaining 12 keys come from splitmix64 seeded with [`CASTLE_KEY_SEED`],
/// generated in slot order: Black files b..g first, then White files b..g.
pub const CASTLE_FILE_KEYS: [u64; 16] = {
    let mut prng = [0u64; 12];
    let mut state = CASTLE_KEY_SEED;
    let mut i = 0;
    while i < 12 {
        prng[i] = splitmix64(&mut state);
        i += 1;
    }
    let mut keys = [0u64; 16];
    // Black (color index 0) files b..g, then White (color index 1) b..g.
    let mut f = 1usize;
    while f <= 6 {
        keys[f] = prng[f - 1];
        keys[8 + f] = prng[6 + f - 1];
        f += 1;
    }
    // Pin the a/h keys to the Polyglot castling keys.
    // White O-O (h-file) = 1*8+7, White O-O-O (a-file) = 1*8+0,
    // Black O-O (h-file) = 0*8+7, Black O-O-O (a-file) = 0*8+0.
    keys[15] = POLYGLOT_RANDOM_ARRAY[768];
    keys[8] = POLYGLOT_RANDOM_ARRAY[769];
    keys[7] = POLYGLOT_RANDOM_ARRAY[770];
    keys[0] = POLYGLOT_RANDOM_ARRAY[771];
    keys
};

/// Zobrist key for one castling right, keyed by the owning color and the
/// file of the castling rook (ADR-003).
#[inline(always)]
pub fn castle_file_key(color: Color, file: u8) -> u64 {
    CASTLE_FILE_KEYS[color.index() * 8 + file as usize]
}

/// XOR-fold of the Polyglot castle keys for a standard rights bitmask
/// (WK, WQ, BK, BQ -> Polyglot 768..771). Reference/migration tooling.
#[inline(always)]
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

/// XOR-fold of the rook-file castling keys for the rights in `rights`
/// (bit 0..3 = WK, WQ, BK, BQ) whose rooks stand on `rook_sqs`.
#[inline(always)]
pub fn castle_file_keys_xor(rights: u8, rook_sqs: &[u8; 4]) -> u64 {
    let mut h = 0u64;
    let mut b = 0u8;
    while b < 4 {
        if rights & (1 << b) != 0 {
            let color = if b < 2 { Color::White } else { Color::Black };
            h ^= castle_file_key(color, rook_sqs[b as usize] & 7);
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
#[inline(always)]
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
    fn castle_file_keys_match_polyglot_on_a_and_h() {
        // (White, h) = Polyglot W-K, (White, a) = W-Q,
        // (Black, h) = B-K, (Black, a) = B-Q.
        assert_eq!(castle_file_key(Color::White, 7), POLYGLOT_RANDOM_ARRAY[768]);
        assert_eq!(castle_file_key(Color::White, 0), POLYGLOT_RANDOM_ARRAY[769]);
        assert_eq!(castle_file_key(Color::Black, 7), POLYGLOT_RANDOM_ARRAY[770]);
        assert_eq!(castle_file_key(Color::Black, 0), POLYGLOT_RANDOM_ARRAY[771]);
    }

    #[test]
    fn castle_file_keys_standard_fold_parity() {
        // For standard chess (rooks on a/h) the rook-file fold must equal the
        // Polyglot 4-key fold. Rook squares indexed by right bit:
        // [WK h1, WQ a1, BK h8, BQ a8].
        let rooks = [7, 0, 63, 56];
        assert_eq!(castle_file_keys_xor(0x0F, &rooks), castle_keys_xor(0x0F));
        assert_eq!(castle_file_keys_xor(0x01, &rooks), castle_keys_xor(0x01));
        assert_eq!(castle_file_keys_xor(0x08, &rooks), castle_keys_xor(0x08));
    }

    #[test]
    fn castle_file_keys_distinct() {
        // All 16 keys must be pairwise distinct (960 rights distinguishability).
        for i in 0..16usize {
            for j in (i + 1)..16usize {
                assert_ne!(CASTLE_FILE_KEYS[i], CASTLE_FILE_KEYS[j]);
            }
        }
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
