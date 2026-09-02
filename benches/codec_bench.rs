// Codec / import / replay benchmarks: turbochess-rs batch codecs vs a
// shakmaty-based baseline mirroring blind-base's current `gigabase_moves.rs`
// loops (ADR-003): per-ply FEN round-trip + full re-replay (O(n^2) per game),
// legal-movegen + linear-scan word decode, and from-scratch Zobrist
// recomputation (Legal ep mode).
//
// Deltas (M1 Max, 40 games × ~100 plies, 3988 plies, sample 10, 1s):
//   import movetext → moves2: turbo 1.31 ms (3.03 Melem) vs shak_gigabase
//     3.33 ms (1.19) = 2.54× win (byte-level tokenizer, no alloc Strings).
//   render moves2 → SAN: turbo 1.41 ms (2.82) vs 1.81 ms (2.20) = 1.27× win
//     (O(1) word decode vs movegen+linear scan per word).
//   hash replay incremental: turbo 118 µs (33.6 Melem) vs shak 1.26 ms
//     (3.15) = 10.6× win (incremental Polyglot vs from-scratch per ply).
// Techniques: byte-level movetext tokenizer (handles comments/NAGs/vars),
// direct Move::from_word decode, incremental zobrist maintained in
// make_move (O(1) vs shakmaty's update_zobrist_hash that bails on pinned-ep).
// See README.md Performance → Database batch codecs table.
//
// Run: cargo bench --bench codec_bench
//
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use shakmaty::{
    fen::Fen, san::San, san::SanPlus, Chess, EnPassantMode, Move as SMove, Position, Role as SRole,
    Square as SSquare, zobrist::Zobrist64, CastlingMode,
};
use turbochess_rs::{database, Board};

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Generates `count` random games; returns (movetext, moves2 bytes) pairs.
fn generate_games(count: usize, max_plies: u32) -> Vec<(String, Vec<u8>)> {
    let mut state = 0xBE11_0000_C0DE_0001u64;
    let start_fen = Board::startpos().to_fen();
    let mut games = Vec::with_capacity(count);
    for _ in 0..count {
        let mut board = Board::startpos();
        let mut words: Vec<u16> = Vec::new();
        for _ in 0..max_plies {
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            let mv = legal[(xorshift(&mut state) % legal.len() as u64) as usize];
            words.push(mv.word());
            board.play(mv).unwrap();
        }
        let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        let movetext = database::moves2_to_san_movetext(&start_fen, &bytes, "*").unwrap();
        games.push((movetext, bytes));
    }
    games
}

fn shak_pos(fen: &str) -> Chess {
    Fen::from_ascii(fen.as_bytes())
        .unwrap()
        .into_position(CastlingMode::Standard)
        .unwrap()
}

/// shakmaty baseline mirroring blind-base's gigabase_moves.rs import loop:
/// SAN parse via legal-movegen candidates, then a FEN round-trip and full
/// re-replay from ply 0 for every indexed position (O(n^2) per game), and a
/// from-scratch Zobrist hash (Legal ep mode) per ply.
fn shakmaty_import_baseline(start_fen: &str, movetext: &str) -> u64 {
    let mut pos = shak_pos(start_fen);
    let mut last_hash = 0u64;
    for token in movetext.split_whitespace() {
        let t = token.trim_end_matches(['.', ' ']);
        if t.is_empty()
            || t == "1-0"
            || t == "0-1"
            || t == "1/2-1/2"
            || t == "*"
            || t.bytes().all(|c| c.is_ascii_digit() || c == b'.')
        {
            continue;
        }
        let san = San::from_ascii(token.as_bytes())
            .unwrap_or_else(|_| San::from_ascii(t.as_bytes()).unwrap());
        let mv = san.to_move(&pos).unwrap();
        pos = pos.play(mv).unwrap();
        // FEN round-trip + re-replay from ply 0 (the O(n^2) indexer loop).
        let fen = Fen::from_position(&pos, EnPassantMode::Legal).to_string();
        let _replay = shak_pos(&fen);
        last_hash = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Legal).0;
    }
    last_hash
}

