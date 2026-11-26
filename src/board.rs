// Board representation, make/unmake move, and fully-legal move generation
// with check and pin masking (zero heap allocations in the movegen loop).
//
// Board layout: 12 piece bitboards + occupancy + mailbox for O(1) piece
// lookup. The zobrist hash is maintained incrementally in `make_move` and
// restored on `unmake`.
//
// SPDX-License-Identifier: MIT

use crate::attacks;
use crate::bitboard::{bit, lsb, pop_lsb, popcount, BETWEEN, KING_ATT, KNIGHT_ATT, PAWN_ATT};
use crate::moves::Move;
use crate::movegen::{MoveCounter, MoveSink};
#[cfg(not(feature="exp-fused-geometry"))]
use crate::movegen::compute_pinned_split;
use crate::types::{Color, Piece, Role, Square, NO_EP};
use crate::zobrist;
use arrayvec::ArrayVec;

/// Maximum legal moves in any position (hard bound for the stack buffer).
pub const MAX_MOVES: usize = 256;

/// Mailbox sentinel: empty square.
const EMPTY: u8 = u8::MAX;

/// Precomputed Chess960 castling path clearance bitmasks on rank 0.
/// Indexed by `[king_file][rook_file]`. To test clearance on `rank`:
/// `(self.occupied() & (CASTLE_PATH[kf][rf] << (rank * 8))) == 0`.
pub const CASTLE_PATH: [[u64; 8]; 8] = compute_castle_path();

const fn compute_castle_path() -> [[u64; 8]; 8] {
    let mut table = [[0u64; 8]; 8];
    let mut kf = 0usize;
    while kf < 8 {
        let mut rf = 0usize;
        while rf < 8 {
            if kf != rf {
                let kingside = rf > kf;
                let target_k = if kingside { 6usize } else { 2usize };
                let target_r = if kingside { 5usize } else { 3usize };

                let mut mask = 0u64;

                // Between king and rook
                let (min_kr, max_kr) = if kf < rf { (kf, rf) } else { (rf, kf) };
                let mut f = min_kr + 1;
                while f < max_kr {
                    mask |= 1u64 << f;
                    f += 1;
                }

                // Between king and target_k
                let (min_kt, max_kt) = if kf < target_k { (kf, target_k) } else { (target_k, kf) };
                let mut f = min_kt + 1;
                while f < max_kt {
                    mask |= 1u64 << f;
                    f += 1;
                }

                // Between rook and target_r
                let (min_rt, max_rt) = if rf < target_r { (rf, target_r) } else { (target_r, rf) };
                let mut f = min_rt + 1;
                while f < max_rt {
                    mask |= 1u64 << f;
                    f += 1;
                }

                // Target squares
                mask |= 1u64 << target_k;
                mask |= 1u64 << target_r;

                // Exclude starting king and rook squares
                mask &= !(1u64 << kf);
                mask &= !(1u64 << rf);

                table[kf][rf] = mask;
            }
            rf += 1;
        }
        kf += 1;
    }
    table
}

/// Information required to undo a move.
///
/// `prev_checkers` caches the `checkers` bitboard before the move so
/// `in_check()` is `checkers != 0` without an `attackers_to` scan (0.32ns,
/// D3). `hash` is the previous Polyglot zobrist key (`prev_zobrist`) for
/// O(1) `hash()` load (0.34ns). The layout mirrors `ultrachess/position.rs:42`
/// (`Undo { prev_checkers, prev_zobrist, ... }`) with MIT attribution.
#[derive(Copy, Clone, Debug)]
pub struct Undo {
    hash: u64,
    /// Bitboard of checkers before the move (attackers to the mover's king).
    /// Restored on `unmake` so `in_check()==checkers!=0` never recomputes.
    checkers: u64,
    castling: u8,
    ep: u8,
    halfmove: u16,
    captured: u8,
    #[cfg(feature = "exp-undo-role")]
    moved: u8,
    /// True when the move was a castling move (king-from → rook-square).
    castled: bool,
}

/// Position-specific trusted replay token. Does not replace the 16-bit
/// moves2 wire format. Keep only alongside the position/game it came from.
#[cfg(feature="exp-prepared-move")]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct PreparedMove {mv:Move,meta:u16}

#[cfg(feature="exp-prepared-move")]
impl PreparedMove{
    #[inline(always)]
    pub fn wire_move(self)->Move{self.mv}
}

/// Error returned when a move cannot be played.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct IllegalMove;

impl core::fmt::Display for IllegalMove {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "illegal move")
    }
}
impl std::error::Error for IllegalMove {}

/// A chess position: pieces, side to move, castling rights, en-passant
/// square, clocks and the incrementally-maintained Polyglot zobrist hash
/// plus the cached `checkers` bitboard for branch-free `in_check()` (D3).
///
/// `Board` is plain data and [`Copy`]: copying it yields a bit-for-bit
/// snapshot suitable for engine search stacks (ADR-003, decision D6).
/// `checkers` is the set of enemy pieces giving check to the side to move
/// (`in_check()==checkers!=0` → 0.32ns, `ultrachess/position.rs:42` with MIT
/// attribution). `hash` is the incrementally-maintained Polyglot key (0.34ns
/// load). `history_hashes` is deliberately kept outside `Board` to preserve
/// `Copy` (core spec `Board SHALL Be Copy`); perft uses the slim path that
/// skips `zobrist`/`halfmove` maintenance (`position.rs:389` `Safe only for perft`).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(all(feature="exp-align16",not(feature="exp-align32")),repr(align(16)))]
#[cfg_attr(feature="exp-align32",repr(align(32)))]
pub struct Board {
    hash: u64,
    /// Cached checkers: enemy attackers to `king_sq[turn]`. `in_check()` is
    /// `checkers != 0` without scan (ultrachess D3, 0.32ns both states).
    checkers: u64,
    #[cfg(not(feature = "exp-compact-board"))]
    bbs: [[u64; 6]; 2],  // [color_index][role_index]
    #[cfg(all(feature = "exp-compact-board",not(feature="exp-kingless-board"),not(feature="exp-slider-planes")))]
    roles: [u64; 6], // role bitboards intersected with color occupancy
    #[cfg(all(feature="exp-compact-board",feature="exp-kingless-board",not(feature="exp-slider-planes")))]
    roles:[u64;5],
    // Overlapping diagonal and orthogonal slider planes: a queen is
    // the intersection, bishop/rook are the respective differences.
    #[cfg(all(feature="exp-slider-planes",not(feature="exp-slider-planes-kingless")))]
    sliders:[u64;5],
    #[cfg(feature="exp-slider-planes-kingless")]
    sliders:[u64;4],
    #[cfg(feature="exp-nibble-role")]
    role_map:[u8;32],
    #[cfg(feature="exp-role-planes")]
    role_planes:[u64;3],
    #[cfg(feature = "exp-mailbox")]
    mailbox: [u8; 64],
    occ: [u64; 2],       // occupancy per color
    king_sq: [u8; 2],
    /// Rook square backing each castling right (bit 0..3 = WK, WQ, BK, BQ).
    /// Chess960: arbitrary back-rank squares; standard: h1/a1/h8/a8.
    castle_rook_sq: [u8; 4],
    turn: Color,
    castling: u8,
    ep: u8,
    halfmove: u16,
    fullmove: u16,
    /// `true` if Chess960 (king not e1/e8 or rook not a/h) — early gate
    /// for `standard` `ultrachess` parity (`25ns` vs `78ns`).
    is_chess960: bool,
    // Close-gap D3 (task 4.1): `mailbox: [u8;64]` removed — `piece_at` scans
    // the 12 bitboards (`piece_code_at`); `castle_mask: [u8;64]` removed —
    // rights-clearing is derived from `castle_rook_sq[4]` + the mover's role
    // (`castle_rights_after`); `occupied` removed — `occ[0] | occ[1]` (1 OR).
    // Layout shrinks toward the 128B `Copy` target while keeping Chess960
    // `rook_sq[4]` and `hash`/`checkers` for SAN/search.
}

/// Board size in bytes (compact `Copy` layout, close-gap D3).
pub const BOARD_SIZE: usize = core::mem::size_of::<Board>();

impl Board {
    /// Empty board with White to move.
    pub fn empty() -> Board {
        Board {
            #[cfg(not(feature = "exp-compact-board"))]
            bbs: [[0; 6]; 2],
            #[cfg(all(feature = "exp-compact-board",not(feature="exp-kingless-board"),not(feature="exp-slider-planes")))]
            roles: [0;6],
            #[cfg(all(feature="exp-compact-board",feature="exp-kingless-board",not(feature="exp-slider-planes")))]
            roles:[0;5],
            #[cfg(all(feature="exp-slider-planes",not(feature="exp-slider-planes-kingless")))]
            sliders:[0;5],
            #[cfg(feature="exp-slider-planes-kingless")]
            sliders:[0;4],
            #[cfg(feature="exp-nibble-role")]
            role_map:[0xff;32],
            #[cfg(feature="exp-role-planes")]
            role_planes:[0;3],
            #[cfg(feature = "exp-mailbox")]
            mailbox: [EMPTY;64],
            occ: [0; 2],
            turn: Color::White,
            castling: 0,
            castle_rook_sq: [0; 4],
            ep: NO_EP,
            halfmove: 0,
            fullmove: 1,
            hash: 0,
            checkers: 0,
            king_sq: [0; 2],
            is_chess960: false,
        }
    }

    #[inline(always)]
    pub fn is_chess960(&self) -> bool { self.is_chess960 }

    /// The standard chess starting position.
    ///
    /// All state is a private immutable template copied by value. No FEN
    /// parsing, attack-table initialization or mutable shared state.
    #[inline]
    pub const fn startpos() -> Board {
        #[cfg(not(feature = "exp-compact-board"))]
        const BACK_RANK: [u64; 6] = [0xff00, 0x42, 0x24, 0x81, 0x08, 0x10];
        Board {
            hash: 0x463b_9618_1691_fc9c, // canonical Polyglot startpos vector
            checkers: 0,
            #[cfg(not(feature = "exp-compact-board"))]
            bbs: [
                [0xff << 48, 0x42 << 56, 0x24 << 56, 0x81 << 56, 0x08 << 56, 0x10 << 56],
                BACK_RANK,
            ],
            #[cfg(all(feature = "exp-compact-board",not(feature="exp-kingless-board"),not(feature="exp-slider-planes")))]
            roles: [0xff00 | (0xff << 48), 0x42 | (0x42 << 56), 0x24 | (0x24 << 56), 0x81 | (0x81 << 56), 0x08 | (0x08 << 56), 0x10 | (0x10 << 56)],
            #[cfg(all(feature="exp-compact-board",feature="exp-kingless-board",not(feature="exp-slider-planes")))]
            roles:[0xff00 | (0xff << 48), 0x42 | (0x42 << 56), 0x24 | (0x24 << 56), 0x81 | (0x81 << 56), 0x08 | (0x08 << 56)],
            #[cfg(all(feature="exp-slider-planes",not(feature="exp-slider-planes-kingless")))]
            sliders:[0xff00|(0xff<<48),0x42|(0x42<<56),0x2c|(0x2c<<56),0x89|(0x89<<56),0x10|(0x10<<56)],
            #[cfg(feature="exp-slider-planes-kingless")]
            sliders:[0xff00|(0xff<<48),0x42|(0x42<<56),0x2c|(0x2c<<56),0x89|(0x89<<56)],
            #[cfg(feature="exp-nibble-role")]
            role_map:{
                let mut m=[0xff;32];let back=[3u8,1,2,4,5,2,1,3];let mut i=0;
                while i<8{
                    let v=back[i];let shift=(i&1)*4;
                    m[i/2]=(m[i/2]&!(15<<shift))|(v<<shift);
                    m[(56+i)/2]=(m[(56+i)/2]&!(15<<shift))|(v<<shift);
                    m[(8+i)/2]&=!(15<<shift);m[(48+i)/2]&=!(15<<shift);i+=1;
                }m
            },
            #[cfg(feature="exp-role-planes")]
            role_planes:[(0x42|0x81|0x10)|((0x42|0x81|0x10)<<56),(0x24|0x81)|((0x24|0x81)<<56),(0x08|0x10)|((0x08|0x10)<<56)],
            #[cfg(feature = "exp-mailbox")]
            mailbox: {
                let mut a = [EMPTY;64];
                let back = [3,1,2,4,5,2,1,3];
                let mut i=0;
                while i<8 {a[i]=back[i]+6;a[8+i]=6;a[48+i]=0;a[56+i]=back[i];i+=1;}
                a
            },
            occ: [0xffff << 48, 0xffff],
            king_sq: [60, 4],
            castle_rook_sq: [7, 0, 63, 56],
            turn: Color::White,
            castling: 0b1111,
            ep: NO_EP,
            halfmove: 0,
            fullmove: 1,
            is_chess960: false,
        }
    }

    // -- accessors ----------------------------------------------------------

    /// Side to move.
    #[inline(always)]
    pub fn turn(&self) -> Color {
        self.turn
    }

    /// Bitboard of all squares occupied by `color`.
    #[inline(always)]
    pub fn occ_color(&self, color: Color) -> u64 {
        self.occ[color.index()]
    }

    /// Bitboard of every occupied square (`occ[0] | occ[1]` — derived, 1 OR,
    /// close-gap D3 task 4.1).
    #[inline(always)]
    pub fn occupied(&self) -> u64 {
        self.occ[0] | self.occ[1]
    }

    /// Bitboard of all pieces of `role` belonging to `color`.
    #[inline(always)]
    pub fn piece_bb(&self, color: Color, role: Role) -> u64 {
        self.bb(color.index(), role.index())
    }

    #[inline(always)]
    fn bb(&self,color:usize,role:usize)->u64 {
        #[cfg(feature="exp-slider-planes")]
        {
            let p=&self.sliders;
            let raw=match role {
                0=>p[0],1=>p[1],2=>p[2]&!p[3],3=>p[3]&!p[2],4=>p[2]&p[3],
                _=>{
                    #[cfg(feature="exp-slider-planes-kingless")]
                    {!(p[0]|p[1]|p[2]|p[3])}
                    #[cfg(not(feature="exp-slider-planes-kingless"))]
                    {p[4]}
                }
            };
            return raw&self.occ[color];
        }
        #[cfg(all(feature = "exp-compact-board",not(feature="exp-slider-planes")))]
        {
            #[cfg(feature="exp-kingless-board")]
            if role==Role::King.index(){
                // Derive from occupancy, not king_sq, to preserve malformed
                // FEN validation and Board::empty semantics.
                return self.occ[color]&!(self.roles[0]|self.roles[1]|self.roles[2]|self.roles[3]|self.roles[4]);
            }
            self.roles[role] & self.occ[color]
        }
        #[cfg(not(feature = "exp-compact-board"))]
        { self.bbs[color][role] }
    }
    #[inline(always)]
    fn diagonal_sliders(&self,color:usize)->u64{
        #[cfg(feature="exp-slider-planes")]
        {self.sliders[2]&self.occ[color]}
        #[cfg(not(feature="exp-slider-planes"))]
        {self.bb(color,Role::Bishop.index())|self.bb(color,Role::Queen.index())}
    }
    #[inline(always)]
    fn orthogonal_sliders(&self,color:usize)->u64{
        #[cfg(feature="exp-slider-planes")]
        {self.sliders[3]&self.occ[color]}
        #[cfg(not(feature="exp-slider-planes"))]
        {self.bb(color,Role::Rook.index())|self.bb(color,Role::Queen.index())}
    }
    #[cfg(feature="exp-slider-planes")]
    #[inline(always)]
    fn slider_role_at(&self,b:u64)->u8{
        if self.sliders[0]&b!=0{return 0;}
        if self.sliders[1]&b!=0{return 1;}
        // 00 king, 01 bishop, 10 rook, 11 queen.
        let d=(self.sliders[2]&b!=0) as usize;
        let r=(self.sliders[3]&b!=0) as usize;
        [5,2,3,4][d+2*r]
    }
    #[cfg(not(feature="exp-slider-planes"))]
    #[inline(always)]
    fn bb_mut(&mut self,color:usize,role:usize)->&mut u64 {
        #[cfg(feature = "exp-compact-board")]
        { let _=color; &mut self.roles[role] }
        #[cfg(not(feature = "exp-compact-board"))]
        { &mut self.bbs[color][role] }
    }
    #[cfg(feature="exp-nibble-role")]
    #[inline(always)]
    fn set_role_nibble(&mut self,sq:u8,role:u8){
        let shift=(sq&1)*4;let entry=&mut self.role_map[(sq>>1) as usize];
        *entry=(*entry&!(15<<shift))|(role<<shift);
    }
    #[cfg(feature="exp-nibble-role")]
    #[inline(always)]
    fn get_role_nibble(&self,sq:u8)->u8{
        (self.role_map[(sq>>1) as usize]>>((sq&1)*4))&15
    }
    #[cfg(feature="exp-role-planes")]
    #[inline(always)]
    fn get_role_planes(&self,sq:u8)->u8{
        (((self.role_planes[0]>>sq)&1)|(((self.role_planes[1]>>sq)&1)<<1)|(((self.role_planes[2]>>sq)&1)<<2)) as u8
    }

    #[inline(always)]
    fn bb_add(&mut self,color:usize,role:usize,mask:u64){
        #[cfg(feature="exp-nibble-role")]
        self.set_role_nibble(mask.trailing_zeros() as u8,role as u8);
        #[cfg(feature="exp-role-planes")]
        for i in 0..3 {if role&(1<<i)!=0{self.role_planes[i]|=mask;}}
        #[cfg(feature = "exp-mailbox")]
        { self.mailbox[mask.trailing_zeros() as usize]=(color*6+role) as u8; }
        #[cfg(all(feature="exp-compact-board",feature="exp-kingless-board",not(feature="exp-slider-planes")))]
        if role==Role::King.index(){return;}
        #[cfg(feature="exp-slider-planes")]
        {
            match role {
                0..=3=>self.sliders[role]|=mask,
                4=>{self.sliders[2]|=mask;self.sliders[3]|=mask;},
                _=>{
                    #[cfg(not(feature="exp-slider-planes-kingless"))]
                    {self.sliders[4]|=mask;}
                }
            }
        }
        #[cfg(not(feature="exp-slider-planes"))]
        {*self.bb_mut(color,role)|=mask;}
    }
    #[inline(always)]
    fn bb_remove(&mut self,color:usize,role:usize,mask:u64){
        #[cfg(feature="exp-nibble-role")]
        self.set_role_nibble(mask.trailing_zeros() as u8,15);
        #[cfg(feature="exp-role-planes")]
        for i in 0..3 {if role&(1<<i)!=0{self.role_planes[i]&=!mask;}}
        #[cfg(feature = "exp-mailbox")]
        { self.mailbox[mask.trailing_zeros() as usize]=EMPTY; }
        #[cfg(all(feature="exp-compact-board",feature="exp-kingless-board",not(feature="exp-slider-planes")))]
        if role==Role::King.index(){return;}
        #[cfg(feature="exp-slider-planes")]
        {
            match role {
                0..=3=>self.sliders[role]&=!mask,
                4=>{self.sliders[2]&=!mask;self.sliders[3]&=!mask;},
                _=>{
                    #[cfg(not(feature="exp-slider-planes-kingless"))]
                    {self.sliders[4]&=!mask;}
                }
            }
        }
        #[cfg(not(feature="exp-slider-planes"))]
        {*self.bb_mut(color,role)&=!mask;}
    }
    #[cfg(feature = "exp-mutation")]
    #[inline(always)]
    fn bb_toggle(&mut self,color:usize,role:usize,mask:u64){
        #[cfg(feature="exp-nibble-role")]
        if mask!=0{
            let from=(self.occ[color]&mask).trailing_zeros() as u8;
            let to=(!self.occ[color]&mask).trailing_zeros() as u8;
            self.set_role_nibble(from,15);self.set_role_nibble(to,role as u8);
        }
        #[cfg(feature="exp-role-planes")]
        for i in 0..3 {if role&(1<<i)!=0{self.role_planes[i]^=mask;}}
        #[cfg(feature = "exp-mailbox")]
        {
            let from=(self.occ[color]&mask).trailing_zeros() as usize;
            let to=(!self.occ[color]&mask).trailing_zeros() as usize;
            if mask!=0 { self.mailbox[from]=EMPTY;self.mailbox[to]=(color*6+role) as u8; }
        }
        #[cfg(all(feature="exp-compact-board",feature="exp-kingless-board",not(feature="exp-slider-planes")))]
        if role==Role::King.index(){return;}
        #[cfg(feature="exp-slider-planes")]
        {
            match role {
                0..=3=>self.sliders[role]^=mask,
                4=>{self.sliders[2]^=mask;self.sliders[3]^=mask;},
                _=>{
                    #[cfg(not(feature="exp-slider-planes-kingless"))]
                    {self.sliders[4]^=mask;}
                }
            }
        }
        #[cfg(not(feature="exp-slider-planes"))]
        {*self.bb_mut(color,role)^=mask;}
    }

