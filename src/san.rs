// Zero-allocation SAN parser and disambiguator.
//
// `move_to_san` renders a legal move with minimal disambiguation plus
// check/mate annotations; `san_to_move` parses SAN against a position and
// resolves disambiguation by matching against the legal move list.
//
// SPDX-License-Identifier: MIT

use crate::attacks;
use crate::bitboard::{bit, pop_lsb, KING_ATT, KNIGHT_ATT, PAWN_ATT};
use crate::board::Board;
use crate::moves::Move;
use crate::types::{Color, Role, Square};
use arrayvec::{ArrayString, ArrayVec};

/// Maximum rendered SAN length (longest: "Qh4xe1=Q+" style strings).
pub type San = ArrayString<12>;

/// Renders `mv` (which must be legal in `board`) in SAN.
///
/// Returns `None` if the move is not legal in the position.
///
/// Branchless via `tables::between` (for `SAN` disambig pre-filter) +
/// `make`/`unmake` suffix and `attacks_from_target` pre-filter, copying
/// `ultrachess/src/san.rs:1` `1.43µs/48` path (MIT attribution).
#[inline(always)]
pub fn move_to_san(board: &Board, mv: Move) -> Option<San> {
    let mut out = move_to_san_body(board, mv)?;
    if let Some(c) = check_mate_suffix_after_make(board, mv) {
        out.push(c);
    }
    Some(out)
}

