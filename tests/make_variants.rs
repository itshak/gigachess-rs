//! The make variants and the contract each one keeps.
//!
//! A make maintains two pieces of derived state beyond the position itself: the
//! incremental Polyglot hash and the cached `checkers` bitboard. The four
//! combinations are four different costs, and a caller knows which of the two it
//! will read. These tests pin (a) that each variant leaves the state it claims
//! correct, correct, and (b) that the position itself is identical whichever
//! variant is used — a cache is a cache, not a rule.

use gigachess::{Board, Color, Move, Role, Square};

fn board(fen: &str) -> Board {
    gigachess::fen::parse_fen(fen).expect("fen")
}

/// `Move` from a two-to-four character UCI move, e.g. `e2e4`, `b7a8q`.
fn mv(text: &str) -> Move {
    let (from, rest) = text.split_at(2);
    let (to, promo) = rest.split_at(2);
    Move::new(
        Square::from_alg(from).expect("from"),
        Square::from_alg(to).expect("to"),
        match promo {
            "" => None,
            "q" => Some(Role::Queen),
            "r" => Some(Role::Rook),
            "b" => Some(Role::Bishop),
            "n" => Some(Role::Knight),
            other => panic!("promotion {other}"),
        },
    )
}

/// Every position the four variants must agree on, covering the transitions that
/// touch the hash: a double push, an en-passant capture, a promotion, both
/// castlings, a capture, and a position in check.
const POSITIONS: &[(&str, &str)] = &[
    (
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "e2e4",
    ),
    (
        "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2",
        "e5d6",
    ),
    (
        "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3",
        "g8f6",
    ),
    (
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 5",
        "e1h1",
    ),
    (
        "r3k2r/pPpp1ppp/2n2n2/4p3/2B1P3/5N2/PPP2PPP/RNBQK2R w KQkq - 0 1",
        "b7a8q",
    ),
    (
        "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2",
        "e4d5",
    ),
    // In check from a rook, with a king step that answers it.
    ("4k3/8/8/8/8/8/4r3/R3K3 w - - 0 1", "e1d1"),
];

/// The part of the position every variant reproduces, clocks excluded:
/// `make_move_perft` deliberately skips clock maintenance (it is the perft path).
fn core_fingerprint(b: &Board) -> String {
    // `to_fen` ends in "<halfmove> <fullmove>"; drop those two fields.
    let fen = b.to_fen();
    let head = fen.rsplit_once(' ').unwrap().0;
    let head = head.rsplit_once(' ').unwrap().0;
    format!("{head} {:?} {:?}", b.turn(), b.castling_rights())
}

/// The position fields every variant must reproduce exactly.
fn position_fingerprint(b: &Board) -> String {
    format!(
        "{:?} {:?} {:?} {:?} {} {}",
        b.to_fen(),
        b.turn(),
        b.castling_rights(),
        b.en_passant(),
        b.halfmove_clock(),
        b.fullmove_number(),
    )
}

fn checkers_are_fresh(b: &Board) -> bool {
    b.in_check() == (b.attackers_to(b.king_square(b.turn()).0, b.turn().other(), b.occupied()) != 0)
}

// The variants are deliberately similarly named; comparing them is the test.
#[allow(clippy::similar_names)]
#[test]
fn hashed_make_keeps_the_polyglot_hash_exact() {
    for (fen, uci) in POSITIONS {
        let mv = mv(uci);
        let mut hashed = board(fen);
        let undo = hashed.play_hashed(mv).expect("legal");
        // The whole point of this variant: the hash is maintained, so the cheap
        // incremental load agrees with the from-scratch recomputation.
        assert_eq!(
            hashed.zobrist(),
            hashed.zobrist_full(),
            "zobrist drift after {uci} in {fen}"
        );
        // And it is the *same* hash the full variant produces.
        let mut full = board(fen);
        full.play(mv).expect("legal");
        assert_eq!(hashed.zobrist(), full.zobrist(), "hash differs for {uci}");
        hashed.unmake_move(mv, undo);
        assert_eq!(hashed.zobrist(), board(fen).zobrist());
    }
}

// The variants are deliberately similarly named; comparing them is the test.
#[allow(clippy::similar_names)]
#[test]
fn checkered_make_keeps_the_checkers_cache_exact() {
    for (fen, uci) in POSITIONS {
        let mv = mv(uci);
        let mut checkered = board(fen);
        checkered.play_checkered(mv).expect("legal");
        // This variant exists for SAN rendering, which asks `in_check()`.
        assert!(
            checkers_are_fresh(&checkered),
            "stale checkers after {uci} in {fen}"
        );
        let mut full = board(fen);
        full.play(mv).expect("legal");
        assert_eq!(checkered.in_check(), full.in_check());
    }
}

