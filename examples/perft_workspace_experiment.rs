//! Experimental per-ply reusable workspace comparison. Benchmark-only binary.
//! cargo run --release --locked --example perft_workspace_experiment
use gigachess::fen::parse_fen;
use std::hint::black_box;
use std::time::Instant;

const CASES: [(&str, &str, u32, u64); 5] = [
    (
        "start",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        5,
        4_865_609,
    ),
    (
        "kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        4,
        4_085_603,
    ),
    (
        "endgame",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        5,
        674_624,
    ),
    (
        "promotion",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        4,
        2_103_487,
    ),
    (
        "pos4",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        4,
        422_333,
    ),
];
fn median(a: &[f64]) -> f64 {
    let mut s = a.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    s[s.len() / 2]
}
fn main() {
    println!(
        "RUST_PERFT_COMPARISON release optimized, version {}",
        env!("CARGO_PKG_VERSION")
    );
    for (name, fen, depth, expected) in CASES {
        let board = parse_fen(fen).unwrap();
        assert_eq!(board.perft(depth), expected, "{} baseline", name);
        assert_eq!(board.perft_reuse(depth), expected, "{} workspace", name);
        let warm = depth.min(3);
        for _ in 0..3 {
            black_box(board.perft(warm));
            black_box(board.perft_reuse(warm));
        }
        let mut baseline = Vec::new();
        let mut reuse = Vec::new();
        for round in 0..9 {
            let order = if round % 2 == 0 { [0, 1] } else { [1, 0] };
            for variant in order {
                let t = Instant::now();
                let n = if variant == 0 {
                    black_box(board.perft(black_box(depth)))
                } else {
                    black_box(board.perft_reuse(black_box(depth)))
                };
                assert_eq!(n, expected);
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if variant == 0 {
                    baseline.push(ms);
                } else {
                    reuse.push(ms);
                }
            }
        }
        let ba = median(&baseline);
        let re = median(&reuse);
        println!("{} d{} nodes={} originalMnps={:.3} workspaceMnps={:.3} ratio={:.4} baselineMs={:?} workspaceMs={:?}",
         name, depth, expected, expected as f64*0.001/ba, expected as f64*0.001/re, ba/re, baseline, reuse);
    }
}
