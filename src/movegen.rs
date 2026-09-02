// MoveSink bulk abstraction + split pin helpers (D2).
//
// Copies `ultrachess/rust/core/src/movegen.rs:31` bulk pawn and pinned-split
// technique with MIT attribution: `MoveSink::{push_targets,push_pawn_targets_offset,
// push_pawn_promotions_offset,push_one}` with `MoveList {buf:[MaybeUninit<Move>;256]}`
// (512B, skip memset) and `MoveCounter {count}` (`count+=popcount` without `pop_lsb`).
// Also adds `compute_pinned_split → (pinned_hv,pinned_diag)` to avoid per-slider
// `tables::line()` dependent load (`ultrachess/movegen.rs:214`).
//
// SPDX-License-Identifier: MIT
// Attribution: technique copied from ultrachess (MIT) with adaptation to u64
// Chess960 board (plain-data Copy + castle_rook_sq).

use core::mem::MaybeUninit;

use crate::bitboard::{bit, lsb, pop_lsb, popcount};
use crate::moves::Move;
use crate::types::{Role, Square};

/// Maximum legal moves in any position (hard bound for stack buffer).
pub const MAX_MOVES: usize = 256;

/// Generic sink for generated moves — materialising (`MoveList`) or counting
/// (`MoveCounter`) without branching on the hot path (monomorphised ×2, LTO fat).
pub trait MoveSink {
    /// Push all `targets` from `from` (one square → bitboard of destinations).
    fn push_targets(&mut self, from: u8, targets: u64);
    /// Bulk-pawn push: `targets` are destination squares, `offset = to - from`
    /// (white +8/+7/+9/+16, black −8/−7/−9/−16). Generates one move per target.
    fn push_pawn_targets_offset(&mut self, targets: u64, offset: i8);
    /// Bulk-pawn promotion: each target yields 4 moves (N,B,R,Q) with same offset.
    fn push_pawn_promotions_offset(&mut self, targets: u64, offset: i8);
    /// Push a single fully-formed move (king, castling, en-passant, pinned pawn).
    fn push_one(&mut self, mv: Move);
}

/// Stack buffer for materialised movegen: `MaybeUninit<Move,256>` avoids the
/// `memset` that `ArrayVec::new()` would otherwise pay (512B, plain data).
pub struct MoveList {
    pub buf: [MaybeUninit<Move>; MAX_MOVES],
    pub len: usize,
}