    /// Piece standing on `sq`, if any.
    #[inline(always)]
    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        Piece::from_code(self.piece_code_at(sq.index() as u8))
    }

    /// Piece code on `sq` (0..11) or [`EMPTY`] — scans at most 6 bitboards
    /// after checking which color occupies the square (close-gap D3: mailbox
    /// removed). Empty = 2 tests, occupied = 2 + ≤6 tests (vs 12 previously).
    #[inline(always)]
    pub(crate) fn piece_code_at(&self, sq: u8) -> u8 {
        #[cfg(feature = "exp-mailbox")]
        { return self.mailbox[sq as usize]; }
        #[allow(unreachable_code)]
        let b = bit(sq);
        let occ0 = self.occ[0] & b;
        let occ1 = self.occ[1] & b;
        if occ0 == 0 && occ1 == 0 {
            return EMPTY;
        }
        #[cfg(feature="exp-nibble-role")]
        {return self.get_role_nibble(sq)+(occ1!=0) as u8*6;}
        #[cfg(all(feature="exp-role-planes",not(feature="exp-nibble-role")))]
        {return self.get_role_planes(sq)+(occ1!=0) as u8*6;}
        #[cfg(feature="exp-slider-planes")]
        {return self.slider_role_at(b)+(occ1!=0) as u8*6;}
        #[cfg(not(feature="exp-slider-planes"))]
        {
        #[allow(unreachable_code)]
        if occ0 != 0 {
            // Black piece — scan Black's 6 bitboards (unrolled).
            #[cfg(not(feature = "exp-compact-board"))]
            let col = &self.bbs[0];
            #[cfg(feature = "exp-compact-board")]
            let col = &self.roles;
            if col[0] & b != 0 {
                return 0;
            }
            if col[1] & b != 0 {
                return 1;
            }
            if col[2] & b != 0 {
                return 2;
            }
            if col[3] & b != 0 {
                return 3;
            }
            if col[4] & b != 0 {
                return 4;
            }
            debug_assert!(self.bb(0,5) & b != 0);
            5
        } else {
            // White piece — scan White's 6 bitboards.
            #[cfg(not(feature = "exp-compact-board"))]
            let col = &self.bbs[1];
            #[cfg(feature = "exp-compact-board")]
            let col = &self.roles;
            if col[0] & b != 0 {
                return 6;
            }
            if col[1] & b != 0 {
                return 7;
            }
            if col[2] & b != 0 {
                return 8;
            }
            if col[3] & b != 0 {
                return 9;
            }
            if col[4] & b != 0 {
                return 10;
            }
            debug_assert!(self.bb(1,5) & b != 0);
            11
        }
        }
    }


    #[cfg(feature="exp-known-role")]
    #[inline(always)]
    fn role_at_occupied(&self,sq:u8,color:Color)->u8{
        let b=bit(sq);
        debug_assert_ne!(self.occ[color.index()]&b,0);
        #[cfg(feature="exp-nibble-role")]
        {return self.get_role_nibble(sq);}
        #[cfg(all(feature="exp-role-planes",not(feature="exp-nibble-role")))]
        {return self.get_role_planes(sq);}
        #[cfg(feature="exp-slider-planes")]
        {return self.slider_role_at(b);}
        #[cfg(not(feature="exp-slider-planes"))]
        {
        #[allow(unreachable_code)]
        #[cfg(feature="exp-compact-board")]
        let col=&self.roles;
        #[cfg(not(feature="exp-compact-board"))]
        let col=&self.bbs[color.index()];
        #[cfg(feature="exp-role-select")]
        {
            if col[0]&b!=0{return 0;}
            let king=self.bb(color.index(),5);
            return ((col[1]|col[3]|king)&b!=0) as u8+
                2*((col[2]|col[3])&b!=0) as u8+
                4*((col[4]|king)&b!=0) as u8;
        }
        #[cfg(not(feature="exp-role-select"))]
        if col[0]&b!=0{0}else if col[1]&b!=0{1}else if col[2]&b!=0{2}else if col[3]&b!=0{3}else if col[4]&b!=0{4}else{5}
        }
    }


    /// Fast piece lookup for a square known to hold `color`'s piece (or empty).
    /// Scans only 6 bitboards of that color — 2× faster than `piece_code_at`
    /// for the `from` square and castling rook squares (hot make/unmake path).
    #[inline(always)]
    pub(crate) fn piece_code_at_color(&self, sq: u8, color: Color) -> u8 {
        #[cfg(feature = "exp-mailbox")]
        {
            let code=self.mailbox[sq as usize];
            return if code<12 && code/6==color as u8 { code } else { EMPTY };
        }
        #[allow(unreachable_code)]
        let b = bit(sq);
        if self.occ[color.index()] & b == 0 {
            return EMPTY;
        }
        #[cfg(feature="exp-nibble-role")]
        {return self.get_role_nibble(sq)+color as u8*6;}
        #[cfg(all(feature="exp-role-planes",not(feature="exp-nibble-role")))]
        {return self.get_role_planes(sq)+color as u8*6;}
        #[cfg(feature="exp-slider-planes")]
        {return self.slider_role_at(b)+color as u8*6;}
        #[cfg(not(feature="exp-slider-planes"))]
        {
        #[allow(unreachable_code)]
        #[cfg(not(feature = "exp-compact-board"))]
            let col = &self.bbs[color.index()];
            #[cfg(feature = "exp-compact-board")]
            let col = &self.roles;
        // Unrolled 6 checks — branch predictor friendly, no loop overhead.
        #[cfg(feature = "exp-role-lookup")]
        {
            let r0 = ((col[1] | col[3] | self.bb(color.index(),5)) & b != 0) as u8;
            let r1 = ((col[2] | col[3]) & b != 0) as u8;
            let r2 = ((col[4] | self.bb(color.index(),5)) & b != 0) as u8;
            return (color as u8) * 6 + r0 + (r1 << 1) + (r2 << 2);
        }
        #[allow(unreachable_code)]
        if col[0] & b != 0 {
            return (color as u8) * 6;
        }
        #[cfg(feature="exp-role-select")]
        {
            let r0=((col[1]|col[3]|self.bb(color.index(),5))&b!=0) as u8;
            let r1=((col[2]|col[3])&b!=0) as u8;
            let r2=((col[4]|self.bb(color.index(),5))&b!=0) as u8;
            return color as u8*6+r0+2*r1+4*r2;
        }
        #[allow(unreachable_code)]

        if col[1] & b != 0 {
            return (color as u8) * 6 + 1;
        }
        if col[2] & b != 0 {
            return (color as u8) * 6 + 2;
        }
        if col[3] & b != 0 {
            return (color as u8) * 6 + 3;
        }
        if col[4] & b != 0 {
            return (color as u8) * 6 + 4;
        }
        debug_assert!(self.bb(color.index(),5) & b != 0);
        (color as u8) * 6 + 5
        }
    }


    /// Captured pawn on `sq` is always an opponent pawn — single bit test
    /// (avoids 6-way scan). Returns pawn code or EMPTY.
    #[inline(always)]
    pub(crate) fn pawn_code_at(&self, sq: u8, color: Color) -> u8 {
        let b = bit(sq);
        if self.bb(color.index(), Role::Pawn.index()) & b != 0 {
            (color as u8) * 6
        } else {
            EMPTY
        }
    }

    /// Rights that survive a move of `role`/`us` from `from` to `to`,
    /// derived from [`Board::castle_rook_sq`] and the mover's role.
    /// Standard fast path uses `CASTLE_CLEAR_STD` table (like `ultrachess`
    /// `CASTLING_CLEAR`, 2 loads vs 8 compares). Chess960 falls back to the
    /// `rook_sq[4]` scan (rare, `is_chess960==true` only).
    #[inline(always)]
    fn castle_rights_after(&self, role: u8, us: Color, from: u8, to: u8) -> u8 {
        if cfg!(feature = "exp-standard-castle") && self.castling == 0 { return 0; }
        if !self.is_chess960 {
            // Standard: `E1` clears both white, `A1`/`H1` clear one, same for 8th.
            const CASTLE_CLEAR_STD: [u8; 64] = {
                let mut t = [0u8; 64];
                t[0] = 1 << 1; // A1 WQ
                t[7] = 1 << 0; // H1 WK
                t[4] = (1 << 0) | (1 << 1); // E1 both
                t[56] = 1 << 3; // A8 BQ
                t[63] = 1 << 2; // H8 BK
                t[60] = (1 << 2) | (1 << 3); // E8 both
                t
            };
            let mut keep = 0x0F;
            let mask = CASTLE_CLEAR_STD[from as usize] | CASTLE_CLEAR_STD[to as usize];
            keep &= !mask;
            if role == Role::King as u8 && (!cfg!(feature = "exp-standard-castle") || from != if us == Color::White {4} else {60}) {
                let both = (1u8 << crate::types::castle_right_bit(us, true))
                    | (1u8 << crate::types::castle_right_bit(us, false));
                keep &= !both;
            }
            return keep;
        }
        let mut keep: u8 = 0x0F;
        if role == Role::King as u8 {
            let both = (1u8 << crate::types::castle_right_bit(us, true))
                | (1u8 << crate::types::castle_right_bit(us, false));
            keep &= !both;
        }
        for rb in 0..4usize {
            if self.castle_rook_sq[rb] == from || self.castle_rook_sq[rb] == to {
                keep &= !(1u8 << rb);
            }
        }
        keep
    }

    /// King square of `color`.
    #[inline(always)]
    pub fn king_square(&self, color: Color) -> Square {
        Square(self.king_sq[color.index()])
    }

    /// Castling rights bitmask (bits 0..3 = WK, WQ, BK, BQ).
    #[inline(always)]
    pub fn castling_rights(&self) -> u8 {
        self.castling
    }

    /// The rook square backing castling right `right_bit` (0..3 = WK, WQ, BK,
    /// BQ). Chess960-aware: standard rights map to files a/h.
    #[inline(always)]
    pub fn castling_rook_square(&self, right_bit: u8) -> Square {
        Square(self.castle_rook_sq[right_bit as usize])
    }

    /// En-passant target square, if any.
    #[inline(always)]
    pub fn en_passant(&self) -> Option<Square> {
        if self.ep == NO_EP {
            None
        } else {
            Some(Square(self.ep))
        }
    }

    /// Halfmove clock (plies since the last pawn move or capture).
    #[inline(always)]
    pub fn halfmove_clock(&self) -> u16 {
        self.halfmove
    }

    /// Fullmove number.
    #[inline(always)]
    pub fn fullmove_number(&self) -> u16 {
        self.fullmove
    }

    /// Incrementally-maintained Polyglot zobrist hash of the position.
    #[inline(always)]
    pub fn zobrist(&self) -> u64 {
        self.hash
    }

    /// FIDE repetition-equivalence key. Unlike Polyglot `zobrist()`, an
    /// en-passant square contributes only if at least one EP capture is LEGAL.
    /// Standard and Chess960 use the same king-to-rook castling representation
    /// and rook-file castling keys as the TypeScript implementation.
    ///
    /// This reads the incrementally maintained hash; after `play_fast` or
    /// perft-only makes, use a fully state-maintaining make before reading it.
    /// Hashing for Polyglot opening books remains intentionally unchanged.
    #[inline]
    pub fn repetition_key(&self) -> u64 {
        if !self.ep_relevant(self.ep, self.turn) {
            return self.hash;
        }
        let mut candidate_pawns = PAWN_ATT[self.turn.other().index()][self.ep as usize]
            & self.piece_bb(self.turn, Role::Pawn);
        while candidate_pawns != 0 {
            let from = pop_lsb(&mut candidate_pawns);
            if self.is_legal(Move::quiet(Square(from), Square(self.ep))) {
                return self.hash;
            }
        }
        // Polyglot hashes geometrically capturable EP, even when a pin or
        // discovered check makes all such captures illegal for repetition.
        self.hash ^ zobrist::ep_key(Square(self.ep))
    }

    /// Cached checkers bitboard (attackers to the side-to-move king).
    /// `in_check()` is `checkers != 0`; `0.34ns` hash load is `zobrist()`.
    #[inline(always)]
    pub fn checkers_bb(&self) -> u64 {
        self.checkers
    }

    // -- internal piece bookkeeping (hash-aware) ----------------------------

    #[inline(always)]
    fn role_from_index(role: usize) -> Role {
        match role {
            0 => Role::Pawn,
            1 => Role::Knight,
            2 => Role::Bishop,
            3 => Role::Rook,
            4 => Role::Queen,
            _ => Role::King,
        }
    }

    #[inline(always)]
    fn color_from_index(color: usize) -> Color {
        if color == 1 {
            Color::White
        } else {
            Color::Black
        }
    }

    #[inline(always)]
    pub(crate) fn put_piece_no_hash(&mut self, sq: u8, code: u8) {
        let color = (code / 6) as usize;
        let role = (code % 6) as usize;
        let b = bit(sq);
        self.bb_add(color, role, b);
        self.occ[color] |= b;
        if role == Role::King.index() {
            self.king_sq[color] = sq;
        }
    }

    #[inline(always)]
    pub(crate) fn put_piece(&mut self, sq: u8, code: u8) {
        let color = (code / 6) as usize;
        let role = (code % 6) as usize;
        let b = bit(sq);
        self.bb_add(color, role, b);
        self.occ[color] |= b;
        self.hash ^= if cfg!(feature = "exp-direct-hash") {
            crate::zobrist::piece_key_indices(color,role,sq)
        } else { zobrist::piece_key(Self::color_from_index(color),Self::role_from_index(role),Square(sq)) };
        if role == Role::King.index() {
            self.king_sq[color] = sq;
        }
    }

    #[inline(always)]
    fn remove_piece(&mut self, sq: u8, code: u8) {
        let color = (code / 6) as usize;
        let role = (code % 6) as usize;
        let b = bit(sq);
        self.bb_remove(color, role, b);
        self.occ[color] &= !b;
        self.hash ^= if cfg!(feature = "exp-direct-hash") {
            crate::zobrist::piece_key_indices(color,role,sq)
        } else { zobrist::piece_key(Self::color_from_index(color),Self::role_from_index(role),Square(sq)) };
    }

    #[inline(always)]
    fn move_piece(&mut self, from: u8, to: u8, code: u8) {
        #[cfg(feature = "exp-mutation")]
        {
            let color = (code / 6) as usize;
            let role = (code % 6) as usize;
            let delta = bit(from) ^ bit(to);
            self.bb_toggle(color, role, delta);
            self.occ[color] ^= delta;
            self.hash ^= zobrist::piece_key(Self::color_from_index(color), Self::role_from_index(role), Square(from))
                ^ zobrist::piece_key(Self::color_from_index(color), Self::role_from_index(role), Square(to));
            if role == Role::King.index() { self.king_sq[color] = to; }
        }
        #[cfg(not(feature = "exp-mutation"))]
        {
            self.remove_piece(from, code);
            self.put_piece(to, code);
        }
    }

    /// True when a pawn of `side` can pseudo-legally capture on the
    /// en-passant square (Polyglot ep-hash relevance condition).
    #[inline(always)]
    fn ep_relevant(&self, ep: u8, side: Color) -> bool {
        if ep == NO_EP {
            return false;
        }
        PAWN_ATT[side.other().index()][ep as usize]
            & self.bb(side.index(), Role::Pawn.index())
            != 0
    }

    /// Squares occupied by pieces of `by` that attack `sq` under occupancy `occ`.
    pub fn attackers_to(&self, sq: u8, by: Color, occ: u64) -> u64 {
        #[cfg(all(feature="exp-compact-attacks",feature="exp-compact-board"))]
        {
            let allowed=self.occ[by.index()];
            let k=sq as usize;
            #[cfg(feature="exp-slider-planes")]
            let p=&self.sliders;
            #[cfg(not(feature="exp-slider-planes"))]
            let p=&self.roles;
            let mut result=(PAWN_ATT[by.other().index()][k]&p[0])|
                (KNIGHT_ATT[k]&p[1])|(KING_ATT[k]&self.bb(by.index(),5));
            let bishops=self.diagonal_sliders(by.index())&crate::bitboard::EMPTY_BISHOP_RAYS[k];
            let rooks=self.orthogonal_sliders(by.index())&crate::bitboard::EMPTY_ROOK_RAYS[k];
            result&=allowed;
            #[cfg(feature="exp-checker-rays")]
            {
                let mut candidates=bishops|rooks;
                #[cfg(feature="exp-checker-first")]
                if candidates!=0{
                    let first=pop_lsb(&mut candidates);
                    let clear=crate::bitboard::BETWEEN[k][first as usize]&occ==0;
                    result|=bit(first)&0u64.wrapping_sub(clear as u64);
                }
                while candidates!=0{
                    let a=pop_lsb(&mut candidates);
                    if crate::bitboard::BETWEEN[k][a as usize]&occ==0{result|=bit(a);}
                }
            }
            #[cfg(not(feature="exp-checker-rays"))]
            {
                if bishops!=0{result|=attacks::bishop_attacks(sq,occ)&bishops;}
                if rooks!=0{result|=attacks::rook_attacks(sq,occ)&rooks;}
            }
            return result;
        }
        #[allow(unreachable_code)]
        #[cfg(feature="exp-attack-descriptor")]
        {
            const DESCS:[[u64;6];64]={
                let mut arr=[[0;6];64];let mut i=0;
                while i<64{
                    arr[i]=[PAWN_ATT[0][i],PAWN_ATT[1][i],KNIGHT_ATT[i],KING_ATT[i],
                        crate::bitboard::EMPTY_BISHOP_RAYS[i],crate::bitboard::EMPTY_ROOK_RAYS[i]];
                    i+=1;
                }arr
            };
            let k=sq as usize;let d=&DESCS[k];let ci=by.index();
            let mut a=(d[by.other().index()]&self.bb(ci,0))|(d[2]&self.bb(ci,1))|(d[3]&self.bb(ci,5));
            let b=self.diagonal_sliders(ci)&d[4];let r=self.orthogonal_sliders(ci)&d[5];
            #[cfg(feature="exp-checker-rays")]
            {
                let mut candidates=b|r;
                while candidates!=0{
                    let x=pop_lsb(&mut candidates);
                    if crate::bitboard::BETWEEN[k][x as usize]&occ==0{a|=bit(x);}
                }
            }
            #[cfg(not(feature="exp-checker-rays"))]
            {
                if b!=0{a|=attacks::bishop_attacks(sq,occ)&b;}
                if r!=0{a|=attacks::rook_attacks(sq,occ)&r;}
            }
            return a;
        }
        #[allow(unreachable_code)]
        let bi = by.index();
        let mut a = 0u64;
        // Pawns of `by` attacking sq stand where a pawn of the other color
        // placed on sq would attack.
        a |= PAWN_ATT[by.other().index()][sq as usize] & self.bb(bi, Role::Pawn.index());
        a |= KNIGHT_ATT[sq as usize] & self.bb(bi, Role::Knight.index());
        a |= KING_ATT[sq as usize] & self.bb(bi, Role::King.index());
        let bq = self.diagonal_sliders(bi);
        let rq = self.orthogonal_sliders(bi);
        #[cfg(feature="exp-checker-rays")]
        {
            let mut candidates=(bq&crate::bitboard::EMPTY_BISHOP_RAYS[sq as usize])|
                (rq&crate::bitboard::EMPTY_ROOK_RAYS[sq as usize]);
            while candidates!=0 {
                let attacker=pop_lsb(&mut candidates);
                if crate::bitboard::BETWEEN[sq as usize][attacker as usize]&occ==0 {a|=bit(attacker);}
            }
            return a;
        }
        #[allow(unreachable_code)]
        if !cfg!(feature = "exp-ray-prefilter") || bq & crate::bitboard::EMPTY_BISHOP_RAYS[sq as usize] != 0 {
            a |= attacks::bishop_attacks(sq, occ) & bq;
        }
        if !cfg!(feature = "exp-ray-prefilter") || rq & crate::bitboard::EMPTY_ROOK_RAYS[sq as usize] != 0 {
            a |= attacks::rook_attacks(sq, occ) & rq;
        }
        a
    }

    #[cfg(feature="exp-bool-attacks")]
    #[inline(always)]
    fn attacked_except(&self,sq:u8,by:Color,occ:u64,removed:u64)->bool{
        let ci=by.index();let k=sq as usize;let allowed=!removed;
        if ((PAWN_ATT[by.other().index()][k]&self.bb(ci,0))|
            (KNIGHT_ATT[k]&self.bb(ci,1))|(KING_ATT[k]&self.bb(ci,5)))&allowed!=0{return true;}
        let b=self.diagonal_sliders(ci)&crate::bitboard::EMPTY_BISHOP_RAYS[k]&allowed;
        let r=self.orthogonal_sliders(ci)&crate::bitboard::EMPTY_ROOK_RAYS[k]&allowed;
        #[cfg(feature="exp-checker-rays")]
        {
            let mut candidates=b|r;
            while candidates!=0{
                let a=pop_lsb(&mut candidates);
                if crate::bitboard::BETWEEN[k][a as usize]&occ==0{return true;}
            }
            false
        }
        #[cfg(not(feature="exp-checker-rays"))]
        { (b!=0&&attacks::bishop_attacks(sq,occ)&b!=0)||(r!=0&&attacks::rook_attacks(sq,occ)&r!=0) }
    }

    /// True when the king of `color` is attacked (full scan; used by
    /// `king_attacked` callers that need a color-specific check, e.g. `play`
    /// legality after `make_move_unchecked` for the mover).
    #[inline(always)]
    pub fn king_attacked(&self, color: Color) -> bool {
        self.attackers_to(self.king_sq[color.index()], color.other(), self.occupied()) != 0
    }

    /// True when the side to move is in check — branch-free `checkers != 0`
    /// (0.32ns both states, D3). `checkers` is refreshed in `make`/`unmake`
    /// via `attackers_to` (+2ns/make) and restored from `Undo`.
    #[inline(always)]
    pub fn in_check(&self) -> bool {
        self.checkers != 0
    }

    /// XOR-fold of the rook-file castling keys for the given rights bitmask
    /// (ADR-003: keys are per (color, rook file)).
    #[inline(always)]
    fn castle_rights_hash(&self, rights: u8) -> u64 {
        #[cfg(feature="exp-fast-castle-keys")]
        {
            const HASHES:[u64;16]={
                let mut t=[0;16];let mut i=1;
                while i<16 {let low=(i as u32).trailing_zeros() as usize;
                    t[i]=t[i&(i-1)]^crate::polyglot_keys::POLYGLOT_RANDOM_ARRAY[768+low];i+=1;}
                t
            };
            // Absent rights may have unspecified rook squares. Check only the
            // affected rights, even during FEN parsing before is_chess960 is set.
            const FILES:[u8;4]=[7,0,7,0];
            let mut bad=0u8;
            for i in 0..4 {bad|=((self.castle_rook_sq[i]&7!=FILES[i]) as u8)<<i;}
            if rights&bad==0{return HASHES[rights as usize];}
        }

        #[cfg(feature="exp-castle-hash-table")]
        {
            const STANDARD:[u64;16]={
                let mut keys=[0;16];let mut i=0;
                while i<16 {let mut b=0;while b<4{if i&(1<<b)!=0{keys[i]^=crate::polyglot_keys::POLYGLOT_RANDOM_ARRAY[768+b];}b+=1;}i+=1;}keys
            };
            // Rook squares for absent rights can be unspecified. Check only
            // active rights, including initial FEN state before is_chess960.
            let mut valid=0u8;
            if self.castle_rook_sq[0]&7==7 {valid|=1;}
            if self.castle_rook_sq[1]&7==0 {valid|=2;}
            if self.castle_rook_sq[2]&7==7 {valid|=4;}
            if self.castle_rook_sq[3]&7==0 {valid|=8;}
            if rights&!valid==0{return STANDARD[rights as usize];}
        }
        zobrist::castle_file_keys_xor(rights, &self.castle_rook_sq)
    }

    /// Recomputes the Polyglot zobrist hash from scratch (verification aid;
    /// the incremental hash in `self.hash` must always agree with this).
    pub fn zobrist_full(&self) -> u64 {
        let mut h = 0u64;
        #[cfg(all(feature="exp-hash-role-scan",feature="exp-compact-board"))]
        {
            #[inline(always)]
            fn fold<const R:usize>(b:&Board)->u64{
                let mut bits=b.bb(0,R)|b.bb(1,R);let white=b.occ[1];
                let mut h=0;
                while bits!=0{
                    let sq=pop_lsb(&mut bits);
                    let color=((white>>sq)&1) as usize;
                    h^=zobrist::piece_key_indices(color,R,sq);
                }
                h
            }
            h=fold::<0>(self)^fold::<1>(self)^fold::<2>(self)^fold::<3>(self)^fold::<4>(self)^fold::<5>(self);
        }
        #[cfg(all(feature="exp-hash-unroll",not(all(feature="exp-hash-role-scan",feature="exp-compact-board"))))]
        {
            #[inline(always)]
            fn fold<const C:usize,const R:usize>(b:&Board)->u64{
                let mut bits=b.bb(C,R);
                let mut value=0;
                while bits!=0 {value^=zobrist::piece_key_indices(C,R,pop_lsb(&mut bits));}
                value
            }
            h=fold::<0,0>(self)^fold::<0,1>(self)^fold::<0,2>(self)^fold::<0,3>(self)^fold::<0,4>(self)^fold::<0,5>(self)^
              fold::<1,0>(self)^fold::<1,1>(self)^fold::<1,2>(self)^fold::<1,3>(self)^fold::<1,4>(self)^fold::<1,5>(self);
        }
        #[cfg(not(any(feature="exp-hash-unroll",all(feature="exp-hash-role-scan",feature="exp-compact-board"))))]
        for ci in 0..2 {
            #[cfg(feature="exp-hash-pawn-bytes")]
            {h^=zobrist::pawn_byte_hash(ci,self.bb(ci,0));}
            for r in (if cfg!(feature="exp-hash-pawn-bytes"){1}else{0})..6 {
                let mut b = self.bb(ci, r);
                while b != 0 {
                    let sq = pop_lsb(&mut b);
                    h ^= zobrist::piece_key(
                        Self::color_from_index(ci),
                        Self::role_from_index(r),
                        Square(sq),
                    );
                }
            }
        }
        h ^= self.castle_rights_hash(self.castling);
        h ^= zobrist::ep_contribution(
            self.ep,
            self.bb(self.turn.index(), Role::Pawn.index()),
            PAWN_ATT[self.turn.other().index()][self.ep as usize & 63],
        );
        if self.turn == Color::White {
            h ^= zobrist::turn_key();
        }
        h
    }
}

