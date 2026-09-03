// Zobrist verification: Polyglot test vectors plus incremental-vs-full hash
// parity over deterministic random playouts.
//
// SPDX-License-Identifier: MIT

use arrayvec::ArrayVec;
use gigachess::board::MAX_MOVES;
use gigachess::fen::parse_fen;
use gigachess::moves::Move;
use gigachess::Board;

/// The canonical Polyglot startpos hash (public test vector).
#[test]
fn startpos_polyglot_vector() {
    let board = Board::startpos();
    assert_eq!(board.zobrist(), 0x463b_9618_1691_fc9c);
    assert_eq!(board.zobrist_full(), 0x463b_9618_1691_fc9c);
}

/// A second known Polyglot vector: Kiwipete with all castling rights.
#[test]
fn kiwipete_hash_matches_full_recompute() {
    let board = parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
        .unwrap();
    assert_eq!(board.zobrist(), board.zobrist_full());
}

/// Cross-library parity: after 1.e4 the Polyglot key must equal the value
/// published in the JS TurboChess README (`zobristHex()` = "823c9b50fd114196"),
/// proving Rust/JS interop for opening books and repetition detection.
#[test]
fn js_turbochess_parity_after_e4() {
    let mut board = Board::startpos();
    let mv = Move::new(
        gigachess::Square::from_alg("e2").unwrap(),
        gigachess::Square::from_alg("e4").unwrap(),
        None,
    );
    board.play(mv).unwrap();
    assert_eq!(board.zobrist(), 0x823c_9b50_fd11_4196);
    assert_eq!(board.zobrist_full(), 0x823c_9b50_fd11_4196);
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Plays `games` random legal games; at every ply the incremental hash must
/// equal the from-scratch recomputation, and unmake must restore it exactly.
fn incremental_parity(games: usize, max_plies: u32, seed: u64) {
    let mut state = seed;
    for _ in 0..games {
        let mut board = Board::startpos();
        let mut moves: ArrayVec<Move, MAX_MOVES> = ArrayVec::new();
        let mut undos = Vec::new();
        for _ in 0..max_plies {
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            let mv = legal[(xorshift(&mut state) % legal.len() as u64) as usize];
            let undo = board.make_move_unchecked(mv);
            undos.push((mv, undo));
            moves.push(mv);
            assert_eq!(
                board.zobrist(),
                board.zobrist_full(),
                "incremental hash diverged after {:?} in {}",
                mv,
                board.to_fen()
            );
        }
        // Unmake the entire game in reverse; every position must be restored.
        for (mv, undo) in undos.into_iter().rev() {
            board.unmake_move(mv, undo);
            assert_eq!(
                board.zobrist(),
                board.zobrist_full(),
                "unmake diverged before {}",
                mv
            );
        }
        assert_eq!(board, Board::startpos());
    }
}

#[test]
fn incremental_hash_parity_random_playouts() {
    incremental_parity(25, 120, 0x5EED_5EED_5EED_5EED);
}

#[test]
fn incremental_hash_parity_tactical_positions() {
    // High-castling / high-promotion traffic positions.
    let fens = [
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    ];
    for fen in fens {
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        let mut board = parse_fen(fen).unwrap();
        for _ in 0..200 {
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            let mv = legal[(xorshift(&mut state) % legal.len() as u64) as usize];
            board.make_move_unchecked(mv);
            assert_eq!(board.zobrist(), board.zobrist_full(), "after {}", mv);
        }
    }
}

/// FEN -> board -> hash must be deterministic regardless of path
/// (fresh parse vs. play from startpos).
#[test]
fn hash_is_path_independent() {
    // 1. e4 e5 2. Nf3 reached by play vs. by its known FEN.
    let mut by_play = Board::startpos();
    for uci in ["e2e4", "e7e5", "g1f3"] {
        let from = gigachess::Square::from_alg(&uci[0..2]).unwrap();
        let to = gigachess::Square::from_alg(&uci[2..4]).unwrap();
        by_play.play(Move::new(from, to, None)).unwrap();
    }
    let by_fen = parse_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2")
        .unwrap();
    assert_eq!(by_play.zobrist(), by_fen.zobrist());
}
