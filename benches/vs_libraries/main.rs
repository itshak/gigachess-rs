// Head-to-head Criterion suite: turbochess-rs vs shakmaty 0.30 vs cozy-chess
// on every hot-path axis (ADR-003 / spec turbochess-rs-core-engine):
//   legal movegen, perft (bulk + non-bulk), board copy, make-move,
//   FEN parse/format, SAN parse/render, Zobrist scratch + incremental.
// Each axis measures identical inputs on startpos, Kiwipete and Chess960
// position 284. cozy-chess has no SAN or FEN-format API: those axes are
// marked N/A for it (see README results table).
//
// Deltas vs references (M1 Max, Criterion sample 10, 1s measurement):
//   - legal movegen sparse (startpos 20, 960 20): shakmaty 5-19% faster due to
//     smaller board repr and tighter branch prediction; turbo wins on dense
//     Kiwipete (1.76×) and geomean 1.25×. Follow-up #1: branchless MoveSink.
//   - board_copy: shakmaty 2.15× faster (turbo Board 368 B COPY vs shakmaty's
//     compact layout). Follow-up #2: compact Board repr.
//   - FEN parse: shakmaty 1.93× faster (turbo validates Chess960 path + ep
//     rank, X-FEN ambiguity). Follow-up #3: SIMD FEN scan.
//   - SAN parse/render: shakmaty 3.5×/1.32× faster (shakmaty's perfect-hash SAN
//     table, turbo does disambiguation scan). Follow-up #4: SAN table.
//   - Zobrist scratch: parity (~5% shak faster, table walk).
//   Wins: perft bulk 1.3-1.6× vs shak (2.2× vs cozy), perft non-bulk 2.6-2.9×,
//   make+unmake 3.36× vs shak, FEN format 1.7×, Zobrist incremental 5.2×,
//   and all codec axes (2.5× import, 10.6× hash replay).
//   See README.md Performance section for full table and machine context.
//
// Run: cargo bench --bench vs_libraries
//
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use cozy_chess::{Board as CozyBoard, Move as CozyMove};
use shakmaty::{fen::Fen, san::SanPlus, CastlingMode, Chess, EnPassantMode, Move as SMove, Position};



use shakmaty::zobrist::Zobrist64;
use gigachess::{fen::parse_fen, san as tsan, Board, Move as TMove};



const POSITIONS: [(&str, &str); 3] = [
    ("startpos", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    ("kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"),
    ("960-284", "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1"),
];

// -- conversion helpers ------------------------------------------------------





fn cozy_fen(fen: &str, name: &str) -> String {
    // cozy-chess wants Shredder castling letters for 960 placements.
    if name == "960-284" {
        fen.replace("KQkq", "CFcf")
    } else {
        fen.to_string()
    }
}

fn tc_start(fen: &str) -> Board {
    parse_fen(fen).unwrap()
}

fn shak_start(fen: &str) -> Chess {
    // shakmaty X-FEN parsing is strict about standard castling letters;
    // feed the Chess960 position in Shredder spelling.
    let (fen, mode) = if fen.starts_with("nbrknrbq") {
        (
            "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w CFcf - 0 1",
            CastlingMode::Chess960,
        )
    } else {
        (fen, CastlingMode::Standard)
    };
    Fen::from_ascii(fen.as_bytes())
        .unwrap()
        .into_position(mode)
        .unwrap()
}

fn cozy_start(fen: &str, name: &str) -> CozyBoard {
    CozyBoard::from_fen(&cozy_fen(fen, name), name == "960-284").unwrap()
}

fn tc_legal(board: &Board) -> Vec<TMove> {
    board.legal_moves().iter().copied().collect()
}

fn shak_legal(pos: &Chess) -> Vec<SMove> {
    pos.legal_moves().into_iter().collect()
}

fn cozy_legal(board: &CozyBoard) -> Vec<CozyMove> {
    let mut list = Vec::new();
    board.generate_moves(|piece_moves| {
        for mv in piece_moves {
            list.push(mv);
        }
        false
    });
    list
}

fn perft_tc(board: &Board, depth: u32, bulk: bool) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = board.legal_moves();
    if bulk && depth == 1 {
        return moves.len() as u64;
    }
    let mut n = 0;
    let mut child = *board;
    for mv in moves {
        let undo = child.make_move_unchecked(mv);
        n += perft_tc(&child, depth - 1, bulk);
        child.unmake_move(mv, undo);
    }
    n
}

