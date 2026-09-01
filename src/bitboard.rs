// Native u64 bitboard primitives: shifts, CTZ/popcount helpers, const-computed
// leap-attack tables (knight, king, pawn) and precomputed 64x64 between/line
// ray tables for O(1) check and pin verification (design D3).
//
// SPDX-License-Identifier: MIT

use crate::types::{Color, FILE_BB};

/// Number of set bits.
#[inline]
pub const fn popcount(b: u64) -> u32 {
    b.count_ones()
}

/// Index of the least-significant set bit (undefined for `b == 0`).
#[inline]
pub const fn lsb(b: u64) -> u8 {
    b.trailing_zeros() as u8
}

/// Removes and returns the index of the least-significant set bit.
#[inline]
pub fn pop_lsb(b: &mut u64) -> u8 {
    let s = lsb(*b);
    *b &= *b - 1;
    s
}

/// Single-bit bitboard for a square index.
#[inline]
pub const fn bit(sq: u8) -> u64 {
    1u64 << sq
}

/// Shifts a bitboard one rank up (towards rank 8).
#[inline]
pub const fn shift_up(b: u64) -> u64 {
    b << 8
}

/// Shifts a bitboard one rank down (towards rank 1).
#[inline]
pub const fn shift_down(b: u64) -> u64 {
    b >> 8
}

/// Shifts a bitboard one file left (towards file a), excluding wrap-around.
#[inline]
pub const fn shift_left(b: u64) -> u64 {
    (b & !FILE_BB[0]) >> 1
}

/// Shifts a bitboard one file right (towards file h), excluding wrap-around.
#[inline]
pub const fn shift_right(b: u64) -> u64 {
    (b & !FILE_BB[7]) << 1
}

/// Squares attacked by a knight standing on each square.
pub const KNIGHT_ATT: [u64; 64] = {
    let mut t = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        let (f, r) = ((sq % 8) as i32, (sq / 8) as i32);
        let mut m = 0u64;
        let mut i = 0;
        while i < 8 {
            let (df, dr) = [
                (1, 2),
                (2, 1),
                (2, -1),
                (1, -2),
                (-1, -2),
                (-2, -1),
                (-2, 1),
                (-1, 2),
            ][i];
            let (nf, nr) = (f + df, r + dr);
            if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                m |= 1u64 << (nr * 8 + nf);
            }
            i += 1;
        }
        t[sq] = m;
        sq += 1;
    }
    t
};

/// Squares attacked by a king standing on each square.
pub const KING_ATT: [u64; 64] = {
    let mut t = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        let (f, r) = ((sq % 8) as i32, (sq / 8) as i32);
        let mut m = 0u64;
        let mut df = -1;
        while df <= 1 {
            let mut dr = -1;
            while dr <= 1 {
                if df != 0 || dr != 0 {
                    let (nf, nr) = (f + df, r + dr);
                    if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                        m |= 1u64 << (nr * 8 + nf);
                    }
                }
                dr += 1;
            }
            df += 1;
        }
        t[sq] = m;
        sq += 1;
    }
    t
};

/// Squares attacked by a pawn of `color` standing on each square.
pub const PAWN_ATT: [[u64; 64]; 2] = {
    let mut t = [[0u64; 64]; 2];
    let mut sq = 0;
    while sq < 64 {
        let (f, r) = ((sq % 8) as i32, (sq / 8) as i32);
        // White pawn attacks upwards
        if r < 7 {
            if f > 0 {
                t[Color::White as usize][sq] |= 1u64 << ((r + 1) * 8 + f - 1);
            }
            if f < 7 {
                t[Color::White as usize][sq] |= 1u64 << ((r + 1) * 8 + f + 1);
            }
        }
        // Black pawn attacks downwards
        if r > 0 {
            if f > 0 {
                t[Color::Black as usize][sq] |= 1u64 << ((r - 1) * 8 + f - 1);
            }
            if f < 7 {
                t[Color::Black as usize][sq] |= 1u64 << ((r - 1) * 8 + f + 1);
            }
        }
        sq += 1;
    }
    t
};

const fn signum(x: i32) -> i32 {
    (x > 0) as i32 - (x < 0) as i32
}

/// True when squares a and b share a rank, file or diagonal.
const fn aligned(fa: i32, ra: i32, fb: i32, rb: i32) -> bool {
    let df = fb - fa;
    let dr = rb - ra;
    df == 0 || dr == 0 || df.abs() == dr.abs()
}

/// `BETWEEN[a][b]`: squares strictly between aligned squares a and b
/// (empty if a and b are not aligned or adjacent).
pub static BETWEEN: [[u64; 64]; 64] = {
    let mut t = [[0u64; 64]; 64];
    let mut a = 0;
    while a < 64 {
        let (fa, ra) = ((a % 8) as i32, (a / 8) as i32);
        let mut b = 0;
        while b < 64 {
            let (fb, rb) = ((b % 8) as i32, (b / 8) as i32);
            if a != b && aligned(fa, ra, fb, rb) {
                let (sf, sr) = (signum(fb - fa), signum(rb - ra));
                let mut m = 0u64;
                let (mut f, mut r) = (fa + sf, ra + sr);
                while f != fb || r != rb {
                    m |= 1u64 << (r * 8 + f);
                    f += sf;
                    r += sr;
                }
                t[a][b] = m;
            }
            b += 1;
        }
        a += 1;
    }
    t
};

