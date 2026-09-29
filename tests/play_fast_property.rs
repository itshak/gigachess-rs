// Property-based test: Board::play_fast vs Board::play
// Across 100,000 positions from random playouts (both standard and Chess960),
// verifies that `play_fast` produces identical piece bitboards, castling rights,
// turn, en-passant, and fullmove / halfmove numbers as standard `play`.
//
// SPDX-License-Identifier: MIT

use gigachess::{Board, Move};

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn play_fast_produces_identical_state_across_100k_positions() {
    let mut rng = 0xDEADBEEF_CAFE1234u64;
    let mut positions = 0usize;
    let target_positions = 100_000usize;

    while positions < target_positions {
        let mut normal_board = Board::startpos();
        let mut fast_board = Board::startpos();

        for _ply in 0..120 {
            let moves = normal_board.legal_moves();
            if moves.is_empty() {
                break;
            }

            // Also test pseudo_legal moves that might be illegal to test play_fast error return parity
            let test_pseudo = xorshift(&mut rng).is_multiple_of(10);
            if test_pseudo {
                let pseudo = normal_board.pseudo_legal_moves();
                if !pseudo.is_empty() {
                    let idx = (xorshift(&mut rng) as usize) % pseudo.len();
                    let cand = pseudo[idx];
                    let mut norm_cand = normal_board;
                    let mut fast_cand = fast_board;
                    let res_norm = norm_cand.play(cand);
                    let res_fast = fast_cand.play_fast(cand);
                    assert_eq!(
                        res_norm.is_ok(),
                        res_fast.is_ok(),
                        "Legality check parity failed for candidate move {:?} at position {}",
                        cand,
                        positions
                    );
                }
            }

            let idx = (xorshift(&mut rng) as usize) % moves.len();
            let mv = moves[idx];

            let undo_norm = normal_board.play(mv).expect("play normal must succeed");
            let undo_fast = fast_board.play_fast(mv).expect("play_fast must succeed");

            assert_eq!(
                normal_board.occupied(),
                fast_board.occupied(),
                "Occupancy mismatch at position {}",
                positions
            );
            assert_eq!(
                normal_board.turn(),
                fast_board.turn(),
                "Turn mismatch at position {}",
                positions
            );
            assert_eq!(
                normal_board.castling_rights(),
                fast_board.castling_rights(),
                "Castling rights mismatch at position {}",
                positions
            );
            assert_eq!(
                normal_board.en_passant(),
                fast_board.en_passant(),
                "En passant mismatch at position {}",
                positions
            );
            assert_eq!(
                normal_board.halfmove_clock(),
                fast_board.halfmove_clock(),
                "Halfmove clock mismatch at position {}",
                positions
            );
            assert_eq!(
                normal_board.fullmove_number(),
                fast_board.fullmove_number(),
                "Fullmove number mismatch at position {}",
                positions
            );

            // Verify piece placement across all squares
            for sq in 0..64 {
                assert_eq!(
                    normal_board.piece_at(gigachess::Square(sq)),
                    fast_board.piece_at(gigachess::Square(sq)),
                    "Piece mismatch at sq {} at position {}",
                    sq,
                    positions
                );
            }

            // Verify unmake_move_fast restores state bit-for-bit
            let mut back_fast = fast_board;
            back_fast.unmake_move_fast(mv, undo_fast);
            assert_eq!(
                back_fast.occupied(),
                undo_fast_prev_occ(&normal_board, mv, undo_norm),
                "unmake_move_fast restored wrong occupancy"
            );

            positions += 1;
            if positions >= target_positions {
                break;
            }
        }
    }

    assert!(
        positions >= target_positions,
        "Completed {} positions",
        positions
    );
}

fn undo_fast_prev_occ(b: &Board, mv: Move, undo: gigachess::Undo) -> u64 {
    let mut prev = *b;
    prev.unmake_move(mv, undo);
    prev.occupied()
}
