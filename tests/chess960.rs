// Chess960 validation: castling rights as rook squares, FEN dialects,
// path-based castling legality, moves2 king-to-rook encoding, rook-file
// hashing, Board Copy semantics and pseudo-legal filter equivalence.
// Perft references verified against python-chess (scripts/diff_python_chess.py).
//
// SPDX-License-Identifier: MIT

use turbochess_rs::fen::parse_fen;
use turbochess_rs::moves::Move;
use turbochess_rs::types::{castle_right_bit, Color, Role, Square};
use turbochess_rs::Board;

fn castle_move(board: &Board, kingside: bool) -> Move {
    let us = board.turn();
    let rb = castle_right_bit(us, kingside);
    Move::new(board.king_square(us), board.castling_rook_square(rb), None)
}

#[test]
fn standard_rights_map_to_files_a_and_h() {
    let board = parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
        .unwrap();
    assert_eq!(board.castling_rights(), 0x0F);
    assert_eq!(board.castling_rook_square(0), Square(7)); // WK  h1
    assert_eq!(board.castling_rook_square(1), Square(0)); // WQ  a1
    assert_eq!(board.castling_rook_square(2), Square(63)); // BK h8
    assert_eq!(board.castling_rook_square(3), Square(56)); // BQ a8
    assert_eq!(
        board.to_fen(),
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    );
}

#[test]
fn chess960_shredder_fen_round_trip() {
    // Chess960 start position 284: king d1, rooks c1/f1 (files c/f, not a/h).
    let fen = "nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1";
    let board = parse_fen(fen).unwrap();
    assert_eq!(board.to_fen(), fen);
    assert_eq!(
        board.castling_rook_square(castle_right_bit(Color::White, true)),
        Square(5) // f1
    );
    assert_eq!(
        board.castling_rook_square(castle_right_bit(Color::White, false)),
        Square(2) // c1
    );
    assert_eq!(
        board.castling_rook_square(castle_right_bit(Color::Black, true)),
        Square(61) // f8
    );
    assert_eq!(
        board.castling_rook_square(castle_right_bit(Color::Black, false)),
        Square(58) // c8
    );
}

#[test]
fn chess960_100_position_fen_round_trip_byte_equal() {
    // Close-gap task 4.1 gate: 100 deterministic Chess960 start positions
    // (LCG-seeded Scharnagl-style placement: bishops on opposite colors,
    // king between rooks) must round-trip parse → render byte-equal with
    // matching full zobrist — validates the compact 144B Board layout.
    let mut state = 0x9E37_79B9_97F4_7C15u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for i in 0..100 {
        let mut rank = [b'.'; 8];
        // Bishops on opposite colors.
        loop {
            let light = (next() % 4) as usize * 2; // even files: a, c, e, g
            let dark = (next() % 4) as usize * 2 + 1; // odd files: b, d, f, h
            if light != dark {
                rank[light] = b'B';
                rank[dark] = b'B';
                break;
            }
        }
        let free: Vec<usize> = (0..8).filter(|&f| rank[f] == b'.').collect();
        // Queen on a random empty square.
        let q = free[(next() % free.len() as u64) as usize];
        rank[q] = b'Q';
        let free: Vec<usize> = (0..8).filter(|&f| rank[f] == b'.').collect();
        // Two knights.
        let n1 = free[(next() % free.len() as u64) as usize];
        rank[n1] = b'N';
        let free: Vec<usize> = (0..8).filter(|&f| rank[f] == b'.').collect();
        let n2 = free[(next() % free.len() as u64) as usize];
        rank[n2] = b'N';
        // Remaining three squares: R, K, R in order.
        let rest: Vec<usize> = (0..8).filter(|&f| rank[f] == b'.').collect();
        assert_eq!(rest.len(), 3, "960 placement leaves exactly R,K,R");
        rank[rest[0]] = b'R';
        rank[rest[1]] = b'K';
        rank[rest[2]] = b'R';
        let back: String = rank.iter().map(|&b| b as char).collect();
        let white_rank: String = back.chars().map(|c| c.to_ascii_uppercase()).collect();
        let black_rank = back.to_ascii_lowercase();
        let fen = format!("{black_rank}/pppppppp/8/8/8/8/PPPPPPPP/{white_rank} w KQkq - 0 1");
        let board = parse_fen(&fen).unwrap_or_else(|e| panic!("pos {i}: parse {fen}: {e:?}"));
        assert_eq!(board.to_fen(), fen, "pos {i}: FEN round-trip must be byte-equal");
        assert_eq!(board.zobrist(), board.zobrist_full(), "pos {i}: zobrist parity");
    }
}

