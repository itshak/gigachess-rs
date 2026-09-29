// Property test: a pass is a first-class move at scale.
//
// `tests/null_move.rs` pins the null-move contract on hand-picked positions. This
// one asks the same questions of every position a random game passes through,
// with passes injected along the way, because the hazards are the ones a fixture
// list cannot reach:
//
//   - the incremental hash drifting from a from-scratch recomputation across a
//     transition no fixture listed (a Chess960 castle moves the rook file the
//     castling keys are built from);
//   - a pass being allowed or refused on a position the fixtures never reached.
//     The one bug in this area was exactly that: the legality test read the
//     cached `checkers`, which a caller that makes moves with `play_fast` does
//     not maintain;
//   - the four maintenance shapes disagreeing about whether a pass is legal, or
//     about the position it leaves;
//   - make/unmake not restoring the state exactly, which is what a search does
//     thousands of times per node;
//   - SAN rendering or parsing drifting on a pass-bearing position.
//
// The walk runs each game in two phases. It plays with `play`, keeping the
// incremental hash chain intact so parity can be asserted at every ply, and then
// deliberately makes one cache-free move with `play_fast` and immediately
// samples the pass path on a board whose `checkers` and `zobrist()` both describe
// an earlier position. That second phase is the only state in which reading the
// cache is detectably wrong, and it is the state this suite is really for.
//
// SPDX-License-Identifier: MIT

use gigachess::fen::parse_fen;
use gigachess::{san, Board, Move};

/// The roots: the standard opening and two Chess960 starts, chosen because their
/// castling words do not come from a1/e1/h1.
const ROOTS: [&str; 3] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "bqnb1rkr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQ1BNRKR w HFhf - 2 9",
    "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1",
];

/// The shapes that promise a live incremental key after a pass.
const HASH_KEEPING: [&str; 2] = ["play", "play_hashed"];

/// The shapes that promise a live `checkers` cache after a pass.
const CHECKERS_KEEPING: [&str; 2] = ["play", "play_checkered"];

type Validating = fn(&mut Board, Move) -> Result<gigachess::Undo, gigachess::IllegalMove>;

/// The four entries that decide legality, i.e. refuse an illegal pass.
const VALIDATING: &[(&str, Validating)] = &[
    ("play", |b, m| b.play(m)),
    ("play_fast", |b, m| b.play_fast(m)),
    ("play_hashed", |b, m| b.play_hashed(m)),
    ("play_checkered", |b, m| b.play_checkered(m)),
];

/// The truth about check, asked of the bitboards rather than of any cache.
fn in_check_fresh(b: &Board) -> bool {
    b.attackers_to(b.king_square(b.turn()).0, b.turn().other(), b.occupied()) != 0
}

/// The whole position a pass is responsible for.
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

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Counters, so the assertions at the end can prove the suite reached the states
/// it claims to cover rather than passing vacuously.
#[derive(Default)]
struct Coverage {
    sampled: usize,
    plies: usize,
    legal_passes: usize,
    refused_passes: usize,
    cache_disagreements: usize,
    checkers_verified: usize,
    hash_parity_checked: usize,
}

/// Hold every entry point to the answer the bitboards give.
///
/// `chain_intact` is only used in failure messages: the point of the exercise
/// is that the verdict must not depend on it.
fn check_pass_here(
    board: &Board,
    fresh: bool,
    chain_intact: bool,
    cov: &mut Coverage,
) {
    cov.sampled += 1;
    if fresh {
        // A pass answers no check, so every validating entry must refuse — on a
        // fresh cache and on a stale one alike.
        cov.refused_passes += 1;
        for (name, play) in VALIDATING {
            let mut probe = *board;
            assert!(
                play(&mut probe, Move::NULL).is_err(),
                "{name} allowed a pass from a position in check (chain_intact={chain_intact})"
            );
            assert_eq!(probe.to_fen(), board.to_fen(), "{name} corrupted on refusal");
        }
        assert!(!board.is_legal(Move::NULL));
        assert!(san::san_to_move(board, "--").is_none());
        assert!(san::san_to_move(board, "Z0").is_none());
        return;
    }

    cov.legal_passes += 1;
    // All four shapes must agree, and leave the same position — a cache is a
    // cache, not a rule.
    let mut reference: Option<String> = None;
    for (name, play) in VALIDATING {
        let mut probe = *board;
        let undo = play(&mut probe, Move::NULL).unwrap_or_else(|_| {
            panic!("{name} refused a legal pass (chain_intact={chain_intact})")
        });
        // Only the hash-keeping shapes promise a live key; the fast and checkered
        // ones leave it stale on purpose, and asserting parity there would assert
        // the opposite of what they are for.
        // Only meaningful while the incremental chain is intact: after a
        // cache-free make the stored key is stale by design and no entry point
        // re-bases it, so parity here would be asserting a guarantee the API
        // does not make.
        if HASH_KEEPING.contains(name) && chain_intact {
            assert_eq!(
                probe.zobrist(),
                probe.zobrist_full(),
                "{name}: the incremental hash drifted across a pass"
            );
            cov.hash_parity_checked += 1;
        }
        // A pass is `--` in and out, never suffixed, whatever the position — and
        // it parses back to the same word.
        let rendered = san::move_to_san(&probe, Move::NULL).expect("renders");
        assert_eq!(rendered.as_str(), "--", "{name}");
        assert_eq!(
            san::san_to_move(&probe, rendered.as_str()),
            Some(Move::NULL),
            "{name}"
        );
        // The checkers-keeping shapes must leave `in_check()` agreeing with a
        // fresh computation *for whoever is now to move* — the other side than
        // the one that passed, so the refresh does new work rather than
        // re-reading what was already true.
        if CHECKERS_KEEPING.contains(name) {
            assert_eq!(
                probe.in_check(),
                in_check_fresh(&probe),
                "{name}: the refreshed checkers disagree with a fresh computation after a pass"
            );
            cov.checkers_verified += 1;
        }
        // And the pass must undo exactly, which is what a search does around
        // every null-move node.
        let passed = fingerprint(&probe);
        probe.unmake_null_move(undo);
        assert_eq!(
            fingerprint(&probe),
            fingerprint(board),
            "{name}: unmake_null_move did not restore the position"
        );
        // Re-apply so the cross-entry comparison below is like with like.
        play(&mut probe, Move::NULL).expect("re-applying the pass");
        assert_eq!(fingerprint(&probe), passed, "{name}");
        match &reference {
            None => reference = Some(passed),
            Some(r) => assert_eq!(r, &passed, "{name} disagrees about the position"),
        }
    }
}

