// Zero-allocation SAN parser and disambiguator.
//
// `move_to_san` renders a legal move with minimal disambiguation plus
// check/mate annotations; `san_to_move` parses SAN against a position and
// resolves disambiguation by matching against the legal move list.
//
// SPDX-License-Identifier: MIT

use crate::board::{Board, MAX_MOVES};
use crate::moves::Move;
use crate::types::{Role, Square};
use arrayvec::{ArrayString, ArrayVec};

/// Maximum rendered SAN length (longest: "Qh4xe1=Q+" style strings).
pub type San = ArrayString<12>;

/// Renders `mv` (which must be legal in `board`) in SAN.
///
/// Returns `None` if the move is not legal in the position.
pub fn move_to_san(board: &Board, mv: Move) -> Option<San> {
    let legal = board.legal_moves();
    if !legal.contains(&mv) {
        return None;
    }

    let from = mv.from();
    let to = mv.to();
    let piece = board.piece_at(from)?;
    let mut out = San::new();

    // Castling: the destination holds the mover's own rook (ADR-003 D3
    // encoding, valid for standard chess and Chess960 alike).
    let is_castle = piece.role == Role::King
        && board.piece_at(to) == Some(crate::types::Piece::new(piece.color, Role::Rook));
    if is_castle {
        out.push_str(if to.file() > from.file() { "O-O" } else { "O-O-O" });
    } else {
        let is_capture = board.piece_at(to).is_some()
            || (piece.role == Role::Pawn
                && board.en_passant() == Some(to)
                && from.file() != to.file());

        if piece.role != Role::Pawn {
            out.push(piece.role.char_upper());
            // Disambiguation: other legal moves of the same role to the same
            // target from a different origin.
            let mut others: ArrayVec<Move, MAX_MOVES> = ArrayVec::new();
            for m in &legal {
                if m.to() == to
                    && m.from() != from
                    && board.piece_at(m.from()).map(|p| p.role) == Some(piece.role)
                {
                    others.push(*m);
                }
            }
            if !others.is_empty() {
                let same_file = others.iter().any(|m| m.from().file() == from.file());
                let same_rank = others.iter().any(|m| m.from().rank() == from.rank());
                if !same_file {
                    out.push((b'a' + from.file()) as char);
                } else if !same_rank {
                    out.push((b'1' + from.rank()) as char);
                } else {
                    out.push((b'a' + from.file()) as char);
                    out.push((b'1' + from.rank()) as char);
                }
            }
        } else if is_capture {
            // Pawn captures always carry the origin file.
            out.push((b'a' + from.file()) as char);
        }

        if is_capture {
            out.push('x');
        }
        let [tf, tr] = to.to_alg();
        out.push(tf as char);
        out.push(tr as char);
        if let Some(p) = mv.promotion() {
            out.push('=');
            out.push(p.char_upper());
        }
    }

    // Check / mate annotation.
    let mut after = board.clone();
    let undo = after.make_move_unchecked(mv);
    if after.in_check() {
        out.push(if after.legal_moves().is_empty() {
            '#'
        } else {
            '+'
        });
    }
    after.unmake_move(mv, undo);
    Some(out)
}

