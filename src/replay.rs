// High-throughput `moves2` batch replay engine.
//
// Games are stored as slices of 16-bit packed moves (see `crate::moves`).
// `replay_moves2_stream` replays one game, reporting either the final
// Polyglot zobrist hash or the ply at which the first illegal move occurred.
// `replay_moves2_batch` replays many games in parallel across all CPU cores
// using scoped threads (chunked, order-preserving).
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

/// Replays a batch of games in parallel across all CPU cores.
///
/// Results are returned in the same order as the input games. Games are
/// chunked across scoped worker threads; each game itself is replayed on a
/// single thread (per-game replay is allocation-light and embarrassingly
/// parallel at the batch level).
pub fn replay_moves2_batch(games: &[&[u16]]) -> Vec<ReplayOutcome> {
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    // Small batches or single-core machines: sequential path, no thread spawn.
    if threads <= 1 || games.len() < 64 {
        return games.iter().map(|g| replay_moves2_stream(g)).collect();
    }

    let chunk_len = games.len().div_ceil(threads);
    std::thread::scope(|scope| {
        let handles: Vec<_> = games
            .chunks(chunk_len)
            .map(|chunk| scope.spawn(move || chunk.iter().map(|g| replay_moves2_stream(g)).collect::<Vec<_>>()))
            .collect();
        let mut out = Vec::with_capacity(games.len());
        for h in handles {
            out.extend(h.join().expect("replay worker panicked"));
        }
        out
    })
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
}