impl Board {
    /// Generates every strictly legal move into a stack-allocated
    /// `ArrayVec<Move, 256>` (zero heap allocations).
    #[inline(always)]
    pub fn legal_moves(&self) -> ArrayVec<Move, MAX_MOVES> {
        let mut out = ArrayVec::new();
        self.generate_moves_cached(&mut out);
        out
    }

    /// Legal UCI moves in lexical order. Standard castling uses king
    /// destinations, Chess960 uses the canonical king-to-rook encoding.
    /// Sort compact numeric keys before allocating output strings.
    #[cfg(feature="exp-uci-sort-key")]
    pub fn legal_moves_uci_sorted(&self)->Vec<String>{
        #[inline(always)]
        fn coordinate(s:u8)->u16{(((s&7)<<3)|(s>>3)) as u16}
        let mut keys:ArrayVec<u16,MAX_MOVES>=ArrayVec::new();
        let k=self.king_sq[self.turn.index()];
        for mv in self.legal_moves(){
            let f=mv.from().0;let mut t=mv.to().0;
            if !self.is_chess960 && f==k && self.occ[self.turn.index()]&bit(t)!=0{
                t=(f&56)|if t>f{6}else{2};
            }
            let p=match mv.promotion(){Some(Role::Bishop)=>1,Some(Role::Knight)=>2,Some(Role::Queen)=>3,Some(Role::Rook)=>4,_=>0};
            keys.push((coordinate(f)<<9)|(coordinate(t)<<3)|p);
        }
        keys.sort_unstable();
        let mut result=Vec::with_capacity(keys.len());
        for key in keys{
            let f=(key>>9) as u8;let t=((key>>3)&63) as u8;let p=(key&7) as usize;
            let bytes=[b'a'+(f>>3),b'1'+(f&7),b'a'+(t>>3),b'1'+(t&7),b" bnqr"[p]];
            let len=if p==0{4}else{5};
            // SAFETY: coordinates and promotion bytes are always ASCII.
            result.push(unsafe{String::from_utf8_unchecked(bytes[..len].to_vec())});
        }
        result
    }

    /// Generic move generation into any [`MoveVisitor`](crate::movegen::MoveVisitor)
    /// (D1: visitor pattern, no `Move` materialisation). Shares the
    /// `generate_moves_into` body verbatim — `compute_pinned_split` + bulk
    /// pawn shifts — through `VisitorAdapter` (monomorphised, `LTO=fat`).
    pub fn generate_visitor<V: crate::movegen::MoveVisitor>(&self, visitor: &mut V) {
        let mut adapter = crate::movegen::VisitorAdapter::new(visitor);
        self.generate_moves_into(&mut adapter);
    }

    /// Alloc-free count of strictly legal moves via [`MoveCounter`] bulk path
    /// (`count+=popcount` without `pop_lsb`) — the `geomean 1.23× vs cozy` win.
    #[inline(always)]
    pub fn count_legal_moves(&self) -> u32 {
        let mut counter = MoveCounter::new();
        self.generate_moves_into(&mut counter);
        counter.count
    }

    /// Generic move generation into any `MoveSink` (D2: `generate_moves_into<S:MoveSink>`).
    ///
    /// Dispatch wrapper (close-gap D2, task 3.1): keeps the public API and
    /// forwards to [`Board::generate_moves_templated`] with `WHITE`
    /// const-folded so every `if white` pawn branch/shift specialises at
    /// compile time (Gigantua's colour template expansion, clean-room).
    pub fn generate_moves_into<S: MoveSink>(&self, sink: &mut S) {
        if cfg!(feature = "exp-standard-no-special") && !self.is_chess960 && self.castling==0 && self.ep==NO_EP {
            if self.turn==Color::White {self.generate_moves_templated::<true,false,S>(sink);}
            else {self.generate_moves_templated::<false,false,S>(sink);}
        } else if self.turn == Color::White {
            self.generate_moves_templated::<true,true,S>(sink);
        } else {
            self.generate_moves_templated::<false,true,S>(sink);
        }
    }

    #[inline(always)]
    fn generate_moves_cached<S: MoveSink>(&self, sink: &mut S) {
        if cfg!(feature = "exp-standard-no-special") && !self.is_chess960 && self.castling==0 && self.ep==NO_EP {
            if self.turn==Color::White {self.generate_moves_templated::<true,false,S>(sink);}
            else {self.generate_moves_templated::<false,false,S>(sink);}
        } else if self.turn == Color::White {
            self.generate_moves_templated::<true,true,S>(sink);
        } else {
            self.generate_moves_templated::<false,true,S>(sink);
        }
    }