/// Parses a SAN token against `board` and returns the legal move it denotes.
///
/// Accepts standard SAN (with optional `+`, `#`, `!`, `?` suffixes, the
/// `=Q` promotion spelling and both `O-O` / `0-0` castling spellings).
pub fn san_to_move(board: &Board, san: &str) -> Option<Move> {
    let s = san.trim();
    // Strip annotation suffixes.
    let s = s.trim_end_matches(['+', '#', '!', '?']);
    if s.is_empty() {
        return None;
    }
    let legal = board.legal_moves();

    // Castling (accept O-O, 0-0, O-O-O, 0-0-0 with or without dashes).
    // The move words are king-from → rook-square (ADR-003 D3); the side is
    // determined by the rook file relative to the king file.
    let normalized: String = s
        .chars()
        .filter(|c| *c != '-')
        .map(|c| if c == '0' { 'O' } else { c })
        .collect();
    if normalized == "OO" || normalized == "OOO" {
        let kingside = normalized == "OO";
        return legal.into_iter().find(|m| {
            board.piece_at(m.from()).map(|p| p.role) == Some(Role::King)
                && board.piece_at(m.to()) == Some(crate::types::Piece::new(board.turn(), Role::Rook))
                && (if kingside {
                    m.to().file() > m.from().file()
                } else {
                    m.to().file() < m.from().file()
                })
        });
    }

    let bytes = s.as_bytes();

    // Promotion suffix.
    let (body, promo) = if bytes.len() >= 2 && bytes[bytes.len() - 2] == b'=' {
        let p = match bytes[bytes.len() - 1] {
            b'N' => Some(Role::Knight),
            b'B' => Some(Role::Bishop),
            b'R' => Some(Role::Rook),
            b'Q' => Some(Role::Queen),
            _ => return None,
        };
        (&bytes[..bytes.len() - 2], p)
    } else {
        (bytes, None)
    };

    // Piece letter (absence => pawn move).
    let (role, rest) = match body.first()? {
        b'N' => (Role::Knight, &body[1..]),
        b'B' => (Role::Bishop, &body[1..]),
        b'R' => (Role::Rook, &body[1..]),
        b'Q' => (Role::Queen, &body[1..]),
        b'K' => (Role::King, &body[1..]),
        b'P' => (Role::Pawn, &body[1..]),
        _ => (Role::Pawn, body),
    };

    // Target square = last two characters of the remainder.
    if rest.len() < 2 {
        return None;
    }
    let to = Square::from_alg(core::str::from_utf8(&rest[rest.len() - 2..]).ok()?)?;
    let prefix = &rest[..rest.len() - 2];

    // Disambiguation hints from the prefix (e.g. "b" in Nbd2, "1" in N1d2,
    // "h4" in Qh4e1, "e" in exd5). 'x' is ignored.
    let mut hint_file: Option<u8> = None;
    let mut hint_rank: Option<u8> = None;
    for &c in prefix {
        match c {
            b'a'..=b'h' => hint_file = Some(c - b'a'),
            b'1'..=b'8' => hint_rank = Some(c - b'1'),
            b'x' | b'X' => {}
            _ => return None,
        }
    }

    let mut result: Option<Move> = None;
    for m in legal {
        if m.to() != to
            || board.piece_at(m.from()).map(|p| p.role) != Some(role)
            || m.promotion() != promo
        {
            continue;
        }
        if let Some(f) = hint_file {
            if m.from().file() != f {
                continue;
            }
        }
        if let Some(r) = hint_rank {
            if m.from().rank() != r {
                continue;
            }
        }
        if result.is_some() {
            return None; // ambiguous
        }
        result = Some(m);
    }
    result
}

