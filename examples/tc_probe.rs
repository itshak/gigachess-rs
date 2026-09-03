// Differential-testing probe: reads one command per stdin line, prints one
// result line. Used by scripts/diff_python_chess.py (python-chess oracle).
//
// Commands:
//   moves <fen>     - legal moves as raw words (king→rook castling), space-sep
//   perft <fen> <d> - perft node count
//   hash <fen>      - Polyglot zobrist hash (hex)
//   fen <fen>       - round-tripped FEN
//   play <fen> <words..> - FEN after playing the given raw words
//
// SPDX-License-Identifier: MIT
use std::io::{self, BufRead, Write};

use gigachess::fen::parse_fen;
use gigachess::moves::Move;
use gigachess::san;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let result = match cmd {
            "moves" | "perft" | "hash" | "fen" | "play" => {
                let fen_parts: Vec<&str> = parts.by_ref().take(6).collect();
                let fen = fen_parts.join(" ");
                match parse_fen(&fen) {
                    Ok(mut board) => match cmd {
                        "moves" => board
                            .legal_moves()
                            .iter()
                            .map(|m| m.word().to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                        "perft" => {
                            let d: u32 = parts.next().unwrap_or("1").parse().unwrap_or(1);
                            board.perft(d).to_string()
                        }
                        "hash" => format!("{:x}", board.zobrist()),
                        "fen" => board.to_fen(),
                        "play" => {
                            let mut ok = true;
                            for w in parts {
                                let word: u16 = match w.parse() {
                                    Ok(v) => v,
                                    Err(_) => {
                                        ok = false;
                                        break;
                                    }
                                };
                                if board.play(Move::from_word(word)).is_err() {
                                    ok = false;
                                    break;
                                }
                            }
                            if ok { board.to_fen() } else { "illegal".into() }
                        }
                        _ => unreachable!(),
                    },
                    Err(e) => format!("error: {}", e),
                }
            }
            "san" => {
                // san <fen> <word>: SAN rendering of a legal move.
                let fen: Vec<&str> = parts.by_ref().take(6).collect();
                let word: u16 = match parts.next().and_then(|w| w.parse().ok()) {
                    Some(w) => w,
                    None => {
                        writeln!(out, "error: bad word").unwrap();
                        continue;
                    }
                };
                match parse_fen(&fen.join(" ")) {
                    Ok(board) => match san::move_to_san(&board, Move::from_word(word)) {
                        Some(s) => s.as_str().to_string(),
                        None => "error: illegal".into(),
                    },
                    Err(e) => format!("error: {}", e),
                }
            }
            "quit" => break,
            _ => "error: unknown command".to_string(),
        };
        writeln!(out, "{}", result).unwrap();
        out.flush().unwrap();
    }
}
