// Null moves: a pass flips the side to move without moving a piece.
// `gigachess` supports them as a board primitive (not a `Move`): CBH stores
// pass turns as moves, and legality around them is checked against the
// position they produce.
//
// SPDX-License-Identifier: MIT

use gigachess::fen::parse_fen;
use gigachess::{Board, Color};

fn uci(board: &mut Board, text: &str) {
    let (from, to) = text.split_at(2);
    let mv = gigachess::Move::new(
        gigachess::Square::from_alg(from).unwrap(),
        gigachess::Square::from_alg(to).unwrap(),
        None,
    );
    board
        .play(mv)
        .unwrap_or_else(|_| panic!("uci {text} must be legal in {}", board.to_fen()));
}

/// A null move flips the side to move and preserves the placement.
#[test]
fn null_move_passes_the_turn() {
    let mut board =
        parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
    let before = board;
    let undo = board.make_null_move().unwrap();
    assert_eq!(board.turn(), Color::White);
    assert_eq!(board.en_passant(), None, "the double push lapses");
    assert_eq!(board.halfmove_clock(), before.halfmove_clock() + 1);
    assert_eq!(
        board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2",
        "a pass is a move: Black's pass completes the move"
    );
    assert_eq!(
        board.zobrist(),
        board.zobrist_full(),
        "incremental hash parity"
    );
    board.unmake_null_move(undo);
    assert_eq!(board, before, "unmake restores the exact position");
}

/// A pass is a move, not a half-move: after the pass White is to move with the
/// number advanced, whoever passed.
#[test]
fn black_null_completes_the_move() {
    let mut board =
        parse_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    uci(&mut board, "d2d4"); // White to move -> Black to move, still move 2
    let before = board;
    assert_eq!(before.turn(), Color::Black);
    let undo = board.make_null_move().unwrap();
    assert_eq!(board.turn(), Color::White);
    assert_eq!(board.fullmove_number(), 3);
    assert_eq!(
        board.zobrist(),
        board.zobrist_full(),
        "incremental hash parity"
    );
    board.unmake_null_move(undo);
    assert_eq!(board, before, "unmake restores the exact position");
}

/// White's pass also completes the move: the number advances too, since after
/// the pass White is to move again at the next number.
#[test]
fn white_null_completes_the_move() {
    let mut board =
        parse_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let before = board;
    let undo = board.make_null_move().unwrap();
    assert_eq!(board.turn(), Color::Black);
    assert_eq!(board.fullmove_number(), 3);
    assert_eq!(
        board.zobrist(),
        board.zobrist_full(),
        "incremental hash parity"
    );
    board.unmake_null_move(undo);
    assert_eq!(board, before, "unmake restores the exact position");
}

/// A null move answers no check.
#[test]
fn null_move_in_check_is_illegal() {
    // Scholar's mate pattern: Qxf7+ leaves Black in check. The f-pawn must be
    // gone first (Qh5xf7 captures it), so play e4 e5 Qh5 Nf6?? Qxf7+.
    let mut board = Board::startpos();
    uci(&mut board, "e2e4");
    uci(&mut board, "e7e5");
    uci(&mut board, "d1h5");
    uci(&mut board, "g8f6");
    uci(&mut board, "h5f7"); // Qxf7+: Black to move, in check
    assert!(board.in_check());
    assert!(board.make_null_move().is_err());
}

/// Null then unmake restores hash and checkers, paired like ordinary unmake.
///
/// Each null must be unmade before an older one, with no ordinary moves
/// interleaved past the matching make — exactly like `unmake_move`. Ordinary
/// moves mutate bitboards the null `Undo` does not record, so unmaking a
/// null across an ordinary move restores the clocks, turn and hash but not
/// the pieces.
#[test]
fn null_make_unmake_round_trips() {
    let mut board = Board::startpos();
    for mv in ["e2e4", "e7e5", "g1f3", "b8c6"] {
        uci(&mut board, mv);
    }
    let before = board;
    let u1 = board.make_null_move().unwrap();
    assert_eq!(
        board.zobrist(),
        board.zobrist_full(),
        "incremental hash parity after the outer null"
    );
    let u2 = board.make_null_move().unwrap();
    assert_eq!(
        board.zobrist(),
        board.zobrist_full(),
        "incremental hash parity after the inner null"
    );
    // Unmake in reverse order restores each null's own prior state bit-for-bit.
    board.unmake_null_move(u2);
    let mut back = before;
    back.make_null_move().unwrap();
    assert_eq!(
        board, back,
        "unmaking the inner null restores its position bit-for-bit"
    );
    board.unmake_null_move(u1);
    assert_eq!(
        board, before,
        "unmaking the outer null restores its position bit-for-bit"
    );
}