    /// Colour-templated generation core: `WHITE` is a compile-time constant,
    /// so pawn push/capture shifts (`<<8/>>8/<<7/<<9/>>7/>>9`), offsets,
    /// promo/start ranks and the `PAWN_ATT` colour index all const-fold
    /// (monomorphised ×2 per sink, `LTO=fat`).
    #[inline(always)]
    fn generate_moves_templated<const WHITE: bool,const SPECIAL:bool, S: MoveSink>(&self, sink: &mut S) {
        #[cfg(feature="exp-color-gen")]
        let us=if WHITE{Color::White}else{Color::Black};
        #[cfg(not(feature="exp-color-gen"))]
        let us=self.turn;
        let them=us.other();
        let ui = us.index();
        let ti = them.index();
        let ksq = self.king_sq[ui];
        let occ = self.occupied();
        let their_bq = self.diagonal_sliders(ti);
        let their_rq = self.orthogonal_sliders(ti);

        // Enemy attack map with our king removed (x-ray safe king moves).
        let occ_no_king = occ ^ bit(ksq);
        let mut danger = 0u64;
        let possible_king = KING_ATT[ksq as usize] & !self.occ[ui];
        let mut needed = possible_king;
        let mut local = false;
        if cfg!(feature = "exp-king-local") && !self.is_chess960 {
            let rights=(self.castling >> ((1-ui)*2))&3;
            let shift=if WHITE{0}else{56};
            if rights==0 || ksq==shift+4 {
                local=true;
                if self.checkers==0 || cfg!(feature="exp-fused-geometry") {
                    if rights&1!=0 && occ&(0x60u64<<shift)==0 {needed|=0x60u64<<shift;}
                    if rights&2!=0 && occ&(0x0eu64<<shift)==0 {needed|=0x0cu64<<shift;}
                }
            }
        }
        let targeted = (local && needed.count_ones()<=3)
            || (cfg!(feature = "exp-king-targets")
            && self.castling & (3 << ((1-ui)*2)) == 0
            && possible_king.count_ones() <= 3);
        if targeted {
            let mut targets = needed;
            while targets != 0 {
                let to = pop_lsb(&mut targets);
                if cfg!(feature = "exp-king-local") {
                    let k=to as usize;
                    let leapers=(PAWN_ATT[ui][k]&self.bb(ti,Role::Pawn.index()))|(KNIGHT_ATT[k]&self.bb(ti,Role::Knight.index()))|(KING_ATT[k]&self.bb(ti,Role::King.index()));
                    if leapers!=0
                        || (crate::bitboard::EMPTY_BISHOP_RAYS[k]&their_bq!=0 && attacks::bishop_attacks(to,occ_no_king)&their_bq!=0)
                        || (crate::bitboard::EMPTY_ROOK_RAYS[k]&their_rq!=0 && attacks::rook_attacks(to,occ_no_king)&their_rq!=0) {danger|=bit(to);}
                } else if self.attackers_to(to, them, occ_no_king) != 0 { danger |= bit(to); }
            }
        } else {
        // Pawns — bulk shifts templated on WHITE (no runtime `them` branch, const-folded).
        {
            let pawns = self.bb(ti, Role::Pawn.index());
            if WHITE {
                // `WHITE` true => us=White, them=Black -> Black pawns attack south.
                danger |= (pawns & !crate::types::FILE_BB[0]) >> 9;
                danger |= (pawns & !crate::types::FILE_BB[7]) >> 7;
            } else {
                // us=Black, them=White -> White pawns attack north.
                danger |= (pawns & !crate::types::FILE_BB[0]) << 7;
                danger |= (pawns & !crate::types::FILE_BB[7]) << 9;
            }
        }
        let mut p = self.bb(ti, Role::Knight.index());
        while p != 0 {
            let sq = pop_lsb(&mut p);
            danger |= KNIGHT_ATT[sq as usize];
        }
        danger |= KING_ATT[self.king_sq[ti] as usize];
        let mut p = their_bq;
        while p != 0 {
            let sq = pop_lsb(&mut p);
            danger |= attacks::bishop_attacks(sq, occ_no_king);
        }
        let mut p = their_rq;
        while p != 0 {
            let sq = pop_lsb(&mut p);
            danger |= attacks::rook_attacks(sq, occ_no_king);
        }

        }
        // King steps (castling handled separately) — bulk via push_targets.
        {
            let king_targets = KING_ATT[ksq as usize] & !self.occ[ui] & !danger;
            sink.push_targets(ksq, king_targets);
        }

        #[cfg(feature = "exp-fused-geometry")]
        let (checkers,pinned_hv,pinned_diag) = {
            let (sliders,hv,diag)=crate::movegen::geometry(ksq,occ,their_bq,their_rq,self.occ[ui]);
            let check=sliders|(PAWN_ATT[ui][ksq as usize]&self.bb(ti,Role::Pawn.index()))
                |(KNIGHT_ATT[ksq as usize]&self.bb(ti,Role::Knight.index()))
                |(KING_ATT[ksq as usize]&self.bb(ti,Role::King.index()));
            (check,hv,diag)
        };
        #[cfg(not(feature = "exp-fused-geometry"))]
        let checkers = self.checkers;
        if popcount(checkers) >= 2 {
            return; // double check: only king moves are legal
        }

        // When in check, non-king moves must capture the checker or block it.
        let allowed = if checkers == 0 {
            !0u64
        } else {
            let csq = lsb(checkers);
            bit(csq) | BETWEEN[ksq as usize][csq as usize]
        };

        // Split pinned masks (ultrachess compute_pinned_split, enemy occupancy blocks rays).
        #[cfg(not(feature = "exp-fused-geometry"))]
        let (pinned_hv, pinned_diag) = compute_pinned_split(ksq, occ, their_bq, their_rq, self.occ[ui], self.occ[ti]);
        let pinned_any = pinned_hv | pinned_diag;

        // Fast path when nothing is pinned (common, startpos) — avoids extra
        // ANDs and second loops for pinned sliders.
        if pinned_any == 0 {
            {
                let mut p = self.bb(ui, Role::Knight.index());
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = KNIGHT_ATT[from as usize] & !self.occ[ui] & allowed;
                    sink.push_targets(from, t);
                }
            }
            {
                let mut p = self.diagonal_sliders(ui);
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = attacks::bishop_attacks(from, occ) & !self.occ[ui] & allowed;
                    sink.push_targets(from, t);
                }
            }
            {
                let mut p = self.orthogonal_sliders(ui);
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = attacks::rook_attacks(from, occ) & !self.occ[ui] & allowed;
                    sink.push_targets(from, t);
                }
            }
        } else {
            // Knights (a pinned knight can never move) — split avoids LINE load.
            {
                let knights = self.bb(ui, Role::Knight.index()) & !pinned_any;
                let mut p = knights;
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = KNIGHT_ATT[from as usize] & !self.occ[ui] & allowed;
                    sink.push_targets(from, t);
                }
            }

            // Bishops and queens (diagonal moves) — unpinned bulk, pinned_diag via LINE.
            {
                let bq = self.diagonal_sliders(ui);
                // Unpinned (and hv-pinned bishops are stuck — skip).
                let bq_unpinned = bq & !pinned_hv & !pinned_diag;
                let mut p = bq_unpinned;
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = attacks::bishop_attacks(from, occ) & !self.occ[ui] & allowed;
                    sink.push_targets(from, t);
                }
                // Pinned diagonally: only along pin_line (LINE[king][from]).
                let mut p = bq & pinned_diag;
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = attacks::bishop_attacks(from, occ) & !self.occ[ui] & allowed & crate::bitboard::LINE[ksq as usize][from as usize];
                    sink.push_targets(from, t);
                }
            }

            // Rooks and queens (straight moves) — unpinned bulk, pinned_hv via LINE.
            {
                let rq = self.orthogonal_sliders(ui);
                let rq_unpinned = rq & !pinned_hv & !pinned_diag;
                let mut p = rq_unpinned;
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = attacks::rook_attacks(from, occ) & !self.occ[ui] & allowed;
                    sink.push_targets(from, t);
                }
                let mut p = rq & pinned_hv;
                while p != 0 {
                    let from = pop_lsb(&mut p);
                    let t = attacks::rook_attacks(from, occ) & !self.occ[ui] & allowed & crate::bitboard::LINE[ksq as usize][from as usize];
                    sink.push_targets(from, t);
                }
            }
        }

        self.gen_pawn_moves_templated::<WHITE,SPECIAL,S>(sink, allowed, pinned_hv, pinned_diag, ksq);

        // Castling is only possible when not in check.
        if SPECIAL && checkers == 0 {
            self.gen_castling_sink(sink, danger);
        }
    }

    /// Pawn move generation via `MoveSink` bulk shifts (promo split) + per-pinned
    /// handling — runtime colour branch (2 monomorphisations per sink vs 4 with
    /// `WHITE` const, smaller `.text`, better icache).
    fn gen_pawn_moves_templated<const WHITE: bool,const SPECIAL:bool,S: MoveSink>(
        &self,
        sink: &mut S,
        allowed: u64,
        pinned_hv: u64,
        pinned_diag: u64,
        ksq: u8,
    ) {
        let us = self.turn;
        let ui = us.index();
        let occ = self.occupied();
        let them_occ = self.occ[us.other().index()];
        let empty = !occ;
        let pawns = self.bb(ui, Role::Pawn.index());
        let pinned_any = pinned_hv | pinned_diag;

        // Bulk for unpinned pawns — `north(pawns)&!occ&check_mask` split promo/non-promo.
        let unpinned = pawns & !pinned_any;
        if unpinned != 0 {
            let promo_rank = if WHITE { crate::types::RANK_BB[7] } else { crate::types::RANK_BB[0] };
            let start_rank_bb = if WHITE { crate::types::RANK_BB[1] } else { crate::types::RANK_BB[6] };

            // Single pushes: north(pawns) & empty
            let single = if WHITE { unpinned << 8 } else { unpinned >> 8 } & empty;
            let single_allowed = single & allowed;
            let single_promo = single_allowed & promo_rank;
            let single_nonpromo = single_allowed & !promo_rank;
            if single_nonpromo != 0 {
                let offset: i8 = if WHITE { 8 } else { -8 };
                sink.push_pawn_targets_offset(single_nonpromo, offset);
            }
            if single_promo != 0 {
                let offset: i8 = if WHITE { 8 } else { -8 };
                sink.push_pawn_promotions_offset(single_promo, offset);
            }

            // Double pushes: pawns on start rank with both squares empty.
            let pawns_start = unpinned & start_rank_bb;
            let intermediate = if WHITE { pawns_start << 8 } else { pawns_start >> 8 } & empty;
            let double = if WHITE { intermediate << 8 } else { intermediate >> 8 } & empty & allowed;
            if double != 0 {
                let offset: i8 = if WHITE { 16 } else { -16 };
                sink.push_pawn_targets_offset(double, offset);
            }

            // Captures — bulk shifts with file masks.
            let (left_caps, right_caps, left_off, right_off): (u64, u64, i8, i8) = if WHITE {
                let l = (unpinned & !crate::types::FILE_BB[0]) << 7 & them_occ & allowed;
                let r = (unpinned & !crate::types::FILE_BB[7]) << 9 & them_occ & allowed;
                (l, r, 7, 9)
            } else {
                let l = (unpinned & !crate::types::FILE_BB[0]) >> 9 & them_occ & allowed;
                let r = (unpinned & !crate::types::FILE_BB[7]) >> 7 & them_occ & allowed;
                (l, r, -9, -7)
            };
            let left_nonpromo = left_caps & !promo_rank;
            let left_promo = left_caps & promo_rank;
            let right_nonpromo = right_caps & !promo_rank;
            let right_promo = right_caps & promo_rank;
            if left_nonpromo != 0 {
                sink.push_pawn_targets_offset(left_nonpromo, left_off);
            }
            if left_promo != 0 {
                sink.push_pawn_promotions_offset(left_promo, left_off);
            }
            if right_nonpromo != 0 {
                sink.push_pawn_targets_offset(right_nonpromo, right_off);
            }
            if right_promo != 0 {
                sink.push_pawn_promotions_offset(right_promo, right_off);
            }
        }

        // Pinned pawns — per-pawn with pin line (LINE[king][from]) masking.
        let mut p = pawns & pinned_any;
        while p != 0 {
            let from = pop_lsb(&mut p);
            let allowed_here = allowed & crate::bitboard::LINE[ksq as usize][from as usize];
            // For pinned pawns, forward vs capture legality depends on pin direction:
            // hv pins allow only file moves, diag pins allow only captures along pin.
            // Masking with pin_line achieves this: pushes/captures off pin_line are stripped.
            // Reuse the original per-pawn logic but via sink.push_one.
            let promo_rank: u8 = if WHITE { 7 } else { 0 };
            let start_rank: u8 = if WHITE { 1 } else { 6 };

            // Pushes (single/double) — intersect with pin_line.
            let one_empty = (if WHITE { bit(from) << 8 } else { bit(from) >> 8 }) & empty;
            if one_empty != 0 {
                if from / 8 == start_rank {
                    let two = (if WHITE { one_empty << 8 } else { one_empty >> 8 }) & empty & allowed_here;
                    if two != 0 {
                        let to = lsb(two);
                        sink.push_one(Move::new(Square(from), Square(to), None));
                    }
                }
                if one_empty & allowed_here != 0 {
                    let to = lsb(one_empty);
                    if to / 8 == promo_rank {
                        for r in [Role::Queen, Role::Rook, Role::Bishop, Role::Knight] {
                            sink.push_one(Move::new(Square(from), Square(to), Some(r)));
                        }
                    } else {
                        sink.push_one(Move::new(Square(from), Square(to), None));
                    }
                }
            }

            // Captures (including promo split).
            let att = PAWN_ATT[ui][from as usize];
            let mut caps = att & them_occ & allowed_here;
            while caps != 0 {
                let to = pop_lsb(&mut caps);
                if to / 8 == promo_rank {
                    for r in [Role::Queen, Role::Rook, Role::Bishop, Role::Knight] {
                        sink.push_one(Move::new(Square(from), Square(to), Some(r)));
                    }
                } else {
                    sink.push_one(Move::new(Square(from), Square(to), None));
                }
            }

            // En passant for pinned pawn — must still verify discovered check via simulation
            // (capturing two pawns, rank pin). Handled in common EP section below so we skip here
            // to avoid double-count; EP is handled after the pinned loop for all pawns via scan.
        }

        // En passant — verified by direct simulation (handles rank pins through both
        // disappearing pawns and check-evasion-by-EP). Works for pinned & unpinned.
        if SPECIAL && self.ep != crate::types::NO_EP {
            let ep = self.ep;
            let cap_sq = if WHITE { ep - 8 } else { ep + 8 };
            // Iterate candidate pawns that geometrically can capture EP.
            let mut p = if cfg!(feature = "exp-ep-candidates") {pawns & PAWN_ATT[us.other().index()][ep as usize]} else {pawns};
            while p != 0 {
                let from = pop_lsb(&mut p);
                let att = PAWN_ATT[ui][from as usize];
                if att & bit(ep) == 0 {
                    continue;
                }
                // For en-passant to be pseudo-legal, the capture must also respect
                // check_mask when in check (EP capturing the checker or blocking).
                // The allowed mask already encodes check evasion; EP destination must be in allowed
                // or the capture must expose? Actually EP capture's destination is ep, so if checkers
                // exists, ep must be capture/block. Our generic allowed check applies:
                // if checkers !=0 and bit(ep) not in allowed, EP cannot be legal (except when
                // EP captures checker pawn? But pawn double push checker is not on ep square.
                // We'll enforce allowed for EP as well.
                if allowed != !0u64 && bit(ep) & allowed == 0 {
                    // However EP can still be legal when it captures the checker pawn that is not on ep
                    // (the pawn that moved 2 squares). In double-check scenario we already early-return.
                    // In single check, EP captures the pawn that moved, but checker is that pawn? No.
                    // Double push checker is pawn on its destination? Actually checker could be pawn that just pushed
                    // but EP captures it on different square (the passed square). That EP capture does capture checker
                    // even though destination not equal checker square, because checker is on cap_sq? Wait.
                    // For EP to capture checker, the pawn that gave check via double push would be on cap_sq?
                    // Let's keep simple: EP legality is decided by simulation below, which includes discovered check.
                    // So we still simulate even if bit(ep) not in allowed — simulation will show attacker still exists.
                    // So we don't skip on allowed for EP; we simulate and let attackers decide.
                }
                let occ_after = (occ ^ bit(from) ^ bit(cap_sq)) | bit(ep);
                let ti = us.other().index();
                let their_pawns = self.bb(ti, Role::Pawn.index()) & !bit(cap_sq);
                let ksq_after = self.king_sq[ui];
                // Use the occupancy after EP for sliding attacks.
                let mut attackers = attacks::bishop_attacks(ksq_after, occ_after)
                    & (self.diagonal_sliders(ti));
                attackers |= attacks::rook_attacks(ksq_after, occ_after)
                    & (self.orthogonal_sliders(ti));
                attackers |= KNIGHT_ATT[ksq_after as usize] & self.bb(ti, Role::Knight.index());
                attackers |= PAWN_ATT[ui][ksq_after as usize] & their_pawns;
                attackers |= KING_ATT[ksq_after as usize] & self.bb(ti, Role::King.index());
                if attackers == 0 {
                    sink.push_one(Move::new(Square(from), Square(ep), None));
                }
            }
        }
    }

    /// Castling generation via `MoveSink` (never while in check; caller gates).
    fn gen_castling_sink<S: MoveSink>(&self, sink: &mut S, danger: u64) {
        let us = self.turn;
        let ui = us.index();
        let ksq = self.king_sq[ui];
        #[cfg(feature = "exp-standard-movegen")]
        if !self.is_chess960 && ksq==if us==Color::White {4}else{60} {
            let shift=if us==Color::White {0}else{56};
            let rights=(self.castling>>if us==Color::White {0}else{2})&3;
            let occ=self.occupied()>>shift;let safe=danger>>shift;
            if rights&1!=0 && occ&0x60==0 && safe&0x60==0 {sink.push_one(Move::quiet(Square(ksq),Square(shift+7)));}
            if rights&2!=0 && occ&0x0e==0 && safe&0x0c==0 {sink.push_one(Move::quiet(Square(ksq),Square(shift)));}
            return;
        }
        for rb in [
            crate::types::castle_right_bit(us, true) as usize,
            crate::types::castle_right_bit(us, false) as usize,
        ] {
            if self.castling & (1 << rb) == 0 {
                continue;
            }
            let rook = self.castle_rook_sq[rb];
            let kingside = rb % 2 == 0;
            if ksq & 7 == 4 && (rook & 7) == if kingside { 7 } else { 0 } {
                let rank = ksq >> 3;
                let kf = (rank << 3) | if kingside { 6 } else { 2 };
                let between = if kingside {
                    bit(kf - 1) | bit(kf)
                } else {
                    bit(kf - 1) | bit(kf) | bit(kf + 1)
                };
                let safe = if kingside {
                    bit(kf - 1) | bit(kf)
                } else {
                    bit(kf) | bit(kf + 1)
                };
                if self.occupied() & between == 0 && danger & safe == 0 {
                    sink.push_one(Move::new(Square(ksq), Square(rook), None));
                }
            } else if self.castle_path(rb).is_some() {
                sink.push_one(Move::new(Square(ksq), Square(rook), None));
            }
        }
    }

    /// Pawn moves: single/double pushes, captures, promotions, en passant.
    fn gen_pawn_moves(
        &self,
        moves: &mut ArrayVec<Move, MAX_MOVES>,
        allowed: u64,
        pinned: u64,
        pin_line: &[u64; 64],
    ) {
        let us = self.turn;
        let them = us.other();
        let ui = us.index();
        let white = us == Color::White;
        let occ = self.occupied();
        let them_occ = self.occ[them.index()];
        let start_rank: u8 = if white { 1 } else { 6 };
        let promo_rank: u8 = if white { 7 } else { 0 };

        let mut p = self.bb(ui, Role::Pawn.index());
        while p != 0 {
            let from = pop_lsb(&mut p);
            let allowed_here = if pinned & bit(from) != 0 {
                allowed & pin_line[from as usize]
            } else {
                allowed
            };

            // Pushes.
            let one_empty = (if white { bit(from) << 8 } else { bit(from) >> 8 }) & !occ;
            if one_empty != 0 {
                if from / 8 == start_rank {
                    let two = (if white { one_empty << 8 } else { one_empty >> 8 })
                        & !occ
                        & allowed_here;
                    if two != 0 {
                        let to = crate::bitboard::lsb(two);
                        moves.push(Move::new(Square(from), Square(to), None));
                    }
                }
                if one_empty & allowed_here != 0 {
                    let to = crate::bitboard::lsb(one_empty);
                    if to / 8 == promo_rank {
                        for r in [Role::Queen, Role::Rook, Role::Bishop, Role::Knight] {
                            moves.push(Move::new(Square(from), Square(to), Some(r)));
                        }
                    } else {
                        moves.push(Move::new(Square(from), Square(to), None));
                    }
                }
            }

            // Captures.
            let att = PAWN_ATT[ui][from as usize];
            let mut caps = att & them_occ & allowed_here;
            while caps != 0 {
                let to = pop_lsb(&mut caps);
                if to / 8 == promo_rank {
                    for r in [Role::Queen, Role::Rook, Role::Bishop, Role::Knight] {
                        moves.push(Move::new(Square(from), Square(to), Some(r)));
                    }
                } else {
                    moves.push(Move::new(Square(from), Square(to), None));
                }
            }

            // En passant: verified by direct simulation, which correctly
            // handles rank pins through both disappearing pawns and
            // check-evasion-by-EP (capturing the double-pushed checker).
            if self.ep != NO_EP && att & bit(self.ep) != 0 {
                let ep = self.ep;
                let cap_sq = if white { ep - 8 } else { ep + 8 };
                let occ_after = (occ ^ bit(from) ^ bit(cap_sq)) | bit(ep);
                let ti = them.index();
                let their_pawns = self.bb(ti, Role::Pawn.index()) & !bit(cap_sq);
                let ksq = self.king_sq[ui];
                let mut attackers =
                    attacks::bishop_attacks(ksq, occ_after)
                        & (self.diagonal_sliders(ti));
                attackers |= attacks::rook_attacks(ksq, occ_after)
                    & (self.orthogonal_sliders(ti));
                attackers |= KNIGHT_ATT[ksq as usize] & self.bb(ti, Role::Knight.index());
                attackers |= PAWN_ATT[ui][ksq as usize] & their_pawns;
                attackers |= KING_ATT[ksq as usize] & self.bb(ti, Role::King.index());
                if attackers == 0 {
                    moves.push(Move::new(Square(from), Square(ep), None));
                }
            }
        }
    }

    /// Validates one castling right (path-based, ADR-003 decision D4):
    /// all squares strictly between king and rook, the king's transit squares
    /// and both final squares empty (the king's and rook's own squares
    /// excluded), and every square on the king's path safe in the
    /// post-castling occupancy (king removed for x-rays, rook displaced to
    /// its final square — which also enforces the rook-not-pinned rule in
    /// Chess960 rank-pin configurations).
    /// Returns the (king_final, rook_final) squares.
    fn castle_path(&self, right_bit: usize) -> Option<(u8, u8)> {
        let ksq = self.king_sq[self.turn.index()];
        let rook = self.castle_rook_sq[right_bit];
        let kingside = (right_bit & 1) == 0; // bits 0, 2 = kingside; 1, 3 = queenside
        let rank = ksq >> 3;
        let kf = (rank << 3) | if kingside { 6 } else { 2 };
        let rf = (rank << 3) | if kingside { 5 } else { 3 };
        let them = self.turn.other();

        // Emptiness: strictly between king and rook, king's transit squares,
        // and both final squares — minus the king's and rook's own squares
        // (evaluated via precomputed CASTLE_PATH bitmask table).
        let need_empty = CASTLE_PATH[(ksq & 7) as usize][(rook & 7) as usize] << (rank << 3);
        if self.occupied() & need_empty != 0 {
            return None;
        }

        // King-path safety in the resulting occupancy: the king vacates its
        // square (x-rays through it count) and the rook leaves its square and
        // lands on `rf`.
        let occ_after = (self.occupied() & !(bit(ksq) | bit(rook))) | bit(kf) | bit(rf);
        let mut path = BETWEEN[ksq as usize][kf as usize] | bit(ksq) | bit(kf);
        while path != 0 {
            let s = pop_lsb(&mut path);
            if self.attackers_to(s, them, occ_after) != 0 {
                return None;
            }
        }
        Some((kf, rf))
    }

    /// Full legality of a castling move encoded **king-from → rook-square**.
    fn is_castling_move(&self, from: u8, to: u8) -> bool {
        let us = self.turn;
        let ui = us.index();
        if from != self.king_sq[ui] {
            return false;
        }
        let kingside = (to & 7) > (from & 7);
        let rb = crate::types::castle_right_bit(us, kingside) as usize;
        if self.castling & (1 << rb) == 0 || self.castle_rook_sq[rb] != to {
            return false;
        }
        self.castle_path(rb).is_some()
    }

    /// Castling generation (never while in check; the caller gates on
    /// `checkers == 0`). Emits moves as **king-from → rook-square**
    /// (ADR-003, decision D3) for both standard chess and Chess960; the
    /// destination square holds the mover's own rook.
    #[allow(dead_code)]
    fn gen_castling(&self, moves: &mut ArrayVec<Move, MAX_MOVES>, danger: u64) {
        let us = self.turn;
        let ui = us.index();
        let ksq = self.king_sq[ui];
        for rb in [
            crate::types::castle_right_bit(us, true) as usize,
            crate::types::castle_right_bit(us, false) as usize,
        ] {
            if self.castling & (1 << rb) == 0 {
                continue;
            }
            let rook = self.castle_rook_sq[rb];
            let kingside = rb % 2 == 0;
            // Standard-geometry fast path (king on the e-file, rook on the
            // a/h-file): provably identical to the path-based check — the
            // h1/a1/d1/f1 square set means the rook can neither uncover nor
            // block a relevant attack ray — but reuses the precomputed
            // king-removed danger map instead of per-square attack probes.
            if ksq & 7 == 4 && (rook & 7) == if kingside { 7 } else { 0 } {
                let rank = ksq >> 3;
                let kf = (rank << 3) | if kingside { 6 } else { 2 };
                let between = if kingside {
                    bit(kf - 1) | bit(kf) // f, g
                } else {
                    bit(kf - 1) | bit(kf) | bit(kf + 1) // b, c, d
                };
                let safe = if kingside {
                    bit(kf - 1) | bit(kf) // f, g
                } else {
                    bit(kf) | bit(kf + 1) // c, d
                };
                if self.occupied() & between == 0 && danger & safe == 0 {
                    moves.push(Move::new(Square(ksq), Square(rook), None));
                }
            } else if self.castle_path(rb).is_some() {
                moves.push(Move::new(Square(ksq), Square(rook), None));
            }
        }
    }
}

impl Board {
    /// Generates every pseudo-legal move into a stack-allocated
    /// `ArrayVec<Move, 256>` (zero heap allocations): piece moves without
    /// king-safety filtering, for engines that apply their own legality
    /// handling (ADR-003, decision D6).
    ///
    /// Castling words carry full path-based legality (the king-path safety
    /// check cannot be restored by a later filter), so:
    /// `legal_moves() == pseudo_legal_moves().filter(|m| { let mut b = *self;
    /// let mover = b.turn(); b.make_move_unchecked(m); !b.king_attacked(mover) })`.
    pub fn pseudo_legal_moves(&self) -> ArrayVec<Move, MAX_MOVES> {
        let us = self.turn;
        let ui = us.index();
        let occ = self.occupied();
        let mut moves = ArrayVec::new();

        // Knights.
        let mut p = self.bb(ui, Role::Knight.index());
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = KNIGHT_ATT[from as usize] & !self.occ[ui];
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        // Bishops and queens (diagonals).
        let mut p = self.diagonal_sliders(ui);
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = attacks::bishop_attacks(from, occ) & !self.occ[ui];
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        // Rooks and queens (straights).
        let mut p = self.orthogonal_sliders(ui);
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = attacks::rook_attacks(from, occ) & !self.occ[ui];
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        // King steps (castling handled below with full path legality).
        let ksq = self.king_sq[ui];
        let mut p = KING_ATT[ksq as usize] & !self.occ[ui];
        while p != 0 {
            let to = pop_lsb(&mut p);
            moves.push(Move::new(Square(ksq), Square(to), None));
        }

        // Pawns: pushes, captures, promotions, en passant (en passant is
        // verified by direct simulation inside the generator, castling by the
        // full path check below — both are safe under a king-safety filter).
        self.gen_pawn_moves(&mut moves, !0u64, 0, &[0u64; 64]);

        let wk = crate::types::castle_right_bit(us, true) as usize;
        let wq = crate::types::castle_right_bit(us, false) as usize;
        for rb in [wk, wq] {
            if self.castling & (1 << rb) != 0 && self.castle_path(rb).is_some() {
                moves.push(Move::new(
                    Square(self.king_sq[ui]),
                    Square(self.castle_rook_sq[rb]),
                    None,
                ));
            }
        }

        moves
    }
}

