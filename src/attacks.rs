// Sliding attack generation.
//
// Two interchangeable indexing schemes share one table layout:
//  - Fancy Magic (default, portable): index = ((occ & mask) * magic) >> shift
//  - Hardware PEXT (feature = "pext", x86-64 BMI2): index = pext(occ, mask)
//
// Tables are built lazily exactly once. The `pext` feature additionally
// verifies BMI2 support at runtime and transparently falls back to the
// cache-compact Fancy Magic path on unsupported CPUs (ARM / Apple Silicon).
//
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use crate::bitboard::popcount;

pub use crate::bitboard::{BETWEEN, LINE};

const ROOK_DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

/// Per-square fancy-magic description plus its slice of the attack array.
pub struct SliderTable {
    /// Occupancy bits that affect the attack set.
    pub mask: [u64; 64],
    /// Magic multiplier (unused in PEXT mode).
    pub magic: [u64; 64],
    /// Right-shift applied to the product (`64 - bits`).
    pub shift: [u8; 64],
    /// Offset of this square's block inside `attacks`.
    pub base: [u32; 64],
    /// Concatenated attack sets for all relevant occupancies.
    pub attacks: Vec<u64>,
}

/// All precomputed attack data.
pub struct Tables {
    pub rook: SliderTable,
    pub bishop: SliderTable,
    /// True when PEXT indexing is used (BMI2 hardware + `pext` feature).
    pub use_pext: bool,
}

static TABLES: OnceLock<Tables> = OnceLock::new();

/// Returns the process-wide attack tables, initializing them on first use.
#[inline]
pub fn tables() -> &'static Tables {
    TABLES.get_or_init(init_tables)
}

#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn pext(x: u64, mask: u64) -> u64 {
    std::arch::x86_64::_pext_u64(x, mask)
}

#[inline]
fn pext_available() -> bool {
    #[cfg(all(target_arch = "x86_64", feature = "pext"))]
    {
        std::arch::is_x86_feature_detected!("bmi2")
    }
    #[cfg(not(all(target_arch = "x86_64", feature = "pext")))]
    {
        false
    }
}

#[inline(always)]
pub fn rook_attacks(sq: u8, occ: u64) -> u64 {
    let t = tables();
    let s = sq as usize;
    #[cfg(all(target_arch = "x86_64", feature = "pext"))]
    {
        if t.use_pext {
            // SAFETY: only reached when `is_x86_feature_detected!("bmi2")`.
            let idx = unsafe { pext(occ, t.rook.mask[s]) as usize };
            return unsafe { *t.rook.attacks.get_unchecked(t.rook.base[s] as usize + idx) };
        }
    }
    let idx = ((occ & t.rook.mask[s]).wrapping_mul(t.rook.magic[s]) >> t.rook.shift[s]) as usize;
    unsafe { *t.rook.attacks.get_unchecked(t.rook.base[s] as usize + idx) }
}

#[inline(always)]
pub fn bishop_attacks(sq: u8, occ: u64) -> u64 {
    let t = tables();
    let s = sq as usize;
    #[cfg(all(target_arch = "x86_64", feature = "pext"))]
    {
        if t.use_pext {
            let idx = unsafe { pext(occ, t.bishop.mask[s]) as usize };
            return unsafe { *t.bishop.attacks.get_unchecked(t.bishop.base[s] as usize + idx) };
        }
    }
    let idx = ((occ & t.bishop.mask[s]).wrapping_mul(t.bishop.magic[s]) >> t.bishop.shift[s]) as usize;
    unsafe { *t.bishop.attacks.get_unchecked(t.bishop.base[s] as usize + idx) }
}

/// Queen attack set (rook | bishop) for `occ` occupancy from `sq`.
#[inline]
pub fn queen_attacks(sq: u8, occ: u64) -> u64 {
    rook_attacks(sq, occ) | bishop_attacks(sq, occ)
}

