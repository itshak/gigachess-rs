//! Replay real null-bearing ChessBase games through the null-move contract.
//!
//! Reads the dump written by cbvault's `dump_null_games` example — the mainline
//! `moves2` words of every game in a ChessBase database that contains a pass —
//! and holds each one to the contract `Move::NULL` promises:
//!
//!   - every word replays through `play`, which is the public entry point a
//!     generic `moves2` consumer uses, so a pass needs no special case;
//!   - the incremental hash equals a full recompute at every ply, including
//!     across a pass;
//!   - the pass is legal exactly where the database says it is, i.e. a pass
//!     recorded in a real game is never one this engine would refuse;
//!   - the SAN spelling is `--` and the movetext round-trips byte-identically.
//!
//! Usage: `cargo run --release --example corpus_null -- <dump.bin>`

use std::io::Read;

use gigachess::database;
use gigachess::san;
use gigachess::Move;

struct Record {
    fen: String,
    words: Vec<u16>,
}

fn read_dump(path: &str) -> std::io::Result<Vec<Record>> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?.read_to_end(&mut bytes)?;
    let mut at = 0usize;
    let u32_at = |b: &[u8], i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]) as usize;
    let mut out = Vec::new();
    while at + 4 <= bytes.len() {
        let fen_len = u32_at(&bytes, at);
        at += 4;
        let fen = String::from_utf8_lossy(&bytes[at..at + fen_len]).into_owned();
        at += fen_len;
        let n = u32_at(&bytes, at);
        at += 4;
        let mut words = Vec::with_capacity(n);
        for _ in 0..n {
            words.push(u16::from_le_bytes([bytes[at], bytes[at + 1]]));
            at += 2;
        }
        out.push(Record { fen, words });
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("usage: corpus_null <dump.bin>");
    let games = read_dump(&path)?;
    assert!(!games.is_empty(), "the dump is empty");

    let mut plies = 0u64;
    let mut nulls = 0u64;
    let mut hashes_checked = 0u64;
    let mut san_checked = 0u64;
    let mut chess960 = 0u64;
    let mut setups = 0u64;

    for (gi, rec) in games.iter().enumerate() {
        let mut board = gigachess::fen::parse_fen(&rec.fen)
            .unwrap_or_else(|e| panic!("game {gi}: bad start FEN {:?}: {e}", rec.fen));
        if board.is_chess960() {
            chess960 += 1;
        }
        if rec.fen != "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" {
            setups += 1;
        }

        for (ply, &word) in rec.words.iter().enumerate() {
            let mv = Move::from_word(word);
            // The recorded pass must be one this engine considers legal. If a
            // real database contained a pass from a position in check, the
            // contract would be refusing data that exists.
            if mv.is_null() {
                assert!(
                    board.is_legal(Move::NULL),
                    "game {gi} ply {ply}: a real recorded pass was refused as illegal"
                );
                nulls += 1;
            }
            board
                .play(mv)
                .unwrap_or_else(|_| panic!("game {gi} ply {ply}: word {word:#06x} was refused"));
            assert_eq!(
                board.zobrist(),
                board.zobrist_full(),
                "game {gi} ply {ply}: the incremental hash drifted from a full recompute"
            );
            hashes_checked += 1;
            plies += 1;
        }

        // SAN: every pass must render as `--` with no suffix, and the whole
        // movetext must survive a render/reparse round trip byte for byte.
        let mut bytes = Vec::with_capacity(rec.words.len() * 2);
        for &w in &rec.words {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
        let rendered = database::moves2_to_san_movetext(&rec.fen, &bytes, "")
            .unwrap_or_else(|e| panic!("game {gi}: movetext render failed: {e:?}"));
        let reparsed = database::parse_movetext_to_moves2(&rec.fen, &rendered)
            .unwrap_or_else(|e| panic!("game {gi}: movetext reparse failed: {e:?}"));
        assert_eq!(
            bytes, reparsed,
            "game {gi}: a real null-bearing game did not survive a PGN round trip"
        );
        san_checked += 1;

        // `--` must be what a pass renders as, from the position it is played in.
        let mut walk = gigachess::fen::parse_fen(&rec.fen).expect("fen");
        for (ply, &w) in rec.words.iter().enumerate() {
            let mv = Move::from_word(w);
            if mv.is_null() {
                let s = san::move_to_san(&walk, mv).expect("a pass always renders");
                assert_eq!(s.as_str(), "--", "game {gi} ply {ply}");
                assert!(!s.as_str().ends_with('+') && !s.as_str().ends_with('#'));
                assert_eq!(san::san_to_move(&walk, s.as_str()), Some(Move::NULL));
            }
            walk.play(mv).expect("legal");
        }
    }

    println!(
        "games={} plies={} nulls={} hash_parity_checks={} movetext_round_trips={} \
         chess960_games={} setup_games={}",
        games.len(),
        plies,
        nulls,
        hashes_checked,
        san_checked,
        chess960,
        setups
    );
    Ok(())
}
