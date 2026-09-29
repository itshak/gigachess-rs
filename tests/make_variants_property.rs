// Property test: each make variant keeps exactly what it promises, at scale.
//
// `tests/make_variants.rs` pins the contracts on hand-picked positions - a
// double push, an en passant, a promotion, castling, a check evasion. This one
// asks the same questions of every position a random game passes through, over
// 200,000 sampled moves from the standard opening and two Chess960 starts,
// because the hazards are the ones a fixture cannot reach:
//
//   - the incremental hash drifting from the from-scratch recomputation on a
//     transition the fixture list forgot (a Chess960 castle moves the rook file
//     the castling keys are built from, a different key path from anything the
//     standard fixtures touch);
//   - the checkers cache being wrong at some ply, which is the one that decides
//     `+` and `#` in a SAN export;
//   - a variant silently differing from `play` in the position it leaves;
//   - make/unmake not restoring the state, which is what a search does
//     thousands of times per node.
//
// The null move is exercised the same way, because its legality test is the one
// that used to read a cache its caller might not maintain.
//
// SPDX-License-Identifier: MIT

use gigachess::fen::parse_fen;
use gigachess::{Board, Move};

/// The roots: the standard opening and two Chess960 starts, chosen because their
/// castling words do not come from a1/e1/h1.
const ROOTS: [&str; 3] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "bqnb1rkr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQ1BNRKR w HFhf - 2 9",
    "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1",
];

/// The whole position a make is responsible for. `make_move_perft` is excluded
/// on purpose: it documents that it does not advance the clocks.
fn fingerprint(b: &Board) -> String {
    format!(
        "{} {:?} {:?} {:?} {} {}",
        b.to_fen(),
        b.turn(),
        b.castling_rights(),
        b.en_passant(),
        b.halfmove_clock(),
        b.fullmove_number()
    )
}