/// `LINE[a][b]`: every square on the rank/file/diagonal through aligned
/// squares a and b (zero when not aligned). Includes both endpoints.
pub static LINE: [[u64; 64]; 64] = {
    let mut t = [[0u64; 64]; 64];
    let mut a = 0;
    while a < 64 {
        let (fa, ra) = ((a % 8) as i32, (a / 8) as i32);
        let mut b = 0;
        while b < 64 {
            let (fb, rb) = ((b % 8) as i32, (b / 8) as i32);
            if a != b && aligned(fa, ra, fb, rb) {
                let (sf, sr) = (signum(fb - fa), signum(rb - ra));
                // walk from a through b to the edge
                let mut m = 0u64;
                let (mut f, mut r) = (fa, ra);
                while f >= 0 && f < 8 && r >= 0 && r < 8 {
                    m |= 1u64 << (r * 8 + f);
                    f += sf;
                    r += sr;
                }
                // walk from a in the opposite direction to the edge
                let (mut f, mut r) = (fa - sf, ra - sr);
                while f >= 0 && f < 8 && r >= 0 && r < 8 {
                    m |= 1u64 << (r * 8 + f);
                    f -= sf;
                    r -= sr;
                }
                t[a][b] = m;
            }
            b += 1;
        }
        a += 1;
    }
    t
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{RANK_BB, Square};

    #[test]
    fn popcount_and_lsb() {
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(0xFFFF), 16);
        assert_eq!(lsb(0x40), 6);
        let mut b = 0b1010u64;
        assert_eq!(pop_lsb(&mut b), 1);
        assert_eq!(pop_lsb(&mut b), 3);
        assert_eq!(b, 0);
    }

    #[test]
    fn shifts_do_not_wrap() {
        assert_eq!(shift_left(FILE_BB[0]), 0);
        assert_eq!(shift_right(FILE_BB[7]), 0);
        assert_eq!(shift_left(0xFF), 0x7F);
        // a1 + h1 shifted right: h1 wraps off, a1 becomes b1.
        assert_eq!(shift_right(0x81), 0x02);
        assert_eq!(shift_down(RANK_BB[0]), 0);
        assert_eq!(shift_up(RANK_BB[7]), 0);
    }

    #[test]
    fn knight_and_king_tables() {
        // Knight on d4: b3, b5, c2, c6, e2, e6, f3, f5.
        let expected: u64 = [17u64, 33, 10, 42, 12, 44, 21, 37]
            .iter()
            .map(|s| 1u64 << s)
            .sum();
        assert_eq!(KNIGHT_ATT[Square(27).index()], expected);
        assert_eq!(KNIGHT_ATT[Square(0).index()], (1u64 << 10) | (1u64 << 17));
        assert_eq!(KING_ATT[Square(0).index()], (1u64 << 1) | (1u64 << 8) | (1u64 << 9));
        assert_eq!(popcount(KING_ATT[28]), 8);
    }

    #[test]
    fn pawn_attack_table() {
        // White pawn on e4 attacks d5 and f5; black pawn attacks d3 and f3.
        let e4 = Square(28).index();
        assert_eq!(PAWN_ATT[Color::White as usize][e4], bit(35) | bit(37));
        assert_eq!(PAWN_ATT[Color::Black as usize][e4], bit(19) | bit(21));
        // Edge pawns attack only one square.
        assert_eq!(popcount(PAWN_ATT[Color::White as usize][0]), 1);
        assert_eq!(popcount(PAWN_ATT[Color::White as usize][7]), 1);
    }

    #[test]
    fn between_table() {
        let a1 = Square(0).index();
        let h8 = Square(63).index();
        let expected: u64 = (1..7).map(|i| 1u64 << (i * 8 + i)).sum();
        assert_eq!(BETWEEN[a1][h8], expected);
        assert_eq!(BETWEEN[a1][a1], 0);
        assert_eq!(BETWEEN[a1][7], 0x7E);
        // not aligned
        assert_eq!(BETWEEN[a1][Square(17).index()], 0);
    }

    #[test]
    fn line_table() {
        let a1 = Square(0).index();
        let h8 = Square(63).index();
        let diag: u64 = (0..8).map(|i| 1u64 << (i * 8 + i)).sum();
        assert_eq!(LINE[a1][h8], diag);
        // a1 and a8 share the a-file.
        assert_eq!(LINE[a1][Square(56).index()], FILE_BB[0]);
        // Not aligned at all.
        assert_eq!(LINE[a1][Square(17).index()], 0);
        assert_eq!(LINE[a1][Square(3).index()], RANK_BB[0]);
    }
}