#[test]
fn xfen_side_letters_for_unambiguous_960_rights() {
    // python-chess X-FEN: 960 position with king on c1/c8, rooks a/d:
    // unambiguous rights are written with side letters, not file letters.
    let fen = "rnkrqbbn/pppppppp/8/8/8/8/PPPPPPPP/RNKRQBBN w KQkq - 0 1";
    let board = parse_fen(fen).unwrap();
    assert_eq!(board.to_fen(), fen);
    // 'K' = kingside with the outermost rook (d1); 'Q' = a1.
    assert_eq!(
        board.castling_rook_square(castle_right_bit(Color::White, true)),
        Square(3)
    );
    assert_eq!(
        board.castling_rook_square(castle_right_bit(Color::White, false)),
        Square(0)
    );
}

#[test]
fn shredder_and_xfen_notations_yield_identical_rights() {
    // python-chess X-FEN for position 654 (king c1/c8, rooks a/d) is KQkq;
    // the Shredder spelling of the same rights is DAda. Both must parse to
    // identical positions.
    let xfen = "rnkrqbbn/pppppppp/8/8/8/8/PPPPPPPP/RNKRQBBN w KQkq - 0 1";
    let shredder = "rnkrqbbn/pppppppp/8/8/8/8/PPPPPPPP/RNKRQBBN w DAda - 0 1";
    let a = parse_fen(xfen).unwrap();
    let b = parse_fen(shredder).unwrap();
    assert_eq!(a.zobrist(), b.zobrist());
    assert_eq!(a.castling_rights(), b.castling_rights());
    for rb in 0..4u8 {
        assert_eq!(a.castling_rook_square(rb), b.castling_rook_square(rb));
    }
    // Emission is X-FEN (side letters, unambiguous here).
    assert_eq!(a.to_fen(), xfen);
}

#[test]
fn castling_rights_cleared_when_rook_moves() {
    let mut board = parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    // Rook h1 -> h2: white kingside right gone.
    board.play(Move::new(Square(7), Square(15), None)).unwrap();
    assert_eq!(board.castling_rights() & 0x01, 0);
    assert_eq!(board.castling_rights() & 0x0E, 0x0E);
    assert_eq!(board.zobrist(), board.zobrist_full());
}

#[test]
fn adjacent_king_rook_castling_swaps_pieces() {
    // King d1, rook c1: queenside castling swaps them (Kc1, Rd1).
    let mut board = parse_fen("4k3/8/8/8/8/8/8/2RKR3 w Q - 0 1").unwrap();
    let mv = castle_move(&board, false); // king d1 -> rook c1
    assert_eq!(mv.from(), Square(3));
    assert_eq!(mv.to(), Square(2));
    let undo = board.make_move_unchecked(mv);
    assert_eq!(
        board.piece_at(Square(2)),
        Some(turbochess_rs::types::Piece::new(Color::White, Role::King))
    );
    assert_eq!(
        board.piece_at(Square(3)),
        Some(turbochess_rs::types::Piece::new(Color::White, Role::Rook))
    );
    assert_eq!(board.zobrist(), board.zobrist_full());
    board.unmake_move(mv, undo);
    assert_eq!(
        board.piece_at(Square(3)),
        Some(turbochess_rs::types::Piece::new(Color::White, Role::King))
    );
    assert_eq!(
        board.piece_at(Square(2)),
        Some(turbochess_rs::types::Piece::new(Color::White, Role::Rook))
    );
    assert_eq!(board.zobrist(), board.zobrist_full());
}

#[test]
fn moves2_castling_round_trip_standard_and_960() {
    // Standard: O-O word is e1h1 (king -> rook square), not e1g1.
    let mut board = Board::startpos();
    for san in ["e4", "e5", "Nf3", "Nc6", "Bc4", "Bc5"] {
        let mv = turbochess_rs::san::san_to_move(&board, san).unwrap();
        board.play(mv).unwrap();
    }
    let oo = turbochess_rs::san::san_to_move(&board, "O-O").unwrap();
    assert_eq!(oo.from(), Square::from_alg("e1").unwrap());
    assert_eq!(oo.to(), Square::from_alg("h1").unwrap());
    // Decode by destination-own-rook detection: replaying the word castles.
    board.play(oo).unwrap();
    assert_eq!(
        board.king_square(Color::White),
        Square::from_alg("g1").unwrap()
    );
    assert_eq!(
        board.piece_at(Square::from_alg("f1").unwrap()).map(|p| p.role),
        Some(Role::Rook)
    );
    assert_eq!(board.zobrist(), board.zobrist_full());

    // Chess960: king d1, adjacent kingside rook e1 -> castling word d1e1
    // (a normal king step can never land on an own rook, so this decodes
    // unambiguously). King ends on g1, rook on f1.
    let mut board = parse_fen("4k3/8/8/8/8/8/8/3KR3 w K - 0 1").unwrap();
    let mv = turbochess_rs::san::san_to_move(&board, "O-O").unwrap();
    assert_eq!(mv.from(), Square::from_alg("d1").unwrap());
    assert_eq!(mv.to(), Square::from_alg("e1").unwrap());
    board.play(mv).unwrap();
    assert_eq!(
        board.king_square(Color::White),
        Square::from_alg("g1").unwrap()
    );
    assert_eq!(
        board.piece_at(Square::from_alg("f1").unwrap()).map(|p| p.role),
        Some(Role::Rook)
    );
    assert_eq!(board.zobrist(), board.zobrist_full());
}

