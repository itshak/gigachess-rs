// SAN disambiguation: when two same-role pieces can legally reach a square,
// the move must carry a file (or rank) qualifier. Probe positions taken from
// Mega Database 2025 games where ChessBase's export and this library's output
// were compared.
//
// SPDX-License-Identifier: MIT

use gigachess::fen::parse_fen;
use gigachess::san::move_to_san;
use gigachess::{Move, Square};

fn san(fen: &str, from: &str, to: &str) -> String {
    let board = parse_fen(fen).unwrap_or_else(|e| panic!("fen {fen}: {e:?}"));
    let mv = Move::new(Square::from_alg(from).unwrap(), Square::from_alg(to).unwrap(), None);
    move_to_san(&board, mv).expect("legal move").as_str().to_owned()
}

/// Two black rooks (c8, b2) can both legally reach c2: `Rcc2`/`Rbc2`.
#[test]
fn two_rooks_to_one_square_disambiguate() {
    let fen = "2r3k1/7p/p5p1/3pPn2/5P2/PN1R1NP1/1r5P/5K2 b - - 5 26";
    assert_eq!(san(fen, "c8", "c2"), "Rcc2", "the c-file rook carries its file");
    assert_eq!(san(fen, "b2", "c2"), "Rbc2", "the b-file rook carries its file");
}

/// One of two knights is pinned (the d5 knight shields e6 from Bc4), so only
/// c6-e7 is legal and the move needs no qualifier: `Ne7`, not `Nce7`.
#[test]
fn pinned_twin_needs_no_qualifier() {
    let fen = "r1bq1b1r/ppp3pp/2n1k3/3np3/2B5/2N2Q2/PPPP1PPP/R1B1K2R b KQ - 3 8";
    assert_eq!(san(fen, "c6", "e7"), "Ne7");
}
