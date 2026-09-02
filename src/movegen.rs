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
        // Bulk copy via `copy_nonoverlapping` (40 B for startpos) vs per-element loop.
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.buf.as_ptr() as *const Move,
                out.as_mut_ptr(),
                self.len,
            );
            out.set_len(self.len);
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

impl MoveSink for arrayvec::ArrayVec<Move, MAX_MOVES> {
    #[inline]
    fn push_targets(&mut self, from: u8, mut targets: u64) {
        while targets != 0 {
            let to = pop_lsb(&mut targets);
            unsafe { self.push_unchecked(Move::new(Square(from), Square(to), None)) };
        }
    }

    #[inline]
    fn push_pawn_targets_offset(&mut self, mut targets: u64, offset: i8) {
        while targets != 0 {
            let to = pop_lsb(&mut targets);
            let from = (to as i16 - offset as i16) as u8;
            unsafe { self.push_unchecked(Move::new(Square(from), Square(to), None)) };
        }
    }

    #[inline]
    fn push_pawn_promotions_offset(&mut self, mut targets: u64, offset: i8) {
        while targets != 0 {
            let to = pop_lsb(&mut targets);
            let from = (to as i16 - offset as i16) as u8;
            for r in [Role::Queen, Role::Rook, Role::Bishop, Role::Knight] {
                unsafe { self.push_unchecked(Move::new(Square(from), Square(to), Some(r))) };
            }
        }
    }

