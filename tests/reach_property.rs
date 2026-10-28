// Property-based test: ReachSignature pruning soundness
// Across 50,000 positions from random legal playouts, verifies that
// NO position pruned as unreachable is followed by the target position.
//
// SPDX-License-Identifier: MIT

use gigachess::{Board, ReachSignature};

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn reach_signature_soundness_across_random_games() {
    let mut rng = 0x5EED_1234_5678_9ABCu64;
    let mut total_positions = 0usize;
    let target_positions = 50_000usize;

    while total_positions < target_positions {
        let mut board = Board::startpos();
        let mut history: Vec<Board> = vec![board];

        for _ply in 0..100 {
            let moves = board.legal_moves();
            if moves.is_empty() {
                break;
            }
            let idx = (xorshift(&mut rng) as usize) % moves.len();
            board.play(moves[idx]).expect("legal move must play");
            history.push(board);
            total_positions += 1;
        }

        let n = history.len();
        if n < 2 {
            continue;
        }

        // Pick sample target positions from the game's history
        let step = (n / 5).max(1);
        for target_idx in (1..n).step_by(step) {
            let target_board = &history[target_idx];
            let sig = ReachSignature::from_board(target_board);

            // Assert that for every predecessor j <= target_idx,
            // sig.unreachable_from(&history[j]) is FALSE.
            for j in 0..=target_idx {
                assert!(
                    !sig.unreachable_from(&history[j]),
                    "Soundness violation! Target at ply {} was pruned as unreachable from predecessor at ply {}",
                    target_idx,
                    j
                );
            }
        }
    }
}