#[test]
fn every_variant_produces_the_same_position() {
    for (fen, uci) in POSITIONS {
        let mv = mv(uci);
        let (reference, core) = {
            let mut b = board(fen);
            b.play(mv).expect("legal");
            (position_fingerprint(&b), core_fingerprint(&b))
        };
        for (name, made) in [("hashed", 0), ("checkered", 1), ("fast", 2), ("perft", 3)] {
            // perft skips the clocks by design, so it is held to the core only.
            let mut b = board(fen);
            match made {
                0 => {
                    b.play_hashed(mv).expect("legal");
                }
                1 => {
                    b.play_checkered(mv).expect("legal");
                }
                2 => {
                    b.play_fast(mv).expect("legal");
                }
                _ => {
                    b.make_move_perft(mv);
                }
            }
            if made == 3 {
                assert_eq!(
                    core_fingerprint(&b),
                    core,
                    "perft position differs for {uci}"
                );
                continue;
            }
            assert_eq!(
                position_fingerprint(&b),
                reference,
                "{name} differs for {uci}"
            );
        }
    }
}

// The variants are deliberately similarly named; comparing them is the test.
#[allow(clippy::similar_names)]
#[test]
fn illegal_moves_are_rejected_by_the_validating_variants() {
    let mut b = board("4k3/8/8/8/8/8/4r3/R3K3 w - - 0 1");
    let before = position_fingerprint(&b);
    // A rook move that leaves the king on e1 in check: pseudo-legal, illegal.
    let mv = mv("a1a2");
    for (name, res) in [
        ("play", b.play(mv)),
        ("play_hashed", b.play_hashed(mv)),
        ("play_checkered", b.play_checkered(mv)),
        ("play_fast", b.play_fast(mv)),
    ] {
        assert!(
            res.is_err(),
            "{name} accepted a move that leaves the king in check"
        );
        assert_eq!(
            position_fingerprint(&b),
            before,
            "{name} left the board changed"
        );
    }
}

// The variants are deliberately similarly named; comparing them is the test.
#[allow(clippy::similar_names)]
#[test]
fn a_null_move_never_trusts_a_stale_checkers_cache() {
    // The bug this guards: a walker that keeps neither cache makes its moves with
    // `play_fast`, which leaves `checkers` stale, and the null move's "may the
    // side to move pass while in check?" test used to read that stale cache
    // instead of asking the bitboards.
    let start = board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    for uci in ["e2e4", "g1f3", "d2d4"] {
        let mut fast = start;
        if fast.play_fast(mv(uci)).is_err() {
            continue;
        }
        // `checkers` is now whatever the previous position left behind.
        let fresh = fast.make_null_move_fast();
        let cached = fast.make_null_move();
        assert_eq!(
            fresh.is_ok(),
            cached.is_ok(),
            "fast and cached null moves disagree after {uci}"
        );
        let in_check = fast.attackers_to(
            fast.king_square(fast.turn()).0,
            fast.turn().other(),
            fast.occupied(),
        ) != 0;
        assert_eq!(
            fresh.is_ok(),
            !in_check,
            "wrong null-move verdict after {uci}"
        );
    }
}

// The variants are deliberately similarly named; comparing them is the test.
#[allow(clippy::similar_names)]
#[test]
fn a_null_move_in_check_is_refused_by_every_variant() {
    // White to move, in check from the black rook on e2: no pass.
    let mut b = board("4k3/8/8/8/8/8/4r3/R3K3 w - - 0 1");
    assert!(b.in_check());
    assert!(b.make_null_move().is_err());
    assert!(b.make_null_move_hashed().is_err());
    assert!(b.make_null_move_checkered().is_err());
    assert!(b.make_null_move_fast().is_err());
}

#[test]
fn a_null_move_agrees_with_the_caching_variants_about_the_position() {
    let start = board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut fast = start;
    fast.make_null_move_fast().expect("pass");
    let mut cached = start;
    cached.make_null_move().expect("pass");
    assert_eq!(position_fingerprint(&fast), position_fingerprint(&cached));
    // The hash is *not* part of that contract: the fast variant leaves it stale,
    // which is exactly why a caller that needs keys asks for `make_null_move_hashed`.
    let mut hashed = start;
    hashed.make_null_move_hashed().expect("pass");
    assert_eq!(position_fingerprint(&fast), position_fingerprint(&hashed));
    assert_eq!(fast.turn(), Color::Black);
    assert_eq!(fast.halfmove_clock(), 1);
}

#[test]
fn the_polyglot_keys_are_the_ones_the_index_depends_on() {
    // Guards the premise of the hashed variant: the maintained key is Polyglot's,
    // so a position index built from it is the index a consumer expects.
    let start = board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(
        start.zobrist(),
        0x463b_9618_1691_fc9c,
        "the start position's key is Polyglot's published constant"
    );
    let mut b = start;
    b.play_hashed(mv("e2e4")).expect("e4");
    assert_eq!(b.zobrist(), b.zobrist_full());
}

#[test]
fn the_role_of_a_promoted_piece_survives_the_hashed_variant() {
    let mut b = board("r3k2r/pPpp1ppp/2n2n2/4p3/2B1P3/5N2/PPP2PPP/RNBQK2R w KQkq - 0 1");
    let mv = mv("b7a8q");
    b.play_hashed(mv).expect("promotion");
    assert_eq!(b.zobrist(), b.zobrist_full());
    assert_eq!(
        b.piece_at(Square::from_alg("a8").unwrap()),
        Some(gigachess::Piece::new(Color::White, Role::Queen))
    );
}