    #[inline]
    fn push_one(&mut self, mv: Move) {
        unsafe { self.push_unchecked(mv) };
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

/// Visitor sink: processes generated moves **without materialising `Move`
/// values** (Gigantua's visitor pattern — "make/unmake and a movelist is not
/// needed and 2× slower"; D1, studied clean-room, MIT-clean per D5).
///
/// Whole bitboards reach the visitor so a counting visitor pays only
/// `popcount` — no `pop_lsb`, no `Move::new`, no stack buffer writes.
/// Shares the exact generator body with [`MoveSink`] via
/// [`VisitorAdapter`] (the monomorphiser duplicates the code per sink,
/// same effect as macro-sharing, `LTO=fat` mitigates bloat).
pub trait MoveVisitor {
    /// Visit all `targets` from `from` (one square → bitboard of destinations).
    fn visit_targets(&mut self, from: u8, targets: u64);
    /// Bulk-pawn push: `targets` are destination squares, `offset = to - from`
    /// (white +8/−8 black, captures ±7/±9, double push ±16). One move per target.
    fn visit_pawn_offset(&mut self, targets: u64, offset: i8);
    /// Bulk-pawn promotion: each target yields 4 moves (N,B,R,Q) with same offset.
    fn visit_promotion_offset(&mut self, targets: u64, offset: i8);
    /// Visit a single fully-formed move (king, castling, en-passant, pinned pawn).
    fn visit_one(&mut self, mv: Move);
}

/// Forwards [`MoveSink`] calls into any [`MoveVisitor`] — this is how
/// `Board::generate_visitor` shares the `generate_moves_into` body
/// (including `compute_pinned_split` and bulk pawn shifts) verbatim.
pub struct VisitorAdapter<V: MoveVisitor>(pub V);

impl<V: MoveVisitor> VisitorAdapter<V> {
    #[inline]
    pub fn new(visitor: V) -> Self {
        Self(visitor)
    }
}

impl<V: MoveVisitor> MoveSink for VisitorAdapter<V> {
    #[inline]
    fn push_targets(&mut self, from: u8, targets: u64) {
        self.0.visit_targets(from, targets);
    }
    #[inline]
    fn push_pawn_targets_offset(&mut self, targets: u64, offset: i8) {
        self.0.visit_pawn_offset(targets, offset);
    }
    #[inline]
    fn push_pawn_promotions_offset(&mut self, targets: u64, offset: i8) {
        self.0.visit_promotion_offset(targets, offset);
    }
    #[inline]
    fn push_one(&mut self, mv: Move) {
        self.0.visit_one(mv);
    }
}

/// Forwarding impl so `&mut S` (the generator's sink parameter shape) is
/// itself a `MoveSink` — lets callers pass `&mut adapter` without an extra
/// deref layer (also handy for `&mut MoveList` style plumbing).
impl<T: MoveSink + ?Sized> MoveSink for &mut T {
    #[inline]
    fn push_targets(&mut self, from: u8, targets: u64) {
        (**self).push_targets(from, targets);
    }
    #[inline]
    fn push_pawn_targets_offset(&mut self, targets: u64, offset: i8) {
        (**self).push_pawn_targets_offset(targets, offset);
    }
    #[inline]
    fn push_pawn_promotions_offset(&mut self, targets: u64, offset: i8) {
        (**self).push_pawn_promotions_offset(targets, offset);
    }
    #[inline]
    fn push_one(&mut self, mv: Move) {
        (**self).push_one(mv);
    }
}

/// Leaf-counting visitor for `perft depth==1` / `count_legal_moves` —
/// `count += popcount` **without `Move` materialisation or `pop_lsb`**
/// (goes one step further than [`MoveCounter`], which is already
/// `popcount`-only but pays the `MoveSink` indirection; D1).
#[derive(Default, Debug, Clone, Copy)]
pub struct CountingVisitor {
    pub count: u32,
}

impl CountingVisitor {
    #[inline]
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl MoveVisitor for CountingVisitor {
    #[inline]
    fn visit_targets(&mut self, _from: u8, targets: u64) {
        self.count += popcount(targets);
    }

    #[inline]
    fn visit_pawn_offset(&mut self, targets: u64, _offset: i8) {
        self.count += popcount(targets);
    }

    #[inline]
    fn visit_promotion_offset(&mut self, targets: u64, _offset: i8) {
        self.count += popcount(targets) * 4;
    }

    #[inline]
    fn visit_one(&mut self, _mv: Move) {
        self.count += 1;
    }
}

/// Forwarding impl so `&mut V` is itself a `MoveVisitor` — mirrors the
/// `&mut T: MoveSink` impl above and lets `generate_visitor(&self, visitor:
/// &mut V)` wrap the borrowed visitor in an adapter.
impl<V: MoveVisitor + ?Sized> MoveVisitor for &mut V {
    #[inline]
    fn visit_targets(&mut self, from: u8, targets: u64) {
        (**self).visit_targets(from, targets);
    }
    #[inline]
    fn visit_pawn_offset(&mut self, targets: u64, offset: i8) {
        (**self).visit_pawn_offset(targets, offset);
    }
    #[inline]
    fn visit_promotion_offset(&mut self, targets: u64, offset: i8) {
        (**self).visit_promotion_offset(targets, offset);
    }
    #[inline]
    fn visit_one(&mut self, mv: Move) {
        (**self).visit_one(mv);
    }
}

/// `MoveVisitor` for `MoveList`: materialises every move (delegates to the
/// [`MoveSink`] impl) — lets visitor-generic callers collect when needed.
impl MoveVisitor for MoveList {
    #[inline]
    fn visit_targets(&mut self, from: u8, targets: u64) {
        MoveSink::push_targets(self, from, targets);
    }
    #[inline]
    fn visit_pawn_offset(&mut self, targets: u64, offset: i8) {
        MoveSink::push_pawn_targets_offset(self, targets, offset);
    }
    #[inline]
    fn visit_promotion_offset(&mut self, targets: u64, offset: i8) {
        MoveSink::push_pawn_promotions_offset(self, targets, offset);
    }
    #[inline]
    fn visit_one(&mut self, mv: Move) {
        MoveSink::push_one(self, mv);
    }
}

/// `MoveVisitor` for `MoveCounter`: `count += popcount` (parity with its
/// [`MoveSink`] impl — baseline for the visitor leaf benchmark).
impl MoveVisitor for MoveCounter {
    #[inline]
    fn visit_targets(&mut self, _from: u8, targets: u64) {
        self.count += popcount(targets);
    }
    #[inline]
    fn visit_pawn_offset(&mut self, targets: u64, _offset: i8) {
        self.count += popcount(targets);
    }
    #[inline]
    fn visit_promotion_offset(&mut self, targets: u64, _offset: i8) {
        self.count += popcount(targets) * 4;
    }
    #[inline]
    fn visit_one(&mut self, _mv: Move) {
        self.count += 1;
    }
}

/// Computes `pinned_hv` / `pinned_diag` (ultrachess `compute_pinned_split`).
///
/// Returns `(pinned_hv, pinned_diag)`. Uses `their_occ` (enemy occupancy)
/// for sniper ray to match `ultrachess` `bishop_attacks(king, their_pieces)`
/// — fewer snipers than `occ=0` (all diagonals) and matches the `their_pieces`
/// blocking semantics. Callers use `LINE[ksq][from]` for pinned-slider masking.
#[inline(always)]
pub fn compute_pinned_split(
    ksq: u8,
    occ: u64,
    their_bq: u64,
    their_rq: u64,
    own_occ: u64,
    their_occ: u64,
) -> (u64, u64) {
    use crate::bitboard::BETWEEN;
    use crate::attacks;

    let mut pinned_diag = 0u64;
    let mut pinned_hv = 0u64;

    // Diagonal snipers — enemy occupancy blocks rays (like ultrachess).
    let mut diag_snipers = attacks::bishop_attacks(ksq, their_occ) & their_bq;
    while diag_snipers != 0 {
        let sniper = pop_lsb(&mut diag_snipers);
        let between = BETWEEN[ksq as usize][sniper as usize] & occ;
        if popcount(between) == 1 && between & own_occ != 0 {
            let pinned_sq = lsb(between);
            pinned_diag |= bit(pinned_sq);
        }
    }

    // Orthogonal snipers — enemy occupancy blocks rays.
    let mut orth_snipers = attacks::rook_attacks(ksq, their_occ) & their_rq;
    while orth_snipers != 0 {
        let sniper = pop_lsb(&mut orth_snipers);
        let between = BETWEEN[ksq as usize][sniper as usize] & occ;
        if popcount(between) == 1 && between & own_occ != 0 {
            let pinned_sq = lsb(between);
            pinned_hv |= bit(pinned_sq);
        }
    }

    (pinned_hv, pinned_diag)
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
        let their_occ = 0xFFFF_0000_0000_0000u64;
        let (hv, diag) = compute_pinned_split(ksq, occ, their_bq, their_rq, own, their_occ);
        assert_eq!(hv, 0);
        assert_eq!(diag, 0);
    }
}