impl MoveList {
    #[inline]
    pub fn new() -> Self {
        Self {
            // SAFETY: array of MaybeUninit need not be initialized.
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Drains the buffer into an `ArrayVec` for the public `legal_moves()` API.
    #[inline]
    pub fn into_arrayvec(self) -> arrayvec::ArrayVec<Move, MAX_MOVES> {
        let mut out = arrayvec::ArrayVec::new();
        for i in 0..self.len {
            // SAFETY: first `len` entries are initialized.
            unsafe { out.push_unchecked(*self.buf[i].assume_init_ref()) };
        }
        out
    }

    #[inline]
    pub fn as_slice(&self) -> &[Move] {
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr() as *const Move, self.len) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl MoveSink for MoveList {
    #[inline]
    fn push_targets(&mut self, from: u8, mut targets: u64) {
        while targets != 0 {
            let to = pop_lsb(&mut targets);
            debug_assert!(self.len < MAX_MOVES);
            self.buf[self.len].write(Move::new(Square(from), Square(to), None));
            self.len += 1;
        }
    }

    #[inline]
    fn push_pawn_targets_offset(&mut self, mut targets: u64, offset: i8) {
        while targets != 0 {
            let to = pop_lsb(&mut targets);
            let from = (to as i16 - offset as i16) as u8;
            debug_assert!(self.len < MAX_MOVES);
            self.buf[self.len].write(Move::new(Square(from), Square(to), None));
            self.len += 1;
        }
    }

    #[inline]
    fn push_pawn_promotions_offset(&mut self, mut targets: u64, offset: i8) {
        while targets != 0 {
            let to = pop_lsb(&mut targets);
            let from = (to as i16 - offset as i16) as u8;
            for r in [Role::Queen, Role::Rook, Role::Bishop, Role::Knight] {
                debug_assert!(self.len < MAX_MOVES);
                self.buf[self.len].write(Move::new(Square(from), Square(to), Some(r)));
                self.len += 1;
            }
        }
    }

    #[inline]
    fn push_one(&mut self, mv: Move) {
        debug_assert!(self.len < MAX_MOVES);
        self.buf[self.len].write(mv);
        self.len += 1;
    }
}

/// Counting sink for `perft depth==1` and `count_legal_moves()` —
// `count+=popcount` without `pop_lsb` (the `geomean 1.23× vs cozy` win,
// `BENCH.md: caveat 6`).
#[derive(Default, Debug, Clone, Copy)]
pub struct MoveCounter {
    pub count: u32,
}

impl MoveCounter {
    #[inline]
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl MoveSink for MoveCounter {
    #[inline]
    fn push_targets(&mut self, _from: u8, targets: u64) {
        self.count += popcount(targets);
    }

    #[inline]
    fn push_pawn_targets_offset(&mut self, targets: u64, _offset: i8) {
        self.count += popcount(targets);
    }

    #[inline]
    fn push_pawn_promotions_offset(&mut self, targets: u64, _offset: i8) {
        self.count += popcount(targets) * 4;
    }

    #[inline]
    fn push_one(&mut self, _mv: Move) {
        self.count += 1;
    }
}

/// Result of the split-pinned computation.
#[derive(Copy, Clone, Debug)]
pub struct PinnedSplit {
    pub hv: u64,
    pub diag: u64,
    pub line: [u64; 64],
}

impl Default for PinnedSplit {
    fn default() -> Self {
        Self {
            hv: 0,
            diag: 0,
            line: [0u64; 64],
        }
    }
}

/// Computes `pinned_hv` / `pinned_diag` plus the per-pinned-square `LINE`
/// to avoid a dependent `LINE` load for unpinned sliders (ultrachess
/// `compute_pinned_split` / `pinned_split`).
///
/// `ksq` is the king square of the side to move; `occ` is total occupancy;
/// `their_bq` / `their_rq` are enemy bishop/queen and rook/queen bitboards;
/// `own_occ` is the mover's occupancy.
#[inline]
pub fn compute_pinned_split(
    ksq: u8,
    occ: u64,
    their_bq: u64,
    their_rq: u64,
    own_occ: u64,
) -> PinnedSplit {
    use crate::bitboard::{BETWEEN, LINE};
    use crate::attacks;

    let mut out = PinnedSplit::default();

    // Diagonal snipers (bishop/queen on the king's diagonals).
    let mut diag_snipers = attacks::bishop_attacks(ksq, 0) & their_bq;
    while diag_snipers != 0 {
        let sniper = pop_lsb(&mut diag_snipers);
        let between = BETWEEN[ksq as usize][sniper as usize] & occ;
        if popcount(between) == 1 && between & own_occ != 0 {
            let pinned_sq = lsb(between);
            out.diag |= bit(pinned_sq);
            out.line[pinned_sq as usize] = LINE[ksq as usize][sniper as usize];
        }
    }

    // Orthogonal snipers (rook/queen on rank/file).
    let mut orth_snipers = attacks::rook_attacks(ksq, 0) & their_rq;
    while orth_snipers != 0 {
        let sniper = pop_lsb(&mut orth_snipers);
        let between = BETWEEN[ksq as usize][sniper as usize] & occ;
        if popcount(between) == 1 && between & own_occ != 0 {
            let pinned_sq = lsb(between);
            out.hv |= bit(pinned_sq);
            out.line[pinned_sq as usize] = LINE[ksq as usize][sniper as usize];
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movelist_push_targets_roundtrip() {
        let mut list = MoveList::new();
        list.push_targets(10, (1u64 << 20) | (1u64 << 21));
        assert_eq!(list.len(), 2);
        assert_eq!(list.as_slice()[0].from().0, 10);
        assert_eq!(list.as_slice()[1].from().0, 10);
    }

    #[test]
    fn movecounter_popcount() {
        let mut c = MoveCounter::new();
        c.push_targets(0, 0b1011);
        assert_eq!(c.count, 3);
        c.push_pawn_promotions_offset(0b11, 8);
        assert_eq!(c.count, 3 + 8);
    }

    #[test]
    fn pinned_split_no_pinned_startpos() {
        // Startpos has no pinned pieces.
        let ksq = 4u8; // e1
        let occ = 0xFFFF_0000_0000_FFFFu64;
        let their_bq = 0x2C00_0000_0000_0000u64;
        let their_rq = 0x8100_0000_0000_0081u64;
        let own = 0x0000_0000_0000_FFFFu64;
        let p = compute_pinned_split(ksq, occ, their_bq, their_rq, own);
        assert_eq!(p.hv, 0);
        assert_eq!(p.diag, 0);
    }
}