impl Board {
    // -- making and unmaking moves ------------------------------------------

    /// Applies `mv` without any legality validation (fast internal path).
    /// Returns the [`Undo`] information needed by `unmake_move`.
    ///
    /// Maintains `checkers` (cached attackers to the new side-to-move's king)
    /// for branch-free `in_check()` (0.32ns) and increments/decrements `hash`
    /// for `zobrist()` load (0.34ns). Cost +2ns/make `attackers_to` to refresh
    /// `checkers` is kept per `BENCH.md: Deliberate make+unmake tradeoff`.
    #[inline(always)]
    fn make_move_unchecked_with<const HASH: bool, const CHECKERS: bool>(
        &mut self,mv:Move
    )->Undo{
        #[cfg(feature="exp-split-quiet")]
        {
            let from=mv.from().0;let to=mv.to().0;
            let us=self.turn;let them=us.other();let ui=us.index();let ti=them.index();
            let target=bit(to);
            if mv.promotion().is_none()&&self.occupied()&target==0 {
                let moved=self.piece_code_at_color(from,us);
                let role=(moved%6) as usize;
                let pawn=role==Role::Pawn.index();
                if moved!=EMPTY&&!(pawn&&from&7!=to&7){
                    let undo=Undo{hash:self.hash,checkers:self.checkers,castling:self.castling,ep:self.ep,halfmove:self.halfmove,captured:EMPTY,
                        #[cfg(feature="exp-undo-role")] moved,
                        castled:false};
                    let mut hash=self.hash;
                    if HASH&&self.ep_relevant(self.ep,us){hash^=zobrist::ep_key(Square(self.ep));}
                    let delta=bit(from)^target;
                    #[cfg(feature="exp-mutation")]
                    self.bb_toggle(ui,role,delta);
                    #[cfg(not(feature="exp-mutation"))]
                    {self.bb_remove(ui,role,bit(from));self.bb_add(ui,role,target);}
                    self.occ[ui]^=delta;
                    hash^=zobrist::piece_key_indices(ui,role,from)^zobrist::piece_key_indices(ui,role,to);
                    if role==Role::King.index(){self.king_sq[ui]=to;}
                    if self.castling!=0{
                        let next=self.castling&self.castle_rights_after(role as u8,us,from,to);
                        if HASH&&next!=self.castling{hash^=self.castle_rights_hash(self.castling^next);}
                        self.castling=next;
                    }
                    self.ep=if pawn&&to.abs_diff(from)==16{(from+to)/2}else{NO_EP};
                    if HASH&&self.ep_relevant(self.ep,them){hash^=zobrist::ep_key(Square(self.ep));}
                    self.halfmove=if pawn{0}else{self.halfmove+1};
                    self.fullmove+=(us==Color::Black) as u16;
                    self.turn=them;
                    if HASH{hash^=zobrist::turn_key();}
                    self.hash=hash;
                    if CHECKERS{self.checkers=self.attackers_to(self.king_sq[ti],us,self.occupied());}
                    return undo;
                }
            }
        }
        self.make_move_hinted::<HASH,CHECKERS,false>(mv,EMPTY)
    }

    #[inline(always)]
    fn make_move_hinted<const HASH:bool,const CHECKERS:bool,const HINT:bool>(&mut self,mv:Move,code:u8)->Undo{
        #[cfg(feature="exp-color-make")]
        {
            if self.turn==Color::White{return self.make_move_impl::<HASH,CHECKERS,HINT,1,false>(mv,code,EMPTY);}
            return self.make_move_impl::<HASH,CHECKERS,HINT,0,false>(mv,code,EMPTY);
        }
        #[cfg(not(feature="exp-color-make"))]
        self.make_move_impl::<HASH,CHECKERS,HINT,2,false>(mv,code,EMPTY)
    }

    #[cfg_attr(feature="exp-inline-make",inline(always))]
    fn make_move_impl<const HASH:bool,const CHECKERS:bool,const HINT:bool,const SIDE:u8,const PREPARED:bool>(
        &mut self,mv:Move,hint:u8,capture_hint:u8
    )->Undo{
        // A pass is not a move with squares. Every entry that could reach here
        // dispatches `Move::NULL` onto the null transition first, so finding one
        // means a new entry point bypassed that dispatch — fail loudly in debug
        // rather than corrupting the board in release.
        debug_assert!(!mv.is_null(), "null move reached the generic make body");
        let mut undo = Undo {
            hash: self.hash,
            checkers: self.checkers,
            castling: self.castling,
            ep: self.ep,
            halfmove: self.halfmove,
            captured: EMPTY,
            #[cfg(feature = "exp-undo-role")]
            moved: EMPTY,
            castled: false,
        };
        let from = mv.from().0;
        let to = mv.to().0;
        let us=if SIDE==1{Color::White}else if SIDE==0{Color::Black}else{self.turn};
        let them = us.other();
        #[cfg(feature="exp-pawn-kernel")]
        if !PREPARED && mv.promotion().is_none() && from&7==to&7 {
            let ui=us.index();let ti=them.index();let f=bit(from);let t=bit(to);
            if self.bb(ui,0)&f!=0 && self.occupied()&t==0 {
                #[cfg(feature="exp-undo-role")]
                {undo.moved=us as u8*6;}
                let mut hash=self.hash;
                if HASH&&self.ep_relevant(self.ep,us){hash^=zobrist::ep_key(Square(self.ep));}
                let delta=f^t;
                #[cfg(feature="exp-mutation")]
                self.bb_toggle(ui,0,delta);
                #[cfg(not(feature="exp-mutation"))]
                {self.bb_remove(ui,0,f);self.bb_add(ui,0,t);}
                self.occ[ui]^=delta;
                hash^=zobrist::piece_move_delta::<HASH>(ui,0,from,to);
                // Standard rights only change on the back ranks. Preserve
                // nonstandard rights behavior, including permissive inputs.
                if self.castling!=0 && (self.is_chess960||((f|t)&0xff000000000000ff)!=0){
                    let old=self.castling;
                    self.castling&=self.castle_rights_after(0,us,from,to);
                    if HASH&&old!=self.castling{hash^=self.castle_rights_hash(old^self.castling);}
                }
                self.ep=if from.abs_diff(to)==16{(from+to)/2}else{NO_EP};
                if HASH&&self.ep_relevant(self.ep,them){hash^=zobrist::ep_key(Square(self.ep));}
                self.halfmove=0;
                self.fullmove+=(us==Color::Black) as u16;
                self.turn=them;self.hash=hash;
                if CHECKERS{self.checkers=self.attackers_to(self.king_sq[ti],us,self.occupied());}
                return undo;
            }
        }

        #[cfg(feature="exp-known-role")]
        let role=if HINT{hint%6}else{self.role_at_occupied(from,us)};
        #[cfg(feature="exp-known-role")]
        let moved=role+us as u8*6;
        #[cfg(not(feature="exp-known-role"))]
        let moved=if HINT{hint}else{self.piece_code_at_color(from,us)};
        #[cfg(feature = "exp-undo-role")]
        { undo.moved = mv.promotion().map_or(moved, |r| Piece::new(us,r).code()); }
        debug_assert_ne!(moved, EMPTY, "make_move on empty square");
        #[cfg(not(feature="exp-known-role"))]
        let role = moved % 6;
        let is_pawn = role == Role::Pawn as u8;
        let diag = from & 7 != to & 7;

        #[cfg(feature="exp-ordinary-kernel")]
        if !PREPARED && mv.promotion().is_none() {
            let ui=us.index();let ti=them.index();
            let target=bit(to);let enemy=self.occ[ti]&target;
            if self.occ[ui]&target==0 && !(is_pawn&&diag&&enemy==0) {
                let mut hash=self.hash;
                if HASH&&self.ep_relevant(self.ep,us){hash^=zobrist::ep_key(Square(self.ep));}
                if enemy!=0 {
                    #[cfg(feature="exp-known-role")]
                    let cap_role=self.role_at_occupied(to,them) as usize;
                    #[cfg(not(feature="exp-known-role"))]
                    let cap_role=(self.piece_code_at_color(to,them)%6) as usize;
                    undo.captured=(ti*6+cap_role) as u8;
                    self.bb_remove(ti,cap_role,target);
                    self.occ[ti]^=target;
                    hash^=zobrist::piece_key_indices(ti,cap_role,to);
                }
                let r=role as usize;let delta=bit(from)^target;
                #[cfg(feature="exp-mutation")]
                self.bb_toggle(ui,r,delta);
                #[cfg(not(feature="exp-mutation"))]
                {self.bb_remove(ui,r,bit(from));self.bb_add(ui,r,target);}
                self.occ[ui]^=delta;
                hash^=zobrist::piece_move_delta::<HASH>(ui,r,from,to);
                if r==Role::King.index(){self.king_sq[ui]=to;}
                let old=self.castling;
                if old!=0{
                    let next=old&self.castle_rights_after(role,us,from,to);
                    if HASH&&old!=next{hash^=self.castle_rights_hash(old^next);}
                    self.castling=next;
                }
                self.ep=NO_EP;
                if is_pawn {
                    self.halfmove=0;
                    if from.abs_diff(to)==16{
                        self.ep=(from+to)/2;
                        if HASH&&self.ep_relevant(self.ep,them){hash^=zobrist::ep_key(Square(self.ep));}
                    }
                }else{self.halfmove=if enemy!=0{0}else{self.halfmove+1};}
                self.fullmove+=(us==Color::Black) as u16;
                self.turn=them;
                self.hash=hash;
                if CHECKERS{self.checkers=self.attackers_to(self.king_sq[ti],us,self.occupied());}
                return undo;
            }
        }

        // Hash out the old en-passant contribution (relevance uses the
        // mover's pawns, which have not changed yet).
        if HASH && self.ep_relevant(self.ep, us) {
            self.hash ^= zobrist::ep_key(Square(self.ep));
        }

        // Independent normal-move kernel: avoid decoding color/role a second
        // time, keep a single XOR for the moved role and color occupancy.
        #[cfg(feature = "exp-normal-kernel")]
        if PREPARED || (mv.promotion().is_none() && self.occ[us.index()] & bit(to) == 0
            && !(is_pawn && diag && self.occ[them.index()] & bit(to) == 0))
        {
            let ui=us.index(); let ti=them.index(); let target=bit(to);
            if if PREPARED {capture_hint!=EMPTY}else{self.occ[ti]&target!=0} {
                let cap=if PREPARED {capture_hint}else{self.piece_code_at_color(to,them)};
                undo.captured=cap;
                self.bb_remove(ti,(cap%6) as usize,target);
                self.occ[ti]&=!target;
                self.hash^=zobrist::piece_key_indices(ti,(cap%6) as usize,to);
            }
            let delta=bit(from)^target;
            #[cfg(feature="exp-mutation")]
            self.bb_toggle(ui,role as usize,delta);
            #[cfg(not(feature="exp-mutation"))]
            {self.bb_remove(ui,role as usize,bit(from));self.bb_add(ui,role as usize,target);}
            self.occ[ui]^=delta;
            self.hash^=zobrist::piece_move_delta::<false>(ui,role as usize,from,to);
            if role==Role::King as u8 {self.king_sq[ui]=to;}
            if self.castling!=0 {
                let next=self.castling&self.castle_rights_after(role,us,from,to);
                if HASH && next!=self.castling {self.hash^=self.castle_rights_hash(self.castling^next);}
                self.castling=next;
            }
            self.ep=if is_pawn&&to.abs_diff(from)==16 {(from+to)/2}else{NO_EP};
            if HASH&&self.ep_relevant(self.ep,them){self.hash^=zobrist::ep_key(Square(self.ep));}
            self.halfmove=if is_pawn||undo.captured!=EMPTY {0}else{self.halfmove+1};
            self.fullmove+= (us==Color::Black) as u16;
            self.turn=them;
            if HASH{self.hash^=zobrist::turn_key();}
            if CHECKERS{self.checkers=self.attackers_to(self.king_sq[ti],us,self.occupied());}
            return undo;
        }

        #[cfg(all(feature = "exp-quiet-fast", not(feature = "exp-normal-kernel")))]
        if mv.promotion().is_none() && self.occupied()&bit(to)==0 && !(is_pawn&&diag) {
            let ui=us.index();let ti=them.index();
            self.bb_remove(ui,role as usize,bit(from));
            self.bb_add(ui,role as usize,bit(to));
            self.occ[ui]^=bit(from)^bit(to);
            if role==Role::King as u8 {self.king_sq[ui]=to;}
            // Preserve the baseline's partial/stale piece hash update on CHECKERS-only makes.
            self.hash^=zobrist::piece_move_delta::<false>(ui,role as usize,from,to);
            if self.castling!=0 {
                let new_castling=self.castling&self.castle_rights_after(role,us,from,to);
                if HASH && new_castling!=self.castling {self.hash^=self.castle_rights_hash(self.castling^new_castling);}
                self.castling=new_castling;
            }
            self.ep=if is_pawn&&to.abs_diff(from)==16 {(from+to)/2}else{NO_EP};
            if HASH&&self.ep_relevant(self.ep,them){self.hash^=zobrist::ep_key(Square(self.ep));}
            if is_pawn {self.halfmove=0;}else{self.halfmove+=1;}
            if us==Color::Black{self.fullmove+=1;}
            self.turn=them;
            if HASH{self.hash^=zobrist::turn_key();}
            if CHECKERS{self.checkers=self.attackers_to(self.king_sq[ti],us,self.occupied());}
            return undo;
        }

        // Castling is encoded king-from → rook-square (ADR-003, decision D3):
        // a king move landing on the mover's own rook. Unambiguous — a normal
        // king move can never land on an own rook. Fast dest lookup: own vs them.
        let dest = if self.occ[us.index()] & bit(to) != 0 {
            self.piece_code_at_color(to, us)
        } else if self.occ[them.index()] & bit(to) != 0 {
            self.piece_code_at_color(to, them)
        } else {
            EMPTY
        };
        let is_castle = role == Role::King as u8
            && dest != EMPTY
            && dest % 6 == Role::Rook as u8
            && (dest / 6) == us as u8;

        // Captures (including en passant onto an empty square).
        let mut captured_sq = to;
        if is_pawn && diag && dest == EMPTY {
            captured_sq = if us == Color::White { to - 8 } else { to + 8 };
        }
        if !is_castle && captured_sq != from {
            if captured_sq != to {
                // En passant: captured pawn is always opponent pawn on cap_sq.
                let cap = self.pawn_code_at(captured_sq, them);
                if cap != EMPTY {
                    undo.captured = cap;
                    self.remove_piece(captured_sq, cap);
                }
            } else if dest != EMPTY {
                // Normal capture: dest already holds the captured piece.
                undo.captured = dest;
                self.remove_piece(to, dest);
            }
        }

        // Move the piece (handling promotion and castling rook relocation).
        match mv.promotion() {
            Some(r) => {
                self.remove_piece(from, moved);
                self.put_piece(to, Piece::new(us, r).code());
            }
            None => {
                if is_castle {
                    let rank = from >> 3;
                    let kingside = (to & 7) > (from & 7);
                    let kf = (rank << 3) | if kingside { 6 } else { 2 };
                    let rf = (rank << 3) | if kingside { 5 } else { 3 };
                    undo.castled = true;
                    // Chess960: the king's destination can be the rook's
                    // square and vice versa (adjacent K+R castling swaps
                    // them), so remove both pieces first, then place both.
                    let rook_code = dest;
                    self.remove_piece(from, moved);
                    self.remove_piece(to, rook_code);
                    self.put_piece(kf, moved);
                    self.put_piece(rf, rook_code);
                } else {
                    self.move_piece(from, to, moved);
                }
            }
        }

        // Castling rights (touching king/rook squares or rook captured) —
        // derived from `castle_rook_sq` + mover role (close-gap D3, task 4.1).
        let new_castling = self.castling & self.castle_rights_after(role, us, from, to);
        if new_castling != self.castling {
            if HASH {
                self.hash ^= self.castle_rights_hash(self.castling ^ new_castling);
            }
            self.castling = new_castling;
        }

        // En-passant square after a double pawn push.
        self.ep = if is_pawn && to.abs_diff(from) == 16 {
            (from + to) / 2
        } else {
            NO_EP
        };
        if HASH && self.ep_relevant(self.ep, us.other()) {
            self.hash ^= zobrist::ep_key(Square(self.ep));
        }

        // Clocks.
        if is_pawn || undo.captured != EMPTY {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }
        if us == Color::Black {
            self.fullmove += 1;
        }

        self.turn = us.other();
        if HASH {
            self.hash ^= zobrist::turn_key();
        }
        // Refresh cached checkers for the new side to move (+2ns/make, D3).
        if CHECKERS {
            self.checkers = self.attackers_to(
                self.king_sq[self.turn.index()],
                self.turn.other(),
                self.occupied(),
            );
        }
        undo
    }

    /// Prepare a move already known legal for this exact board. Normal moves
    /// carry moving and captured piece codes; rare specials use the wire path.
    #[cfg(feature="exp-prepared-move")]
    pub fn prepare_move_unchecked(&self,mv:Move)->PreparedMove{
        if mv.is_null(){return PreparedMove{mv,meta:0};}
        let f=mv.from().0;let t=mv.to().0;
        let moved=self.piece_code_at_color(f,self.turn);
        let captured=self.piece_code_at_color(t,self.turn.other());
        let normal=moved!=EMPTY && mv.promotion().is_none() &&
            self.occ[self.turn.index()]&bit(t)==0 &&
            !(moved%6==Role::Pawn as u8&&f&7!=t&7&&captured==EMPTY);
        PreparedMove{mv,meta:(moved as u16&15)|((captured as u16&15)<<4)|((normal as u16)<<8)}
    }

    /// Apply position-specific prepared metadata with full hash and checker
    /// maintenance. Caller must use the board the token was prepared for.
    #[cfg(feature="exp-prepared-move")]
    #[inline(always)]
    pub fn make_prepared_unchecked(&mut self,m:PreparedMove)->Undo{
        if m.meta&256==0{return self.make_move_unchecked(m.mv);}
        let moved=(m.meta&15) as u8;
        let cap=((m.meta>>4)&15) as u8;let cap=if cap==15{EMPTY}else{cap};
        debug_assert_eq!(moved,self.piece_code_at_color(m.mv.from().0,self.turn));
        debug_assert_eq!(cap,self.piece_code_at_color(m.mv.to().0,self.turn.other()));
        #[cfg(feature="exp-color-make")]
        {
            if self.turn==Color::White{return self.make_move_impl::<true,true,true,1,true>(m.mv,moved,cap);}
            return self.make_move_impl::<true,true,true,0,true>(m.mv,moved,cap);
        }
        #[cfg(not(feature="exp-color-make"))]
        self.make_move_impl::<true,true,true,2,true>(m.mv,moved,cap)
    }

    /// Construct the exact full-state child of a trusted move. Unlike
    /// make/unmake, this path does not need to build an Undo record.
    #[cfg(feature="exp-child-copy")]
    #[inline(always)]
    pub fn child_after_unchecked(&self,mv:Move)->Board{
        if !mv.is_null()&&mv.promotion().is_none(){
            let us=self.turn;let them=us.other();let ui=us.index();let ti=them.index();
            let from=mv.from().0;let to=mv.to().0;let f=bit(from);let t=bit(to);
            let moved=self.piece_code_at_color(from,us);
            let role=(moved%6) as usize;
            let enemy=self.occ[ti]&t;
            if moved!=EMPTY&&self.occ[ui]&t==0&&!(role==0&&from&7!=to&7&&enemy==0){
                let mut child=*self;
                let mut hash=self.hash;
                if self.ep_relevant(self.ep,us){hash^=zobrist::ep_key(Square(self.ep));}
                if enemy!=0 {
                    let capture=(self.piece_code_at_color(to,them)%6) as usize;
                    child.bb_remove(ti,capture,t);child.occ[ti]^=t;
                    hash^=zobrist::piece_key_indices(ti,capture,to);
                }
                #[cfg(feature="exp-mutation")]
                child.bb_toggle(ui,role,f^t);
                #[cfg(not(feature="exp-mutation"))]
                {child.bb_remove(ui,role,f);child.bb_add(ui,role,t);}
                child.occ[ui]^=f^t;
                hash^=zobrist::piece_move_delta::<true>(ui,role,from,to);
                if role==5{child.king_sq[ui]=to;}
                if self.castling!=0{
                    child.castling=self.castling&self.castle_rights_after(role as u8,us,from,to);
                    if self.castling!=child.castling{hash^=self.castle_rights_hash(self.castling^child.castling);}
                }
                child.ep=if role==0&&from.abs_diff(to)==16{(from+to)/2}else{NO_EP};
                if child.ep_relevant(child.ep,them){hash^=zobrist::ep_key(Square(child.ep));}
                child.halfmove=if role==0||enemy!=0{0}else{self.halfmove+1};
                child.fullmove+=(us==Color::Black) as u16;
                child.turn=them;child.hash=hash;
                child.checkers=child.attackers_to(child.king_sq[ti],us,child.occupied());
                return child;
            }
        }
        let mut child=*self;
        child.make_move_unchecked(mv);
        child
    }

