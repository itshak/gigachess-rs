// Single-call micro harness matching `ultrachess/BENCH.md` shape.
//
// 8 rows × 3 FENs (startpos / Kiwipete / 960-284):
//   FEN write, FEN parse, movegen one-shot, make+unmake 48-ply,
//   isCheck in, isCheck out, hash (zobrist), SAN 48, clone.
// Each bench reports `ns/op` with `Throughput::Elements` so Criterion
// renders `Melem/s` / `ns per element` consistently. The shape mirrors
// `ultrachess/benches/micro.rs` (≈ criterion `BenchmarkGroup` per
// position, `Throughput::Elements` on every bench).
//
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use gigachess::{fen::parse_fen, san::move_to_san, Move, Undo};

// The three reference positions used across `vs_libraries` and this
// harness — the same triple ultrachess measures.
const POSITIONS: [(&str, &str); 3] = [
    ("startpos", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    ("kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"),
    ("960-284", "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1"),
];

// FEN where the side to move is in check: black king e8, white queen e2.
// Used for the `isCheck in` row (0.32ns target after `Undo.prev_checkers`
// cache, `ultrachess/BENCH.md` D3).
const CHECK_IN_FEN: &str = "4k3/8/8/8/8/8/4Q3/4K3 b - - 0 1";

// Simple xorshift64 for deterministic 48-ply line generation.
fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Deterministic 48-ply line from `fen`: repeatedly pick a random legal
/// move (xorshift) until `max` or terminal. Returns the move list.
fn gen_line(fen: &str, max: usize) -> Vec<Move> {
    let mut board = parse_fen(fen).unwrap();
    let mut out = Vec::with_capacity(max);
    let mut rng = 0x9E37_79B9_7F4A_7C15u64 ^ (fen.len() as u64).wrapping_mul(0x85EB_CA6B);
    for _ in 0..max {
        let legal = board.legal_moves();
        if legal.is_empty() {
            break;
        }
        let mv = legal[(xorshift(&mut rng) % legal.len() as u64) as usize];
        out.push(mv);
        let _ = board.play(mv);
        // Avoid rebuilding beyond legal — play already validates.
    }
    out
}

fn bench_micro(c: &mut Criterion) {
    // Pre-generate 48-ply lines per position so the bench measures only
    // the 48× make/unmake or SAN render, not move picking.
    let mut lines: Vec<(&str, Vec<Move>)> = Vec::new();
    for (name, fen) in POSITIONS {
        let line = gen_line(fen, 48);
        // Ensure we have at least 1 ply; if position quickly mates, extend
        // from startpos fallback so every `make+unmake 48-ply` bench has
        // meaningful work.
        let line = if line.is_empty() {
            gen_line(POSITIONS[0].1, 48)
        } else {
            line
        };
        lines.push((name, line));
    }

    for (name, fen) in POSITIONS {
        let board = parse_fen(fen).unwrap();
        let fen_string = fen.to_string();
        let line = lines
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, l)| l.clone())
            .unwrap_or_default();
        let line_len = line.len() as u64;

        let mut g = c.benchmark_group(format!("micro/{name}"));

        // 1) FEN write — branchless `ArrayVec<u8,128>` / `PIECE_CHAR` target 88ns.
        g.throughput(Throughput::Elements(1));
        g.bench_function("fen_write", |b| {
            b.iter(|| black_box(&board).to_fen())
        });

        // 2) FEN parse — flat ASCII lookup target ~144ns (ultrachess deliberate
        // loss vs shak 125ns; kept — Chess960 X-FEN ambiguity path).
        g.throughput(Throughput::Elements(1));
        g.bench_function("fen_parse", |b| {
            b.iter(|| parse_fen(black_box(&fen_string)).unwrap())
        });

        // 3) movegen one-shot — stack `ArrayVec<Move,256>` (cozy 19ns vs ours
        // 25ns deliberate; `MoveSink` bulk win is perft d1, not this row).
        g.throughput(Throughput::Elements(1));
        g.bench_function("movegen_one_shot", |b| {
            b.iter(|| black_box(&board).legal_moves())
        });

        // 4) make+unmake 48-ply — `Undo.prev_checkers` + slim perft path
        // kept per `BENCH.md: Deliberate 503ns vs cozy 353ns` (pays +2ns/make
        // for 8× isCheck). Batch so clone/setup not counted. Sequential 48
        // makes then 48 unmakes (mirrors ultrachess `make+unmake 48-ply` row).
        {
            let len = line_len.max(1);
            g.throughput(Throughput::Elements(len));
            g.bench_function("make_unmake_48", |b| {
                b.iter_batched(
                    || board,
                    |mut b| {
                        // Store undos to unwind in reverse (true 48-ply cycle).
                        let mut undos: arrayvec::ArrayVec<Undo, 48> =
                            arrayvec::ArrayVec::new();
                        for &mv in black_box(&line) {
                            undos.push(b.make_move_unchecked(mv));
                        }
                        for (mv, undo) in black_box(&line).iter().zip(undos).rev() {
                            b.unmake_move(*mv, undo);
                        }
                        black_box(b)
                    },
                    BatchSize::SmallInput,
                )
            });
        }

        // 5) isCheck in — branch-free `checkers!=0` target 0.32ns.
        {
            let check_in = parse_fen(CHECK_IN_FEN).unwrap();
            assert!(check_in.in_check());
            g.throughput(Throughput::Elements(1));
            g.bench_function("is_check/in", |b| {
                b.iter(|| black_box(&check_in).in_check())
            });
        }

        // 6) isCheck out — same 0.32ns both states.
        {
            g.throughput(Throughput::Elements(1));
            g.bench_function("is_check/out", |b| {
                b.iter(|| black_box(&board).in_check())
            });
        }

        // 7) hash — incremental load target 0.34ns (`Board.zobrist()` is a
        // single `u64` load after `Undo.prev_zobrist` cache).
        g.throughput(Throughput::Elements(1));
        g.bench_function("hash", |b| {
            b.iter(|| black_box(&board).zobrist())
        });

        // 8) SAN 48 — `tables::between` + make/unmake suffix + disambig
        // pre-filter `attacks_from_target`, target 1.43µs/48.
        {
            let len = line_len.max(1);
            g.throughput(Throughput::Elements(len));
            g.bench_function("san_48", |b| {
                b.iter(|| {
                    let mut cur = black_box(board);
                    for &mv in black_box(&line) {
                        let _san = move_to_san(&cur, mv).unwrap();
                        // Advance so disambiguation + check suffix stay realistic.
                        let _ = cur.make_move_unchecked(mv);
                    }
                    black_box(cur)
                })
            });
        }

        // 9) clone — plain-data Copy `Board` (~368B, `clone 3.3ns vs cozy
        // 1.7ns` deliberate; 960 rook squares + mailbox kept).
        g.throughput(Throughput::Elements(1));
        g.bench_function("clone", |b| {
            b.iter(|| *black_box(&board))
        });

        // 10) perft_visitor — D1 visitor leaf path (CountingVisitor, no
        // `Move` materialisation) at d3, vs the MoveCounter bulk `perft`
        // row in `perft_bench`. Measured 1.0× (parity — MoveCounter was
        // already popcount-only); kept as the additive visitor API hook.
        {
            let nodes = board.perft_visitor(3) as u64;
            g.throughput(Throughput::Elements(nodes));
            g.bench_function("perft_visitor", |b| {
                b.iter(|| black_box(&board).perft_visitor(3))
            });
        }

        // 11) san_visitor — SAN 48 render (same `move_to_san` path; this row
        // tracks the mate-check improvement once `count_legal_moves()==0`
        // replaces `legal_moves().is_empty()` in close-gap task 5.1, so the
        // `±%` vs this pre-fix baseline shows the D4 win directly).
        {
            let len = line_len.max(1);
            g.throughput(Throughput::Elements(len));
            g.bench_function("san_visitor", |b| {
                b.iter(|| {
                    let mut cur = black_box(board);
                    for &mv in black_box(&line) {
                        let _san = move_to_san(&cur, mv).unwrap();
                        let _ = cur.make_move_unchecked(mv);
                    }
                    black_box(cur)
                })
            });
        }

        g.finish();
    }
}

criterion_group!(benches, bench_micro);
criterion_main!(benches);