#[test]
fn chess960_perft_reference_positions() {
    // References computed with python-chess (from_chess960_pos), depth 3.
    let cases: [(&str, u64); 4] = [
        ("bbqnnrkr/pppppppp/8/8/8/8/PPPPPPPP/BBQNNRKR w KQkq - 0 1", 9006), // pos 0
        ("nbrknrbq/pppppppp/8/8/8/8/PPPPPPPP/NBRKNRBQ w KQkq - 0 1", 8950), // pos 284
        ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 8902), // pos 518
        ("rkrnnqbb/pppppppp/8/8/8/8/PPPPPPPP/RKRNNQBB w KQkq - 0 1", 9006), // pos 959
    ];
    for (fen, expected) in cases {
        let board = parse_fen(fen).unwrap();
        assert_eq!(board.perft(3), expected, "960 perft mismatch for {fen}");
    }
}

#[test]
fn pinned_ep_hash_included_pseudo_condition() {
    // Polyglot/Pseudo condition: the ep key is XORed even when the only
    // capturer is pinned (hash would differ under shakmaty's Legal mode).
    let board = parse_fen("8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1").unwrap();
    assert_eq!(format!("{:x}", board.zobrist()), "83bf25e378cb17d0");
    assert_eq!(board.zobrist(), board.zobrist_full());
}

#[test]
fn standard_chess_polyglot_vectors() {
    let start = Board::startpos();
    assert_eq!(format!("{:x}", start.zobrist()), "463b96181691fc9c");
    let kiwipete =
        parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    assert_eq!(format!("{:x}", kiwipete.zobrist()), "c3ce103f01d15e1d");
}

#[test]
fn board_copy_is_bit_for_bit_snapshot() {
    let board = parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    let snapshot = board; // Copy semantics
    let mut moved = board;
    moved
        .play(Move::new(Square(4), Square(7), None)) // e1 -> h1: castling word
        .unwrap();
    // The copy is untouched.
    assert_eq!(snapshot.to_fen(), "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    assert_eq!(snapshot.zobrist(), snapshot.zobrist_full());
    // The move was a real castling (king ends on g1).
    assert_eq!(moved.king_square(Color::White), Square::from_alg("g1").unwrap());
}

#[test]
fn pseudo_legal_filter_equivalence() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        // 960 with adjacent king/rook (swap castling).
        "nnrkrbbq/pppppppp/8/8/8/8/PPPPPPPP/NNRKRBBQ w KQkq - 0 1",
        "bbnnrkrq/pppppppp/8/8/8/8/PPPPPPPP/BBNNRKRQ w KQkq - 0 1",
        // Pinned-ep position.
        "8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1",
    ];
    for fen in fens {
        let mut board = parse_fen(fen).unwrap();
        for _ in 0..4 {
            let legal: std::collections::HashSet<u16> =
                board.legal_moves().iter().map(|m| m.word()).collect();
            let pseudo: Vec<Move> = board.pseudo_legal_moves().iter().copied().collect();
            // Every legal move is pseudo-legal.
            for m in &legal {
                assert!(
                    pseudo.iter().any(|p| p.word() == *m),
                    "{fen}: legal move {} not pseudo-legal",
                    Move::from_word(*m)
                );
            }
            // King-safety filtering pseudo-legal yields exactly the legal set.
            let mut filtered = std::collections::HashSet::new();
            for m in pseudo {
                let mut child = board;
                let mover = child.turn();
                child.make_move_unchecked(m);
                if !child.king_attacked(mover) {
                    filtered.insert(m.word());
                }
            }
            assert_eq!(filtered, legal, "filter equivalence failed for {fen}");
            // Advance one ply.
            let moves = board.legal_moves();
            if moves.is_empty() {
                break;
            }
            board.play(moves[0]).unwrap();
        }
    }
}
