// Generates `tests/data/samplefen1000.epd`: 1000 deterministic FEN positions
// sampled from seeded random playouts (including castling, en passant and
// promotion traffic). Run from the repo root:
//   cargo run --release --example generate_samplefen
//
// SPDX-License-Identifier: MIT

use turbochess_rs::Board;

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn main() {
    let mut state = 0xF00D_BEEF_CAFE_0001u64;
    let mut fens: Vec<String> = Vec::new();
    fens.push(Board::startpos().to_fen());
    while fens.len() < 1000 {
        let mut board = Board::startpos();
        // Sample several positions from each playout.
        for _ in 0..120 {
            let legal = board.legal_moves();
            if legal.is_empty() {
                break;
            }
            let mv = legal[(xorshift(&mut state) % legal.len() as u64) as usize];
            board.play(mv).expect("legal move");
            if fens.len() < 1000 && xorshift(&mut state).is_multiple_of(4) {
                fens.push(board.to_fen());
            }
        }
    }

    let out = "tests/data/samplefen1000.epd";
    std::fs::create_dir_all("tests/data").unwrap();
    std::fs::write(out, fens.join("\n") + "\n").unwrap();
    println!("wrote {} positions to {}", fens.len(), out);
}
