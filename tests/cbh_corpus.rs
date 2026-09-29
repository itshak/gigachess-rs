//! The null-move contract, checked against real ChessBase data.
//!
//! `tests/null_move.rs` pins the contract on hand-written fixtures and
//! `tests/null_move_property.rs` exercises it on synthetic playouts. Both are
//! things this repository invented, so both could agree with each other and
//! both be wrong about what a real database actually contains.
//!
//! This suite closes that gap. It replays the mainline `moves2` words of the
//! games in a ChessBase database that contain a pass, as extracted by cbvault's
//! `dump_null_games` example reading the `.cbg`/`.cbh` files directly.
//!
//! **The data is not in this repository.** Game records from a licensed database
//! are copyrighted, so an extraction of them has no place in a public repo — a
//! 28 KB sample was briefly committed here and has been removed from the history.
//! Point `GIGACHESS_CBH_NULL_GAMES` at a dump you generated yourself:
//!
//! ```text
//! cbvault: cargo run --release -p cbvault-cli --example dump_null_games -- \
//!   'Mega Database 2025/Mega Database 2025' /tmp/nullgames.bin
//! export GIGACHESS_CBH_NULL_GAMES=/tmp/nullgames.bin
//! cargo test --test cbh_corpus -- --include-ignored
//! ```
//!
//! Without that variable every test here reports itself skipped, so the suite is
//! inert by default and never silently passes for lack of data. The last run
//! against *Mega Database 2025* (11,149,379 games, 883,141,297 plies) covered all
//! 1,002 null-bearing games: 75,501 plies, 1,208 passes, hash parity at every ply,
//! every movetext round-tripping byte-identically, and every recorded pass
//! confirmed legal by this engine.
//!
//! What it asserts, per game:
//!
//!   - the whole line replays through `play`, the entry point a generic `moves2`
//!     consumer uses, so a pass needs no special case anywhere in the replay;
//!   - the incremental hash equals a from-scratch recompute at every ply,
//!     including immediately after a pass;
//!   - every pass the database records is one this engine considers **legal** —
//!     the corpus is a real-world cross-check on the refusal rule, not just on
//!     the acceptance path;
//!   - a pass renders as `--` and the movetext round-trips byte-identically.
//!
//! ## What this suite deliberately does not cover
//!
//! The replay uses `play` throughout, so the `checkers` cache is always fresh
//! here. **Deciding a pass from a stale cache is therefore structurally
//! unreachable in this file**, and reverting the fresh pass test to the cached
//! one leaves all four tests green — verified. That path belongs to
//! `tests/null_move.rs` (one hand-built stale-cache position) and
//! `tests/null_move_property.rs` (which breaks the caches with `play_fast` and
//! reaches 1,364 positions where the cache and a fresh computation disagree).
//! Both of those were checked to fail when the fix is reverted.
//!
//! What this suite *does* catch, verified by reverting each and watching it fail:
//! a pass that does not flip the turn (3 of 4 tests fail), and a pass that omits
//! the turn-key hash XOR (hash parity fails).
//!
//! SPDX-License-Identifier: MIT

use gigachess::database;
use gigachess::san;
use gigachess::Move;

/// Where the caller put the dump. Absent means "not run", never "passed".
const DATA_ENV: &str = "GIGACHESS_CBH_NULL_GAMES";

struct Game {
    fen: String,
    words: Vec<u16>,
}

/// The dump, as written by cbvault's `dump_null_games` example: a
/// length-prefixed binary stream, one record per game —
/// `u32` FEN length, those FEN bytes, `u32` word count, then that many `u16`
/// `moves2` words.
fn load() -> Vec<Game> {
    let path = std::env::var(DATA_ENV)
        .unwrap_or_else(|_| panic!("{DATA_ENV} is not set; see the module docs for how to build it"));
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("{DATA_ENV}={path} could not be read: {e}"));
    let u32_at = |i: usize| {
        u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]) as usize
    };
    let mut games = Vec::new();
    let mut at = 0usize;
    while at + 4 <= bytes.len() {
        let fen_len = u32_at(at);
        at += 4;
        assert!(at + fen_len + 4 <= bytes.len(), "{DATA_ENV}={path} is truncated");
        let fen = String::from_utf8_lossy(&bytes[at..at + fen_len]).into_owned();
        at += fen_len;
        let n = u32_at(at);
        at += 4;
        assert!(at + 2 * n <= bytes.len(), "{DATA_ENV}={path} is truncated");
        let words = (0..n)
            .map(|i| u16::from_le_bytes([bytes[at + 2 * i], bytes[at + 2 * i + 1]]))
            .collect();
        at += 2 * n;
        games.push(Game { fen, words });
    }
    assert!(!games.is_empty(), "{DATA_ENV}={path} contained no games");
    games
}