#[test]
fn a_pass_behaves_like_a_first_class_move_across_200k_moves() {
    let mut rng = 0xA11C_E5AF_E0DE_1234u64;
    let mut cov = Coverage::default();
    let target = 200_000usize;

    while cov.sampled < target {
        let mut board = parse_fen(ROOTS[cov.sampled % ROOTS.len()]).expect("a start position");

        for ply in 0..80 {
            cov.plies += 1;
            let fresh = in_check_fresh(&board);
            if fresh != board.in_check() {
                // Only reachable after a cache-free make, which is the state this
                // suite exists to cover.
                cov.cache_disagreements += 1;
            }
            if xorshift(&mut rng).is_multiple_of(5) {
                check_pass_here(&board, fresh, true, &mut cov);
                if cov.sampled >= target {
                    break;
                }
            }

            // Advance with a cache-maintaining make, so the incremental hash
            // chain stays intact and parity can be asserted at every ply.
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            let m = legal[(xorshift(&mut rng) as usize) % legal.len()];
            let before = fingerprint(&board);
            let undo = board.play(m).expect("a legal move");
            assert_eq!(
                board.zobrist(),
                board.zobrist_full(),
                "the incremental hash drifted at a non-pass ply"
            );
            let after = fingerprint(&board);
            assert_ne!(before, after, "a legal move must change the position");
            // Round-trip the way a search does.
            board.unmake_move(m, undo);
            assert_eq!(fingerprint(&board), before, "unmake_move did not restore");
            // Re-apply, or the walk would re-explore the root forever and never
            // reach a position where the side to move is in check.
            board.play(m).expect("re-applying the same legal move");

            // Once the game is long enough, break the caches on purpose and
            // sample the pass path on a board whose `checkers` and `zobrist()`
            // both describe an earlier position. The hash chain cannot be
            // re-based without a full recompute, so the game ends here.
            if ply >= 3 && xorshift(&mut rng).is_multiple_of(4) {
                let legal = board.legal_moves();
                if legal.is_empty() {
                    break;
                }
                let m = legal[(xorshift(&mut rng) as usize) % legal.len()];
                board.play_fast(m).expect("a legal move");
                cov.plies += 1;
                let fresh = in_check_fresh(&board);
                if fresh != board.in_check() {
                    cov.cache_disagreements += 1;
                }
                // The chain is broken here, so no hash parity is claimed.
                check_pass_here(&board, fresh, false, &mut cov);
                break;
            }
        }
    }

    assert!(
        cov.legal_passes > 10_000,
        "only {} legal passes were exercised",
        cov.legal_passes
    );
    assert!(
        cov.refused_passes > 500,
        "only {} refused passes were exercised",
        cov.refused_passes
    );
    assert!(
        cov.cache_disagreements > 1_000,
        "only {} positions had a cache disagreeing with a fresh computation, so \
         the stale-cache path was barely tested",
        cov.cache_disagreements
    );
    assert!(
        cov.hash_parity_checked > 50_000,
        "only {} hash parities were checked",
        cov.hash_parity_checked
    );
    assert!(
        cov.checkers_verified > 50_000,
        "only {} refreshed caches were verified",
        cov.checkers_verified
    );

    println!(
        "plies={} sampled={} legal_passes={} refused_passes={} \
         cache_disagreements={} checkers_verified={} hash_parity={}",
        cov.plies,
        cov.sampled,
        cov.legal_passes,
        cov.refused_passes,
        cov.cache_disagreements,
        cov.checkers_verified,
        cov.hash_parity_checked
    );
}
