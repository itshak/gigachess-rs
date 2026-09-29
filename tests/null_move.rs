//! The null move as a first-class move.
//!
//! ChessBase (.cbh) databases store a pass as the word `0xffff` in the `moves2`
//! stream, written `--` (or `Z0`) in PGN and `0000` in UCI. Before this change
//! that word reached the board and was treated as an ordinary move with squares:
//! it aborted on an incidental debug assert, and in release it moved whatever
//! decoded out of `0xffff` and destroyed castling rights. This suite pins the
//! contract that replaced that — the word means "pass", everywhere, with one
//! definition of what a pass does and one of when it is legal.
//!
//! Several tests here exist specifically to fail if a tempting shortcut returns:
//! deciding a pass from a stale `checkers` cache, and answering `is_legal` with
//! the generic make-and-test body (see
//! `null_legality_is_decided_for_the_mover_not_the_opponent`).

use gigachess::database;
use gigachess::san;
use gigachess::{Board, Color, IllegalMove, Move, Role, Square};

fn board(fen: &str) -> Board {
    gigachess::fen::parse_fen(fen).expect("fen")
}

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

/// A validating entry point: it applies `mv` if it is legal and reports whether
/// it was.
type Validating = fn(&mut Board, Move) -> Result<(), IllegalMove>;
/// A raw unchecked make: it applies `mv` unconditionally.
type Raw = fn(&mut Board, Move);

/// Decodes a little-endian `moves2` byte stream into words.
fn words(bytes: &[u8]) -> Vec<u16> {
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| u16::from_le_bytes([p[0], p[1]]))
        .collect()
}

const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// A position where the side to move is not in check, so a pass is legal.
const QUIET: &str = START;

/// A position where the side to move is in check, so a pass must be refused.
///
/// White is in check from the h4 queen and has a real move (`Nf3`/`g4`), so a
/// pass here is genuinely illegal rather than merely absurd.
const IN_CHECK: &str = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";

/// The FEN a pass from the start position produces: same pieces, Black to move,
/// halfmove 1, and **fullmove 2** — a pass is a full move, whoever passed.
const AFTER_WHITE_PASSES: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 1 2";

// ---------------------------------------------------------------------------
// The word itself
// ---------------------------------------------------------------------------

#[test]
fn the_null_word_is_ffff() {
    assert_eq!(Move::NULL.word(), 0xffff);
    assert!(Move::NULL.is_null());
    assert_eq!(Move::NULL, Move::from_word(0xffff));
    assert!(Move::from_word(0xffff).is_null());
    assert!(!Move::default().is_null());
}

/// The sentinel has to be unreachable by ordinary move construction, or a real
/// move would be silently swallowed. `0xffff` decomposes as from h1, to h1,
/// promotion nibble 15 — and `h1h1` is a different word entirely.
#[test]
fn no_ordinary_move_collides_with_the_null_word() {
    assert_ne!(mv("h1h1").word(), 0xffff);
    assert!(!mv("h1h1").is_null());
    for from in 0u8..64 {
        for to in 0u8..64 {
            for promo in 0u16..16 {
                let word = u16::from(from) | (u16::from(to) << 6) | (promo << 12);
                if Move::from_word(word).is_null() {
                    // Only the all-bits-set word is null, and it is the
                    // unreachable h1h1-with-nonsense-promotion encoding.
                    assert_eq!((from, to, promo), (63, 63, 15));
                }
            }
        }
    }
}

#[test]
fn the_null_word_renders_as_uci_0000() {
    assert_eq!(Move::NULL.to_string(), "0000");
    assert_ne!(Move::NULL.to_string(), mv("h1h1").to_string());
}

// ---------------------------------------------------------------------------
// The pass transition
// ---------------------------------------------------------------------------

