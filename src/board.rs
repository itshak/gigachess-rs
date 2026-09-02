// Board representation, make/unmake move, and fully-legal move generation
// with check and pin masking (zero heap allocations in the movegen loop).
//
// Board layout: 12 piece bitboards + occupancy + mailbox for O(1) piece
// lookup. The zobrist hash is maintained incrementally in `make_move` and
// restored on `unmake`.
//
// SPDX-License-Identifier: MIT

use crate::attacks;
use crate::bitboard::{
    bit, pop_lsb, popcount, BETWEEN, KING_ATT, KNIGHT_ATT, LINE, PAWN_ATT,
};
use crate::moves::Move;
use crate::types::{Color, Piece, Role, Square, NO_EP, CASTLE_BK, CASTLE_BQ, CASTLE_WK, CASTLE_WQ};
use crate::zobrist;
use arrayvec::ArrayVec;

/// Maximum legal moves in any position (hard bound for the stack buffer).
pub const MAX_MOVES: usize = 256;

/// Mailbox sentinel: empty square.
const EMPTY: u8 = u8::MAX;

/// Information required to undo a move.
#[derive(Copy, Clone, Debug)]
pub struct Undo {
    hash: u64,
    castling: u8,
    ep: u8,
    halfmove: u16,
    captured: u8,
    /// True when the move was a castling move (king-from → rook-square).
    castled: bool,
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
/// square, clocks and the incrementally-maintained Polyglot zobrist hash.
///
/// `Board` is plain data and [`Copy`]: copying it yields a bit-for-bit
/// snapshot suitable for engine search stacks (ADR-003, decision D6).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
    bbs: [[u64; 6]; 2],  // [color_index][role_index]
    occ: [u64; 2],       // occupancy per color
    occupied: u64,       // total occupancy
    mailbox: [u8; 64],   // piece code or EMPTY
    turn: Color,
    castling: u8,
    /// Rook square backing each castling right (bit 0..3 = WK, WQ, BK, BQ).
    /// Chess960: arbitrary back-rank squares; standard: h1/a1/h8/a8.
    castle_rook_sq: [u8; 4],
    /// Per-square rights-clear mask: `castling &= castle_mask[from] & castle_mask[to]`.
    castle_mask: [u8; 64],
    ep: u8,
    halfmove: u16,
    fullmove: u16,
    hash: u64,
    king_sq: [u8; 2],
}

impl Board {
    /// Empty board with White to move.
    pub fn empty() -> Board {
        Board {
            bbs: [[0; 6]; 2],
            occ: [0; 2],
            occupied: 0,
            mailbox: [EMPTY; 64],
            turn: Color::White,
            castling: 0,
            castle_rook_sq: [0; 4],
            castle_mask: [0x0F; 64],
            ep: NO_EP,
            halfmove: 0,
            fullmove: 1,
            hash: 0,
            king_sq: [0; 2],
        }
    }

    /// The standard chess starting position.
    pub fn startpos() -> Board {
        crate::fen::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("valid startpos FEN")
    }

    // -- accessors ----------------------------------------------------------

    /// Side to move.
    #[inline]
    pub fn turn(&self) -> Color {
        self.turn
    }

    /// Bitboard of all squares occupied by `color`.
    #[inline]
    pub fn occ_color(&self, color: Color) -> u64 {
        self.occ[color.index()]
    }

    /// Bitboard of every occupied square.
    #[inline]
    pub fn occupied(&self) -> u64 {
        self.occupied
    }

    /// Bitboard of all pieces of `role` belonging to `color`.
    #[inline]
    pub fn piece_bb(&self, color: Color, role: Role) -> u64 {
        self.bbs[color.index()][role.index()]
    }

