// Perft throughput benchmark: startpos perft(5) with bulk counting,
// plus the visitor leaf (`perft_visitor`, D1 — CountingVisitor, no `Move`
// materialisation) head-to-head vs the MoveCounter bulk path.
//
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use turbochess_rs::Board;

fn bench_perft(c: &mut Criterion) {
    let board = Board::startpos();
    let nodes = board.perft(5); // 4,865,609 leaf nodes

    let mut group = c.benchmark_group("perft");
    group.throughput(Throughput::Elements(nodes));
    group.bench_function("startpos_d5", |b| b.iter(|| board.perft(black_box(5))));

    // Visitor leaf win gate (D1/2.2): `perft_visitor(1)` via CountingVisitor
    // must beat `perft(1)` via MoveCounter by >15% median, else revert.
    let leaf_nodes = board.perft(1) as u64; // 20
    group.throughput(Throughput::Elements(leaf_nodes));
    group.bench_function("startpos_d1_bulk_counter", |b| {
        b.iter(|| board.perft(black_box(1)))
    });
    group.bench_function("startpos_d1_visitor", |b| {
        b.iter(|| board.perft_visitor(black_box(1)))
    });

    // Full-depth visitor path (leaf-dominated at d5).
    group.throughput(Throughput::Elements(nodes));
    group.bench_function("startpos_d5_visitor", |b| {
        b.iter(|| board.perft_visitor(black_box(5)))
    });

    group.finish();
}

criterion_group!(benches, bench_perft);
criterion_main!(benches);