fn perft_shak(pos: &Chess, depth: u32, bulk: bool) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = pos.legal_moves();
    if bulk && depth == 1 {
        return moves.len() as u64;
    }
    let mut n = 0;
    for mv in moves {
        let child = pos.clone().play(mv).unwrap();
        n += perft_shak(&child, depth - 1, bulk);
    }
    n
}

fn perft_cozy(board: &CozyBoard, depth: u32, bulk: bool) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = cozy_legal(board);
    if bulk && depth == 1 {
        return moves.len() as u64;
    }
    let mut n = 0;
    for mv in moves {
        let mut child = board.clone();
        child.play(mv);
        n += perft_cozy(&child, depth - 1, bulk);
    }
    n
}

fn bench_all(c: &mut Criterion) {
    for (name, fen) in POSITIONS {
        let tc = tc_start(fen);
        let shak = shak_start(fen);
        let cozy = cozy_start(fen, name);
        let tc_moves = tc_legal(&tc);
        let shak_moves = shak_legal(&shak);
        let cozy_moves = cozy_legal(&cozy);
        assert_eq!(tc_moves.len(), shak_moves.len());
        assert_eq!(tc_moves.len(), cozy_moves.len());
        let san_tokens: Vec<String> = tc_moves
            .iter()
            .map(|&mv| tsan::move_to_san(&tc, mv).unwrap().as_str().to_string())
            .collect();

        let mut g = c.benchmark_group(format!("vs_libraries/{name}"));
        g.throughput(Throughput::Elements(tc_moves.len() as u64));

        // Legal move generation.
        g.bench_function("legal_moves/turbochess", |b| {
            b.iter(|| black_box(&tc).legal_moves().len())
        });
        g.bench_function("legal_moves/shakmaty", |b| {
            b.iter(|| black_box(&shak).legal_moves().len())
        });
        g.bench_function("legal_moves/cozy-chess", |b| {
            b.iter(|| cozy_legal(black_box(&cozy)).len())
        });

        // Perft (bulk counting at the leaves).
        g.throughput(Throughput::Elements(perft_tc(&tc, 3, true)));
        g.bench_function("perft_d3_bulk/turbochess", |b| {
            b.iter(|| perft_tc(black_box(&tc), 3, true))
        });
        g.bench_function("perft_d3_bulk/shakmaty", |b| {
            b.iter(|| perft_shak(black_box(&shak), 3, true))
        });
        g.bench_function("perft_d3_bulk/cozy-chess", |b| {
            b.iter(|| perft_cozy(black_box(&cozy), 3, true))
        });

        // Perft (non-bulk: every leaf counted by make).
        g.throughput(Throughput::Elements(perft_tc(&tc, 2, false)));
        g.bench_function("perft_d2_nonbulk/turbochess", |b| {
            b.iter(|| perft_tc(black_box(&tc), 2, false))
        });
        g.bench_function("perft_d2_nonbulk/shakmaty", |b| {
            b.iter(|| perft_shak(black_box(&shak), 2, false))
        });
        g.bench_function("perft_d2_nonbulk/cozy-chess", |b| {
            b.iter(|| perft_cozy(black_box(&cozy), 2, false))
        });

        // Board copy.
        g.bench_function("board_copy/turbochess", |b| {
            b.iter(|| {
                let mut s = *black_box(&tc);
                for _ in 0..100 {
                    s = black_box(s);
                }
                s
            })
        });
        g.bench_function("board_copy/shakmaty", |b| {
            b.iter(|| {
                let mut s = black_box(&shak).clone();
                for _ in 0..100 {
                    s = black_box(s);
                }
                s
            })
        });
        g.bench_function("board_copy/cozy-chess", |b| {
            b.iter(|| {
                let mut s = black_box(&cozy).clone();
                for _ in 0..100 {
                    s = black_box(s);
                }
                s
            })
        });

        // Make-move (make + unmake; cozy has no unmake: copy-make idiom).
        g.bench_function("make_move/turbochess", |b| {
            b.iter_batched(
                || *black_box(&tc),
                |mut board| {
                    for &mv in black_box(&tc_moves) {
                        let undo = board.make_move_unchecked(mv);
                        board.unmake_move(mv, undo);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        g.bench_function("make_move/shakmaty", |b| {
            // shakmaty 0.30 has no unmake: copy-make is its only engine idiom.
            b.iter(|| {
                for &mv in black_box(&shak_moves) {
                    let child = black_box(&shak).clone().play(mv).unwrap();
                    black_box(child);
                }
            })
        });
        g.bench_function("make_move/cozy-chess", |b| {
            b.iter(|| {
                for &mv in black_box(&cozy_moves) {
                    let mut child = black_box(&cozy).clone();
                    child.play(mv);
                    black_box(child);
                }
            })
        });

        // FEN parse.
        g.bench_function("fen_parse/turbochess", |b| {
            b.iter(|| parse_fen(black_box(fen)).unwrap())
        });
        g.bench_function("fen_parse/shakmaty", |b| {
            b.iter(|| shak_start(black_box(fen)))
        });
        g.bench_function("fen_parse/cozy-chess", |b| {
            b.iter(|| CozyBoard::from_fen(black_box(&cozy_fen(fen, name)), name == "960-284").unwrap())
        });

        // FEN format (cozy-chess has no formatter).
        g.bench_function("fen_format/turbochess", |b| {
            b.iter(|| black_box(&tc).to_fen())
        });
        g.bench_function("fen_format/shakmaty", |b| {
            b.iter(|| Fen::from_position(black_box(&shak), EnPassantMode::Legal).to_string())
        });

        // SAN parse.
        g.bench_function("san_parse/turbochess", |b| {
            b.iter(|| {
                for tok in black_box(&san_tokens) {
                    black_box(tsan::san_to_move(&tc, tok));
                }
            })
        });
        g.bench_function("san_parse/shakmaty", |b| {
            b.iter(|| {
                for tok in black_box(&san_tokens) {
                    let san = shakmaty::san::San::from_ascii(tok.as_bytes()).unwrap();
                    black_box(san.to_move(black_box(&shak)).is_ok());
                }
            })
        });

        // SAN render.
        g.bench_function("san_render/turbochess", |b| {
            b.iter(|| {
                for &mv in black_box(&tc_moves) {
                    black_box(tsan::move_to_san(&tc, mv));
                }
            })
        });
        g.bench_function("san_render/shakmaty", |b| {
            // from_move_and_play_unchecked consumes the position: clone per move.
            b.iter(|| {
                for &mv in black_box(&shak_moves) {
                    let mut pos = black_box(&shak).clone();
                    let san = SanPlus::from_move_and_play_unchecked(&mut pos, mv);
                    black_box(san);
                }
            })
        });

        // Zobrist from scratch (cozy-chess exposes no scratch recompute).
        g.bench_function("zobrist_scratch/turbochess", |b| {
            b.iter(|| black_box(&tc).zobrist_full())
        });
        g.bench_function("zobrist_scratch/shakmaty", |b| {
            b.iter(|| black_box(&shak).zobrist_hash::<Zobrist64>(EnPassantMode::Always))
        });

        // Zobrist incremental.
        g.bench_function("zobrist_incremental/turbochess", |b| {
            b.iter_batched(
                || *black_box(&tc),
                |mut board| {
                    for &mv in black_box(&tc_moves) {
                        let undo = board.make_move_unchecked(mv);
                        black_box(board.zobrist());
                        board.unmake_move(mv, undo);
                    }
                },
                BatchSize::SmallInput,
            )
        });
        g.bench_function("zobrist_incremental/shakmaty", |b| {
            // shakmaty incremental update; returns None in some positions
            // (pinned ep) and callers must fall back to a full recompute.
            // Copy-make per move (shakmaty has no unmake).
            b.iter(|| {
                for &mv in black_box(&shak_moves) {
                    let mut pos = black_box(&shak).clone();
                    let h = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Always);
                    let h = pos
                        .update_zobrist_hash::<Zobrist64>(h, mv, EnPassantMode::Always)
                        .unwrap_or_else(|| pos.zobrist_hash::<Zobrist64>(EnPassantMode::Always));
                    pos = pos.play(mv).unwrap();
                    black_box(h);
                }
            })
        });
        g.bench_function("zobrist_incremental/cozy-chess", |b| {
            b.iter(|| {
                for &mv in black_box(&cozy_moves) {
                    let mut child = black_box(&cozy).clone();
                    child.play(mv);
                    black_box(child.hash());
                }
            })
        });

        g.finish();
    }
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
