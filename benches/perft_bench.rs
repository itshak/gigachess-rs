// Perft throughput benchmark: startpos perft(5) with bulk counting.
//
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use turbochess_rs::Board;

fn bench_perft(c: &mut Criterion) {
    let board = Board::startpos();
    let nodes = board.perft(5); // 4,865,609 leaf nodes

    let mut group = c.benchmark_group("perft");
    group.throughput(Throughput::Elements(nodes));
    group.bench_function("startpos_d5", |b| {
        b.iter(|| board.perft(black_box(5)))
    });
    group.finish();
}

criterion_group!(benches, bench_perft);
criterion_main!(benches);