    /// Applies `mv` without legality validation, maintaining the incremental
    /// Polyglot hash and the cached `checkers`. The default for callers that read
    /// both.
    ///
    /// A [`Move::NULL`] is dispatched onto the null-move transition, unchecked
    /// like any other move here; pair it with [`Board::unmake_null_move`].
    #[inline(always)]
    pub fn make_move_unchecked(&mut self, mv: Move) -> Undo {
        if mv.is_null() {
            debug_assert!(
                self.pass_is_legal(),
                "make_move_unchecked(Move::NULL) while in check"
            );
            return self.null_transition::<true, true>();
        }
        self.make_move_unchecked_with::<true, true>(mv)
    }

    /// Applies `mv` without legality validation, maintaining the incremental
    /// Polyglot hash but **not** the cached `checkers` — for a caller that
    /// indexes positions and never asks `in_check()`. Skips the `attackers_to`
    /// refresh the caching make pays.
    ///
    /// `zobrist()` stays correct; `in_check()` is **stale** and must not be read.
    #[inline(always)]
    pub fn make_move_hashed(&mut self, mv: Move) -> Undo {
        if mv.is_null() {
            debug_assert!(
                self.pass_is_legal(),
                "make_move_hashed(Move::NULL) while in check"
            );
            return self.null_transition::<true, false>();
        }
        self.make_move_unchecked_with::<true, false>(mv)
    }

    /// Applies `mv` without legality validation, maintaining the cached
    /// `checkers` but **not** the incremental Polyglot hash — for a caller that
    /// renders SAN and never indexes a position.
    ///
    /// `in_check()` stays correct; `zobrist()` is **stale** and must not be read.
    #[inline(always)]
    pub fn make_move_checkered(&mut self, mv: Move) -> Undo {
        if mv.is_null() {
            debug_assert!(
                self.pass_is_legal(),
                "make_move_checkered(Move::NULL) while in check"
            );
            return self.null_transition::<false, true>();
        }
        self.make_move_unchecked_with::<false, true>(mv)
    }

    /// Plays a null move (pass): the position stays, the side to move flips.
    ///
    /// Semantics mirror an ordinary move's state transition: castling rights are untouched, the
    /// en-passant square clears (any pending double push lapses), the halfmove clock advances and
    /// the move always completes a full move (a pass is a move, not a half-move: after the pass
    /// White is to move with the number advanced, whoever passed). The incremental Polyglot hash
    /// and the cached `checkers` are maintained, so [`Board::zobrist`] still equals
    /// [`Board::zobrist_full`] and [`Board::in_check`] stays branch-free.
    ///
    /// Returns [`IllegalMove`] when the side to move is in check: a null move answers no check.
    /// Unlike a normal move there is no [`Move`] to play or unmake, so this takes and returns
    /// nothing but the [`Undo`] snapshot.
    ///
    /// This is the primitive CBH null-move tokens (`0xffff` in `moves2`) decode onto: the format
    /// stores pass turns as moves, and legality of the surrounding moves is checked against the
    /// position it produces.
    #[inline(always)]
    fn make_null_move_with<const HASH: bool, const CHECKERS: bool>(
        &mut self,
    ) -> Result<Undo, IllegalMove> {
        if !self.pass_is_legal() {
            return Err(IllegalMove);
        }
        Ok(self.null_transition::<HASH, CHECKERS>())
    }

    /// Whether a pass is legal here: the side to move must not be in check.
    ///
    /// This asks the bitboards, never the cache — in every variant, not just the
    /// fast one. A caller that reached here after `play_fast` (or any other
    /// cache-free make) holds a `checkers` value that describes some earlier
    /// position, and reading it would allow a pass while the side to move is in
    /// check. The scan costs one `attackers_to` on a path taken roughly once per
    /// ten thousand plies.
    #[inline(always)]
    fn pass_is_legal(&self) -> bool {
        self.attackers_to(
            self.king_sq[self.turn.index()],
            self.turn.other(),
            self.occupied(),
        ) == 0
    }

    /// The null-move state transition, with no legality test.
    ///
    /// Split out of [`Board::make_null_move_with`] so that the raw unchecked make
    /// entries can dispatch a [`Move::NULL`] onto exactly the same transition the
    /// validating entries use, instead of each re-deriving it. There is one
    /// definition of what a pass does to a board.
    #[inline(always)]
    fn null_transition<const HASH: bool, const CHECKERS: bool>(&mut self) -> Undo {
        let undo = Undo {
            hash: self.hash,
            checkers: self.checkers,
            castling: self.castling,
            ep: self.ep,
            halfmove: self.halfmove,
            captured: EMPTY,
            #[cfg(feature = "exp-undo-role")]
            moved: EMPTY,
            castled: false,
        };
        let us = self.turn;
        // Hash out the old en-passant contribution (relevance uses the mover's pawns, which have
        // not changed yet) before clearing it.
        if HASH && self.ep_relevant(self.ep, us) {
            self.hash ^= zobrist::ep_key(Square(self.ep));
        }
        self.ep = NO_EP;
        self.halfmove = self.halfmove.saturating_add(1);
        // A pass is a move, not a half-move: after the pass White is to move with the number
        // advanced, whoever passed (this is what keeps FEN's "White to move" and number increments
        // consistent).
        self.fullmove += 1;
        self.turn = us.other();
        if HASH {
            self.hash ^= zobrist::turn_key();
        }
        if CHECKERS {
            self.checkers = self.attackers_to(
                self.king_sq[self.turn.index()],
                self.turn.other(),
                self.occupied(),
            );
        }
        undo
    }

    /// Plays a null move, maintaining the hash and the cached `checkers`.
    #[inline(always)]
    pub fn make_null_move(&mut self) -> Result<Undo, IllegalMove> {
        self.make_null_move_with::<true, true>()
    }

    /// Plays a null move maintaining the hash but not the cached `checkers`; the
    /// keyed-conversion counterpart of [`Board::make_move_hashed`].
    #[inline(always)]
    pub fn make_null_move_hashed(&mut self) -> Result<Undo, IllegalMove> {
        self.make_null_move_with::<true, false>()
    }

    /// Plays a null move maintaining the cached `checkers` but not the hash; the
    /// SAN counterpart of [`Board::make_move_checkered`].
    #[inline(always)]
    pub fn make_null_move_checkered(&mut self) -> Result<Undo, IllegalMove> {
        self.make_null_move_with::<false, true>()
    }

    /// Plays a null move maintaining neither the hash nor the cached `checkers` —
    /// the null-move counterpart of [`Board::play_fast`], for a walker that keeps
    /// neither cache current.
    ///
    /// The check test is computed from the bitboards rather than read from the
    /// cache: after a fast make the cache is stale, and a null move that trusted it
    /// would accept or refuse the wrong positions. `zobrist()` and `in_check()`
    /// are both stale afterwards and must not be read.
    #[inline(always)]
    pub fn make_null_move_fast(&mut self) -> Result<Undo, IllegalMove> {
        self.make_null_move_with::<false, false>()
    }

    /// Reverts the most recent null move, restoring the exact prior position.
    ///
    /// Each null must be unmade before an older one, with no ordinary moves interleaved past the
    /// matching make — exactly like `unmake_move`. Ordinary moves mutate state the null [`Undo`]
    /// does not record, so unmaking a null across an ordinary move restores the clocks, turn and
    /// hash but not the pieces.
    #[inline(always)]
    pub fn unmake_null_move(&mut self, undo: Undo) {
        self.turn = self.turn.other();
        self.fullmove -= 1;
        self.hash = undo.hash;
        self.checkers = undo.checkers;
        self.castling = undo.castling;
        self.ep = undo.ep;
        self.halfmove = undo.halfmove;
    }

    /// Slim perft path — like `ultrachess/position.rs:389` `Safe only for perft`.
    ///
    /// Skips `zobrist` XORs, `history_hashes` push, `halfmove`/`fullmove` clock
    /// maintenance and `checkers` refresh (perft generation recomputes `checkers`
    /// via `attackers_to` from scratch, so the cache is not needed). `PERFT`
    /// uses this path; UI/search use the caching `make_move_unchecked` path.
    /// `unmake_move_perft` must be paired with this.
    #[inline(always)]
    pub fn make_move_perft(&mut self, mv: Move) -> Undo {
        #[cfg(feature="exp-known-role")]
        let moved=self.role_at_occupied(mv.from().0,self.turn)+self.turn as u8*6;
        #[cfg(not(feature="exp-known-role"))]
        let moved=self.piece_code_at_color(mv.from().0,self.turn);
        self.make_move_perft_known::<true>(mv,moved)
    }

    #[inline(always)]
    fn make_move_perft_known<const CHECKERS:bool>(&mut self, mv: Move, moved:u8) -> Undo {
        let mut undo = Undo {
            hash: 0,
            checkers: self.checkers,
            castling: self.castling,
            ep: self.ep,
            halfmove: 0,
            captured: EMPTY,
            #[cfg(feature = "exp-undo-role")]
            moved: EMPTY,
            castled: false,
        };
        let from = mv.from().0;
        let to = mv.to().0;
        let us = self.turn;
        let them = us.other();
        #[cfg(feature = "exp-undo-role")]
        { undo.moved = mv.promotion().map_or(moved, |r| Piece::new(us,r).code()); }
        debug_assert_ne!(moved, EMPTY, "make_move_perft on empty square");
        let role = moved % 6;
        let is_pawn = role == Role::Pawn as u8;
        let diag = from & 7 != to & 7;

        #[cfg(feature = "exp-perft-kernel")]
        if mv.promotion().is_none() && self.occ[us.index()]&bit(to)==0
            && !(is_pawn && diag && self.occupied()&bit(to)==0) {
            let ui=us.index(); let ti=them.index();
            if self.occ[ti]&bit(to)!=0 {
                let cap=self.piece_code_at_color(to,them);
                undo.captured=cap;
                self.bb_remove(ti,(cap%6) as usize,bit(to));
                self.occ[ti]&=!bit(to);
            }
            let delta=bit(from)^bit(to);
            #[cfg(all(feature="exp-perft-xor",feature="exp-mutation",not(feature="exp-no-perft-xor")))]
            self.bb_toggle(ui,role as usize,delta);
            #[cfg(not(all(feature="exp-perft-xor",feature="exp-mutation",not(feature="exp-no-perft-xor"))))]
            {self.bb_remove(ui,role as usize,bit(from));self.bb_add(ui,role as usize,bit(to));}
            self.occ[ui]^=delta;
            if role==Role::King as u8 {self.king_sq[ui]=to;}
            if self.castling!=0 {self.castling&=self.castle_rights_after(role,us,from,to);}
            self.ep=if is_pawn && to.abs_diff(from)==16 {(from+to)/2}else{NO_EP};
            self.turn=them;
            if CHECKERS {self.checkers=self.attackers_to(self.king_sq[ti],us,self.occupied());}
            return undo;
        }

        // Castling encoded king-from → rook-square — fast dest.
        let dest = if self.occ[us.index()] & bit(to) != 0 {
            self.piece_code_at_color(to, us)
        } else if self.occ[them.index()] & bit(to) != 0 {
            self.piece_code_at_color(to, them)
        } else {
            EMPTY
        };
        let is_castle = role == Role::King as u8
            && dest != EMPTY
            && dest % 6 == Role::Rook as u8
            && (dest / 6) == us as u8;

        // Captures (including en passant onto an empty square) — without hash.
        let mut captured_sq = to;
        if is_pawn && diag && dest == EMPTY {
            captured_sq = if us == Color::White { to - 8 } else { to + 8 };
        }
        if !is_castle && captured_sq != from {
            if captured_sq != to {
                let cap = self.pawn_code_at(captured_sq, them);
                if cap != EMPTY {
                    undo.captured = cap;
                    let color = (cap / 6) as usize;
                    let role_idx = (cap % 6) as usize;
                    let b = bit(captured_sq);
                    self.bb_remove(color, role_idx, b);
                    self.occ[color] &= !b;
                }
            } else if dest != EMPTY {
                undo.captured = dest;
                let color = (dest / 6) as usize;
                let role_idx = (dest % 6) as usize;
                let b = bit(to);
                self.bb_remove(color, role_idx, b);
                self.occ[color] &= !b;
            }
        }

        // Move the piece (promotion and castling rook relocation) — without hash.
        match mv.promotion() {
            Some(r) => {
                // Remove from
                {
                    let color = (moved / 6) as usize;
                    let role_idx = (moved % 6) as usize;
                    let b = bit(from);
                    self.bb_remove(color, role_idx, b);
                    self.occ[color] &= !b;
                    if role_idx == Role::King.index() {
                        // king_sq updated via put below
                    }
                }
                let code = Piece::new(us, r).code();
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(to);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
                if role_idx == Role::King.index() {
                    self.king_sq[color] = to;
                }
            }
            None => {
                if is_castle {
                    let rank = from >> 3;
                    let kingside = (to & 7) > (from & 7);
                    let kf = (rank << 3) | if kingside { 6 } else { 2 };
                    let rf = (rank << 3) | if kingside { 5 } else { 3 };
                    undo.castled = true;
                    let rook_code = dest;
                    // Remove both without hash.
                    for sq_code in [(from, moved), (to, rook_code)] {
                        let (sq, code) = sq_code;
                        let color = (code / 6) as usize;
                        let role_idx = (code % 6) as usize;
                        let b = bit(sq);
                        self.bb_remove(color, role_idx, b);
                        self.occ[color] &= !b;
                    }
                    // Place both without hash.
                    for sq_code in [(kf, moved), (rf, rook_code)] {
                        let (sq, code) = sq_code;
                        let color = (code / 6) as usize;
                        let role_idx = (code % 6) as usize;
                        let b = bit(sq);
                        self.bb_add(color, role_idx, b);
                        self.occ[color] |= b;
                        if role_idx == Role::King.index() {
                            self.king_sq[color] = sq;
                        }
                    }
                } else {
                    // Normal move without hash — manual move_piece without hash.
                    let color = (moved / 6) as usize;
                    let role_idx = (moved % 6) as usize;
                    let b_from = bit(from);
                    let b_to = bit(to);
                    #[cfg(feature = "exp-mutation")]
                    {
                        let delta = b_from ^ b_to;
                        self.bb_toggle(color, role_idx, delta);
                        self.occ[color] ^= delta;
                    }
                    #[cfg(not(feature = "exp-mutation"))]
                    {
                        self.bb_remove(color, role_idx, b_from);
                        self.bb_add(color, role_idx, b_to);
                        self.occ[color] &= !b_from;
                        self.occ[color] |= b_to;
                    }
                    if role_idx == Role::King.index() {
                        self.king_sq[color] = to;
                    }
                }
            }
        }

        // Castling rights (without hash) — derived mask (close-gap D3, 4.1).
        self.castling &= self.castle_rights_after(role, us, from, to);

        // En-passant square after a double pawn push (without hash relevance calc).
        self.ep = if is_pawn && to.abs_diff(from) == 16 {
            (from + to) / 2
        } else {
            NO_EP
        };

        // Turn flip (without hash) — but keep `checkers` cached so movegen can use
        // `self.checkers` instead of recomputing `attackers_to` per node (saves ~20ns).
        self.turn = us.other();
        if CHECKERS {
        self.checkers = self.attackers_to(
            self.king_sq[self.turn.index()],
            self.turn.other(),
            self.occupied(),
        );
        }
        undo
    }

    /// Reverts a perft slim move (paired with `make_move_perft`).
    #[inline(always)]
    pub fn unmake_move_perft(&mut self, mv: Move, undo: Undo) {
        debug_assert!(
            !mv.is_null(),
            "unmake_move_perft(Move::NULL): a pass pairs with unmake_null_move"
        );
        let us = self.turn.other(); // mover
        self.turn = us;
        self.checkers = undo.checkers;
        let from = mv.from().0;
        let to = mv.to().0;

        if undo.castled {
            debug_assert_ne!(to, from, "castling move with from == to");
            let rank = from >> 3;
            let kingside = (to & 7) > (from & 7);
            let kf = (rank << 3) | if kingside { 6 } else { 2 };
            let rf = (rank << 3) | if kingside { 5 } else { 3 };
            let king_code = self.piece_code_at_color(kf, us);
            let rook_code = self.piece_code_at_color(rf, us);
            debug_assert_eq!(king_code % 6, Role::King as u8);
            debug_assert_eq!(rook_code % 6, Role::Rook as u8);
            for sq_code in [(kf, king_code), (rf, rook_code)] {
                let (sq, code) = sq_code;
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(sq);
                self.bb_remove(color, role_idx, b);
                self.occ[color] &= !b;
            }
            for sq_code in [(from, king_code), (to, rook_code)] {
                let (sq, code) = sq_code;
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(sq);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
                if role_idx == Role::King.index() {
                    self.king_sq[color] = sq;
                }
            }
            self.castling = undo.castling;
            self.ep = undo.ep;
            return;
        }

        #[cfg(feature = "exp-undo-role")]
        let moved = undo.moved;
        #[cfg(not(feature = "exp-undo-role"))]
        let moved = self.piece_code_at_color(to, us);
        debug_assert_ne!(moved, EMPTY, "unmake_move_perft on empty destination");
        // Remove to without hash.
        {
            let color = (moved / 6) as usize;
            let role_idx = (moved % 6) as usize;
            let b = bit(to);
            self.bb_remove(color, role_idx, b);
            self.occ[color] &= !b;
        }
        match mv.promotion() {
            Some(_) => {
                let code = Piece::new(us, Role::Pawn).code();
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(from);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
            }
            None => {
                let color = (moved / 6) as usize;
                let role_idx = (moved % 6) as usize;
                let b = bit(from);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
                if role_idx == Role::King.index() {
                    self.king_sq[color] = from;
                }
                // Undo standard castling rook relocation (king-from→rook-square encoding
                // already handled via castled flag; remaining O-O rook hop is standard geometry
                // for legacy compatibility — not used for Chess960 castle words, which use
                // the castled path above).
                if moved % 6 == Role::King as u8 && (to & 7).abs_diff(from & 7) == 2 {
                    let (rfrom, rto) = match to {
                        6 => (5, 7),
                        2 => (3, 0),
                        62 => (61, 63),
                        _ => (59, 56),
                    };
                    let rook = self.piece_code_at_color(rfrom, us);
                    if rook != EMPTY {
                        let color = (rook / 6) as usize;
                        let role_idx = (rook % 6) as usize;
                        let b_from = bit(rfrom);
                        let b_to = bit(rto);
                        self.bb_remove(color, role_idx, b_from);
                        self.bb_add(color, role_idx, b_to);
                        self.occ[color] &= !b_from;
                        self.occ[color] |= b_to;
                    }
                }
            }
        }
        if undo.captured != EMPTY {
            let ep_capture = moved % 6 == Role::Pawn as u8
                && from & 7 != to & 7
                && undo.ep != crate::types::NO_EP
                && to == undo.ep;
            let cap_sq = if ep_capture {
                if us == Color::White { to - 8 } else { to + 8 }
            } else { to };
            let code = undo.captured;
            let color = (code / 6) as usize;
            let role_idx = (code % 6) as usize;
            let b = bit(cap_sq);
            self.bb_add(color, role_idx, b);
            self.occ[color] |= b;
        }
        self.castling = undo.castling;
        self.ep = undo.ep;
    }

