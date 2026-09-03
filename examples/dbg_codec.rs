use shakmaty::{fen::Fen, CastlingMode, Chess, Position, Square as SSquare};
use gigachess::{database, Board};

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn main() {
    let start_fen = Board::startpos().to_fen();
    let mut state = 0xBE11_0000_C0DE_0001u64;
    for game in 0..40 {
    let mut board = Board::startpos();
    let mut words: Vec<u16> = Vec::new();
    for _ in 0..100 {
        let legal = board.legal_moves();
        if legal.is_empty() {
            break;
        }
        let mv = legal[(xorshift(&mut state) % legal.len() as u64) as usize];
        words.push(mv.word());
        board.play(mv).unwrap();
    }
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let movetext = database::moves2_to_san_movetext(&start_fen, &bytes, "*").unwrap();

    // Re-import with turbochess (must succeed).
    let reimported = database::parse_movetext_to_moves2(&start_fen, &movetext).unwrap();
    assert_eq!(reimported, bytes);

    // Now replay with shakmaty word-decode and find the failure.
    let mut pos: Chess = Fen::from_ascii(start_fen.as_bytes())
        .unwrap()
        .into_position(CastlingMode::Standard)
        .unwrap();
    for (ply, pair) in bytes.chunks_exact(2).enumerate() {
        let word = u16::from_le_bytes([pair[0], pair[1]]);
        let from = SSquare::new((word & 0x3f) as u32);
        let to = SSquare::new(((word >> 6) & 0x3f) as u32);
        let list = pos.legal_moves();
        let found = list
            .iter()
            .find(|m| m.from() == Some(from) && m.to() == to);
        match found {
            Some(mv) => {
                pos = pos.play(*mv).unwrap();
            }
            None => {
                println!("game {}: ply {}: word {} (from {:?} to {:?}) not found", game, ply, word, from, to);
                println!("FEN before: {}", {
                    use shakmaty::EnPassantMode;
                    shakmaty::fen::Fen::from_position(&pos, EnPassantMode::Always).to_string()
                });
                println!("piece on from (shakmaty): {:?}", pos.board().piece_at(from));
                println!("turbochess FEN before: {}", {
                    let mut b = Board::startpos();
                    for &w in words.iter().take(ply) {
                        use gigachess::Move;
                        b.play(Move::from_word(w)).unwrap();
                    }
                    b.to_fen()
                });
                return;
            }
        }
    }
    }
    println!("all games decoded");
}
