// Reachability signature for early pruning during position searches (design D1).
//
// Computes piece and pawn invariants from a target board. A candidate position
// is unreachable if it violates pawn non-reversal, piece/pawn conservation,
// or promotion budgets.
//
// SPDX-License-Identifier: MIT

use crate::board::Board;
use crate::types::{Color, Role, RANK_BB};

/// A reachability signature derived from a target chess position (design D1).
///
/// Can determine in a few bitboard operations whether a candidate board can
/// legally transition into the target board through zero or more moves.
///
/// If `unreachable_from(&board)` returns true, it is mathematically impossible
/// for `board` to reach the target position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReachSignature {
    /// White pawns on their home rank (Rank 2) in the target position.
    pub white_home_pawns: u64,
    /// Black pawns on their home rank (Rank 7) in the target position.
    pub black_home_pawns: u64,
    /// Total count of white men (pieces + pawns) in the target position.
    pub white_men: u8,
    /// Total count of black men (pieces + pawns) in the target position.
    pub black_men: u8,
    /// White pawn count in the target position.
    pub white_pawns: u8,
    /// Black pawn count in the target position.
    pub black_pawns: u8,
    /// White piece counts by role: [Knight, Bishop, Rook, Queen] in the target position.
    pub white_pieces: [u8; 4],
    /// Black piece counts by role: [Knight, Bishop, Rook, Queen] in the target position.
    pub black_pieces: [u8; 4],
}

impl ReachSignature {
    /// Computes the reachability signature from `target`.
    pub fn from_board(target: &Board) -> Self {
        let white_pawn_bb = target.piece_bb(Color::White, Role::Pawn);
        let black_pawn_bb = target.piece_bb(Color::Black, Role::Pawn);

        let white_home_pawns = white_pawn_bb & RANK_BB[1];
        let black_home_pawns = black_pawn_bb & RANK_BB[6];

        let white_men = target.occ_color(Color::White).count_ones() as u8;
        let black_men = target.occ_color(Color::Black).count_ones() as u8;

        let white_pawns = white_pawn_bb.count_ones() as u8;
        let black_pawns = black_pawn_bb.count_ones() as u8;

        let white_pieces = [
            target.piece_bb(Color::White, Role::Knight).count_ones() as u8,
            target.piece_bb(Color::White, Role::Bishop).count_ones() as u8,
            target.piece_bb(Color::White, Role::Rook).count_ones() as u8,
            target.piece_bb(Color::White, Role::Queen).count_ones() as u8,
        ];
        let black_pieces = [
            target.piece_bb(Color::Black, Role::Knight).count_ones() as u8,
            target.piece_bb(Color::Black, Role::Bishop).count_ones() as u8,
            target.piece_bb(Color::Black, Role::Rook).count_ones() as u8,
            target.piece_bb(Color::Black, Role::Queen).count_ones() as u8,
        ];

        Self {
            white_home_pawns,
            black_home_pawns,
            white_men,
            black_men,
            white_pawns,
            black_pawns,
            white_pieces,
            black_pieces,
        }
    }