/// Slow reference ray-walk attack (used to build tables and by tests).
fn slow_attacks(sq: u8, occ: u64, dirs: &[(i32, i32); 4]) -> u64 {
    let (f, r) = ((sq % 8) as i32, (sq / 8) as i32);
    let mut out = 0u64;
    for &(df, dr) in dirs {
        let (mut nf, mut nr) = (f + df, r + dr);
        while (0..8).contains(&nf) && (0..8).contains(&nr) {
            let s = (nr * 8 + nf) as u8;
            out |= 1u64 << s;
            if occ & (1u64 << s) != 0 {
                break;
            }
            nf += df;
            nr += dr;
        }
    }
    out
}

/// Relevant occupancy mask: blocker squares whose presence can truncate the
/// ray, excluding each ray's final edge square.
fn relevant_mask(sq: u8, dirs: &[(i32, i32); 4]) -> u64 {
    let (f, r) = ((sq % 8) as i32, (sq / 8) as i32);
    let mut out = 0u64;
    for &(df, dr) in dirs {
        let (mut nf, mut nr) = (f + df, r + dr);
        while (0..8).contains(&nf) && (0..8).contains(&nr) {
            let s = 1u64 << (nr * 8 + nf);
            nf += df;
            nr += dr;
            if !(0..8).contains(&nf) || !(0..8).contains(&nr) {
                break; // last square on this ray: irrelevant for occupancy
            }
            out |= s;
        }
    }
    out
}

/// Deterministic SplitMix64 for magic-number search (fixed seed: reproducible
/// tables across runs and platforms).
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Sparse candidate magic (few high bits → better mapping spread).
    fn sparse(&mut self) -> u64 {
        self.next() & self.next() & self.next()
    }
}

fn build_slider_table(dirs: &[(i32, i32); 4], use_pext: bool) -> SliderTable {
    let mut t = SliderTable {
        mask: [0; 64],
        magic: [0; 64],
        shift: [0; 64],
        base: [0; 64],
        attacks: Vec::new(),
    };
    // Lay out per-square blocks sized 2^popcount(mask).
    let mut total = 0u32;
    for sq in 0..64u8 {
        let mask = relevant_mask(sq, dirs);
        let bits = popcount(mask);
        t.mask[sq as usize] = mask;
        t.shift[sq as usize] = (64 - bits) as u8;
        t.base[sq as usize] = total;
        total += 1u32 << bits;
    }
    t.attacks = vec![0u64; total as usize];

    for sq in 0..64u8 {
        let s = sq as usize;
        let mask = t.mask[s];
        let bits = popcount(mask);

        // Reference attack sets for every subset of the mask.
        let size = 1usize << bits;
        let mut subsets = Vec::with_capacity(size);
        let mut occ = 0u64;
        loop {
            subsets.push((occ, slow_attacks(sq, occ, dirs)));
            occ = occ.wrapping_sub(mask) & mask;
            if occ == 0 {
                break;
            }
        }

        // PEXT needs no magic multiplier: the mask itself is the index.
        let magic = if use_pext {
            0
        } else {
            find_magic(sq, mask, bits, dirs)
        };
        t.magic[s] = magic;

        for &(occ, att) in &subsets {
            let idx = if use_pext {
                #[cfg(target_arch = "x86_64")]
                {
                    // SAFETY: pext_available() was checked in init_tables.
                    unsafe { pext(occ, mask) as usize }
                }
                #[cfg(not(target_arch = "x86_64"))]
                {
                    let _ = mask;
                    unreachable!()
                }
            } else {
                (occ.wrapping_mul(magic) >> (64 - bits)) as usize
            };
            let slot = t.base[s] as usize + idx;
            debug_assert!(
                t.attacks[slot] == 0 || t.attacks[slot] == att,
                "magic index collision with a different attack set"
            );
            t.attacks[slot] = att;
        }
    }
    t
}