/// Renders everything of `mv`'s SAN except the check/mate suffix: the piece
/// letter, the minimal disambiguation, the capture `x`, the target squares and
/// the promotion — all of which need only the position the move is played from.
///
/// Returns `None` if there is no piece on the move's origin square.
///
/// A caller that already holds the position *after* `mv` — a replay engine, a
/// database exporter, anything whose walk makes the move anyway — appends
/// [`check_mate_suffix`] itself and so pays neither the board copy nor the
/// make/unmake [`move_to_san`] needs to reach that position itself.
/// [`move_to_san`] is exactly this function plus that suffix.
#[inline(always)]
pub fn move_to_san_body(board: &Board, mv: Move) -> Option<San> {
    let from = mv.from();
    let to = mv.to();
    let piece = board.piece_at(from)?;

    // Fast legality: `debug_assert` only in debug, assume legal in release
    // (ultrachess `debug_assert_move_is_legal` — saves 1 `make/unmake` per SAN,
    // `48` `make/unmake` for `SAN 48` bench `→ ~1µs` win).
    debug_assert!(
        board.is_pseudo_legal(mv) && {
            let mut tmp = *board;
            let undo = tmp.make_move_unchecked(mv);
            let ok =
                tmp.attackers_to(tmp.king_square(board.turn()).0, tmp.turn(), tmp.occupied()) == 0;
            tmp.unmake_move(mv, undo);
            ok
        },
        "move_to_san called with illegal move"
    );

    let mut out = San::new();

    // Castling: the destination holds the mover's own rook (ADR-003 D3
    // encoding, valid for standard chess and Chess960 alike).
    let is_castle = piece.role == Role::King
        && board.piece_at(to) == Some(crate::types::Piece::new(piece.color, Role::Rook));
    if is_castle {
        out.push_str(if to.file() > from.file() {
            "O-O"
        } else {
            "O-O-O"
        });
    } else {
        let is_capture = board.piece_at(to).is_some()
            || (piece.role == Role::Pawn
                && board.en_passant() == Some(to)
                && from.file() != to.file());

        if piece.role != Role::Pawn {
            out.push(piece.role.char_upper());
            // Disambiguation via the `attacks_from_target` pre-filter, then a
            // direct question per candidate (ultrachess `san.rs:1` 1.43µs/48).
            //
            // The pre-filter is exact pseudo-legality for the roles that reach
            // here: a knight or king cannot be blocked, and a slider on the set
            // has an unobstructed ray to `to` (the ray stops at the first
            // blocker, so a piece behind one is correctly excluded). A pawn is
            // disambiguated by its file and never gets here. So legality is the
            // one remaining question, and asking it per candidate is cheaper
            // than generating every legal move in the position to reach the same
            // answer: one stack copy, one `make_move_unchecked`, one
            // king-safety query and one `unmake_move` — the same
            // make/unmake the movegen performed, for each of at most eight
            // squares instead of all thirty-five.
            let same_bb = board.piece_bb(piece.color, piece.role);
            let attackers_bb = {
                let occ = board.occupied();
                let att = match piece.role {
                    Role::Knight => KNIGHT_ATT[to.index()],
                    Role::Bishop => attacks::bishop_attacks(to.0, occ),
                    Role::Rook => attacks::rook_attacks(to.0, occ),
                    Role::Queen => attacks::queen_attacks(to.0, occ),
                    Role::King => KING_ATT[to.index()],
                    _ => 0,
                };
                att & same_bb & !bit(from.0)
            };
            if attackers_bb != 0 {
                // The mover, read from the caller's board: a candidate's
                // legality is about *its* king, attacked by the side that moves
                // next, and both values have to be taken before the candidate is
                // made. That is what the `debug_assert` above spells out — a
                // friendly piece on the king's own file defends it, so asking
                // the wrong question drops hints the notation requires.
                let us = board.turn();
                let mut others: ArrayVec<Square, 8> = ArrayVec::new();
                let mut candidates = attackers_bb;
                while candidates != 0 {
                    let sq = Square::new(pop_lsb(&mut candidates));
                    let cm = Move::new(sq, to, mv.promotion());
                    let mut tmp = *board;
                    let undo = tmp.make_move_unchecked(cm);
                    let legal =
                        tmp.attackers_to(tmp.king_square(us).0, us.other(), tmp.occupied()) == 0;
                    tmp.unmake_move(cm, undo);
                    if legal {
                        others.push(sq);
                        if others.len() >= 8 {
                            break;
                        }
                    }
                }
                if !others.is_empty() {
                    let same_file = others.iter().any(|s| s.file() == from.file());
                    let same_rank = others.iter().any(|s| s.rank() == from.rank());
                    if !same_file {
                        out.push((b'a' + from.file()) as char);
                    } else if !same_rank {
                        out.push((b'1' + from.rank()) as char);
                    } else {
                        out.push((b'a' + from.file()) as char);
                        out.push((b'1' + from.rank()) as char);
                    }
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

    Some(out)
}

/// The check/mate suffix of `after` — the position a move leads to, already
/// reached: `Some('#')` when the side to move is checkmated, `Some('+')` when
/// it is in check with a legal reply, `None` when it is not in check.
///
/// `in_check()` is the O(1) `checkers != 0` cache (0.32 ns, D3), so the
/// expensive mate test runs only in check (ultrachess `san.rs:1`
/// `append_check_suffix`).
///
/// The position must have been reached through a make that maintains
/// `checkers` — `make_move_unchecked`, `make_move_perft`, or `play` on top of
/// one. The v0.1.4 fast makes (`make_move_fast`, `play_fast`) deliberately skip
/// the cache, and a stale `checkers` reads as "not in check".
#[inline(always)]
pub fn check_mate_suffix(after: &Board) -> Option<char> {
    if !after.in_check() {
        return None;
    }
    // `has_no_legal_moves` via the MoveCounter bulk path (`count +=
    // popcount`, no `Move` materialisation, close-gap D4 task 5.1) — gated
    // behind the O(1) `in_check()` cache.
    Some(if after.count_legal_moves() == 0 {
        '#'
    } else {
        '+'
    })
}

/// [`check_mate_suffix`] for a move that has not been made yet: makes it on a
/// copy — `make`/`unmake`, not `clone`, gigachess's deliberate make+unmake
/// tradeoff — reads the suffix, unmakes.
#[inline(always)]
fn check_mate_suffix_after_make(board: &Board, mv: Move) -> Option<char> {
    let mut tmp = *board;
    let undo = tmp.make_move_unchecked(mv);
    let suffix = check_mate_suffix(&tmp);
    tmp.unmake_move(mv, undo);
    suffix
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

    let bytes = s.as_bytes();

    // Castling (accept O-O, 0-0, O-O-O, 0-0-0 with or without dashes).
    // The move words are king-from → rook-square (ADR-003 D3); the side is
    // determined by the rook file relative to the king file.
    let mut castle_chars = 0u8;
    let mut is_castle = true;
    for &b in bytes {
        if b == b'O' || b == b'0' {
            castle_chars += 1;
        } else if b != b'-' {
            is_castle = false;
            break;
        }
    }
    if is_castle && (castle_chars == 2 || castle_chars == 3) {
        let kingside = castle_chars == 2;
        let us = board.turn();
        let rb = crate::types::castle_right_bit(us, kingside);
        if board.castling_rights() & (1 << rb) == 0 {
            return None;
        }
        let ksq = board.king_square(us);
        let rsq = board.castling_rook_square(rb);
        let mv = Move::quiet(ksq, rsq);
        if board.is_legal(mv) {
            return Some(mv);
        }
        return None;
    }

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
    } else if bytes.len() >= 3
        && matches!(bytes[bytes.len() - 1], b'N' | b'B' | b'R' | b'Q')
        && bytes[bytes.len() - 2].is_ascii_digit()
    {
        let p = match bytes[bytes.len() - 1] {
            b'N' => Some(Role::Knight),
            b'B' => Some(Role::Bishop),
            b'R' => Some(Role::Rook),
            b'Q' => Some(Role::Queen),
            _ => return None,
        };
        (&bytes[..bytes.len() - 1], p)
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
    let file = rest[rest.len() - 2].wrapping_sub(b'a');
    let rank = rest[rest.len() - 1].wrapping_sub(b'1');
    if file > 7 || rank > 7 {
        return None;
    }
    let to = Square::from_coords(file, rank);
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

    let us = board.turn();
    let them = us.other();
    let occ = board.occupied();
    let piece_bb = board.piece_bb(us, role);

    let mut candidates = match role {
        Role::Knight => KNIGHT_ATT[to.index()] & piece_bb,
        Role::Bishop => attacks::bishop_attacks(to.0, occ) & piece_bb,
        Role::Rook => attacks::rook_attacks(to.0, occ) & piece_bb,
        Role::Queen => attacks::queen_attacks(to.0, occ) & piece_bb,
        Role::King => KING_ATT[to.index()] & piece_bb,
        Role::Pawn => {
            let is_cap = board.piece_at(to).is_some()
                || board.en_passant() == Some(to)
                || prefix.iter().any(|&c| c == b'x' || c == b'X')
                || (hint_file.is_some() && hint_file != Some(to.file()));
            if is_cap {
                PAWN_ATT[them.index()][to.index()] & piece_bb
            } else {
                let white = us == Color::White;
                let mut cand = 0u64;
                if occ & bit(to.0) == 0 {
                    if white {
                        if to.0 >= 8 && (piece_bb & bit(to.0 - 8)) != 0 {
                            cand |= bit(to.0 - 8);
                        } else if to.rank() == 3
                            && (occ & bit(to.0 - 8)) == 0
                            && (piece_bb & bit(to.0 - 16)) != 0
                        {
                            cand |= bit(to.0 - 16);
                        }
                    } else {
                        if to.0 <= 55 && (piece_bb & bit(to.0 + 8)) != 0 {
                            cand |= bit(to.0 + 8);
                        } else if to.rank() == 4
                            && (occ & bit(to.0 + 8)) == 0
                            && (piece_bb & bit(to.0 + 16)) != 0
                        {
                            cand |= bit(to.0 + 16);
                        }
                    }
                }
                cand
            }
        }
    };

    if let Some(f) = hint_file {
        candidates &= crate::types::FILE_BB[f as usize];
    }
    if let Some(r) = hint_rank {
        candidates &= crate::types::RANK_BB[r as usize];
    }

    let mut result: Option<Move> = None;
    while candidates != 0 {
        let from = pop_lsb(&mut candidates);
        let mv = Move::new(Square(from), to, promo);
        if board.is_legal(mv) {
            if result.is_some() {
                return None; // ambiguous
            }
            result = Some(mv);
        }
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
    fn san_body_carries_no_suffix_and_the_suffix_seam_agrees() {
        // `f3 e5 g4` leaves Black one move from mate: Qh4 is `#`.
        let board = play_all(&["f3", "e5", "g4"]);
        let mv = san_to_move(&board, "Qh4").unwrap();
        assert_eq!(move_to_san_body(&board, mv).unwrap().as_str(), "Qh4");
        // `Board` is `Copy`: making the move on the copy left `board` alone.
        let untouched = board;
        let mut after = board;
        after.make_move_unchecked(mv);
        assert_eq!(check_mate_suffix(&after), Some('#'));
        // The monolith is the composition, byte for byte.
        let mut joined = move_to_san_body(&board, mv).unwrap();
        joined.push(check_mate_suffix(&after).expect("mate"));
        assert_eq!(joined, move_to_san(&board, mv).unwrap());
        assert_eq!(untouched, board, "the make did not touch the original");
    }

    #[test]
    fn a_quiet_move_has_no_suffix() {
        let board = play_all(&["e4", "e5", "Nf3"]);
        let mv = san_to_move(&board, "Nc6").unwrap();
        let mut after = board;
        after.make_move_unchecked(mv);
        assert_eq!(check_mate_suffix(&after), None);
        assert_eq!(move_to_san(&board, mv).unwrap().as_str(), "Nc6");
    }

    #[test]
    fn a_check_with_a_reply_is_a_plus() {
        // 1. e4 e5 2. Qh5 Nf6 3. Qxe5+ — the e-file is open and Black can block
        // on e7, so this is a `+` and not a `#`.
        let board = play_all(&["e4", "e5", "Qh5", "Nf6"]);
        let mv = san_to_move(&board, "Qxe5+").unwrap();
        assert_eq!(move_to_san_body(&board, mv).unwrap().as_str(), "Qxe5");
        let mut after = board;
        after.make_move_unchecked(mv);
        assert_eq!(check_mate_suffix(&after), Some('+'));
        assert_eq!(move_to_san(&board, mv).unwrap().as_str(), "Qxe5+");
        assert!(
            !after.legal_moves().is_empty(),
            "the reply is what makes it a `+`"
        );
    }

    #[test]
    fn the_split_composition_holds_for_every_notation_shape() {
        // Castling both sides, en passant and a checking move: the body and the
        // suffix compose into `move_to_san` for each.
        let lines: [&[&str]; 4] = [
            &["e4", "e5", "Nf3", "Nc6", "Bc4", "Bc5", "O-O"],
            &["d4", "d5", "Nc3", "Nf6", "Bf4", "e6", "Qd2", "Bd6", "O-O-O"],
            &["e4", "e6", "e5", "d5", "exd6"],
            &["e4", "e5", "Qh5", "Nf6", "Qxe5"],
        ];
        for line in lines {
            let mut board = Board::startpos();
            for san in line {
                let mv = san_to_move(&board, san).unwrap_or_else(|| panic!("parse {san}"));
                let mut joined = move_to_san_body(&board, mv).unwrap();
                let mut after = board;
                after.make_move_unchecked(mv);
                if let Some(c) = check_mate_suffix(&after) {
                    joined.push(c);
                }
                assert_eq!(joined, move_to_san(&board, mv).unwrap(), "line {san}");
                board = after;
            }
        }
        // A disambiguated move (knights on b2 and d2 both reach c4): the body
        // carries the origin file, the suffix nothing.
        let board = crate::fen::parse_fen("5k2/8/8/8/8/8/1N1N4/4K3 w - - 0 1").unwrap();
        let mv = san_to_move(&board, "Nbc4").unwrap();
        assert_eq!(move_to_san_body(&board, mv).unwrap().as_str(), "Nbc4");
        let mut after = board;
        after.make_move_unchecked(mv);
        assert_eq!(check_mate_suffix(&after), None);
        assert_eq!(move_to_san(&board, mv).unwrap().as_str(), "Nbc4");
        // A promotion with check: the seam must not disturb `=Q`.
        let board = crate::fen::parse_fen("7k/P7/8/8/8/8/8/6K1 w - - 0 1").unwrap();
        let mv = san_to_move(&board, "a8=Q+").unwrap();
        assert_eq!(move_to_san_body(&board, mv).unwrap().as_str(), "a8=Q");
        let mut after = board;
        after.make_move_unchecked(mv);
        assert_eq!(check_mate_suffix(&after), Some('+'));
        assert_eq!(move_to_san(&board, mv).unwrap().as_str(), "a8=Q+");
    }

    #[test]
    fn a_stale_checkers_cache_would_be_a_stale_suffix() {
        // Why the doc comment insists on a checkers-maintaining make:
        // `make_move_fast` leaves `checkers` untouched, so the O(1)
        // `in_check()` a suffix reads is stale — pinned here so the trade-off
        // cannot be quietly lost.
        let board = play_all(&["f3", "e5", "g4"]);
        let mv = san_to_move(&board, "Qh4").unwrap();
        let mut after = board;
        after.make_move_fast(mv);
        assert!(!after.in_check(), "a stale `checkers` reads as no check");
        let mut after = board;
        after.make_move_unchecked(mv);
        assert!(after.in_check());
        assert_eq!(check_mate_suffix(&after), Some('#'));
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
            move_to_san(&board, mv)
                .unwrap()
                .trim_end_matches(['+', '#']),
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