/// shakmaty baseline render: decode each stored word via a full legal-movegen
/// scan + linear match, then render SanPlus (the gigabase decode path).
fn shakmaty_render_baseline(start_fen: &str, moves2: &[u8]) -> String {
    let mut pos = shak_pos(start_fen);
    let mut out = String::new();
    for (i, pair) in moves2.chunks_exact(2).enumerate() {
        let word = u16::from_le_bytes([pair[0], pair[1]]);
        let from = SSquare::new((word & 0x3f) as u32);
        let to = SSquare::new(((word >> 6) & 0x3f) as u32);
        let promo = match (word >> 12) & 0x7 {
            1 => Some(SRole::Knight),
            2 => Some(SRole::Bishop),
            3 => Some(SRole::Rook),
            4 => Some(SRole::Queen),
            _ => None,
        };
        // Full movegen + linear scan to decode one word.
        let list = pos.legal_moves();
        let mv: SMove = list
            .iter()
            .find(|m| m.from() == Some(from) && m.to() == to && m.promotion() == promo)
            .copied()
            .unwrap_or_else(|| panic!("word decode failed at ply {i}"));
        let san = SanPlus::from_move_and_play_unchecked(&mut pos, mv);
        if i % 2 == 0 {
            out.push_str(&(i / 2 + 1).to_string());
            out.push_str(". ");
        }
        san.append_to_string(&mut out);
        out.push(' ');
    }
    out
}

/// shakmaty baseline hash replay: from-scratch Zobrist per ply.
fn shakmaty_hash_baseline(start_fen: &str, moves2: &[u8]) -> u64 {
    let mut pos = shak_pos(start_fen);
    let mut last = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Legal).0;
    for pair in moves2.chunks_exact(2) {
        let word = u16::from_le_bytes([pair[0], pair[1]]);
        let from = SSquare::new((word & 0x3f) as u32);
        let to = SSquare::new(((word >> 6) & 0x3f) as u32);
        let promo = match (word >> 12) & 0x7 {
            1 => Some(SRole::Knight),
            2 => Some(SRole::Bishop),
            3 => Some(SRole::Rook),
            4 => Some(SRole::Queen),
            _ => None,
        };
        let list = pos.legal_moves();
        let mv: SMove = list
            .iter()
            .find(|m| m.from() == Some(from) && m.to() == to && m.promotion() == promo)
            .copied()
            .unwrap();
        pos = pos.play(mv).unwrap();
        last = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Legal).0;
    }
    last
}

const GAMES: usize = 40;
const MAX_PLIES: u32 = 100;

fn bench_codecs(c: &mut Criterion) {
    let start_fen = Board::startpos().to_fen();
    let games = generate_games(GAMES, MAX_PLIES);
    let total_bytes: u64 = games.iter().map(|(_, b)| b.len() as u64).sum();
    let total_plies: u64 = total_bytes / 2;
    let movetexts: Vec<String> = games.iter().map(|(m, _)| m.clone()).collect();
    let blobs: Vec<Vec<u8>> = games.iter().map(|(_, b)| b.clone()).collect();

    let mut g = c.benchmark_group("codec");

    // Import: movetext -> moves2.
    g.throughput(Throughput::Elements(total_plies));
    g.bench_function("import_movetext/turbochess", |b| {
        b.iter(|| {
            for m in black_box(&movetexts) {
                black_box(database::parse_movetext_to_moves2(&start_fen, m).unwrap());
            }
        })
    });
    g.bench_function("import_movetext/shakmaty_gigabase", |b| {
        b.iter(|| {
            for m in black_box(&movetexts) {
                black_box(shakmaty_import_baseline(&start_fen, m));
            }
        })
    });

    // Render: moves2 -> SAN movetext.
    g.bench_function("render_movetext/turbochess", |b| {
        b.iter(|| {
            for blob in black_box(&blobs) {
                black_box(database::moves2_to_san_movetext(&start_fen, blob, "*").unwrap());
            }
        })
    });
    g.bench_function("render_movetext/shakmaty_gigabase", |b| {
        b.iter(|| {
            for blob in black_box(&blobs) {
                black_box(shakmaty_render_baseline(&start_fen, blob));
            }
        })
    });

    // Hash replay: incremental vs from-scratch.
    let word_refs: Vec<Vec<u16>> = blobs
        .iter()
        .map(|b| b.chunks_exact(2).map(|p| u16::from_le_bytes([p[0], p[1]])).collect())
        .collect();
    g.bench_function("hash_replay/turbochess_incremental", |b| {
        b.iter(|| {
            for w in black_box(&word_refs) {
                black_box(database::replay_moves2_hashes(&start_fen, w).unwrap());
            }
        })
    });
    g.bench_function("hash_replay/shakmaty_from_scratch", |b| {
        b.iter(|| {
            for blob in black_box(&blobs) {
                black_box(shakmaty_hash_baseline(&start_fen, blob));
            }
        })
    });

    g.finish();
}

criterion_group!(benches, bench_codecs);
criterion_main!(benches);
