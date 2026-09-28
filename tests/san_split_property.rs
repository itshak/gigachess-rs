// Property-based test: the SAN split is byte-identical to the monolith.
//
// `move_to_san` = `move_to_san_body` + a check/mate suffix. A caller that
// already holds the position after the move (a replay engine, a database
// exporter) appends `check_mate_suffix(after)` itself and pays neither the
// board copy nor the make/unmake that `move_to_san` needs to reach that
// position. This asserts over 100,000 positions from random playouts —
// standard and Chess960 — that the two routes render the same string, that the
// body never carries a suffix, and that the suffix read from a maintained
// make is the one `move_to_san` derives internally.
//
// SPDX-License-Identifier: MIT

use gigachess::fen::parse_fen;
use gigachess::moves::Move;
use gigachess::san::{check_mate_suffix, move_to_san, move_to_san_body};

/// Standard and Chess960 start positions, so the disambiguation and the
/// castling rendering are exercised under both sets of rights.
fn roots() -> [&'static str; 3] {
    [
        // The standard opening position.
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // Chess960 start position 518 (Shredder-FEN rights, so the castling
        // words come from rook files that are not a/h).
        "bqnb1rkr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQ1BNRKR w HFhf - 2 9",
        // Chess960 start position 284: king d1, rooks c1/f1.
        "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1",
    ]
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn the_split_matches_the_monolith_across_100k_positions() {
    let mut rng = 0x5AFE_C0DE_1234_ABCDu64;
    let mut positions = 0usize;
    let target_positions = 100_000usize;
    let mut with_suffix = 0usize;
    let fen = roots();

    while positions < target_positions {
        let mut board = parse_fen(fen[(positions / 7) % fen.len()]).expect("a start position");

        for _ply in 0..80 {
            let moves = board.legal_moves();
            if moves.is_empty() {
                break;
            }
            let mv: Move = moves[(xorshift(&mut rng) as usize) % moves.len()];

            let monolith = move_to_san(&board, mv);
            let body = move_to_san_body(&board, mv);

            // The after-position, reached the way a walk reaches it: a make that
            // maintains the cached checkers, which is what the suffix reads.
            let mut after = board;
            let undo = after.make_move_unchecked(mv);
            let suffix = check_mate_suffix(&after);
            if suffix.is_some() {
                with_suffix += 1;
            }

            match (monolith, body) {
                (Some(full), Some(body)) => {
                    // The body never carries a suffix on its own.
                    assert!(
                        !body.ends_with('+') && !body.ends_with('#'),
                        "body {body} carries a suffix at {positions}"
                    );
                    let mut joined = body;
                    if let Some(c) = suffix {
                        joined.push(c);
                    }
                    assert_eq!(
                        joined, full,
                        "split differs from the monolith at position {positions}"
                    );
                }
                (None, None) => {}
                (a, b) => panic!(
                    "body/suffix presence differs at position {positions}: {a:?} against {b:?}"
                ),
            }

            // The make is reversible, so the walk could keep going either way.
            after.unmake_move(mv, undo);
            assert_eq!(
                after, board,
                "make/unmake is not clean at position {positions}"
            );
            board.make_move_unchecked(mv);
            positions += 1;
            if positions >= target_positions {
                break;
            }
        }
    }

    assert!(positions >= target_positions, "only {positions} positions");
    // The suffix seam is not vacuous: a real share of random plies give check.
    assert!(
        with_suffix > positions / 100,
        "only {with_suffix} suffixed of {positions}"
    );
}