    /// Reverts the most recent move, restoring the exact prior position.
    pub fn unmake_move(&mut self, mv: Move, undo: Undo) {
        // A pass pairs with `unmake_null_move`. Round-tripping a null word through
        // here would decode `0xffff` as h1h1 — a layout coincidence, not a
        // contract — and would restore the turn but not the fullmove number the
        // pass advanced.
        debug_assert!(
            !mv.is_null(),
            "unmake_move(Move::NULL): a pass pairs with unmake_null_move"
        );
        let us = self.turn.other(); // the mover
        self.turn = us;
        if us == Color::Black {
            self.fullmove -= 1;
        }
        let from = mv.from().0;
        let to = mv.to().0;

        if undo.castled {
            // Reverse castling (king-from → rook-square encoding): the king
            // now stands on its final square and the rook on `rf`; `to` is
            // the rook's original square.
            debug_assert_ne!(to, from, "castling move with from == to");
            let rank = from >> 3;
            let kingside = (to & 7) > (from & 7);
            let kf = (rank << 3) | if kingside { 6 } else { 2 };
            let rf = (rank << 3) | if kingside { 5 } else { 3 };
            let king_code = self.piece_code_at_color(kf, us);
            let rook_code = self.piece_code_at_color(rf, us);
            debug_assert_eq!(king_code % 6, Role::King as u8);
            debug_assert_eq!(rook_code % 6, Role::Rook as u8);
            // Remove both, then place both (swap-safe, mirrors make_move).
            self.remove_piece(kf, king_code);
            self.remove_piece(rf, rook_code);
            self.put_piece(from, king_code);
            self.put_piece(to, rook_code);
            self.hash = undo.hash;
            self.checkers = undo.checkers;
            self.castling = undo.castling;
            self.ep = undo.ep;
            self.halfmove = undo.halfmove;
            return;
        }

        #[cfg(feature = "exp-undo-role")]
        let moved = undo.moved;
        #[cfg(not(feature = "exp-undo-role"))]
        let moved = self.piece_code_at_color(to, us);
        debug_assert_ne!(moved, EMPTY, "unmake_move on empty destination");

        #[cfg(feature="exp-unmake-xor")]
        if mv.promotion().is_none() && !(moved%6==Role::King as u8 && (to&7).abs_diff(from&7)==2){
            let ui=us.index();let role=(moved%6) as usize;
            let delta=bit(from)^bit(to);
            #[cfg(feature="exp-mutation")]
            self.bb_toggle(ui,role,delta);
            #[cfg(not(feature="exp-mutation"))]
            {self.bb_remove(ui,role,bit(to));self.bb_add(ui,role,bit(from));}
            self.occ[ui]^=delta;
            if role==Role::King.index(){self.king_sq[ui]=from;}
            if undo.captured!=EMPTY{
                let ep=role==Role::Pawn.index()&&from&7!=to&7&&undo.ep!=NO_EP&&to==undo.ep;
                let sq=if ep{if us==Color::White{to-8}else{to+8}}else{to};
                self.put_piece_no_hash(sq,undo.captured);
            }
            self.hash=undo.hash;self.checkers=undo.checkers;self.castling=undo.castling;
            self.ep=undo.ep;self.halfmove=undo.halfmove;
            return;
        }
        self.remove_piece(to, moved);
        match mv.promotion() {
            Some(_) => self.put_piece(from, Piece::new(us, Role::Pawn).code()),
            None => {
                self.put_piece(from, moved);
                if moved % 6 == Role::King as u8 && (to & 7).abs_diff(from & 7) == 2 {
                    let (rfrom, rto) = match to {
                        6 => (5, 7),
                        2 => (3, 0),
                        62 => (61, 63),
                        _ => (59, 56),
                    };
                    let rook = self.piece_code_at_color(rfrom, us);
                    self.move_piece(rfrom, rto, rook);
                }
            }
        }
        if undo.captured != EMPTY {
            // En passant is the only capture where the captured pawn is not on
            // `to`: exactly when the destination equals the pre-move ep square.
            let ep_capture = moved % 6 == Role::Pawn as u8
                && from & 7 != to & 7
                && undo.ep != crate::types::NO_EP
                && to == undo.ep;
            let cap_sq = if ep_capture {
                if us == Color::White {
                    to - 8
                } else {
                    to + 8
                }
            } else {
                to
            };
            self.put_piece(cap_sq, undo.captured);
        }
        self.hash = undo.hash;
        self.checkers = undo.checkers;
        self.castling = undo.castling;
        self.ep = undo.ep;
        self.halfmove = undo.halfmove;
    }

    /// Validates `mv` against pseudo-legal geometry, applies it, and then
    /// confirms the mover's king is safe; rolls back and reports
    /// [`IllegalMove`] otherwise. This is the safe public entry point.
    ///
    /// A [`Move::NULL`] passes: it is legal exactly when the side to move is not
    /// in check, and pairs with [`Board::unmake_null_move`].
    pub fn play(&mut self, mv: Move) -> Result<Undo, IllegalMove> {
        if mv.is_null() {
            return self.make_null_move();
        }
        #[cfg(all(feature="exp-checked-known",feature="exp-direct-legal"))]
        {
            let code=self.piece_code_at_color(mv.from().0,self.turn);
            if !self.is_pseudo_legal_with_code(mv,code){return Err(IllegalMove);}
            if let Some(safe)=self.king_safe_hypothetical_role(mv,code%6){
                return if safe{Ok(self.make_move_hinted::<true,true,true>(mv,code))}else{Err(IllegalMove)};
            }
            let undo=self.make_move_hinted::<true,true,true>(mv,code);
            let mover=self.turn.other();
            if self.attackers_to(self.king_sq[mover.index()],self.turn,self.occupied())!=0{
                self.unmake_move(mv,undo);return Err(IllegalMove);
            }
            return Ok(undo);
        }
        #[allow(unreachable_code)]
        if !self.is_pseudo_legal(mv) {
            return Err(IllegalMove);
        }
        #[cfg(all(feature="exp-checked-direct",feature="exp-direct-legal"))]
        if let Some(safe)=self.king_safe_hypothetical(mv) {
            return if safe {Ok(self.make_move_unchecked(mv))}else{Err(IllegalMove)};
        }
        let undo = self.make_move_unchecked(mv);
        let mover = self.turn.other();
        if self.attackers_to(self.king_sq[mover.index()], self.turn, self.occupied()) != 0 {
            self.unmake_move(mv, undo);
            return Err(IllegalMove);
        }
        Ok(undo)
    }
    /// Applies `mv` without incremental Zobrist hash updates or checkers calculation.
    /// Clocks, turn, castling rights, en-passant, piece bitboards, and king safety
    /// validation are fully maintained.
    ///
    /// Returns the [`Undo`] needed by `unmake_move_fast`.
    ///
    /// A [`Move::NULL`] is dispatched onto the null-move transition.
    #[inline(always)]
    pub fn make_move_fast(&mut self, mv: Move) -> Undo {
        if mv.is_null() {
            debug_assert!(self.pass_is_legal(), "make_move_fast(Move::NULL) while in check");
            return self.null_transition::<false, false>();
        }
        let mut undo = Undo {
            hash: 0,
            checkers: 0,
            castling: self.castling,
            ep: self.ep,
            halfmove: self.halfmove,
            captured: EMPTY,
            #[cfg(feature = "exp-undo-role")]
            moved: EMPTY,
            castled: false,
        };
        let from = mv.from().0;
        let to = mv.to().0;
        let us = self.turn;
        let them = us.other();
        let moved = self.piece_code_at_color(from, us);
        #[cfg(feature = "exp-undo-role")]
        { undo.moved = mv.promotion().map_or(moved, |r| Piece::new(us,r).code()); }
        debug_assert_ne!(moved, EMPTY, "make_move_fast on empty square");
        let role = moved % 6;
        let is_pawn = role == Role::Pawn as u8;
        let diag = from & 7 != to & 7;

        // Castling encoded king-from → rook-square — fast dest.
        let dest = if self.occ[us.index()] & bit(to) != 0 {
            self.piece_code_at_color(to, us)
        } else if self.occ[them.index()] & bit(to) != 0 {
            self.piece_code_at_color(to, them)
        } else {
            EMPTY
        };
        let is_castle = role == Role::King as u8
            && dest != EMPTY
            && dest % 6 == Role::Rook as u8
            && (dest / 6) == us as u8;

        // Captures (including en passant onto an empty square) — without hash.
        let mut captured_sq = to;
        if is_pawn && diag && dest == EMPTY {
            captured_sq = if us == Color::White { to - 8 } else { to + 8 };
        }
        if !is_castle && captured_sq != from {
            if captured_sq != to {
                let cap = self.pawn_code_at(captured_sq, them);
                if cap != EMPTY {
                    undo.captured = cap;
                    let color = (cap / 6) as usize;
                    let role_idx = (cap % 6) as usize;
                    let b = bit(captured_sq);
                    self.bb_remove(color, role_idx, b);
                    self.occ[color] &= !b;
                }
            } else if dest != EMPTY {
                undo.captured = dest;
                let color = (dest / 6) as usize;
                let role_idx = (dest % 6) as usize;
                let b = bit(to);
                self.bb_remove(color, role_idx, b);
                self.occ[color] &= !b;
            }
        }

        // Move the piece (promotion and castling rook relocation) — without hash.
        match mv.promotion() {
            Some(r) => {
                let color = (moved / 6) as usize;
                let role_idx = (moved % 6) as usize;
                let b = bit(from);
                self.bb_remove(color, role_idx, b);
                self.occ[color] &= !b;

                let code = Piece::new(us, r).code();
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(to);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
                if role_idx == Role::King.index() {
                    self.king_sq[color] = to;
                }
            }
            None => {
                if is_castle {
                    let rank = from >> 3;
                    let kingside = (to & 7) > (from & 7);
                    let kf = (rank << 3) | if kingside { 6 } else { 2 };
                    let rf = (rank << 3) | if kingside { 5 } else { 3 };
                    undo.castled = true;
                    let rook_code = dest;
                    for sq_code in [(from, moved), (to, rook_code)] {
                        let (sq, code) = sq_code;
                        let color = (code / 6) as usize;
                        let role_idx = (code % 6) as usize;
                        let b = bit(sq);
                        self.bb_remove(color, role_idx, b);
                        self.occ[color] &= !b;
                    }
                    for sq_code in [(kf, moved), (rf, rook_code)] {
                        let (sq, code) = sq_code;
                        let color = (code / 6) as usize;
                        let role_idx = (code % 6) as usize;
                        let b = bit(sq);
                        self.bb_add(color, role_idx, b);
                        self.occ[color] |= b;
                        if role_idx == Role::King.index() {
                            self.king_sq[color] = sq;
                        }
                    }
                } else {
                    let color = (moved / 6) as usize;
                    let role_idx = (moved % 6) as usize;
                    let b_from = bit(from);
                    let b_to = bit(to);
                    #[cfg(feature = "exp-mutation")]
                    {
                        let delta = b_from ^ b_to;
                        self.bb_toggle(color, role_idx, delta);
                        self.occ[color] ^= delta;
                    }
                    #[cfg(not(feature = "exp-mutation"))]
                    {
                        self.bb_remove(color, role_idx, b_from);
                        self.bb_add(color, role_idx, b_to);
                        self.occ[color] &= !b_from;
                        self.occ[color] |= b_to;
                    }
                    if role_idx == Role::King.index() {
                        self.king_sq[color] = to;
                    }
                }
            }
        }

        self.castling &= self.castle_rights_after(role, us, from, to);

        self.ep = if is_pawn && to.abs_diff(from) == 16 {
            (from + to) / 2
        } else {
            NO_EP
        };

        if is_pawn || undo.captured != EMPTY {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }
        if us == Color::Black {
            self.fullmove += 1;
        }

        self.turn = us.other();
        undo
    }


    /// Reverts a fast move (paired with `make_move_fast`).
    #[inline(always)]
    pub fn unmake_move_fast(&mut self, mv: Move, undo: Undo) {
        debug_assert!(
            !mv.is_null(),
            "unmake_move_fast(Move::NULL): a pass pairs with unmake_null_move"
        );
        let us = self.turn.other(); // mover
        self.turn = us;
        if us == Color::Black {
            self.fullmove -= 1;
        }
        let from = mv.from().0;
        let to = mv.to().0;

        if undo.castled {
            debug_assert_ne!(to, from, "castling move with from == to");
            let rank = from >> 3;
            let kingside = (to & 7) > (from & 7);
            let kf = (rank << 3) | if kingside { 6 } else { 2 };
            let rf = (rank << 3) | if kingside { 5 } else { 3 };
            let king_code = self.piece_code_at_color(kf, us);
            let rook_code = self.piece_code_at_color(rf, us);
            debug_assert_eq!(king_code % 6, Role::King as u8);
            debug_assert_eq!(rook_code % 6, Role::Rook as u8);
            for sq_code in [(kf, king_code), (rf, rook_code)] {
                let (sq, code) = sq_code;
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(sq);
                self.bb_remove(color, role_idx, b);
                self.occ[color] &= !b;
            }
            for sq_code in [(from, king_code), (to, rook_code)] {
                let (sq, code) = sq_code;
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(sq);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
                if role_idx == Role::King.index() {
                    self.king_sq[color] = sq;
                }
            }
            self.castling = undo.castling;
            self.ep = undo.ep;
            self.halfmove = undo.halfmove;
            return;
        }

        #[cfg(feature = "exp-undo-role")]
        let moved = undo.moved;
        #[cfg(not(feature = "exp-undo-role"))]
        let moved = self.piece_code_at_color(to, us);
        debug_assert_ne!(moved, EMPTY, "unmake_move_fast on empty destination");
        {
            let color = (moved / 6) as usize;
            let role_idx = (moved % 6) as usize;
            let b = bit(to);
            self.bb_remove(color, role_idx, b);
            self.occ[color] &= !b;
        }
        match mv.promotion() {
            Some(_) => {
                let code = Piece::new(us, Role::Pawn).code();
                let color = (code / 6) as usize;
                let role_idx = (code % 6) as usize;
                let b = bit(from);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
            }
            None => {
                let color = (moved / 6) as usize;
                let role_idx = (moved % 6) as usize;
                let b = bit(from);
                self.bb_add(color, role_idx, b);
                self.occ[color] |= b;
                if role_idx == Role::King.index() {
                    self.king_sq[color] = from;
                }
                if moved % 6 == Role::King as u8 && (to & 7).abs_diff(from & 7) == 2 {
                    let (rfrom, rto) = match to {
                        6 => (5, 7),
                        2 => (3, 0),
                        62 => (61, 63),
                        _ => (59, 56),
                    };
                    let rook = self.piece_code_at_color(rfrom, us);
                    if rook != EMPTY {
                        let color = (rook / 6) as usize;
                        let role_idx = (rook % 6) as usize;
                        let b_from = bit(rfrom);
                        let b_to = bit(rto);
                        self.bb_remove(color, role_idx, b_from);
                        self.bb_add(color, role_idx, b_to);
                        self.occ[color] &= !b_from;
                        self.occ[color] |= b_to;
                    }
                }
            }
        }
        if undo.captured != EMPTY {
            let ep_capture = moved % 6 == Role::Pawn as u8
                && from & 7 != to & 7
                && undo.ep != crate::types::NO_EP
                && to == undo.ep;
            let cap_sq = if ep_capture {
                if us == Color::White { to - 8 } else { to + 8 }
            } else { to };
            let code = undo.captured;
            let color = (code / 6) as usize;
            let role_idx = (code % 6) as usize;
            let b = bit(cap_sq);
            self.bb_add(color, role_idx, b);
            self.occ[color] |= b;
        }
        self.castling = undo.castling;
        self.ep = undo.ep;
        self.halfmove = undo.halfmove;
    }

    /// Fast validation and application of `mv`: validates pseudo-legal geometry,
    /// applies via `make_move_fast`, and confirms king safety (rolling back via
    /// `unmake_move_fast` and returning [`IllegalMove`] on failure).
    ///
    /// Skips incremental Polyglot Zobrist hashing and checkers bitboard calculation.
    ///
    /// A [`Move::NULL`] passes: it is legal exactly when the side to move is not
    /// in check, and pairs with [`Board::unmake_null_move`].
    #[inline(always)]
    pub fn play_fast(&mut self, mv: Move) -> Result<Undo, IllegalMove> {
        if mv.is_null() {
            return self.make_null_move_fast();
        }
        if !self.is_pseudo_legal(mv) {
            return Err(IllegalMove);
        }
        let undo = self.make_move_fast(mv);
        let mover = self.turn.other();
        if self.attackers_to(self.king_sq[mover.index()], self.turn, self.occupied()) != 0 {
            self.unmake_move_fast(mv, undo);
            return Err(IllegalMove);
        }
        Ok(undo)
    }

    /// Validates and applies `mv`, maintaining the incremental Polyglot hash but
    /// **not** the cached `checkers` — the shape a position indexer wants: it
    /// reads `zobrist()` on every ply and never asks `in_check()`.
    ///
    /// Skips the `attackers_to` refresh [`Board::play`] pays to keep `in_check()`
    /// branch-free. `zobrist()` stays correct; `in_check()` is **stale** after the
    /// move and must not be read.
    ///
    /// A [`Move::NULL`] passes: it is legal exactly when the side to move is not
    /// in check, and pairs with [`Board::unmake_null_move`].
    #[inline(always)]
    pub fn play_hashed(&mut self, mv: Move) -> Result<Undo, IllegalMove> {
        if mv.is_null() {
            return self.make_null_move_hashed();
        }
        if !self.is_pseudo_legal(mv) {
            return Err(IllegalMove);
        }
        let undo = self.make_move_hashed(mv);
        let mover = self.turn.other();
        if self.attackers_to(self.king_sq[mover.index()], self.turn, self.occupied()) != 0 {
            self.unmake_move(mv, undo);
            return Err(IllegalMove);
        }
        Ok(undo)
    }

    /// Validates and applies `mv`, maintaining the cached `checkers` but **not**
    /// the incremental Polyglot hash — the shape a SAN writer wants: it reads
    /// `in_check()` for the check/mate suffix and never indexes a position.
    ///
    /// `in_check()` stays correct; `zobrist()` is **stale** after the move and
    /// must not be read.
    ///
    /// A [`Move::NULL`] passes: it is legal exactly when the side to move is not
    /// in check, and pairs with [`Board::unmake_null_move`].
    #[inline(always)]
    pub fn play_checkered(&mut self, mv: Move) -> Result<Undo, IllegalMove> {
        if mv.is_null() {
            return self.make_null_move_checkered();
        }
        if !self.is_pseudo_legal(mv) {
            return Err(IllegalMove);
        }
        let undo = self.make_move_checkered(mv);
        let mover = self.turn.other();
        if self.attackers_to(self.king_sq[mover.index()], self.turn, self.occupied()) != 0 {
            self.unmake_move(mv, undo);
            return Err(IllegalMove);
        }
        Ok(undo)
    }

    /// True when the move is legal in this position (without playing it).
    ///
    /// A [`Move::NULL`] is answered by its own branch, **not** by the generic body
    /// below. That body asks whether the *mover's* king survives the move — but a
    /// pass moves no king, so there is no mover's king to expose, and the only
    /// question a pass raises is whether the side to move is already in check. The
    /// generic body cannot express that: it would test the opponent's king, and
    /// return `true` on precisely the positions where a pass must be refused.
    /// The test is the same fresh `attackers_to` the pass path uses, never the
    /// cached `in_check()`.
    pub fn is_legal(&self, mv: Move) -> bool {
        if mv.is_null() {
            return self.pass_is_legal();
        }
        if !self.is_pseudo_legal(mv) {
            return false;
        }
        #[cfg(feature = "exp-direct-legal")]
        if let Some(safe)=self.king_safe_hypothetical(mv) {return safe;}
        let mut tmp = *self;
        if cfg!(feature = "exp-san") {
            tmp.make_move_fast(mv);
            let mover = tmp.turn.other();
            return tmp.attackers_to(tmp.king_sq[mover.index()], tmp.turn, tmp.occupied()) == 0;
        }
        let undo = tmp.make_move_unchecked(mv);
        let mover = tmp.turn.other();
        let safe = tmp.attackers_to(tmp.king_sq[mover.index()], tmp.turn, tmp.occupied()) == 0;
        tmp.unmake_move(mv, undo);
        safe
    }