/// The premise, asserted before anything is concluded from it: the fixture really
/// is real pass-bearing data, and it is varied. Without this the suite could pass
/// on a truncated or hand-edited file.
#[test]
#[ignore = "needs a dump from a licensed database; set GIGACHESS_CBH_NULL_GAMES and pass --include-ignored"]
fn the_dump_is_real_pass_bearing_varied_data() {
    let games = load();
    assert!(games.len() >= 500, "only {} games in the dump", games.len());
    let plies: usize = games.iter().map(|g| g.words.len()).sum();
    let nulls: usize = games
        .iter()
        .flat_map(|g| &g.words)
        .filter(|w| Move::from_word(**w).is_null())
        .count();
    assert!(plies > 40_000, "only {plies} plies");
    assert!(nulls > 500, "only {nulls} null moves");
    // Every game must contain at least one pass, or it is not null-bearing data.
    for g in &games {
        assert!(
            g.words.iter().any(|w| Move::from_word(*w).is_null()),
            "dump game from {} has no pass at all",
            g.fen
        );
    }
    // A spread of lengths, so this is not one line copied around.
    let shortest = games.iter().map(|g| g.words.len()).min().unwrap();
    let longest = games.iter().map(|g| g.words.len()).max().unwrap();
    assert!(
        longest > 4 * shortest.max(1),
        "dump is not varied: {shortest}..{longest}"
    );
    // Non-standard starts are present too, so the pass is not only exercised from
    // the opening position.
    let standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    assert!(
        games.iter().any(|g| g.fen != standard),
        "no setup positions in the dump"
    );
}

#[test]
#[ignore = "needs a dump from a licensed database; set GIGACHESS_CBH_NULL_GAMES and pass --include-ignored"]
fn every_real_null_bearing_game_replays_through_play() {
    let games = load();
    let mut plies = 0usize;
    let mut nulls = 0usize;
    for (gi, g) in games.iter().enumerate() {
        let mut board = gigachess::fen::parse_fen(&g.fen)
            .unwrap_or_else(|e| panic!("game {gi}: bad FEN {:?}: {e}", g.fen));
        for (ply, &word) in g.words.iter().enumerate() {
            let mv = Move::from_word(word);
            if mv.is_null() {
                // The corpus is a real-world check on the refusal rule: a pass
                // ChessBase recorded must be a pass this engine accepts. If the
                // legality test were wrong in the strict direction, real data
                // would fail here.
                assert!(
                    board.is_legal(Move::NULL),
                    "game {gi} ply {ply}: a pass recorded in the real database was \
                     refused as illegal"
                );
                nulls += 1;
            }
            board.play(mv).unwrap_or_else(|_| {
                panic!("game {gi} ply {ply}: word {word:#06x} was refused by play")
            });
            assert_eq!(
                board.zobrist(),
                board.zobrist_full(),
                "game {gi} ply {ply}: the incremental hash drifted from a full recompute"
            );
            plies += 1;
        }
    }
    assert!(plies > 40_000 && nulls > 500, "{plies} plies, {nulls} nulls");
}

#[test]
#[ignore = "needs a dump from a licensed database; set GIGACHESS_CBH_NULL_GAMES and pass --include-ignored"]
fn every_real_pass_renders_as_dashes_and_round_trips() {
    for (gi, g) in load().iter().enumerate() {
        let mut bytes = Vec::with_capacity(g.words.len() * 2);
        for &w in &g.words {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
        let rendered = database::moves2_to_san_movetext(&g.fen, &bytes, "")
            .unwrap_or_else(|e| panic!("game {gi}: render failed: {e:?}"));
        let reparsed = database::parse_movetext_to_moves2(&g.fen, &rendered)
            .unwrap_or_else(|e| panic!("game {gi}: reparse of {rendered:?} failed: {e:?}"));
        assert_eq!(
            bytes, reparsed,
            "game {gi}: a real null-bearing game did not survive a PGN round trip"
        );
        // The pass itself spells `--`, parses back, and never takes a suffix.
        let mut board = gigachess::fen::parse_fen(&g.fen).expect("fen");
        let mut passes = 0usize;
        for (ply, &w) in g.words.iter().enumerate() {
            let mv = Move::from_word(w);
            if mv.is_null() {
                let s = san::move_to_san(&board, mv).expect("a pass always renders");
                assert_eq!(s.as_str(), "--", "game {gi} ply {ply}");
                assert!(!s.ends_with('+'), "game {gi} ply {ply}: {s}");
                assert!(!s.ends_with('#'), "game {gi} ply {ply}: {s}");
                assert_eq!(san::san_to_move(&board, s.as_str()), Some(Move::NULL));
                passes += 1;
            }
            board.play(mv).expect("legal");
        }
        assert!(passes > 0, "game {gi} had no pass to check");
    }
}

/// The corpus is a real-world check on the *acceptance* side too: a pass that
/// changes the position must leave a position the engine agrees with, and
/// unmaking it must restore the position exactly.
#[test]
#[ignore = "needs a dump from a licensed database; set GIGACHESS_CBH_NULL_GAMES and pass --include-ignored"]
fn a_real_pass_unmakes_exactly() {
    let mut checked = 0usize;
    for g in load() {
        let mut board = gigachess::fen::parse_fen(&g.fen).expect("fen");
        for &w in &g.words {
            let mv = Move::from_word(w);
            if mv.is_null() {
                let before = board.to_fen();
                let undo = board.play(Move::NULL).expect("a recorded pass is legal");
                assert_ne!(board.to_fen(), before, "a pass changed nothing at all");
                board.unmake_null_move(undo);
                assert_eq!(board.to_fen(), before, "unmake_null_move did not restore");
                checked += 1;
            }
            board.play(mv).expect("legal");
        }
    }
    assert!(checked > 500, "only {checked} real passes were round-tripped");
}