/// Convenience: play a sequence of SAN moves from `board`, returning the
/// index of the first illegal token on failure.
pub fn play_san(board: &mut Board, sans: &[&str]) -> Result<(), usize> {
    for (i, s) in sans.iter().enumerate() {
        let mv = san_to_move(board, s).ok_or(i)?;
        board.play(mv).map_err(|_| i)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Color;

    fn play_all(sans: &[&str]) -> Board {
        let mut board = Board::startpos();
        play_san(&mut board, sans).expect("moves must be legal");
        board
    }

    #[test]
    fn san_roundtrip_startpos_openings() {
        let lines: [&[&str]; 3] = [
            &["e4", "e5", "Nf3", "Nc6", "Bb5", "a6"],
            &["d4", "d5", "c4", "e6", "Nc3", "Nf6"],
            &["e4", "c5", "Nf3", "d6", "d4", "cxd4"],
        ];
        for line in lines {
            let mut board = Board::startpos();
            for san in line {
                let expected: &str = san.trim_end_matches(['+', '#']);
                let mv = san_to_move(&board, san).unwrap_or_else(|| panic!("parse {}", san));
                let rendered = move_to_san(&board, mv).unwrap();
                assert_eq!(rendered.trim_end_matches(['+', '#']), expected, "roundtrip");
                board.play(mv).unwrap();
            }
        }
    }

    #[test]
    fn disambiguation_file_hint() {
        // Knights b2 and d2 can both reach c4.
        let board = crate::fen::parse_fen("5k2/8/8/8/8/8/1N1N4/4K3 w - - 0 1").unwrap();
        let mv = san_to_move(&board, "Nbc4").unwrap();
        assert_eq!(mv.from(), Square::from_alg("b2").unwrap());
        let mv = san_to_move(&board, "Ndc4").unwrap();
        assert_eq!(mv.from(), Square::from_alg("d2").unwrap());
        // Ambiguous without a hint.
        assert!(san_to_move(&board, "Nc4").is_none());
    }

    #[test]
    fn disambiguation_rook_file() {
        // Rooks a1, a4, h4: d4 is reachable by a4 and h4.
        let board = crate::fen::parse_fen("4k3/8/8/8/R6R/8/8/R3K3 w - - 0 1").unwrap();
        let mv = san_to_move(&board, "Rad4").unwrap();
        assert_eq!(mv.from(), Square::from_alg("a4").unwrap());
        let mv = san_to_move(&board, "Rhd4").unwrap();
        assert_eq!(mv.from(), Square::from_alg("h4").unwrap());
    }

    #[test]
    fn check_and_mate_suffixes() {
        let board = play_all(&["f3", "e5", "g4"]);
        let mv = san_to_move(&board, "Qh4").unwrap();
        assert_eq!(move_to_san(&board, mv).unwrap().as_str(), "Qh4#");
        // Suffixed token parses too.
        assert!(san_to_move(&board, "Qh4#").is_some());
    }

    #[test]
    fn castling_san() {
        let board = play_all(&["e4", "e5", "Nf3", "Nc6", "Bc4", "Bc5"]);
        let mv = san_to_move(&board, "O-O").unwrap();
        // Castling words are king-from → rook-square (ADR-003 D3).
        assert_eq!(mv.from(), Square::from_alg("e1").unwrap());
        assert_eq!(mv.to(), Square::from_alg("h1").unwrap());
        assert_eq!(move_to_san(&board, mv).unwrap().as_str(), "O-O");
        // Zero notation accepted.
        assert!(san_to_move(&board, "0-0").is_some());
        // Queenside: word e1a1, renders O-O-O.
        let board = play_all(&["d4", "d5", "Nc3", "Nf6", "Bf4", "e6", "Qd2", "Bd6"]);
        let mv = san_to_move(&board, "O-O-O").unwrap();
        assert_eq!(mv.to(), Square::from_alg("a1").unwrap());
        assert_eq!(move_to_san(&board, mv).unwrap().as_str(), "O-O-O");
    }

    #[test]
    fn promotion_san() {
        let board = crate::fen::parse_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let mv = san_to_move(&board, "a8=Q").unwrap();
        assert_eq!(mv.promotion(), Some(Role::Queen));
        let mv = san_to_move(&board, "a8=N").unwrap();
        assert_eq!(mv.promotion(), Some(Role::Knight));
        assert_eq!(
            move_to_san(&board, san_to_move(&board, "a8=Q").unwrap())
                .unwrap()
                .as_str(),
            "a8=Q+"
        );
        // Promotion bit is mandatory on the last rank.
        assert!(san_to_move(&board, "a8").is_none());
    }

    #[test]
    fn en_passant_san() {
        let board = play_all(&["e4", "Nf6", "e5", "d5"]);
        let mv = san_to_move(&board, "exd6").unwrap();
        assert_eq!(mv.to(), Square::from_alg("d6").unwrap());
        assert_eq!(
            move_to_san(&board, mv).unwrap().trim_end_matches(['+', '#']),
            "exd6"
        );
    }

    #[test]
    fn opera_game_plays_legally() {
        // Morphy vs Duke of Brunswick and Count Isouard, Paris 1858.
        let game: &[&str] = &[
            "e4", "e5", "Nf3", "d6", "d4", "Bg4", "dxe5", "Bxf3", "Qxf3", "dxe5", "Bc4", "Nf6",
            "Qb3", "Qe7", "Nc3", "c6", "Bg5", "b5", "Nxb5", "cxb5", "Bxb5+", "Nbd7", "O-O-O",
            "Rd8", "Rxd7", "Rxd7", "Rd1", "Qe6", "Bxd7+", "Nxd7", "Qb8+", "Nxb8", "Rd8#",
        ];
        let mut board = Board::startpos();
        play_san(&mut board, game).expect("opera game must be fully legal");
        assert_eq!(board.turn(), Color::Black);
        assert!(board.in_check());
        // Every move must round-trip through the renderer.
        let mut board = Board::startpos();
        for san in game {
            let mv = san_to_move(&board, san).unwrap();
            let rendered = move_to_san(&board, mv).unwrap();
            assert_eq!(
                rendered.trim_end_matches(['+', '#']),
                san.trim_end_matches(['+', '#'])
            );
            board.play(mv).unwrap();
        }
    }
}
