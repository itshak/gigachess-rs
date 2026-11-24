// Cross-language FIDE repetition/Polyglot distinction; same FENs as the JS
// maintenance-regression suite. Do not change Polyglot key semantics.
use gigachess::fen::parse_fen;
use gigachess::moves::Move;
use gigachess::{Board, Square};

fn sq(name: &str) -> Square {
    Square::from_alg(name).unwrap()
}

#[test]
fn polyglot_hash_stays_unchanged_but_ep_repetition_is_legal_only() {
    let without = parse_fen("k3r3/8/8/3pP3/8/8/8/4K3 w - - 0 1").unwrap();
    let with = parse_fen("k3r3/8/8/3pP3/8/8/8/4K3 w - d6 0 1").unwrap();
    assert_ne!(
        with.zobrist(),
        without.zobrist(),
        "Polyglot includes pseudo-legal EP"
    );
    assert_eq!(
        with.repetition_key(),
        without.repetition_key(),
        "FIDE repetition ignores pinned EP"
    );
    let legal_ep = parse_fen("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1").unwrap();
    let legal_no = parse_fen("4k3/8/8/3pP3/8/8/8/4K3 w - - 0 1").unwrap();
    assert_ne!(legal_ep.repetition_key(), legal_no.repetition_key());
    let phantom = parse_fen("8/8/8/8/P7/8/8/K6k b - a3 0 1").unwrap();
    let none = parse_fen("8/8/8/8/P7/8/8/K6k b - - 0 1").unwrap();
    assert_eq!(phantom.repetition_key(), none.repetition_key());
}

#[test]
fn packed_move_castling_and_polyglot_start_are_preserved() {
    let mut board = Board::startpos();
    assert_eq!(board.repetition_key(), 0x463b_9618_1691_fc9c);
    let e4 = Move::new(sq("e2"), sq("e4"), None);
    assert_eq!(e4.word(), 12 | (28 << 6));
    board.play(e4).unwrap();
    assert_eq!(board.zobrist(), 0x823c_9b50_fd11_4196);
    let castle = parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    assert!(castle
        .legal_moves()
        .iter()
        .any(|m| m.from() == sq("e1") && m.to() == sq("h1")));
    assert!(castle
        .legal_moves()
        .iter()
        .any(|m| m.from() == sq("e1") && m.to() == sq("a1")));
}

#[test]
fn fen_rejects_extra_fields_bad_side_and_opposite_check() {
    let base = "4k3/8/8/8/8/8/8/4K3";
    assert!(parse_fen(&format!("{base} white - - 0 1")).is_err());
    assert!(parse_fen(&format!("{base} w - - 0 1 extra")).is_err());
    let impossible = "4k3/8/8/8/8/8/4R3/4K3 w - - 0 1";
    assert!(parse_fen(impossible).is_err());
    assert!(gigachess::fen::parse_fen_allow_illegal(impossible).is_ok());
    assert!(parse_fen(&format!("{base} w - - 200 1")).is_ok());
}

#[test]
fn js_rust_shared_golden_position_keys_and_packed_moves() {
    for line in include_str!("data/position-keys-v1.tsv").lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let id = fields.next().unwrap();
        let _variant = fields.next().unwrap();
        let fen = fields.next().unwrap();
        let polyglot = u64::from_str_radix(fields.next().unwrap(), 16).unwrap();
        let repetition = u64::from_str_radix(fields.next().unwrap(), 16).unwrap();
        let b = parse_fen(fen).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(b.zobrist(), polyglot, "{id}: polyglot");
        assert_eq!(b.repetition_key(), repetition, "{id}: repetition");
    }
    for line in include_str!("data/moves2-v1.tsv").lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let id = fields.next().unwrap();
        let from = fields.next().unwrap().parse::<u8>().unwrap();
        let to = fields.next().unwrap().parse::<u8>().unwrap();
        let promotion = fields.next().unwrap().parse::<u8>().unwrap();
        let expected = u16::from_str_radix(fields.next().unwrap(), 16).unwrap();
        let role = match promotion {
            0 => None,
            1 => Some(gigachess::Role::Knight),
            2 => Some(gigachess::Role::Bishop),
            3 => Some(gigachess::Role::Rook),
            4 => Some(gigachess::Role::Queen),
            15 => None,
            _ => panic!("unexpected promotion {id}"),
        };
        let actual = if id == "null" {
            Move::NULL.word()
        } else {
            Move::new(Square(from), Square(to), role).word()
        };
        assert_eq!(actual, expected, "{id}");
    }
}
