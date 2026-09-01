// Batch replay throughput benchmark: 8,000 generated games (~640k plies).
//
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use turbochess_rs::replay::replay_moves2_batch;
use turbochess_rs::Board;

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn generate_games(count: usize, max_plies: u32) -> Vec<Vec<u16>> {
    let mut state = 0xBE11_0000_0000_0001u64;
    let mut games = Vec::with_capacity(count);
    for _ in 0..count {
        let mut board = Board::startpos();
        let mut words = Vec::new();
        for _ in 0..max_plies {
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            let mv = legal[(xorshift(&mut state) % legal.len() as u64) as usize];
            words.push(mv.word());
            board.make_move_unchecked(mv);
        }
        games.push(words);
    }
    games
}

fn bench_replay(c: &mut Criterion) {
    let games = generate_games(8_000, 160);
    let total_plies: u64 = games.iter().map(|g| g.len() as u64).sum();
    let refs: Vec<&[u16]> = games.iter().map(|g| g.as_slice()).collect();

    let mut group = c.benchmark_group("replay");
    group.throughput(Throughput::Elements(games.len() as u64));
    group.bench_function("batch_8000_games", |b| {
        b.iter(|| replay_moves2_batch(black_box(&refs)))
    });
    group.throughput(Throughput::Elements(total_plies));
    group.bench_function("batch_8000_games_plies", |b| {
        b.iter(|| replay_moves2_batch(black_box(&refs)))
    });
    group.finish();
}

criterion_group!(benches, bench_replay);
criterion_main!(benches);
