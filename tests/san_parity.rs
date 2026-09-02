// SAN rendering parity: turbochess-rs move_to_san must be byte-identical to
// shakmaty's SanPlus (disambiguation, captures, promotions, check/mate
// suffixes, castling) — the stored-DB rendering contract.
//
// SPDX-License-Identifier: MIT

use shakmaty::{san::SanPlus, CastlingSide, Chess, Color as SColor, Move as SMove, Role as SRole, Square as SSquare};
use turbochess_rs::{san as tsan, Board, Move, Role, Square};

fn to_shakmaty(board: &Board, mv: Move) -> SMove {
    let from = SSquare::new(u32::from(mv.from().0));
    let to = SSquare::new(u32::from(mv.to().0));
    let piece = board.piece_at(mv.from()).unwrap();
    let color = match piece.color {
        turbochess_rs::types::Color::White => SColor::White,
        turbochess_rs::types::Color::Black => SColor::Black,
    };
    let role = srole(piece.role);
    // Castling: destination holds the mover's own rook.
    if piece.role == Role::King {
        if let Some(cap) = board.piece_at(mv.to()) {
            if cap.role == Role::Rook && cap.color == piece.color {
                let side = if mv.to().file() > mv.from().file() {
                    CastlingSide::KingSide
                } else {
                    CastlingSide::QueenSide
                };
                let (king, rook) = {
                    let _ = color;
                    (from, to)
                };
                let _ = side;
                return SMove::Castle { king, rook };
            }
        }
    }
    if piece.role == Role::Pawn
        && mv.from().file() != mv.to().file()
        && board.piece_at(mv.to()).is_none()
    {
        return SMove::EnPassant { from, to };
    }
    let promotion = mv.promotion().map(|r| match r {
        Role::Knight => SRole::Knight,
        Role::Bishop => SRole::Bishop,
        Role::Rook => SRole::Rook,
        _ => SRole::Queen,
    });
    SMove::Normal {
        role,
        from,
        to,
        promotion,
        capture: board.piece_at(mv.to()).map(|p| srole(p.role)),
    }
}

fn srole(r: Role) -> SRole {
    match r {
        Role::Pawn => SRole::Pawn,
        Role::Knight => SRole::Knight,
        Role::Bishop => SRole::Bishop,
        Role::Rook => SRole::Rook,
        Role::Queen => SRole::Queen,
        Role::King => SRole::King,
    }
}

fn to_turbochess_role(r: SRole) -> Role {
    match r {
        SRole::Pawn => Role::Pawn,
        SRole::Knight => Role::Knight,
        SRole::Bishop => Role::Bishop,
        SRole::Rook => Role::Rook,
        SRole::Queen => Role::Queen,
        SRole::King => Role::King,
    }
}

fn from_shakmaty(board: &Board, m: &SMove) -> Move {
    let sq = |s: SSquare| Square::new(s as usize as u8);
    match *m {
        SMove::Normal { from, to, promotion, .. } => {
            let role = board.piece_at(sq(from)).unwrap().role;
            Move::new(sq(from), sq(to), promotion.map(to_turbochess_role).filter(|r| *r != role))
        }
        SMove::EnPassant { from, to } => Move::new(sq(from), sq(to), None),
        SMove::Put { to, .. } => Move::new(sq(to), sq(to), None),
        SMove::Castle { king, rook } => Move::new(sq(king), sq(rook), None),
    }
}

#[test]
fn san_rendering_matches_shakmaty_on_random_games() {
    let mut state = 0x0DDB_1A5E_0000_0001u64;
    let mut positions = 0u64;
    for game in 0..60 {
        let mut board = Board::startpos();
        let mut pos = Chess::default();
        for _ply in 0..160 {
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let mv = legal[(state % legal.len() as u64) as usize];
            let smove = to_shakmaty(&board, mv);

            let ours = tsan::move_to_san(&board, mv).unwrap();
            let theirs = SanPlus::from_move_and_play_unchecked(&mut pos, smove);
            let mut rendered = String::new();
            theirs.append_to_string(&mut rendered);
            assert_eq!(ours.as_str(), rendered, "game {game}");
            positions += 1;

            board.play(mv).unwrap();
        }
    }
    assert!(positions > 3000);
}

#[test]
fn san_parsing_accepts_shakmaty_rendered_san() {
    // Our parser must accept every SAN shakmaty renders (and resolve to the
    // same move) — covers suffix and disambiguation conventions.
    let mut state = 0x5EED_5EED_0000_0042u64;
    for _game in 0..25 {
        let mut board = Board::startpos();
        let mut pos = Chess::default();
        for _ply in 0..120 {
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let mv = legal[(state % legal.len() as u64) as usize];
            let smove = to_shakmaty(&board, mv);
            let theirs = SanPlus::from_move_and_play_unchecked(&mut pos, smove);
            let mut rendered = String::new();
            theirs.append_to_string(&mut rendered);

            let parsed = tsan::san_to_move(&board, &rendered)
                .unwrap_or_else(|| panic!("cannot parse shakmaty SAN {rendered:?}"));
            let expect = from_shakmaty(&board, &smove);
            assert_eq!(parsed.word(), expect.word(), "SAN {rendered:?}");

            board.play(mv).unwrap();
        }
    }
}
