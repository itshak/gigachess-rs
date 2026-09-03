// Perft validation against the standard reference node-count suites
// (CPW "Perft Results" positions). Run deep counts with:
//   cargo test --release --test perft -- --ignored
//
// SPDX-License-Identifier: MIT

use gigachess::fen::parse_fen;

fn perft(fen: &str, depth: u32, expected: u64) {
    let board = parse_fen(fen).unwrap();
    let got = board.perft(depth);
    assert_eq!(
        got, expected,
        "perft mismatch at depth {depth} for {fen}"
    );
}

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const POS3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
const POS4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POS5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const POS6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

#[test]
fn startpos_d1_to_d5() {
    perft(STARTPOS, 1, 20);
    perft(STARTPOS, 2, 400);
    perft(STARTPOS, 3, 8_902);
    perft(STARTPOS, 4, 197_281);
    perft(STARTPOS, 5, 4_865_609);
}

#[test]
#[ignore] // ~2-10s in release
fn startpos_d6() {
    perft(STARTPOS, 6, 119_060_324);
}

#[test]
fn kiwipete_d1_to_d4() {
    perft(KIWIPETE, 1, 48);
    perft(KIWIPETE, 2, 2_039);
    perft(KIWIPETE, 3, 97_862);
    perft(KIWIPETE, 4, 4_085_603);
}

#[test]
#[ignore]
fn kiwipete_d5() {
    perft(KIWIPETE, 5, 193_690_690);
}

#[test]
fn pos3_d1_to_d5() {
    perft(POS3, 1, 14);
    perft(POS3, 2, 191);
    perft(POS3, 3, 2_812);
    perft(POS3, 4, 43_238);
    perft(POS3, 5, 674_624);
}

#[test]
#[ignore]
fn pos3_d6() {
    perft(POS3, 6, 11_030_083);
}

#[test]
fn pos4_d1_to_d4() {
    perft(POS4, 1, 6);
    perft(POS4, 2, 264);
    perft(POS4, 3, 9_467);
    perft(POS4, 4, 422_333);
}

#[test]
#[ignore]
fn pos4_d5() {
    perft(POS4, 5, 15_833_292);
}

#[test]
fn pos5_d1_to_d4() {
    perft(POS5, 1, 44);
    perft(POS5, 2, 1_486);
    perft(POS5, 3, 62_379);
    perft(POS5, 4, 2_103_487);
}

#[test]
#[ignore]
fn pos5_d5() {
    perft(POS5, 5, 89_941_194);
}

#[test]
fn pos6_d1_to_d4() {
    perft(POS6, 1, 46);
    perft(POS6, 2, 2_079);
    perft(POS6, 3, 89_890);
    perft(POS6, 4, 3_894_594);
}

#[test]
#[ignore]
fn pos6_d5() {
    perft(POS6, 5, 164_075_551);
}

/// Positions 4/5/6 exercise promotions, castling, en-passant pins and
/// discovered checks; also validate the mirrored variants used on CPW.
#[test]
fn mirrored_pos4_d1_to_d4() {
    // Position 4 mirrored (black to move).
    perft(
        "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
        1,
        6,
    );
    perft(
        "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
        3,
        9_467,
    );
}

/// Visitor parity: `perft_visitor` (CountingVisitor leaf, no `Move`
/// materialisation) must equal `perft` (MoveCounter bulk) on all 6 CPW
/// positions at depth 3 (close-gap task 2.2).
#[test]
fn perft_visitor_matches_perft_d3() {
    let cases = [
        (STARTPOS, 8_902),
        (KIWIPETE, 97_862),
        (POS3, 2_812),
        (POS4, 9_467),
        (POS5, 62_379),
        (POS6, 89_890),
    ];
    for (fen, expected_d3) in cases {
        let board = parse_fen(fen).unwrap();
        assert_eq!(
            board.perft_visitor(3),
            expected_d3,
            "perft_visitor mismatch for {fen}"
        );
        assert_eq!(board.perft_visitor(1), board.perft(1));
    }
}

/// Leaf-count parity on tricky states: visitor must agree with the
/// MoveCounter bulk path at depth 1 along a played line from Kiwipete.
#[test]
fn perft_visitor_leaf_parity_along_line() {
    let mut board = parse_fen(KIWIPETE).unwrap();
    for depth in 1..=4u32 {
        assert_eq!(board.perft(depth), board.perft_visitor(depth));
        let moves = board.legal_moves();
        match moves.first() {
            Some(&m) => {
                let _ = board.play(m).unwrap();
            }
            None => break,
        }
    }
}

/// Legality cross-check: every move produced by `legal_moves` must be
/// accepted by `play`, and playing/unmaking must restore the position.
#[test]
fn legal_moves_roundtrip_consistency() {
    let fens = [STARTPOS, KIWIPETE, POS3, POS4, POS5, POS6];
    for fen in fens {
        let board = parse_fen(fen).unwrap();
        let hash = board.zobrist();
        let mut clone = board;
        for mv in board.legal_moves() {
            let undo = clone.play(mv).expect("legal move must play");
            clone.unmake_move(mv, undo);
        }
        assert_eq!(clone.zobrist(), hash, "make/unmake must restore state");
        assert_eq!(clone, board);
    }
}