    /// Evaluates rules (a), (b), and (c) to test if the target is unreachable from `board`.
    ///
    /// Returns `true` if `board` can NEVER reach the target position.
    #[inline]
    pub fn unreachable_from(&self, board: &Board) -> bool {
        let cur_white_pawns_bb = board.piece_bb(Color::White, Role::Pawn);
        let cur_black_pawns_bb = board.piece_bb(Color::Black, Role::Pawn);

        // Rule (a): A target pawn on its home rank is missing from that square.
        // Pawns only advance, never retreat to their starting rank.
        if (cur_white_pawns_bb & self.white_home_pawns) != self.white_home_pawns {
            return true;
        }
        if (cur_black_pawns_bb & self.black_home_pawns) != self.black_home_pawns {
            return true;
        }

        let cur_white_men = board.occ_color(Color::White).count_ones() as u8;
        let cur_black_men = board.occ_color(Color::Black).count_ones() as u8;

        // Rule (b): A side has fewer pawns or fewer men than the target.
        // Total men and pawns never increase.
        if cur_white_men < self.white_men || cur_black_men < self.black_men {
            return true;
        }

        let cur_white_pawns = cur_white_pawns_bb.count_ones() as u8;
        let cur_black_pawns = cur_black_pawns_bb.count_ones() as u8;

        if cur_white_pawns < self.white_pawns || cur_black_pawns < self.black_pawns {
            return true;
        }

        // Rule (c): A side's count of a piece type plus its pawns in excess of the target's pawns
        // is below the target's count of that type.
        // Only promotion adds pieces, and each promotion spends a pawn.
        let white_excess_pawns = cur_white_pawns - self.white_pawns;
        let cur_white_pieces = [
            board.piece_bb(Color::White, Role::Knight).count_ones() as u8,
            board.piece_bb(Color::White, Role::Bishop).count_ones() as u8,
            board.piece_bb(Color::White, Role::Rook).count_ones() as u8,
            board.piece_bb(Color::White, Role::Queen).count_ones() as u8,
        ];
        for i in 0..4 {
            if cur_white_pieces[i] + white_excess_pawns < self.white_pieces[i] {
                return true;
            }
        }

        let black_excess_pawns = cur_black_pawns - self.black_pawns;
        let cur_black_pieces = [
            board.piece_bb(Color::Black, Role::Knight).count_ones() as u8,
            board.piece_bb(Color::Black, Role::Bishop).count_ones() as u8,
            board.piece_bb(Color::Black, Role::Rook).count_ones() as u8,
            board.piece_bb(Color::Black, Role::Queen).count_ones() as u8,
        ];
        for i in 0..4 {
            if cur_black_pieces[i] + black_excess_pawns < self.black_pieces[i] {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen::parse_fen;

    #[test]
    fn rule_a_home_pawn_missing() {
        // Target has white pawns on e2, d2.
        // If current board has already moved e2 to e4, target cannot be reached.
        let target = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let sig = ReachSignature::from_board(&target);

        let after_e4 = parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        assert!(sig.unreachable_from(&after_e4), "Rule (a) must prune because e2 pawn has left home rank");
    }

    #[test]
    fn rule_b_fewer_men_or_pawns() {
        // Target has 16 white men (8 pawns).
        let start = Board::startpos();
        let sig = ReachSignature::from_board(&start);

        // If white has lost a knight or a pawn, startpos cannot be reached.
        let white_missing_knight = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R1BQKBNR w KQkq - 0 1").unwrap();
        assert!(sig.unreachable_from(&white_missing_knight), "Rule (b) must prune when white has fewer men");

        let white_missing_pawn = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert!(sig.unreachable_from(&white_missing_pawn), "Rule (b) must prune when white has fewer pawns");
    }

    #[test]
    fn rule_c_piece_type_promotion_budget() {
        // Target has 2 white queens and 7 pawns.
        // If candidate has 1 queen and 7 pawns (excess pawns = 0), candidate can never get 2 queens
        // without promoting a pawn, which would leave it with 6 pawns (fewer than target's 7).
        let target = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNQ w KQkq - 0 1").unwrap();
        let sig = ReachSignature::from_board(&target);
        assert_eq!(sig.white_pieces[3], 2); // 2 queens
        assert_eq!(sig.white_pawns, 7);

        // Candidate with 1 queen and 7 pawns:
        let candidate = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(candidate.piece_bb(Color::White, Role::Queen).count_ones(), 1);
        assert_eq!(candidate.piece_bb(Color::White, Role::Pawn).count_ones(), 7);

        assert!(sig.unreachable_from(&candidate), "Rule (c) must prune because 1 queen + 0 excess pawns < 2 queens");
    }

    #[test]
    fn reachable_positions_not_pruned() {
        let start = Board::startpos();
        let target = parse_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2").unwrap();
        let sig = ReachSignature::from_board(&target);

        assert!(!sig.unreachable_from(&start), "Target 1.e4 e5 must be reachable from startpos");
        assert!(!sig.unreachable_from(&target), "Target must be reachable from itself");
    }
}