/// The truth about check, asked of the bitboards rather than of any cache.
fn in_check_fresh(b: &Board) -> bool {
    b.attackers_to(b.king_square(b.turn()).0, b.turn().other(), b.occupied()) != 0
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn is_castle(board: &Board, m: Move) -> bool {
    board
        .piece_at(m.from())
        .is_some_and(|p| p.role == gigachess::Role::King)
        && board
            .piece_at(m.to())
            .is_some_and(|p| p.role == gigachess::Role::Rook)
}

fn is_en_passant(board: &Board, m: Move) -> bool {
    let pawn = board
        .piece_at(m.from())
        .is_some_and(|p| p.role == gigachess::Role::Pawn);
    pawn && m.from().file() != m.to().file()
}

#[test]
fn every_variant_keeps_its_promise_across_200k_moves() {
    let mut rng = 0xA11C_E5AF_E0DE_1234u64;
    let mut checked = 0usize;
    let mut castles = 0usize;
    let mut promotions = 0usize;
    let mut en_passants = 0usize;
    let target = 200_000usize;

    while checked < target {
        let mut board = parse_fen(ROOTS[checked % ROOTS.len()]).expect("a start position");

        for _ply in 0..80 {
            let moves = board.legal_moves();
            if moves.is_empty() {
                break;
            }
            for &m in moves.as_slice() {
                if !xorshift(&mut rng).is_multiple_of(3) {
                    continue; // sample a third, so a run costs seconds not minutes
                }
                checked += 1;
                if m.promotion().is_some() {
                    promotions += 1;
                }
                if is_castle(&board, m) {
                    castles += 1;
                }
                if is_en_passant(&board, m) {
                    en_passants += 1;
                }

                let start_fingerprint = fingerprint(&board);
                let start_key = board.zobrist();
                let start_check = in_check_fresh(&board);

                // The reference: the variant that maintains both.
                let mut reference = board;
                let undo = reference.play(m).expect("legal");
                let reference_fingerprint = fingerprint(&reference);
                let reference_key = reference.zobrist();
                assert_eq!(
                    reference_key,
                    reference.zobrist_full(),
                    "play drifted at {m:?}"
                );
                reference.unmake_move(m, undo);
                assert_eq!(fingerprint(&reference), start_fingerprint, "play/unmake");

                // Hash yes, checkers no.
                let mut hashed = board;
                let undo = hashed.play_hashed(m).expect("legal");
                assert_eq!(
                    hashed.zobrist(),
                    hashed.zobrist_full(),
                    "play_hashed left a drifted key after {m:?} in {}",
                    board.to_fen()
                );
                assert_eq!(
                    hashed.zobrist(),
                    reference_key,
                    "play_hashed key != play key"
                );
                assert_eq!(
                    fingerprint(&hashed),
                    reference_fingerprint,
                    "play_hashed position != play position after {m:?}"
                );
                hashed.unmake_move(m, undo);
                assert_eq!(
                    hashed.zobrist(),
                    start_key,
                    "play_hashed unmake lost the key"
                );

                // Checkers yes, hash no.
                let mut checkered = board;
                let undo = checkered.play_checkered(m).expect("legal");
                assert_eq!(
                    checkered.in_check(),
                    in_check_fresh(&checkered),
                    "play_checkered left a stale checkers cache after {m:?} in {}",
                    board.to_fen()
                );
                assert_eq!(
                    fingerprint(&checkered),
                    reference_fingerprint,
                    "play_checkered position != play position after {m:?}"
                );
                checkered.unmake_move(m, undo);
                assert_eq!(
                    checkered.in_check(),
                    start_check,
                    "play_checkered unmake left a stale checkers cache after {m:?}"
                );

                // Neither. Its unmake is the paired `unmake_move_fast`: the two
                // form a set, and handing a fast make's Undo to `unmake_move`
                // silently corrupts the key rather than failing.
                let mut fast = board;
                let undo = fast.play_fast(m).expect("legal");
                assert_eq!(
                    fingerprint(&fast),
                    reference_fingerprint,
                    "play_fast position != play position after {m:?}"
                );
                fast.unmake_move_fast(m, undo);
                // The fast make never touched the key, so it is still the start's.
                assert_eq!(fast.zobrist(), start_key, "play_fast disturbed the key");
            }

            // A null move in each mode, on the way through: the legality test is
            // the one that used to read a cache its caller might not maintain.
            let fresh = in_check_fresh(&board);
            let mut verdicts = Vec::new();
            for (name, made) in [
                ("make_null_move", 0u8),
                ("make_null_move_hashed", 1),
                ("make_null_move_checkered", 2),
                ("make_null_move_fast", 3),
            ] {
                let mut b = board;
                let res = match made {
                    0 => b.make_null_move(),
                    1 => b.make_null_move_hashed(),
                    2 => b.make_null_move_checkered(),
                    _ => b.make_null_move_fast(),
                };
                assert_eq!(
                    res.is_ok(),
                    !fresh,
                    "{name} disagrees with a fresh attackers_to in {}",
                    board.to_fen()
                );
                verdicts.push(res.is_ok());
                if let Ok(undo) = res {
                    let after = fingerprint(&b);
                    b.unmake_null_move(undo);
                    assert_eq!(
                        fingerprint(&b),
                        fingerprint(&board),
                        "{name}/unmake_null_move lost the position"
                    );
                    // A pass keeps the placement; only the side and the clocks move.
                    assert_eq!(
                        after.split(' ').next(),
                        board.to_fen().split(' ').next(),
                        "{name} moved a piece"
                    );
                }
            }
            assert!(
                verdicts.windows(2).all(|w| w[0] == w[1]),
                "the null-move variants disagree: {verdicts:?}"
            );

            // Advance the game on a random legal move.
            let moves = board.legal_moves();
            if moves.is_empty() {
                break;
            }
            let pick = (xorshift(&mut rng) as usize) % moves.as_slice().len();
            if board.play(moves.as_slice()[pick]).is_err() {
                break;
            }
        }
    }

    assert!(checked >= target, "only {checked} moves checked");
    // The sampled transitions must actually include the ones the hash cares
    // about, or this test proves less than it appears to.
    assert!(castles > 0, "no castling move was sampled");
    assert!(promotions > 0, "no promotion was sampled");
    assert!(en_passants > 0, "no en-passant capture was sampled");
    eprintln!(
        "checked {checked} moves: {castles} castles, {promotions} promotions, {en_passants} en passant"
    );
}
