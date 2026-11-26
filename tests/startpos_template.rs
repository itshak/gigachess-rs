// SPDX-License-Identifier: MIT
use gigachess::{fen::parse_fen, san::san_to_move, Board};

#[test]
fn template_is_identical_to_full_fen_parse_and_independently_owned() {
    const FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let reference = parse_fen(FEN).unwrap();
    let mut changed = Board::startpos();
    assert_eq!(changed, reference);
    assert_eq!(changed.zobrist(), changed.zobrist_full());
    assert_eq!(changed.repetition_key(), reference.repetition_key());
    assert_eq!(changed.legal_moves(), reference.legal_moves());
    let mv = san_to_move(&changed, "e4").unwrap();
    let undo = changed.play(mv).unwrap();
    assert_ne!(changed, reference);
    assert_eq!(Board::startpos(), reference);
    changed.unmake_move(mv, undo);
    assert_eq!(changed, reference);
}