/// Searches for a fancy-magic multiplier with a fixed-seed random search.
fn find_magic(sq: u8, mask: u64, bits: u32, dirs: &[(i32, i32); 4]) -> u64 {
    let size = 1usize << bits;
    let mut occ = 0u64;
    let mut subsets = Vec::with_capacity(size);
    loop {
        subsets.push((occ, slow_attacks(sq, occ, dirs)));
        occ = occ.wrapping_sub(mask) & mask;
        if occ == 0 {
            break;
        }
    }

    let mut rng = SplitMix64(0x0DDB_1A5E_5BAD_5EED ^ (sq as u64).wrapping_mul(0x1234_5678_9ABC_DEF1));
    let mut slot_att = vec![0u64; size];
    let mut slot_used = vec![false; size];
    loop {
        let magic = rng.sparse();
        if magic.count_ones() < 6 {
            continue; // too sparse to spread indices
        }
        for u in slot_used.iter_mut() {
            *u = false;
        }
        let mut ok = true;
        for &(occ, att) in &subsets {
            let idx = (occ.wrapping_mul(magic) >> (64 - bits)) as usize;
            if slot_used[idx] {
                if slot_att[idx] != att {
                    ok = false;
                    break;
                }
            } else {
                slot_used[idx] = true;
                slot_att[idx] = att;
            }
        }
        if ok {
            return magic;
        }
    }
}

fn init_tables() -> Tables {
    let use_pext = pext_available();
    let rook = build_slider_table(&ROOK_DIRS, use_pext);
    let bishop = build_slider_table(&BISHOP_DIRS, use_pext);
    Tables { rook, bishop, use_pext }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::bit;

    fn xorshift(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    #[test]
    fn slider_tables_match_ray_walk_reference() {
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        for _ in 0..2000 {
            let occ = xorshift(&mut state) & xorshift(&mut state);
            let sq = (xorshift(&mut state) % 64) as u8;
            assert_eq!(
                rook_attacks(sq, occ),
                slow_attacks(sq, occ, &ROOK_DIRS),
                "rook mismatch sq={} occ={:#x}",
                sq,
                occ
            );
            assert_eq!(
                bishop_attacks(sq, occ),
                slow_attacks(sq, occ, &BISHOP_DIRS),
                "bishop mismatch sq={} occ={:#x}",
                sq,
                occ
            );
        }
        // Empty board: rook on d4 reaches 14 squares (rank+file), bishop 13.
        let d4 = 27u8;
        assert_eq!(popcount(rook_attacks(d4, 0)), 14);
        assert_eq!(popcount(bishop_attacks(d4, 0)), 13);
        // Fully blocked: only the four adjacent capturable squares remain.
        assert_eq!(
            rook_attacks(d4, u64::MAX),
            bit(28) | bit(26) | bit(35) | bit(19)
        );
        assert_eq!(
            bishop_attacks(d4, u64::MAX),
            bit(18) | bit(20) | bit(34) | bit(36)
        );
        // Queen = rook | bishop.
        assert_eq!(queen_attacks(d4, 0), rook_attacks(d4, 0) | bishop_attacks(d4, 0));
    }

    #[test]
    fn table_sizes_are_cache_compact() {
        let t = tables();
        let rook_entries: u32 = (0..64).map(|sq| 1u32 << popcount(t.rook.mask[sq])).sum();
        assert_eq!(rook_entries as usize, t.rook.attacks.len());
        assert_eq!(rook_entries, 102_400);
        let bishop_entries: u32 = (0..64).map(|sq| 1u32 << popcount(t.bishop.mask[sq])).sum();
        assert_eq!(bishop_entries as usize, t.bishop.attacks.len());
        assert_eq!(bishop_entries, 5_248);
    }

    #[test]
    fn relevant_masks_exclude_ray_edges() {
        // Corner rook: 6 squares per ray, edge squares excluded.
        assert_eq!(popcount(relevant_mask(0, &ROOK_DIRS)), 12);
        // Center rook d4: 3+2 (rank, h4/a4 excluded) + 3+2 (file, d8/d1 excluded).
        assert_eq!(popcount(relevant_mask(27, &ROOK_DIRS)), 10);
        // Corner bishop: two diagonals minus the two edge squares.
        assert_eq!(popcount(relevant_mask(0, &BISHOP_DIRS)), 6);
        // Center bishop d4: 3+2+2+2.
        assert_eq!(popcount(relevant_mask(27, &BISHOP_DIRS)), 9);
    }
}

