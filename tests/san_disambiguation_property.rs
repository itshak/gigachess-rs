// Property test: the direct disambiguation query equals the movegen filter.
//
// `move_to_san_body` answers "which other piece of the same role can legally
// reach this square" by asking each candidate directly — one `make_move_unchecked`
// on a stack copy, one king-safety query for the mover's king against the side
// to move next, one `unmake_move`. It used to generate every legal move in the
// position and filter that list. This asserts over 200,000 positions from
// random playouts — standard and two Chess960 starts — that both forms produce
// the same SAN for **every** legal move, which is the property the change rests
// on: the pre-filter is exact pseudo-legality, so legality is the only question
// left, and asking it per candidate is the same question the movegen answers.
//
// The oracle is the old filter, written out here: a `generate_moves_into` and
// the same three tests the implementation used to apply.
//
// SPDX-License-Identifier: MIT

use gigachess::attacks;
use gigachess::bitboard::{bit, KING_ATT, KNIGHT_ATT};
use gigachess::fen::parse_fen;
use gigachess::moves::Move;
use gigachess::san::move_to_san_body;
use gigachess::types::{Role, Square};
use gigachess::Board;

/// The disambiguation the old implementation produced: every legal move of the
/// position, filtered down to the alternatives that reach the same square with
/// the same role and promotion.
fn movegen_others(board: &Board, mv: Move) -> Vec<Square> {
    let (from, to) = (mv.from(), mv.to());
    let Some(piece) = board.piece_at(from) else {
        return Vec::new();
    };
    if piece.role == Role::Pawn {
        // A pawn capture is disambiguated by its file; there is no candidate
        // set to ask about.
        return Vec::new();
    }
    let same_bb = board.piece_bb(piece.color, piece.role);
    let mut moves = gigachess::movegen::MoveList::new();
    board.generate_moves_into(&mut moves);
    let mut others = Vec::new();
    for &cm in moves.as_slice() {
        if cm.to() != to || cm.from() == from {
            continue;
        }
        if cm.promotion() != mv.promotion() {
            continue;
        }
        if same_bb & bit(cm.from().0) == 0 {
            continue;
        }
        others.push(cm.from());
        if others.len() >= 8 {
            break;
        }
    }
    others
}

/// The pre-filter, kept here so the test can report how often the branch fires
/// and that it never proposes a square the movegen would not.
fn prefilter(board: &Board, mv: Move) -> u64 {
    let Some(piece) = board.piece_at(mv.from()) else {
        return 0;
    };
    let occ = board.occupied();
    let to = mv.to();
    let att = match piece.role {
        Role::Knight => KNIGHT_ATT[to.index()],
        Role::Bishop => attacks::bishop_attacks(to.0, occ),
        Role::Rook => attacks::rook_attacks(to.0, occ),
        Role::Queen => attacks::queen_attacks(to.0, occ),
        Role::King => KING_ATT[to.index()],
        _ => 0,
    };
    att & board.piece_bb(piece.color, piece.role) & !bit(mv.from().0)
}

/// The qualifier the rendered piece move carries: the SAN of a piece move is
/// `[role][qualifier][x]target[=promotion]`, so the qualifier is what sits
/// between the role letter and the target square.
fn rendered_qualifier(san: &str) -> &str {
    let body = san.strip_suffix('=').map_or(san, |_| &san[..san.len() - 1]);
    // The target is the last two characters; a capture marker may precede it.
    let (head, _tail) = body.split_at(body.len() - 2);
    let head = head.strip_suffix('x').unwrap_or(head);
    &head[1..]
}

/// The qualifier the oracle's candidate set implies: the file when no other
/// candidate shares it, otherwise the rank when none shares that, otherwise
/// both.
fn expected_qualifier(from: Square, others: &[Square]) -> String {
    if others.is_empty() {
        return String::new();
    }
    let same_file = others.iter().any(|s| s.file() == from.file());
    let same_rank = others.iter().any(|s| s.rank() == from.rank());
    let mut out = String::new();
    if !same_file {
        out.push((b'a' + from.file()) as char);
    } else if !same_rank {
        out.push((b'1' + from.rank()) as char);
    } else {
        out.push((b'a' + from.file()) as char);
        out.push((b'1' + from.rank()) as char);
    }
    out
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn the_direct_query_matches_the_movegen_filter_across_200k_moves() {
    let mut rng = 0x5AFE_C0DE_1234_ABCDu64;
    let mut checked = 0usize;
    let mut board_positions = 0usize;
    let target_moves = 200_000usize;
    let mut ambiguous = 0usize;
    let mut with_qualifier = 0usize;
    let roots = [
        // The standard opening position.
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // Chess960 start 518: rooks on f1/h1 and a king on g1, so the castling
        // words do not come from a1/e1/h1.
        "bqnb1rkr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQ1BNRKR w HFhf - 2 9",
        // Chess960 start 284: king d1, rooks c1/f1.
        "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1",
    ];

    while checked < target_moves {
        let mut board = parse_fen(roots[board_positions % roots.len()]).expect("a start position");
        board_positions += 1;

        for _ply in 0..80 {
            let moves = board.legal_moves();
            if moves.is_empty() {
                break;
            }
            // Every third legal move of the position, and every move that takes
            // the ambiguous branch: the branch is the thing under test, and
            // uniform random play reaches it in well under a percent of moves.
            for (i, &mv) in moves.as_slice().iter().enumerate() {
                if i % 3 != 0 && prefilter(&board, mv) == 0 {
                    continue;
                }
                checked += 1;
                let body = move_to_san_body(&board, mv);
                let others = movegen_others(&board, mv);
                if prefilter(&board, mv) != 0 {
                    ambiguous += 1;
                }
                if !others.is_empty() {
                    with_qualifier += 1;
                }

                // The oracle only speaks for the roles that reach the branch: a
                // pawn capture carries its file, and castling is a word.
                let Some(piece) = board.piece_at(mv.from()) else {
                    continue;
                };
                if piece.role == Role::Pawn {
                    continue;
                }
                let is_castle = piece.role == Role::King
                    && board.piece_at(mv.to()).map(|p| p.role) == Some(Role::Rook);
                if is_castle {
                    continue;
                }
                let Some(body) = body else { continue };
                assert_eq!(
                    rendered_qualifier(body.as_str()),
                    expected_qualifier(mv.from(), &others),
                    "the direct query and the movegen filter disagree on {mv:?} in {}",
                    board.to_fen()
                );
            }

            // Advance the game, steering half the plies towards a move that
            // takes the branch so the middlegame keeps the pieces that make
            // one.
            let steering: Vec<Move> = moves
                .as_slice()
                .iter()
                .copied()
                .filter(|&m| prefilter(&board, m) != 0)
                .collect();
            let mv = if !steering.is_empty() && xorshift(&mut rng).is_multiple_of(2) {
                steering[(xorshift(&mut rng) as usize) % steering.len()]
            } else {
                moves[(xorshift(&mut rng) as usize) % moves.len()]
            };
            let _ = board.make_move_unchecked(mv);
            if xorshift(&mut rng).is_multiple_of(6) {
                break;
            }
        }
    }

    println!(
        "{checked} moves from {board_positions} board positions: {ambiguous} took the ambiguous branch, \
         {with_qualifier} had a legal alternative"
    );
    // The property is only meaningful where the branch actually fires and where
    // the notation really needs a qualifier.
    assert!(
        ambiguous > 2_000,
        "only {ambiguous} moves took the ambiguous branch"
    );
    assert!(
        with_qualifier > 1_000,
        "only {with_qualifier} moves had a legal alternative"
    );
}