    #[cfg(feature = "exp-direct-legal")]
    #[inline(always)]
    fn king_safe_hypothetical(&self,mv:Move)->Option<bool>{
        self.king_safe_hypothetical_role(mv,self.piece_code_at_color(mv.from().0,self.turn)%6)
    }

    #[cfg(feature="exp-direct-legal")]
    #[inline(always)]
    fn king_safe_hypothetical_role(&self,mv:Move,role:u8)->Option<bool>{
        let us=self.turn;let them=us.other();
        let f=mv.from().0;let t=mv.to().0;
        if self.occ[us.index()]&bit(t)!=0 || (mv.promotion().is_some()&&role!=Role::Pawn as u8){return None;}
        let mut captured=bit(t)&self.occ[them.index()];
        if role==Role::Pawn as u8 && f&7!=t&7 && captured==0 {
            let cap=if us==Color::White {t.checked_sub(8)?} else {t.checked_add(8)?};
            if cap>=64{return None;}
            captured=bit(cap)&self.piece_bb(them,Role::Pawn);
        }
        let occ=(self.occupied()&!(bit(f)|captured))|bit(t);
        let k=if role==Role::King as u8 {t}else{self.king_sq[us.index()]};
        // Mask the removed enemy piece at its old square, including en passant.
        #[cfg(feature="exp-bool-attacks")]
        return Some(!self.attacked_except(k,them,occ,captured));
        #[cfg(not(feature="exp-bool-attacks"))]
        Some(self.attackers_to(k,them,occ)&!captured==0)
    }

    #[cfg(feature = "exp-early-san")]
    pub(crate) fn definitely_no_check(&self,mv:Move)->bool{
        let us=self.turn;let them=us.other();let f=mv.from().0;let t=mv.to().0;
        let Some(piece)=self.piece_at(Square(f)) else{return false};
        if self.occ[us.index()]&bit(t)!=0 || mv.promotion().is_some()
            || (piece.role==Role::Pawn && f&7!=t&7 && self.occupied()&bit(t)==0) {return false;}
        let k=self.king_sq[them.index()];
        let source=bit(f);let target=bit(t);
        let (b,r)=(crate::bitboard::EMPTY_BISHOP_RAYS[k as usize],crate::bitboard::EMPTY_ROOK_RAYS[k as usize]);
        if source&(b|r)!=0{return false;}
        let direct=match piece.role {
            Role::Pawn=>PAWN_ATT[them.index()][k as usize],
            Role::Knight=>KNIGHT_ATT[k as usize],
            Role::Bishop=>b,
            Role::Rook=>r,
            Role::Queen=>b|r,
            Role::King=>KING_ATT[k as usize],
        };
        if direct&target!=0{return false;}
        // Legal games never start with the opponent already in check, but permissive FEN can.
        self.attackers_to(k,us,self.occupied())==0
    }

    /// True when `sq` is attacked by any piece of `by`.
    pub fn square_attacked(&self, sq: u8, by: Color) -> bool {
        self.attackers_to(sq, by, self.occupied()) != 0
    }

    /// Structural (pseudo-legal) validation of `mv`: piece geometry,
    /// capture/en-passant/castling preconditions. King safety is verified by
    /// `play` after applying.
    ///
    /// [`Move::NULL`] is pseudo-legal: the sentinel is structurally well-formed,
    /// needing no piece, no geometry and no capture target. Whether a pass is
    /// actually *legal* — the side to move must not be in check — belongs to
    /// [`Board::is_legal`], not to geometry.
    pub fn is_pseudo_legal(&self, mv: Move) -> bool {
        if mv.is_null() {
            return true;
        }
        self.is_pseudo_legal_with_code(mv,self.piece_code_at_color(mv.from().0,self.turn))
    }

    #[inline(always)]
    fn is_pseudo_legal_with_code(&self,mv:Move,code:u8)->bool{
        let from = mv.from().0;
        let to = mv.to().0;
        if from == to || from > 63 || to > 63 {
            return false;
        }
        let us = self.turn;
        let them = us.other();
        if code == EMPTY {
            return false;
        }
        let role = code % 6;
        let df = (to & 7).abs_diff(from & 7);
        let dr = (to / 8) as i32 - (from / 8) as i32;

        // Castling is encoded king-from → rook-square (ADR-003, decision D3):
        // a king move landing on the mover's own rook. Checked before the
        // generic own-piece rejection below.
        if role == Role::King as u8 && self.occ[us.index()] & bit(to) != 0 {
            return self.is_castling_move(from, to);
        }
        if self.occ[us.index()] & bit(to) != 0 {
            return false; // cannot capture own piece
        }

        match role {
            r if r == Role::Pawn as u8 => {
                let white = us == Color::White;
                let push: i32 = if white { 8 } else { -8 };
                let to_is_promo = if white { to / 8 == 7 } else { to / 8 == 0 };
                // Promotion bit must be present exactly on the last rank.
                if mv.promotion().is_some() != to_is_promo || mv.promotion() == Some(Role::King) {
                    return false;
                }
                if df == 0 {
                    if self.occ[0] & bit(to) != 0 || self.occ[1] & bit(to) != 0 {
                        return false;
                    }
                    let sq_diff = to as i32 - from as i32;
                    if sq_diff == push {
                        true
                    } else if sq_diff == 2 * push {
                        if from / 8 != if white { 1 } else { 6 } {
                            return false;
                        }
                        let mid = ((from as i32 + to as i32) / 2) as u8;
                        if self.occ[0] & bit(mid) != 0 || self.occ[1] & bit(mid) != 0 {
                            return false;
                        }
                        true
                    } else {
                        false
                    }
                } else if df == 1 && dr == push.signum() {
                    if self.occ[them.index()] & bit(to) != 0 {
                        true
                    } else if self.occ[us.index()] & bit(to) != 0 {
                        false
                    } else {
                        self.ep != NO_EP && to == self.ep
                    }
                } else {
                    false
                }
            }
            r if r == Role::Knight as u8 => KNIGHT_ATT[from as usize] & bit(to) != 0,
            r if r == Role::Bishop as u8 => {
                attacks::bishop_attacks(from, self.occupied()) & bit(to) != 0
            }
            r if r == Role::Rook as u8 => attacks::rook_attacks(from, self.occupied()) & bit(to) != 0,
            r if r == Role::Queen as u8 => {
                attacks::queen_attacks(from, self.occupied()) & bit(to) != 0
            }
            _ => {
                // King: one-step moves. Castling (king→own-rook words) was
                // validated above; a normal king move can never land on an
                // own rook.
                KING_ATT[from as usize] & bit(to) != 0
            }
        }
    }

    /// Bulk-counting perft: number of leaf nodes at `depth`.
    ///
    /// `depth==1` uses the `MoveCounter` bulk path (`count+=popcount` without
    /// `pop_lsb`) — the `geomean 1.23× vs cozy` win (`BENCH.md: caveat 6`,
    /// D2/2.2). This is what produces the perft lead toward ultrachess `836 Mnps`.
    ///
    /// Interior nodes use the slim `make_move_perft`/`unmake_move_perft` path
    /// which skips `zobrist`/`halfmove`/`fullmove` and `checkers` refresh
    /// (`position.rs:389` `Safe only for perft`, D3). Generation recomputes
    /// `checkers` from scratch via `attackers_to`, so the cache is not needed.
    #[inline(always)]
    fn make_perft_internal(&mut self,mv:Move)->Undo {
        #[cfg(feature="exp-known-role")]
        let code=self.role_at_occupied(mv.from().0,self.turn)+self.turn as u8*6;
        #[cfg(not(feature="exp-known-role"))]
        let code=self.piece_code_at_color(mv.from().0,self.turn);
        self.make_move_perft_known::<{!cfg!(feature="exp-fused-geometry")}>(mv,code)
    }

    pub fn perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        if depth == 1 {
            return self.count_legal_moves() as u64;
        }
        #[cfg(feature = "exp-fused-perft")]
        if depth == 2 {
            let mut sink = PerftLeafSink { child: *self, nodes: 0 };
            self.generate_moves_into(&mut sink);
            return sink.nodes;
        }
        #[cfg(feature = "exp-tagged-perft")]
        {
            let mut tagged=TaggedPerftList{board:self,moves:ArrayVec::new()};
            self.generate_moves_into(&mut tagged);
            let mut nodes=0;
            let mut child=*self;
            for &(mv,code) in &tagged.moves {
                let undo=child.make_move_perft_known::<{!cfg!(feature="exp-fused-geometry")}>(mv,code);
                nodes+=child.perft(depth-1);
                child.unmake_move_perft(mv,undo);
            }
            return nodes;
        }
        #[allow(unreachable_code)]
        let mut list = crate::movegen::MoveList::new();
        self.generate_moves_into(&mut list);
        let mut nodes = 0u64;
        #[cfg(feature = "exp-copy-perft")]
        for &mv in list.as_slice() {
            let mut child = *self;
            child.make_perft_internal(mv);
            nodes += child.perft(depth - 1);
        }
        #[cfg(not(feature = "exp-copy-perft"))]
        {
            let mut child = *self;
            for &mv in list.as_slice() {
                let undo = child.make_perft_internal(mv);
                nodes += child.perft(depth - 1);
                child.unmake_move_perft(mv, undo);
            }
        }
        nodes
    }

    /// Full traversal with an observable slim leaf (same state as repeated
    /// make_move_perft). No bulk counting; callback runs for every leaf.
    #[cfg(feature="exp-full-visitor")]
    pub fn visit_perft_leaves<F:FnMut(&Board)>(&self,depth:u32,visitor:&mut F)->u64{
        if depth==0 {visitor(self);return 1;}
        if depth==1 {
            let mut sink=FullLeafSink{parent:self,visitor,nodes:0};
            self.generate_moves_into(&mut sink);
            return sink.nodes;
        }
        let mut list=crate::movegen::MoveList::new();
        self.generate_moves_into(&mut list);
        let mut nodes=0;
        for &mv in list.as_slice(){
            let mut child=*self;
            child.make_perft_internal(mv);
            nodes+=child.visit_perft_leaves(depth-1,visitor);
        }
        nodes
    }

    /// Experimental depth-scoped reusable move buffers.
    ///
    /// Unlike perft, which creates one temporary MoveList per node,
    /// prepare one list per ply. Each child receives only the remaining
    /// slice, avoiding mutation of its parent's live moves.
    ///
    /// Ordinary perft remains the stable reference until both correct
    /// perft vectors and representative benchmarks are completed.
    pub fn perft_reuse(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut board = *self;
        let mut workspace: Vec<crate::movegen::MoveList> =
            (0..depth).map(|_| crate::movegen::MoveList::new()).collect();

        #[inline]
        fn visit(board: &mut Board, depth: u32, lists: &mut [crate::movegen::MoveList]) -> u64 {
            if depth == 1 {
                return board.count_legal_moves() as u64;
            }
            let (moves, remaining) = lists.split_first_mut().expect("depth-scoped perft buffer");
            moves.len = 0;
            board.generate_moves_into(moves);
            let len = moves.len();
            let mut nodes = 0u64;
            for i in 0..len {
                let mv = moves.as_slice()[i];
                let undo = board.make_move_perft(mv);
                nodes += visit(board, depth - 1, remaining);
                board.unmake_move_perft(mv, undo);
            }
            nodes
        }
        visit(&mut board, depth, &mut workspace)
    }

    /// Visitor-pattern perft (D1): leaf `depth==1` counts via
    /// [`CountingVisitor`](crate::movegen::CountingVisitor) — `count +=
    /// popcount` **without materialising `Move` values and without
    /// `pop_lsb`** (Gigantua's visitor 2× vs movelist; MIT-clean per D5).
    /// Interior nodes still materialise moves for `make_move_perft`
    /// (unavoidable — the visitor contract deliberately carries no board
    /// mutation), so the win is concentrated at the leaves.
    pub fn perft_visitor(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        if depth == 1 {
            let mut v = crate::movegen::CountingVisitor::new();
            self.generate_visitor(&mut v);
            return v.count as u64;
        }
        let mut list = crate::movegen::MoveList::new();
        self.generate_moves_into(&mut list);
        let mut nodes = 0u64;
        let mut child = *self;
        for &mv in list.as_slice() {
            let undo = child.make_move_perft(mv);
            nodes += child.perft_visitor(depth - 1);
            child.unmake_move_perft(mv, undo);
        }
        nodes
    }

    /// Sets the non-piece state (used by the FEN parser) and recomputes the
    /// full zobrist hash from scratch. `castle_rook_sq` gives the rook square
    /// backing each right (bit 0..3 = WK, WQ, BK, BQ); rights-clearing is
    /// derived on the fly in `castle_rights_after` (close-gap D3, task 4.1 —
    /// no 64B `castle_mask` table is stored).
    pub(crate) fn set_state(
        &mut self,
        turn: Color,
        castling: u8,
        castle_rook_sq: [u8; 4],
        ep: u8,
        halfmove: u16,
        fullmove: u16,
    ) {
        self.turn = turn;
        self.castling = castling;
        self.castle_rook_sq = castle_rook_sq;
        self.ep = ep;
        self.halfmove = halfmove;
        self.fullmove = fullmove;
        self.hash = self.zobrist_full();
        // Refresh cached checkers for the new side to move (branch-free in_check).
        self.checkers = self.attackers_to(
            self.king_sq[self.turn.index()],
            self.turn.other(),
            self.occupied(),
        );
        let std_rooks: [u8; 4] = [7, 0, 63, 56];
        let mut is960 = false;
        if castling != 0 {
            for i in 0..4 {
                if castling & (1 << i) != 0 && castle_rook_sq[i] != std_rooks[i] {
                    is960 = true;
                    break;
                }
            }
            if self.king_sq[Color::White.index()] != 4 || self.king_sq[Color::Black.index()] != 60 {
                is960 = true;
            }
        }
        self.is_chess960 = is960;
    }
}


#[cfg(feature = "exp-fused-perft")]
struct PerftLeafSink { child: Board, nodes:u64 }
#[cfg(feature = "exp-fused-perft")]
impl MoveSink for PerftLeafSink {
    #[inline(always)]
    fn push_one(&mut self,mv:Move) {
        let undo=self.child.make_perft_internal(mv);
        self.nodes+=self.child.count_legal_moves() as u64;
        self.child.unmake_move_perft(mv,undo);
    }
    #[inline(always)]
    fn push_targets(&mut self,from:u8,mut targets:u64) {
        while targets!=0 {let to=pop_lsb(&mut targets);self.push_one(Move::quiet(Square(from),Square(to)));}
    }
    #[inline(always)]
    fn push_pawn_targets_offset(&mut self,mut targets:u64,offset:i8) {
        while targets!=0 {let to=pop_lsb(&mut targets);self.push_one(Move::quiet(Square((to as i16-offset as i16) as u8),Square(to)));}
    }
    #[inline(always)]
    fn push_pawn_promotions_offset(&mut self,mut targets:u64,offset:i8) {
        while targets!=0 {
            let to=pop_lsb(&mut targets);
            let base=(to as i16-offset as i16) as u16|((to as u16)<<6);
            for r in 1..=4 {self.push_one(Move(base|(r<<12)));}
        }
    }
}


#[cfg(feature = "exp-tagged-perft")]
struct TaggedPerftList<'a>{board:&'a Board,moves:ArrayVec<(Move,u8),MAX_MOVES>}
#[cfg(feature = "exp-tagged-perft")]
impl MoveSink for TaggedPerftList<'_>{
    #[inline(always)]
    fn push_targets(&mut self,from:u8,mut targets:u64){
        if targets==0{return;}
        let code=self.board.piece_code_at_color(from,self.board.turn);
        while targets!=0{let to=pop_lsb(&mut targets);self.moves.push((Move::quiet(Square(from),Square(to)),code));}
    }
    #[inline(always)]
    fn push_pawn_targets_offset(&mut self,mut targets:u64,offset:i8){
        let code=self.board.turn as u8*6;
        while targets!=0{let to=pop_lsb(&mut targets);self.moves.push((Move::quiet(Square((to as i16-offset as i16)as u8),Square(to)),code));}
    }
    #[inline(always)]
    fn push_pawn_promotions_offset(&mut self,mut targets:u64,offset:i8){
        let code=self.board.turn as u8*6;
        while targets!=0{let to=pop_lsb(&mut targets);let base=(to as i16-offset as i16)as u16|((to as u16)<<6);for r in 1..=4{self.moves.push((Move(base|(r<<12)),code));}}
    }
    #[inline(always)]
    fn push_one(&mut self,mv:Move){
        let code=self.board.piece_code_at_color(mv.from().0,self.board.turn);
        self.moves.push((mv,code));
    }
}

#[cfg(feature="exp-full-visitor")]
struct FullLeafSink<'a,F>{parent:&'a Board,visitor:&'a mut F,nodes:u64}
#[cfg(feature="exp-full-visitor")]
impl<F:FnMut(&Board)> MoveSink for FullLeafSink<'_,F>{
    #[inline(always)]
    fn push_one(&mut self,mv:Move){
        let mut child=*self.parent;
        child.make_move_perft(mv);
        (self.visitor)(&child);
        self.nodes+=1;
    }
    #[inline(always)]
    fn push_targets(&mut self,from:u8,mut targets:u64){
        if targets==0{return;}
        let moved=self.parent.piece_code_at_color(from,self.parent.turn);
        #[cfg(feature="exp-leaf-reuse")]
        {
            let mut child=*self.parent;
            while targets!=0 {
                let to=pop_lsb(&mut targets);let mv=Move::quiet(Square(from),Square(to));
                let undo=child.make_move_perft_known::<true>(mv,moved);
                (self.visitor)(&child);self.nodes+=1;child.unmake_move_perft(mv,undo);
            }
        }
        #[cfg(not(feature="exp-leaf-reuse"))]
        while targets!=0 {
            let to=pop_lsb(&mut targets);
            let mut child=*self.parent;
            child.make_move_perft_known::<true>(Move::quiet(Square(from),Square(to)),moved);
            (self.visitor)(&child);
            self.nodes+=1;
        }
    }
    #[inline(always)]
    fn push_pawn_targets_offset(&mut self,mut targets:u64,offset:i8){
        let moved=self.parent.turn as u8*6;
        #[cfg(feature="exp-leaf-reuse")]
        {
            let mut child=*self.parent;
            while targets!=0{
                let to=pop_lsb(&mut targets);let from=(to as i16-offset as i16) as u8;
                let mv=Move::quiet(Square(from),Square(to));let undo=child.make_move_perft_known::<true>(mv,moved);
                (self.visitor)(&child);self.nodes+=1;child.unmake_move_perft(mv,undo);
            }
        }
        #[cfg(not(feature="exp-leaf-reuse"))]
        while targets!=0{
            let to=pop_lsb(&mut targets);let from=(to as i16-offset as i16) as u8;
            let mut child=*self.parent;
            child.make_move_perft_known::<true>(Move::quiet(Square(from),Square(to)),moved);
            (self.visitor)(&child);
            self.nodes+=1;
        }
    }
    #[inline(always)]
    fn push_pawn_promotions_offset(&mut self,mut targets:u64,offset:i8){
        #[cfg(all(feature="exp-promotion-batch",not(feature="exp-no-promotion-batch")))]
        {
            let us=self.parent.turn;let ui=us.index();let ti=us.other().index();
            while targets!=0{
                let to=pop_lsb(&mut targets);let from=(to as i16-offset as i16) as u8;
                let mut child=*self.parent;
                let mv=Move::new(Square(from),Square(to),Some(Role::Knight));
                child.make_move_perft_known::<true>(mv,us as u8*6);
                (self.visitor)(&child);self.nodes+=1;
                for role in 2..=4{
                    child.bb_remove(ui,role-1,bit(to));child.bb_add(ui,role,bit(to));
                    child.checkers=child.attackers_to(child.king_sq[ti],us,child.occupied());
                    (self.visitor)(&child);self.nodes+=1;
                }
            }
        }
        #[cfg(not(all(feature="exp-promotion-batch",not(feature="exp-no-promotion-batch"))))]
        while targets!=0 {let to=pop_lsb(&mut targets);let from=(to as i16-offset as i16) as u8;for r in 1..=4{self.push_one(Move(from as u16|((to as u16)<<6)|(r<<12)));}}
    }
}