#[test]
fn a_pass_flips_the_turn_and_advances_the_clocks() {
    let mut b = board(QUIET);
    b.play(Move::NULL).expect("legal when not in check");
    assert_eq!(b.turn(), Color::Black);
    assert_eq!(b.fullmove_number(), 2, "a pass is a full move");
    assert_eq!(b.halfmove_clock(), 1);
    assert_eq!(b.to_fen(), AFTER_WHITE_PASSES);
}

/// A pending double push lapses across a pass — the pass is a whole turn, so any
/// en-passant right it created is gone.
#[test]
fn a_pass_clears_a_pending_en_passant_square() {
    let mut b = board("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");
    assert!(b.en_passant().is_some(), "premise: an ep square is set");
    b.play(Move::NULL).expect("legal");
    assert_eq!(b.en_passant(), None);
}

#[test]
fn a_pass_leaves_castling_rights_untouched() {
    let mut b = board("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
    let before = b.castling_rights();
    b.play(Move::NULL).expect("legal");
    assert_eq!(b.castling_rights(), before);
}

#[test]
fn a_pass_is_refused_while_in_check() {
    let mut b = board(IN_CHECK);
    assert!(b.in_check(), "fixture must actually be in check");
    assert!(b.play(Move::NULL).is_err());
    // The refusal left the position exactly as it was.
    assert_eq!(b.to_fen(), board(IN_CHECK).to_fen());
}

// ---------------------------------------------------------------------------
// Dispatch: all eight entries
// ---------------------------------------------------------------------------

/// The four validating entries. These refuse an illegal pass, so they are the
/// ones the refusal tests can safely call on a check position.
const VALIDATING: &[(&str, Validating)] = &[
    ("play", |b, m| b.play(m).map(|_| ())),
    ("play_fast", |b, m| b.play_fast(m).map(|_| ())),
    ("play_hashed", |b, m| b.play_hashed(m).map(|_| ())),
    ("play_checkered", |b, m| b.play_checkered(m).map(|_| ())),
];

/// The four raw unchecked makes. These apply whatever they are given — a null
/// included, like any other unchecked move — and assert in debug that a pass is
/// legal, so they are only called on positions where one is.
const RAW: &[(&str, Raw)] = &[
    ("make_move_unchecked", |b, m| {
        b.make_move_unchecked(m);
    }),
    ("make_move_hashed", |b, m| {
        b.make_move_hashed(m);
    }),
    ("make_move_checkered", |b, m| {
        b.make_move_checkered(m);
    }),
    ("make_move_fast", |b, m| {
        b.make_move_fast(m);
    }),
];

#[test]
fn all_eight_entry_points_dispatch_a_null() {
    for (name, play) in VALIDATING {
        let mut b = board(QUIET);
        play(&mut b, Move::NULL).expect("legal pass");
        assert_eq!(b.to_fen(), AFTER_WHITE_PASSES, "{name} did not pass");
    }
    for (name, make) in RAW {
        let mut b = board(QUIET);
        make(&mut b, Move::NULL);
        assert_eq!(b.to_fen(), AFTER_WHITE_PASSES, "{name} did not pass");
    }
}

#[test]
fn all_four_validating_entry_points_refuse_a_pass_in_check() {
    let b = board(IN_CHECK);
    for (name, play) in VALIDATING {
        let mut probe = b;
        assert!(
            play(&mut probe, Move::NULL).is_err(),
            "{name} allowed a pass while in check"
        );
        assert_eq!(probe.to_fen(), b.to_fen(), "{name} corrupted on refusal");
    }
}

/// The hash-keeping entries must leave `zobrist()` equal to `zobrist_full()` —
/// that equality is the whole contract, and a pass must not break it.
#[test]
fn a_pass_keeps_the_incremental_hash_equal_to_a_full_recompute() {
    let hashed: Vec<(&str, Raw)> = vec![
        ("play", |b, m| {
            b.play(m).expect("legal");
        }),
        ("play_hashed", |b, m| {
            b.play_hashed(m).expect("legal");
        }),
        ("make_move_unchecked", |b, m| {
            b.make_move_unchecked(m);
        }),
        ("make_move_hashed", |b, m| {
            b.make_move_hashed(m);
        }),
    ];
    for (name, make) in hashed {
        let mut b = board(START);
        make(&mut b, Move::NULL);
        assert_eq!(
            b.zobrist(),
            b.zobrist_full(),
            "{name}: the incremental hash diverged from a full recompute"
        );
    }
}

/// The `checkers`-keeping entries must leave `in_check()` correct for the *new*
/// side to move, which is the whole reason they exist.
#[test]
fn a_pass_refreshes_checkers_for_the_new_side_to_move() {
    // Black to move, not in check. After a checkered pass, White is to move and
    // it is White who is in check — so the cache must be set, not merely left
    // alone by accident.
    let mut b = board("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR b KQkq - 1 3");
    assert!(!b.in_check(), "premise: Black is not in check");
    b.make_move_checkered(Move::NULL);
    assert_eq!(b.turn(), Color::White);
    assert!(
        b.in_check(),
        "after Black passes, White is in check and the cache must say so"
    );
}

// ---------------------------------------------------------------------------
// The regression that matters: a stale cache must never decide a pass
// ---------------------------------------------------------------------------

/// Reach the check position through a *cache-free* make, so `checkers` really is
/// stale in the way a real caller leaves it — not hand-caged.
fn stale_cache_in_check() -> Board {
    let mut b = board(START);
    b.play_fast(mv("f2f3")).expect("1. f3");
    b.play_fast(mv("e7e5")).expect("... e5");
    b.play_fast(mv("g2g4")).expect("2. g4");
    b.play_fast(mv("d8h4")).expect("... Qh4+, check");
    b
}

/// The single most important test in this file.
///
/// `play_fast` leaves `checkers` stale. A pass that decided legality by reading
/// that cache — via `in_check()` — sees "not in check" and allows a pass while
/// the side to move is in check. Every entry must refuse, and the premise is
/// asserted first so this test cannot quietly stop being adversarial.
#[test]
fn a_pass_after_a_cache_free_make_is_refused_while_in_check() {
    let mut b = stale_cache_in_check();

    // Premise, in the exact terms of the bug: the cache says "not in check" while
    // a fresh computation says "in check". The two genuinely disagree here.
    assert!(!b.in_check(), "premise: the stale cache reads as 'not in check'");
    assert!(
        b.attackers_to(b.king_square(Color::White).0, Color::Black, b.occupied()) != 0,
        "premise: a fresh computation says White IS in check"
    );

    for (name, play) in VALIDATING {
        let mut probe = b;
        assert!(
            play(&mut probe, Move::NULL).is_err(),
            "{name} allowed a pass while the side to move was in check — it read \
             the stale cache instead of recomputing"
        );
    }
    // And the answer agrees with the entry points that have always computed fresh.
    assert!(!b.is_legal(Move::NULL));
    assert!(b.make_null_move().is_err());
    assert!(b.make_null_move_fast().is_err());
}

/// The same *position* with a fresh cache is a position where a pass is still
/// illegal. Together with the test above this shows the refusal is about the
/// position, not a blanket ban — and that the cache state does not change the
/// answer.
#[test]
fn a_fresh_cache_does_not_change_the_verdict() {
    let mut b = board(START);
    b.play(mv("f2f3")).expect("1. f3");
    b.play(mv("e7e5")).expect("... e5");
    b.play(mv("g2g4")).expect("2. g4");
    b.play(mv("d8h4")).expect("... Qh4+");
    assert!(b.in_check(), "premise: the cache is fresh and reads in-check");
    for (name, play) in VALIDATING {
        let mut probe = b;
        assert!(play(&mut probe, Move::NULL).is_err(), "{name}");
    }
}

// ---------------------------------------------------------------------------
// is_legal / is_pseudo_legal
// ---------------------------------------------------------------------------

#[test]
fn a_null_is_pseudo_legal() {
    // The sentinel is structurally well-formed: no piece, no geometry, no
    // target. The in-check question belongs to legality, not to structure.
    assert!(board(QUIET).is_pseudo_legal(Move::NULL));
    assert!(board(IN_CHECK).is_pseudo_legal(Move::NULL));
}

#[test]
fn a_null_is_legal_exactly_when_not_in_check() {
    assert!(board(QUIET).is_legal(Move::NULL));
    assert!(!board(IN_CHECK).is_legal(Move::NULL));
}

/// The hazard this test exists to prevent.
///
/// `is_legal`'s generic body plays the move and asks whether the **mover's** king
/// survives. A pass moves no king, so that body cannot express the right
/// question — it ends up testing the **opponent's** king, which is safe, and so
/// answers `true` on exactly the positions where a pass must be refused.
///
/// On the stale-cache check position below the two bodies disagree. Asserting
/// the correct answer here means "simplifying" `is_legal` back to the generic
/// body fails the suite.
#[test]
fn null_legality_is_decided_for_the_mover_not_the_opponent() {
    let b = stale_cache_in_check();
    assert!(!b.in_check(), "premise: the cache is stale");
    assert!(
        !b.is_legal(Move::NULL),
        "is_legal(Move::NULL) must be false here; the generic body answers true \
         because it tests the opponent's king"
    );
}

#[test]
fn a_null_is_never_offered_as_a_legal_move() {
    for fen in [START, IN_CHECK] {
        let legal = board(fen).legal_moves();
        assert!(
            !legal.iter().any(|m| m.is_null()),
            "legal_moves() must not contain a pass ({fen})"
        );
    }
}

// ---------------------------------------------------------------------------
// make / unmake pairing
// ---------------------------------------------------------------------------

#[test]
fn a_pass_pairs_with_unmake_null_move() {
    let mut b = board(QUIET);
    let before = b.to_fen();
    let undo = b.play(Move::NULL).expect("legal");
    assert_eq!(b.to_fen(), AFTER_WHITE_PASSES);
    b.unmake_null_move(undo);
    assert_eq!(b.to_fen(), before, "unmake_null_move did not restore");
}

#[test]
fn passes_nest_and_restore_exactly() {
    // A search makes and unmakes a pass around every node, so nested passes have
    // to unwind exactly.
    let mut b = board(QUIET);
    let start = b.to_fen();
    let mut undos = Vec::new();
    for _ in 0..5 {
        undos.push(b.make_null_move().expect("legal"));
    }
    assert_eq!(b.fullmove_number(), 6, "five passes advanced five full moves");
    for undo in undos.into_iter().rev() {
        b.unmake_null_move(undo);
    }
    assert_eq!(b.to_fen(), start);
}

#[test]
fn a_pass_undo_restores_the_fullmove_number() {
    // `unmake_move` on a null word is undefined and asserts in debug, so the
    // pairing is a contract rather than an accident: the null `Undo` restores
    // the fullmove number the pass advanced.
    let mut b = board(QUIET);
    let undo = b.make_null_move().expect("legal");
    assert_eq!(b.fullmove_number(), 2);
    b.unmake_null_move(undo);
    assert_eq!(b.fullmove_number(), 1);
    assert_eq!(b.turn(), Color::White);
}

// ---------------------------------------------------------------------------
// SAN / UCI
// ---------------------------------------------------------------------------

#[test]
fn a_null_renders_as_two_dashes() {
    let b = board(QUIET);
    assert_eq!(
        san::move_to_san(&b, Move::NULL).expect("always renders").as_str(),
        "--"
    );
    assert_eq!(
        san::move_to_san_body(&b, Move::NULL).expect("body").as_str(),
        "--"
    );
}

#[test]
fn a_null_never_takes_a_check_or_mate_suffix() {
    // Rendered on a board whose caches are stale, to prove the suffix logic is
    // never reached: `--` returns before any make.
    let mut b = board(IN_CHECK);
    b.make_move_fast(mv("h2h3"));
    let out = san::move_to_san(&b, Move::NULL).expect("renders");
    assert_eq!(out.as_str(), "--");
    assert!(!out.as_str().ends_with('+'));
    assert!(!out.as_str().ends_with('#'));
}

/// A SAN render is a pure read. For a pass it must be pure even here: it makes no
/// move, so it must leave the (deliberately stale) hash exactly as stale, and
/// leave the turn and clocks alone.
#[test]
fn rendering_a_null_is_cache_neutral() {
    let mut b = board(QUIET);
    b.make_move_fast(mv("a2a3"));
    let stale_hash = b.zobrist();
    let full = b.zobrist_full();
    assert_ne!(stale_hash, full, "premise: the hash is stale");
    let fen_before = b.to_fen();

    let out = san::move_to_san(&b, Move::NULL).expect("renders");
    assert_eq!(out.as_str(), "--");
    assert_eq!(b.zobrist(), stale_hash, "rendering a pass touched the hash");
    assert_eq!(b.zobrist_full(), full, "rendering a pass moved something");
    assert_eq!(b.to_fen(), fen_before, "rendering a pass flipped the turn");
}

#[test]
fn san_parses_both_chessbase_spellings_of_a_pass() {
    let b = board(QUIET);
    assert_eq!(san::san_to_move(&b, "--"), Some(Move::NULL));
    assert_eq!(san::san_to_move(&b, "Z0"), Some(Move::NULL));
    // Whitespace and annotation suffixes are tolerated as for any other token.
    assert_eq!(san::san_to_move(&b, " -- "), Some(Move::NULL));
    assert_eq!(san::san_to_move(&b, "--!"), Some(Move::NULL));
}

#[test]
fn san_refuses_a_pass_while_in_check() {
    let b = board(IN_CHECK);
    assert_eq!(san::san_to_move(&b, "--"), None);
    assert_eq!(san::san_to_move(&b, "Z0"), None);
}

/// Only the two real spellings are accepted. Anything else is rejected rather
/// than guessed at, so an unrecognised token surfaces as an error instead of
/// quietly becoming a pass.
#[test]
fn san_rejects_invented_spellings_of_a_pass() {
    let b = board(QUIET);
    for bogus in ["null", "pass", "NIL", "0000", "skip", "-", "z0", "Z1"] {
        assert_eq!(
            san::san_to_move(&b, bogus),
            None,
            "{bogus} must not parse as a pass"
        );
    }
}

#[test]
fn san_round_trips_a_pass() {
    let b = board(QUIET);
    let rendered = san::move_to_san(&b, Move::NULL).expect("renders");
    assert_eq!(san::san_to_move(&b, rendered.as_str()), Some(Move::NULL));
}

// ---------------------------------------------------------------------------
// Codecs, end to end
// ---------------------------------------------------------------------------

/// The CBH-realistic path: a pass in the `moves2` stream survives a round trip
/// through PGN and back, byte for byte. A pass token means "whoever is to move
/// passes", so in `1. e4 -- 2. Nf3` it is Black who passes and White who then
/// plays Nf3.
#[test]
fn a_null_word_round_trips_through_movetext() {
    let movetext = "1. e4 -- 2. Nf3 1-0";
    let bytes = database::parse_movetext_to_moves2(START, movetext).expect("parse");
    let words = words(&bytes);
    assert_eq!(words.len(), 3, "expected e4, pass, Nf3");
    assert!(words.contains(&0xffff), "the pass was not encoded as 0xffff");
    assert_eq!(words[1], 0xffff, "the pass is the second ply");

    let rendered = database::moves2_to_san_movetext(START, &bytes, "1-0").expect("render");
    assert_eq!(rendered, movetext, "a pass did not survive a PGN round trip");
    let reparsed = database::parse_movetext_to_moves2(START, &rendered).expect("reparse");
    assert_eq!(bytes, reparsed, "the re-encoded bytes differ");
}

#[test]
fn a_pass_renders_as_dashes_and_is_not_dropped() {
    let rendered = database::moves2_to_san_movetext(START, &0xffffu16.to_le_bytes(), "").expect("render");
    assert_eq!(rendered, "1. --", "a pass must render as `--`");
}

/// The strongest single check of the whole contract: a game with a check, a
/// parry, a pass, and more play — replayed through the public API with the
/// incremental hash compared against a full recompute at every ply.
#[test]
fn a_pass_bearing_game_keeps_hash_parity_at_every_ply() {
    // 1. e4 e5 2. Qh5 Nc6 3. Qxe5+ Be7 4. d4 -- Nf3: a real check, a real block,
    // then Black passes (legal — Black answered the check and is no longer in it)
    // and play continues. The hash must not drift across any of it.
    //
    // Note the pass lands on ply 6, where it is Black's turn: a pass consumes a
    // turn, so the moves either side of it belong to opposite sides.
    let words = vec![
        mv("e2e4").word(),
        mv("e7e5").word(),
        mv("d1h5").word(),
        mv("b8c6").word(),
        mv("h5e5").word(), // Qxe5+, checking up the now-empty e-file
        mv("f8e7").word(), // Be7, the block
        mv("d2d4").word(),
        0xffff,           // Black passes
        mv("g1f3").word(),
    ];
    let mut b = board(START);
    for (i, &w) in words.iter().enumerate() {
        let m = Move::from_word(w);
        b.play(m).unwrap_or_else(|_| panic!("ply {i} was refused"));
        assert_eq!(b.zobrist(), b.zobrist_full(), "hash drift at ply {i}");
    }
    assert_eq!(b.turn(), Color::Black, "White played last, so Black is to move");
}

#[test]
fn replaying_a_game_with_a_pass_yields_hashes() {
    let words = vec![
        mv("e2e4").word(),
        0xffff,
        mv("g1f3").word(),
        mv("e7e5").word(),
    ];
    let hashes = database::replay_moves2_hashes(START, &words).expect("replay");
    assert_eq!(hashes.len(), 5, "one hash per ply plus the start position");
    let mut b = board(START);
    for (i, &w) in words.iter().enumerate() {
        b.play(Move::from_word(w)).expect("legal");
        assert_eq!(b.zobrist(), b.zobrist_full(), "hash drift at ply {i}");
        assert_eq!(hashes[i + 1].0, b.zobrist(), "replay hash wrong at ply {i}");
    }
}

#[test]
fn a_pass_produces_its_own_indexed_position() {
    // A pass is a position like any other: distinct hash, counted once, and
    // discoverable by an indexer that only ever saw moves2 words.
    let game = [mv("e2e4").word(), 0xffff, mv("g1f3").word()];
    let games: Vec<&[u16]> = vec![&game];
    let stats = database::position_stats(START, &games);

    let mut b = board(START);
    b.play(mv("e2e4")).expect("legal");
    let before_pass = b.zobrist();
    b.make_null_move().expect("legal");
    let after_pass = b.zobrist();
    assert_ne!(before_pass, after_pass, "a pass must change the position hash");

    // start, after e4, after the pass, after e5.
    assert_eq!(stats.len(), 4);
    assert_eq!(stats[&after_pass].count, 1);
}

/// The shape a CBH decoder actually produces: the raw `0xffff` word fed
/// straight into the render and re-parse paths, with no SAN in sight.
#[test]
fn a_pass_encoded_from_a_compact_word_round_trips() {
    let stream: Vec<u16> = vec![mv("e2e4").word(), 0xffff];
    let mut bytes = Vec::new();
    for w in &stream {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    let rendered = database::moves2_to_san_movetext(START, &bytes, "").expect("render");
    assert!(rendered.contains("--"), "got {rendered:?}");
    let back = database::parse_movetext_to_moves2(START, &rendered).expect("reparse");
    let back_words = words(&back);
    assert_eq!(back_words, stream, "the compact word did not round trip");
}