    /// Piece standing on `sq`, if any.
    #[inline]
    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        Piece::from_code(self.mailbox[sq.index()])
    }

    /// King square of `color`.
    #[inline]
    pub fn king_square(&self, color: Color) -> Square {
        Square(self.king_sq[color.index()])
    }

    /// Castling rights bitmask (bits 0..3 = WK, WQ, BK, BQ).
    #[inline]
    pub fn castling_rights(&self) -> u8 {
        self.castling
    }

    /// The rook square backing castling right `right_bit` (0..3 = WK, WQ, BK,
    /// BQ). Chess960-aware: standard rights map to files a/h.
    #[inline]
    pub fn castling_rook_square(&self, right_bit: u8) -> Square {
        Square(self.castle_rook_sq[right_bit as usize])
    }

    /// En-passant target square, if any.
    #[inline]
    pub fn en_passant(&self) -> Option<Square> {
        if self.ep == NO_EP {
            None
        } else {
            Some(Square(self.ep))
        }
    }

    /// Halfmove clock (plies since the last pawn move or capture).
    #[inline]
    pub fn halfmove_clock(&self) -> u16 {
        self.halfmove
    }

    /// Fullmove number.
    #[inline]
    pub fn fullmove_number(&self) -> u16 {
        self.fullmove
    }

    /// Incrementally-maintained Polyglot zobrist hash of the position.
    #[inline]
    pub fn zobrist(&self) -> u64 {
        self.hash
    }

    // -- internal piece bookkeeping (hash-aware) ----------------------------

    #[inline]
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

    #[inline]
    fn color_from_index(color: usize) -> Color {
        if color == 1 {
            Color::White
        } else {
            Color::Black
        }
    }

    #[inline]
    pub(crate) fn put_piece(&mut self, sq: u8, code: u8) {
        let color = (code / 6) as usize;
        let role = (code % 6) as usize;
        let b = bit(sq);
        self.bbs[color][role] |= b;
        self.occ[color] |= b;
        self.occupied |= b;
        self.mailbox[sq as usize] = code;
        self.hash ^= zobrist::piece_key(
            Self::color_from_index(color),
            Self::role_from_index(role),
            Square(sq),
        );
        if role == Role::King.index() {
            self.king_sq[color] = sq;
        }
    }

    #[inline]
    fn remove_piece(&mut self, sq: u8, code: u8) {
        let color = (code / 6) as usize;
        let role = (code % 6) as usize;
        let b = bit(sq);
        self.bbs[color][role] &= !b;
        self.occ[color] &= !b;
        self.occupied &= !b;
        self.mailbox[sq as usize] = EMPTY;
        self.hash ^= zobrist::piece_key(
            Self::color_from_index(color),
            Self::role_from_index(role),
            Square(sq),
        );
    }

    #[inline]
    fn move_piece(&mut self, from: u8, to: u8, code: u8) {
        self.remove_piece(from, code);
        self.put_piece(to, code);
    }

    /// True when a pawn of `side` can pseudo-legally capture on the
    /// en-passant square (Polyglot ep-hash relevance condition).
    #[inline]
    fn ep_relevant(&self, ep: u8, side: Color) -> bool {
        if ep == NO_EP {
            return false;
        }
        PAWN_ATT[side.other().index()][ep as usize]
            & self.bbs[side.index()][Role::Pawn.index()]
            != 0
    }

    /// Squares occupied by pieces of `by` that attack `sq` under occupancy `occ`.
    pub fn attackers_to(&self, sq: u8, by: Color, occ: u64) -> u64 {
        let bi = by.index();
        let mut a = 0u64;
        // Pawns of `by` attacking sq stand where a pawn of the other color
        // placed on sq would attack.
        a |= PAWN_ATT[by.other().index()][sq as usize] & self.bbs[bi][Role::Pawn.index()];
        a |= KNIGHT_ATT[sq as usize] & self.bbs[bi][Role::Knight.index()];
        a |= KING_ATT[sq as usize] & self.bbs[bi][Role::King.index()];
        a |= attacks::bishop_attacks(sq, occ)
            & (self.bbs[bi][Role::Bishop.index()] | self.bbs[bi][Role::Queen.index()]);
        a |= attacks::rook_attacks(sq, occ)
            & (self.bbs[bi][Role::Rook.index()] | self.bbs[bi][Role::Queen.index()]);
        a
    }

    /// True when the king of `color` is attacked.
    #[inline]
    pub fn king_attacked(&self, color: Color) -> bool {
        self.attackers_to(self.king_sq[color.index()], color.other(), self.occupied) != 0
    }

    /// True when the side to move is in check.
    #[inline]
    pub fn in_check(&self) -> bool {
        self.king_attacked(self.turn)
    }

    /// XOR-fold of the rook-file castling keys for the given rights bitmask
    /// (ADR-003: keys are per (color, rook file)).
    #[inline]
    fn castle_rights_hash(&self, rights: u8) -> u64 {
        zobrist::castle_file_keys_xor(rights, &self.castle_rook_sq)
    }

    /// Recomputes the Polyglot zobrist hash from scratch (verification aid;
    /// the incremental hash in `self.hash` must always agree with this).
    pub fn zobrist_full(&self) -> u64 {
        let mut h = 0u64;
        for ci in 0..2 {
            for r in 0..6 {
                let mut b = self.bbs[ci][r];
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
            self.bbs[self.turn.index()][Role::Pawn.index()],
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
    pub fn legal_moves(&self) -> ArrayVec<Move, MAX_MOVES> {
        let us = self.turn;
        let them = us.other();
        let ui = us.index();
        let ti = them.index();
        let ksq = self.king_sq[ui];
        let occ = self.occupied;
        let mut moves = ArrayVec::new();

        // Enemy attack map with our king removed (x-ray safe king moves).
        let occ_no_king = occ ^ bit(ksq);
        let their_bq = self.bbs[ti][Role::Bishop.index()] | self.bbs[ti][Role::Queen.index()];
        let their_rq = self.bbs[ti][Role::Rook.index()] | self.bbs[ti][Role::Queen.index()];
        let mut danger = 0u64;
        let mut p = self.bbs[ti][Role::Pawn.index()];
        while p != 0 {
            let sq = pop_lsb(&mut p);
            danger |= PAWN_ATT[ti][sq as usize];
        }
        let mut p = self.bbs[ti][Role::Knight.index()];
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

        // King steps (castling handled separately).
        let mut p = KING_ATT[ksq as usize] & !self.occ[ui] & !danger;
        while p != 0 {
            let to = pop_lsb(&mut p);
            moves.push(Move::new(Square(ksq), Square(to), None));
        }

        let checkers = self.attackers_to(ksq, them, occ);
        if popcount(checkers) >= 2 {
            return moves; // double check: only king moves are legal
        }

        // When in check, non-king moves must capture the checker or block it.
        let allowed = if checkers == 0 {
            !0u64
        } else {
            let csq = crate::bitboard::lsb(checkers);
            bit(csq) | BETWEEN[ksq as usize][csq as usize]
        };

        // Pinned pieces and the line each must remain on.
        let mut pinned = 0u64;
        let mut pin_line = [0u64; 64];
        let snipers =
            (attacks::bishop_attacks(ksq, 0) & their_bq) | (attacks::rook_attacks(ksq, 0) & their_rq);
        let mut p = snipers;
        while p != 0 {
            let sniper = pop_lsb(&mut p);
            let between = BETWEEN[ksq as usize][sniper as usize] & occ;
            if popcount(between) == 1 && between & self.occ[ui] != 0 {
                let pinned_sq = crate::bitboard::lsb(between);
                pinned |= bit(pinned_sq);
                pin_line[pinned_sq as usize] = LINE[ksq as usize][sniper as usize];
            }
        }

        // Knights (a pinned knight can never move).
        let mut p = self.bbs[ui][Role::Knight.index()] & !pinned;
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = KNIGHT_ATT[from as usize] & !self.occ[ui] & allowed;
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        // Bishops and queens (diagonal moves).
        let mut p = self.bbs[ui][Role::Bishop.index()] | self.bbs[ui][Role::Queen.index()];
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = attacks::bishop_attacks(from, occ) & !self.occ[ui] & allowed;
            if pinned & bit(from) != 0 {
                t &= pin_line[from as usize];
            }
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        // Rooks and queens (straight moves).
        let mut p = self.bbs[ui][Role::Rook.index()] | self.bbs[ui][Role::Queen.index()];
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = attacks::rook_attacks(from, occ) & !self.occ[ui] & allowed;
            if pinned & bit(from) != 0 {
                t &= pin_line[from as usize];
            }
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        self.gen_pawn_moves(&mut moves, allowed, pinned, &pin_line);

        // Castling is only possible when not in check.
        if checkers == 0 {
            self.gen_castling(&mut moves, danger);
        }

        moves
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
        let occ = self.occupied;
        let them_occ = self.occ[them.index()];
        let start_rank: u8 = if white { 1 } else { 6 };
        let promo_rank: u8 = if white { 7 } else { 0 };

        let mut p = self.bbs[ui][Role::Pawn.index()];
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
                let their_pawns = self.bbs[ti][Role::Pawn.index()] & !bit(cap_sq);
                let ksq = self.king_sq[ui];
                let mut attackers =
                    attacks::bishop_attacks(ksq, occ_after)
                        & (self.bbs[ti][Role::Bishop.index()] | self.bbs[ti][Role::Queen.index()]);
                attackers |= attacks::rook_attacks(ksq, occ_after)
                    & (self.bbs[ti][Role::Rook.index()] | self.bbs[ti][Role::Queen.index()]);
                attackers |= KNIGHT_ATT[ksq as usize] & self.bbs[ti][Role::Knight.index()];
                attackers |= PAWN_ATT[ui][ksq as usize] & their_pawns;
                attackers |= KING_ATT[ksq as usize] & self.bbs[ti][Role::King.index()];
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
        let kingside = right_bit % 2 == 0; // bits 0, 2 = kingside; 1, 3 = queenside
        let rank = ksq >> 3;
        let kf = (rank << 3) | if kingside { 6 } else { 2 };
        let rf = (rank << 3) | if kingside { 5 } else { 3 };
        let them = self.turn.other();

        // Emptiness: strictly between king and rook, king's transit squares,
        // and both final squares — minus the king's and rook's own squares
        // (they are allowed to stand on each other's path in Chess960).
        let need_empty = (BETWEEN[ksq as usize][rook as usize]
            | BETWEEN[ksq as usize][kf as usize]
            | BETWEEN[rook as usize][rf as usize]
            | bit(kf)
            | bit(rf))
            & !bit(ksq)
            & !bit(rook);
        if self.occupied & need_empty != 0 {
            return None;
        }

        // King-path safety in the resulting occupancy: the king vacates its
        // square (x-rays through it count) and the rook leaves its square and
        // lands on `rf`.
        let occ_after = (self.occupied & !(bit(ksq) | bit(rook))) | bit(kf) | bit(rf);
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
                if self.occupied & between == 0 && danger & safe == 0 {
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
        let occ = self.occupied;
        let mut moves = ArrayVec::new();

        // Knights.
        let mut p = self.bbs[ui][Role::Knight.index()];
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = KNIGHT_ATT[from as usize] & !self.occ[ui];
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        // Bishops and queens (diagonals).
        let mut p = self.bbs[ui][Role::Bishop.index()] | self.bbs[ui][Role::Queen.index()];
        while p != 0 {
            let from = pop_lsb(&mut p);
            let mut t = attacks::bishop_attacks(from, occ) & !self.occ[ui];
            while t != 0 {
                let to = pop_lsb(&mut t);
                moves.push(Move::new(Square(from), Square(to), None));
            }
        }

        // Rooks and queens (straights).
        let mut p = self.bbs[ui][Role::Rook.index()] | self.bbs[ui][Role::Queen.index()];
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
    pub fn make_move_unchecked(&mut self, mv: Move) -> Undo {
        let mut undo = Undo {
            hash: self.hash,
            castling: self.castling,
            ep: self.ep,
            halfmove: self.halfmove,
            captured: EMPTY,
            castled: false,
        };
        let from = mv.from().0;
        let to = mv.to().0;
        let us = self.turn;
        let moved = self.mailbox[from as usize];
        debug_assert_ne!(moved, EMPTY, "make_move on empty square");
        let role = moved % 6;
        let is_pawn = role == Role::Pawn as u8;
        let diag = from & 7 != to & 7;

        // Hash out the old en-passant contribution (relevance uses the
        // mover's pawns, which have not changed yet).
        if self.ep_relevant(self.ep, us) {
            self.hash ^= zobrist::ep_key(Square(self.ep));
        }

        // Castling is encoded king-from → rook-square (ADR-003, decision D3):
        // a king move landing on the mover's own rook. Unambiguous — a normal
        // king move can never land on an own rook.
        let is_castle = role == Role::King as u8
            && self.mailbox[to as usize] != EMPTY
            && self.mailbox[to as usize] % 6 == Role::Rook as u8
            && (self.mailbox[to as usize] / 6) == us as u8;

        // Captures (including en passant onto an empty square).
        let mut captured_sq = to;
        if is_pawn && diag && self.mailbox[to as usize] == EMPTY {
            captured_sq = if us == Color::White { to - 8 } else { to + 8 };
        }
        if !is_castle && captured_sq != from && self.mailbox[captured_sq as usize] != EMPTY {
            let cap = self.mailbox[captured_sq as usize];
            undo.captured = cap;
            self.remove_piece(captured_sq, cap);
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
                    let rook_code = self.mailbox[to as usize];
                    self.remove_piece(from, moved);
                    self.remove_piece(to, rook_code);
                    self.put_piece(kf, moved);
                    self.put_piece(rf, rook_code);
                } else {
                    self.move_piece(from, to, moved);
                }
            }
        }

        // Castling rights (touching king/rook squares or rook captured).
        let new_castling =
            self.castling & self.castle_mask[from as usize] & self.castle_mask[to as usize];
        if new_castling != self.castling {
            self.hash ^= self.castle_rights_hash(self.castling ^ new_castling);
            self.castling = new_castling;
        }

        // En-passant square after a double pawn push.
        self.ep = if is_pawn && to.abs_diff(from) == 16 {
            (from + to) / 2
        } else {
            NO_EP
        };
        if self.ep_relevant(self.ep, us.other()) {
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
        self.hash ^= zobrist::turn_key();
        undo
    }

    /// Reverts the most recent move, restoring the exact prior position.
    pub fn unmake_move(&mut self, mv: Move, undo: Undo) {
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
            let king_code = self.mailbox[kf as usize];
            let rook_code = self.mailbox[rf as usize];
            debug_assert_eq!(king_code % 6, Role::King as u8);
            debug_assert_eq!(rook_code % 6, Role::Rook as u8);
            // Remove both, then place both (swap-safe, mirrors make_move).
            self.remove_piece(kf, king_code);
            self.remove_piece(rf, rook_code);
            self.put_piece(from, king_code);
            self.put_piece(to, rook_code);
            self.hash = undo.hash;
            self.castling = undo.castling;
            self.ep = undo.ep;
            self.halfmove = undo.halfmove;
            return;
        }

        let moved = self.mailbox[to as usize];
        debug_assert_ne!(moved, EMPTY, "unmake_move on empty destination");

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
                    let rook = self.mailbox[rfrom as usize];
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
        self.castling = undo.castling;
        self.ep = undo.ep;
        self.halfmove = undo.halfmove;
    }

    /// Validates `mv` against pseudo-legal geometry, applies it, and then
    /// confirms the mover's king is safe; rolls back and reports
    /// [`IllegalMove`] otherwise. This is the safe public entry point.
    pub fn play(&mut self, mv: Move) -> Result<Undo, IllegalMove> {
        if !self.is_pseudo_legal(mv) {
            return Err(IllegalMove);
        }
        let undo = self.make_move_unchecked(mv);
        let mover = self.turn.other();
        if self.attackers_to(self.king_sq[mover.index()], self.turn, self.occupied) != 0 {
            self.unmake_move(mv, undo);
            return Err(IllegalMove);
        }
        Ok(undo)
    }

    /// True when the move is legal in this position (without playing it).
    pub fn is_legal(&self, mv: Move) -> bool {
        self.legal_moves().contains(&mv)
    }

    /// True when `sq` is attacked by any piece of `by`.
    pub fn square_attacked(&self, sq: u8, by: Color) -> bool {
        self.attackers_to(sq, by, self.occupied) != 0
    }

    /// Structural (pseudo-legal) validation of `mv`: piece geometry,
    /// capture/en-passant/castling preconditions. King safety is verified by
    /// `play` after applying.
    pub fn is_pseudo_legal(&self, mv: Move) -> bool {
        let from = mv.from().0;
        let to = mv.to().0;
        if from == to || from > 63 || to > 63 {
            return false;
        }
        let code = self.mailbox[from as usize];
        if code == EMPTY || (code / 6) != self.turn as u8 {
            return false;
        }
        let role = code % 6;
        let us = self.turn;
        let them = us.other();
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
                    if self.mailbox[to as usize] != EMPTY {
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
                        if self.mailbox[mid as usize] != EMPTY {
                            return false;
                        }
                        true
                    } else {
                        false
                    }
                } else if df == 1 && dr == push.signum() {
                    let target = self.mailbox[to as usize];
                    if target != EMPTY {
                        (target / 6) == them as u8
                    } else {
                        self.ep != NO_EP && to == self.ep
                    }
                } else {
                    false
                }
            }
            r if r == Role::Knight as u8 => KNIGHT_ATT[from as usize] & bit(to) != 0,
            r if r == Role::Bishop as u8 => {
                attacks::bishop_attacks(from, self.occupied) & bit(to) != 0
            }
            r if r == Role::Rook as u8 => attacks::rook_attacks(from, self.occupied) & bit(to) != 0,
            r if r == Role::Queen as u8 => {
                attacks::queen_attacks(from, self.occupied) & bit(to) != 0
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
    pub fn perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = self.legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }
        let mut nodes = 0u64;
        let mut child = self.clone();
        for mv in moves {
            let undo = child.make_move_unchecked(mv);
            nodes += child.perft(depth - 1);
            child.unmake_move(mv, undo);
        }
        nodes
    }

    /// Sets the non-piece state (used by the FEN parser) and recomputes the
    /// full zobrist hash from scratch. `castle_rook_sq` gives the rook square
    /// backing each right (bit 0..3 = WK, WQ, BK, BQ); the per-square
    /// rights-clear mask is derived from the king and rook placements.
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
        let mut mask = [0x0Fu8; 64];
        for (rb, color) in [(0usize, Color::White), (1, Color::White), (2, Color::Black), (3, Color::Black)] {
            if castling & (1 << rb) != 0 {
                let both = if color == Color::White { CASTLE_WK | CASTLE_WQ } else { CASTLE_BK | CASTLE_BQ };
                // The king leaving its square forfeits both rights; the rook
                // leaving (or being captured on) its square forfeits its own.
                mask[self.king_sq[color.index()] as usize] &= !both;
                mask[castle_rook_sq[rb] as usize] &= !(1 << rb);
            }
        }
        self.castle_mask = mask;
        self.hash = self.zobrist_full();
    }
}
