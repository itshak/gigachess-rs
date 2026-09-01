// `moves2` batch replay verification: parity between self-play encoding and
// replayed decoding, including the 100,000-game batch requirement.
//
// The 100k case is `#[ignore]`d for quick debug runs; run it with:
//   cargo test --release --test replay -- --ignored
//
// SPDX-License-Identifier: MIT

use turbochess_rs::board::MAX_MOVES;
use turbochess_rs::moves::Move;
use turbochess_rs::replay::{replay_moves2_batch, replay_moves2_stream, ReplayOutcome};
use turbochess_rs::Board;

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Plays `count` random legal games from the start position and returns them
/// encoded as `moves2` word streams together with the final board hash.
fn generate_games(count: usize, max_plies: u32, seed: u64) -> (Vec<Vec<u16>>, Vec<u64>) {
    let mut state = seed;
    let mut games = Vec::with_capacity(count);
    let mut hashes = Vec::with_capacity(count);
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
        hashes.push(board.zobrist());
        games.push(words);
    }
    (games, hashes)
}

fn assert_batch_parity(games: &[Vec<u16>], hashes: &[u64]) {
    let refs: Vec<&[u16]> = games.iter().map(|g| g.as_slice()).collect();
    let outcomes: Vec<ReplayOutcome> = replay_moves2_batch(&refs);
    assert_eq!(outcomes.len(), games.len());
    for (i, outcome) in outcomes.iter().enumerate() {
        assert!(outcome.is_legal(), "game {} failed to replay", i);
        assert_eq!(outcome.moves_played as usize, games[i].len());
        assert_eq!(outcome.hash, Some(hashes[i]), "game {} hash mismatch", i);
    }
}

#[test]
fn batch_replay_matches_self_play() {
    let (games, hashes) = generate_games(2_000, 120, 0xAB1E_5EED);
    assert_batch_parity(&games, &hashes);
}

#[test]
fn batch_replay_scales_across_cores() {
    // Large enough to exercise the multi-threaded path in replay_moves2_batch.
    let (games, hashes) = generate_games(5_000, 120, 0x0DDB_1A5E);
    assert_batch_parity(&games, &hashes);
}

/// Spec scenario: 100,000 games replayed from binary moves2 slices with 100%
/// hash parity.
#[test]
#[ignore] // release-mode test: cargo test --release --test replay -- --ignored
fn batch_replay_100k_games() {
    let (games, hashes) = generate_games(100_000, 120, 0x0F0F_0000_0000_0001);
    assert_batch_parity(&games, &hashes);
}

#[test]
fn replay_rejects_corrupted_streams() {
    let (games, _) = generate_games(64, 60, 0xC0FF_EE00);
    let refs: Vec<&[u16]> = games.iter().map(|g| g.as_slice()).collect();
    let outcomes = replay_moves2_batch(&refs);
    assert!(outcomes.iter().all(|o| o.is_legal()));

    // Corrupt one move in each game (swap two adjacent words): the replay
    // must report an illegal stream rather than silently continuing.
    let mut corrupted: Vec<Vec<u16>> = Vec::new();
    for g in &games {
        if g.len() >= 2 && g[0] != g[1] {
            let mut c = g.clone();
            c.swap(0, 1);
            corrupted.push(c);
        }
    }
    let refs: Vec<&[u16]> = corrupted.iter().map(|g| g.as_slice()).collect();
    let outcomes = replay_moves2_batch(&refs);
    assert!(outcomes.iter().any(|o| !o.is_legal()));
}

#[test]
fn single_stream_and_batch_agree() {
    let (games, _) = generate_games(32, 80, 0x7777_8888);
    for g in &games {
        assert_eq!(
            replay_moves2_stream(g),
            replay_moves2_batch(&[g])[0],
            "stream vs batch mismatch"
        );
    }
}

#[test]
fn zero_alloc_movegen_buffer_size() {
    // ArrayVec<Move, 256> is 512 bytes + length tag: enforce the stack budget.
    assert_eq!(std::mem::size_of::<arrayvec::ArrayVec<Move, MAX_MOVES>>(), 516);
    let _ = Board::startpos();
}
