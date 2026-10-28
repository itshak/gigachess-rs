// High-throughput `moves2` batch replay engine.
//
// Games are stored as slices of 16-bit packed moves (see `crate::moves`).
// `replay_moves2_stream` replays one game, reporting either the final
// Polyglot zobrist hash or the ply at which the first illegal move occurred.
// `replay_moves2_batch` replays many games in parallel on Rayon's global
// work-stealing thread pool (see `openspec/adr/002-parallel-replay-with-rayon.md`
// for the backend decision and benchmark data).
//
// SPDX-License-Identifier: MIT

use crate::board::Board;
use crate::moves::Move;

/// Outcome of replaying one game.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ReplayOutcome {
    /// Final zobrist hash if every move was legal, `None` otherwise.
    pub hash: Option<u64>,
    /// Number of moves successfully played before termination.
    pub moves_played: u32,
}

impl ReplayOutcome {
    /// True when the entire stream was legal.
    #[inline]
    pub fn is_legal(&self) -> bool {
        self.hash.is_some()
    }
}

/// Execution control for replay callbacks.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Flow {
    /// Continue replaying the next move.
    Continue,
    /// Stop replaying moves for this game immediately.
    Break,
}

/// Replays a validated `moves2` stream using fast unvalidated makes (`make_move_hashed`).
///
/// Designed for trusted stores (such as `.bbrb` reference collections or validated `.bbdb` databases)
/// where moves were already verified legal upon ingest.
/// Dispatches `Move::NULL` transparently.
/// Invokes `callback(&board, ply)` at ply 0 (initial board) and after each move.
/// If `callback` returns `Flow::Break`, replay terminates early and returns the current board.
pub fn replay_moves2_trusted<F>(start: &Board, moves: &[u16], mut callback: F) -> Board
where
    F: FnMut(&Board, usize) -> Flow,
{
    let mut board = *start;
    if callback(&board, 0) == Flow::Break {
        return board;
    }
    for (i, &word) in moves.iter().enumerate() {
        let mv = Move::from_word(word);
        board.make_move_hashed(mv);
        if callback(&board, i + 1) == Flow::Break {
            break;
        }
    }
    board
}

/// Replays a single `moves2` stream from the starting position.
pub fn replay_moves2_stream(moves: &[u16]) -> ReplayOutcome {
    let mut board = Board::startpos();
    for (ply, &word) in moves.iter().enumerate() {
        let mv = Move::from_word(word);
        if board.play(mv).is_err() {
            return ReplayOutcome {
                hash: None,
                moves_played: ply as u32,
            };
        }
    }
    ReplayOutcome {
        hash: Some(board.zobrist()),
        moves_played: moves.len() as u32,
    }
}

/// Replays a batch of games in parallel on Rayon's global work-stealing
/// thread pool, using every CPU core.
///
/// Results are returned in the same order as the input games. Games are
/// distributed individually by Rayon's scheduler, so asymmetric cores
/// (e.g. Apple Silicon P/E cores) cannot strand statically-assigned work
/// (ADR-002: ~1.48M games/s vs ~1.21M for the best static-chunking effort).
pub fn replay_moves2_batch(games: &[&[u16]]) -> Vec<ReplayOutcome> {
    use rayon::prelude::*;
    games
        .par_iter()
        .map(|game| replay_moves2_stream(game))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push(mv: &mut Vec<u16>, from: &str, to: &str) {
        let f = crate::types::Square::from_alg(from).unwrap();
        let t = crate::types::Square::from_alg(to).unwrap();
        mv.push(Move::new(f, t, None).word());
    }

    #[test]
    fn replay_legal_stream() {
        // 1. e4 e5 2. Nf3
        let mut game = Vec::new();
        push(&mut game, "e2", "e4");
        push(&mut game, "e7", "e5");
        push(&mut game, "g1", "f3");
        let out = replay_moves2_stream(&game);
        assert!(out.is_legal());
        assert_eq!(out.moves_played, 3);
        let mut board = Board::startpos();
        for &w in &game {
            board.play(Move::from_word(w)).unwrap();
        }
        assert_eq!(out.hash, Some(board.zobrist()));
    }

    #[test]
    fn replay_illegal_move_detected() {
        // 1. e4 e5 2. Qh6 (illegal: queen on d1 cannot reach h6)
        let mut game = Vec::new();
        push(&mut game, "e2", "e4");
        push(&mut game, "e7", "e5");
        push(&mut game, "d1", "h6");
        let out = replay_moves2_stream(&game);
        assert!(!out.is_legal());
        assert_eq!(out.moves_played, 2);
    }

    #[test]
    fn batch_preserves_order() {
        let mut games: Vec<Vec<u16>> = Vec::new();
        for n in 0..200u16 {
            let mut g = Vec::new();
            let openings: [(&str, &str); 4] = [("e2", "e4"), ("d2", "d4"), ("g1", "f3"), ("c2", "c4")];
            let (f, t) = openings[(n % 4) as usize];
            push(&mut g, f, t);
            games.push(g);
        }
        let refs: Vec<&[u16]> = games.iter().map(|g| g.as_slice()).collect();
        let out = replay_moves2_batch(&refs);
        assert_eq!(out.len(), 200);
        for (i, o) in out.iter().enumerate() {
            let single = replay_moves2_stream(&games[i]);
            assert_eq!(*o, single);
        }
    }

    #[test]
    fn replay_moves2_trusted_parity_with_play() {
        let mut game = Vec::new();
        push(&mut game, "e2", "e4");
        push(&mut game, "e7", "e5");
        push(&mut game, "g1", "f3");
        push(&mut game, "b8", "c6");
        push(&mut game, "f1", "b5");
        game.push(Move::NULL.word()); // pass
        game.push(Move::NULL.word()); // reply pass
        push(&mut game, "a7", "a6");

        let mut checked_hashes = Vec::new();
        let mut b_checked = Board::startpos();
        checked_hashes.push(b_checked.zobrist());
        for &w in &game {
            b_checked.play(Move::from_word(w)).unwrap();
            checked_hashes.push(b_checked.zobrist());
        }

        let mut trusted_hashes = Vec::new();
        let final_board = replay_moves2_trusted(&Board::startpos(), &game, |b, _ply| {
            trusted_hashes.push(b.zobrist());
            Flow::Continue
        });

        assert_eq!(trusted_hashes, checked_hashes);
        assert_eq!(final_board.zobrist(), b_checked.zobrist());
    }

    #[test]
    fn replay_moves2_trusted_early_break() {
        let mut game = Vec::new();
        push(&mut game, "e2", "e4");
        push(&mut game, "e7", "e5");
        push(&mut game, "g1", "f3");

        let mut count = 0;
        let final_board = replay_moves2_trusted(&Board::startpos(), &game, |_b, ply| {
            count += 1;
            if ply == 1 {
                Flow::Break
            } else {
                Flow::Continue
            }
        });

        assert_eq!(count, 2); // ply 0 and ply 1
        let mut expected = Board::startpos();
        expected.play(Move::from_word(game[0])).unwrap();
        assert_eq!(final_board.zobrist(), expected.zobrist());
    }
}
