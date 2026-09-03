// FEN verification: known positions, round-trips and the 1000-position
// sample file (`tests/data/samplefen1000.epd`).
//
// SPDX-License-Identifier: MIT

use gigachess::fen::{parse_fen, FenError};

const FENS: &[&str] = &[
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    // En-passant targets, promotions, sparse endgames:
    "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3",
    "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
    "8/8/8/1k6/3Pp3/8/8/4K3 b - d3 0 1",
    "3q2k1/8/8/8/8/8/8/3QK3 w - - 0 1",
];

#[test]
fn known_fens_round_trip() {
    for fen in FENS {
        let board = parse_fen(fen).unwrap();
        assert_eq!(&board.to_fen(), fen, "round trip failed for {}", fen);
    }
}

#[test]
fn rejected_fens() {
    // Not 8 ranks.
    assert!(parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP w KQkq - 0 1").is_err());
    // Two white kings.
    assert!(parse_fen("K1K5/8/8/8/8/8/8/4k3 w - - 0 1").is_err());
    // No black king.
    assert!(parse_fen("8/8/8/8/8/8/8/4K3 w - - 0 1").is_err());
    // Adjacent kings.
    assert!(parse_fen("4k3/4K3/8/8/8/8/8/8 w - - 0 1").is_err());
    // Pawn on the back rank.
    assert!(parse_fen("4k3/8/8/8/8/8/8/P3K3 w - - 0 1").is_err());
    // Bad side to move.
    assert!(parse_fen("4k3/8/8/8/8/8/8/4K3 x - - 0 1").is_err());
    // En-passant square on the wrong rank (e3 is only valid for black to move).
    assert!(parse_fen("4k3/8/8/8/8/8/8/4K3 w - e3 0 1").is_err());
    // Wrong field count.
    assert!(parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").is_err());
    // Fullmove zero.
    assert!(parse_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 0").is_err());
}

#[test]
fn error_type_is_structured() {
    let err: FenError = parse_fen("nope nope nope").unwrap_err();
    assert!(err.to_string().contains("invalid FEN"));
}

/// Reads `tests/data/samplefen1000.epd` (1000 FEN lines, one per line) and
/// round-trips every position through parse -> format -> parse.
#[test]
fn samplefen1000_round_trip() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/samplefen1000.epd");
    let data = std::fs::read_to_string(path).expect("samplefen1000.epd fixture");
    let mut count = 0usize;
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let board = parse_fen(line).unwrap_or_else(|e| panic!("{}: {}", line, e));
        let rendered = board.to_fen();
        assert_eq!(rendered, line, "round trip failed");
        let reparsed = parse_fen(&rendered).unwrap();
        assert_eq!(reparsed.zobrist(), board.zobrist());
        count += 1;
    }
    assert_eq!(count, 1000, "fixture must contain exactly 1000 positions");
}
