// Database batch codecs: movetext <-> moves2 conversion, incremental hash
// replay streams, and position-statistics building for high-throughput
// database workloads (blind-base and similar; spec
// turbochess-rs-database-codecs).
//
// The codecs are shaped exactly on blind-base's `gigabase_moves.rs` call
// sites: `parse_movetext_to_moves2` replaces the O(n^2) FEN-round-trip
// replay loop, `moves2_to_san_movetext` replaces per-ply legal-movegen +
// linear-scan decoding (words decode in O(1)), and `replay_moves2_hashes`
// replaces from-scratch Zobrist recomputation with incremental hashes.
//
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::moves::Move;
use crate::san;

/// Error from the movetext codecs: the ply at which parsing/playing failed
/// and the offending token (empty for a replay legality failure).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CodecError {
    pub ply: usize,
    pub token: String,
}

impl core::fmt::Display for CodecError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.token.is_empty() {
            write!(f, "illegal move at ply {}", self.ply)
        } else {
            write!(f, "unparseable token {:?} at ply {}", self.token, self.ply)
        }
    }
}
impl std::error::Error for CodecError {}

/// True for PGN result tokens (not moves).
fn is_result_token(tok: &[u8]) -> bool {
    tok == b"1-0" || tok == b"0-1" || tok == b"1/2-1/2" || tok == b"*"
}

/// Parses PGN movetext into a little-endian `moves2` byte stream (2 bytes per
/// move), starting from `start_fen`.
///
/// Byte-level tokenizer: move numbers ("12." / "12..."), comments `{...}`,
/// line comments `;`, NAGs `$n`, nested variations `(...)` and result tokens
/// are skipped; every remaining SAN token is resolved against the current
/// position. Invalid SAN is an error.
pub fn parse_movetext_to_moves2(
    start_fen: &str,
    movetext: &str,
) -> Result<Vec<u8>, CodecError> {
    let mut board = crate::fen::parse_fen(start_fen)
        .map_err(|e| CodecError { ply: 0, token: format!("bad start FEN: {}", e) })?;
    let mut out = Vec::with_capacity(movetext.len());
    let b = movetext.as_bytes();
    let mut i = 0usize;
    let mut ply = 0usize;

    while i < b.len() {
        let c = b[i];
        match c {
            b' ' | b'\t' | b'\r' | b'\n' => i += 1,
            b'{' => {
                // Comment: no nesting per PGN.
                while i < b.len() && b[i] != b'}' {
                    i += 1;
                }
                i += 1; // closing brace (or end)
            }
            b';' => {
                // Line comment to end of line.
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'(' => {
                // Variation: skip with nesting, honoring comments inside.
                let mut depth = 1usize;
                i += 1;
                while i < b.len() && depth > 0 {
                    match b[i] {
                        b'(' => depth += 1,
                        b')' => depth -= 1,
                        b'{' => {
                            while i < b.len() && b[i] != b'}' {
                                i += 1;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
            b'$' => {
                // NAG: skip the token.
                while i < b.len() && !b[i].is_ascii_whitespace() {
                    i += 1;
                }
            }
            b'.' | b'*' => {
                i += 1;
            }
            b'0'..=b'9' => {
                // Digit-led tokens are move numbers or results.
                let start = i;
                while i < b.len() && !b[i].is_ascii_whitespace() {
                    i += 1;
                }
                if is_result_token(&b[start..i]) {
                    continue;
                }
                // Move number: consume digits and attached dots.
                let mut j = start;
                while j < b.len() && (b[j].is_ascii_digit() || b[j] == b'.') {
                    j += 1;
                }
                i = j;
            }
            _ => {
                // SAN token: whitespace-delimited (annotations +/#/!? are
                // stripped by the SAN parser).
                let start = i;
                while i < b.len() && !b[i].is_ascii_whitespace() {
                    i += 1;
                }
                let tok = &b[start..i];
                if tok.is_empty() {
                    i += 1;
                    continue;
                }
                let san_str = core::str::from_utf8(tok).map_err(|_| CodecError {
                    ply,
                    token: String::from_utf8_lossy(tok).into_owned(),
                })?;
                let mv = san::san_to_move(&board, san_str).ok_or_else(|| CodecError {
                    ply,
                    token: san_str.to_string(),
                })?;
                out.extend_from_slice(&mv.word().to_le_bytes());
                board.play(mv).map_err(|_| CodecError { ply, token: String::new() })?;
                ply += 1;
            }
        }
    }
    Ok(out)
}

/// Renders a little-endian `moves2` byte stream as SAN movetext with move
/// numbers and a trailing result token, e.g. `1. e4 e5 2. Nf3 1-0`.
///
/// Each word decodes in O(1); SAN rendering (disambiguation + check/mate
/// suffixes) matches shakmaty's `SanPlus` byte-for-byte (differential-tested).
pub fn moves2_to_san_movetext(
    start_fen: &str,
    moves2: &[u8],
    result: &str,
) -> Result<String, CodecError> {
    let mut board = crate::fen::parse_fen(start_fen)
        .map_err(|e| CodecError { ply: 0, token: format!("bad start FEN: {}", e) })?;
    let mut out = String::with_capacity(moves2.len() * 3 + 8);
    for (i, pair) in moves2.chunks_exact(2).enumerate() {
        let word = u16::from_le_bytes([pair[0], pair[1]]);
        let mv = Move::from_word(word);
        let rendered = san::move_to_san(&board, mv).ok_or(CodecError {
            ply: i,
            token: mv.to_string(),
        })?;
        if board.turn() == crate::types::Color::White {
            out.push_str(&board.fullmove_number().to_string());
            out.push_str(". ");
        } else if i == 0 {
            out.push_str(&board.fullmove_number().to_string());
            out.push_str("... ");
        }
        out.push_str(rendered.as_str());
        out.push(' ');
        board.play(mv).map_err(|_| CodecError { ply: i, token: String::new() })?;
    }
    if out.ends_with(' ') {
        out.pop();
    }
    if !result.is_empty() {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(result);
    }
    Ok(out)
}

/// Replays a `moves2` word stream incrementally, yielding `(hash, ply)` for
/// every position from the start position (ply 0) through the final position
/// (ply = number of moves played). Hashes are the incrementally-maintained
/// Polyglot values (O(1) per ply, Pseudo en-passant condition) — never
/// recomputed. Returns the ply of the first illegal move on error.
pub fn replay_moves2_hashes(
    start_fen: &str,
    words: &[u16],
) -> Result<Vec<(u64, u32)>, CodecError> {
    let mut board = crate::fen::parse_fen(start_fen)
        .map_err(|e| CodecError { ply: 0, token: format!("bad start FEN: {}", e) })?;
    let mut out = Vec::with_capacity(words.len() + 1);
    out.push((board.zobrist(), 0));
    for (ply, &word) in words.iter().enumerate() {
        board
            .play(Move::from_word(word))
            .map_err(|_| CodecError { ply, token: String::new() })?;
        out.push((board.zobrist(), (ply + 1) as u32));
    }
    Ok(out)
}

/// [`replay_moves2_hashes`] over raw PGN movetext.
pub fn replay_movetext_hashes(
    start_fen: &str,
    movetext: &str,
) -> Result<Vec<(u64, u32)>, CodecError> {
    let bytes = parse_movetext_to_moves2(start_fen, movetext)?;
    let words: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|p| u16::from_le_bytes([p[0], p[1]]))
        .collect();
    replay_moves2_hashes(start_fen, &words)
}

/// Aggregated statistics for one position hash.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MoveStats {
    /// Total number of times the position was reached.
    pub count: u64,
    /// Smallest (game, ply) witness, ordered lexicographically.
    pub sample_game: Option<u32>,
    pub sample_ply: u32,
}

impl MoveStats {
    fn record(&mut self, game: u32, ply: u32) {
        self.count += 1;
        let better = match (self.sample_game, self.sample_ply) {
            (None, _) => true,
            (Some(g), p) => (game, ply) < (g, p),
        };
        if better {
            self.sample_game = Some(game);
            self.sample_ply = ply;
        }
    }
}

/// Builds per-position statistics over a batch of games (all from
/// `start_fen`), parallelized across CPU cores on Rayon's work-stealing pool.
/// Counts per position include the start position (ply 0).
///
/// Results are identical to a sequential reference: counts sum over all
/// games and samples are the lexicographically smallest (game, ply).
pub fn position_stats(start_fen: &str, games: &[&[u16]]) -> HashMap<u64, MoveStats> {
    use rayon::prelude::*;
    games
        .par_iter()
        .enumerate()
        .fold(HashMap::new, |mut local: HashMap<u64, MoveStats>, (gi, game)| {
            let gi = gi as u32;
            if let Ok(mut board) = crate::fen::parse_fen(start_fen) {
                local.entry(board.zobrist()).or_default().record(gi, 0);
                for (ply, &word) in game.iter().enumerate() {
                    if board.play(Move::from_word(word)).is_err() {
                        break;
                    }
                    local
                        .entry(board.zobrist())
                        .or_default()
                        .record(gi, (ply + 1) as u32);
                }
            }
            local
        })
        .reduce(HashMap::new, |mut a, b| {
            for (h, stats) in b {
                let entry = a.entry(h).or_default();
                entry.count += stats.count;
                let better = match (entry.sample_game, stats.sample_game) {
                    (_, None) => false,
                    (None, Some(_)) => true,
                    (Some(g1), Some(g2)) => (g2, stats.sample_ply) < (g1, entry.sample_ply),
                };
                if better {
                    entry.sample_game = stats.sample_game;
                    entry.sample_ply = stats.sample_ply;
                }
            }
            a
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    const OPERA: &str = "1. e4 e5 2. Nf3 d6 3. d4 Bg4 4. dxe5 Bxf3 5. Qxf3 dxe5 6. Bc4 Nf6 \
        7. Qb3 Qe7 8. Nc3 c6 9. Bg5 b5 10. Nxb5 cxb5 11. Bxb5+ Nbd7 12. O-O-O Rd8 \
        13. Rxd7 Rxd7 14. Rd1 Qe6 15. Bxd7+ Nxd7 16. Qb8+ Nxb8 17. Rd8# 1-0";

    fn start_fen() -> String {
        Board::startpos().to_fen()
    }

    fn to_words(bytes: &[u8]) -> Vec<u16> {
        bytes
            .chunks_exact(2)
            .map(|p| u16::from_le_bytes([p[0], p[1]]))
            .collect()
    }

    #[test]
    fn movetext_parse_and_render_round_trip() {
        let bytes = parse_movetext_to_moves2(&start_fen(), OPERA).expect("opera game parses");
        assert_eq!(bytes.len() % 2, 0);
        assert_eq!(to_words(&bytes).len(), 33);

        let rendered = moves2_to_san_movetext(&start_fen(), &bytes, "1-0").unwrap();
        // Rendering must re-parse to the identical moves2 stream.
        let reparsed = parse_movetext_to_moves2(&start_fen(), &rendered).unwrap();
        assert_eq!(reparsed, bytes);
        assert!(rendered.ends_with("Rd8# 1-0"));
        assert!(rendered.starts_with("1. e4 e5 2. Nf3 d6"));
    }

    #[test]
    fn tokenizer_tolerates_comments_nags_and_variations() {
        let messy = "{opening} 1.e4 {best} $1 (1.d4 d5 (1.c4)) 1... e5 2. ; line comment
            Nf3 Nc6 3. Bb5 {[%clk 0:03:00]} a6 1/2-1/2";
        let clean = "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
        let a = parse_movetext_to_moves2(&start_fen(), messy).unwrap();
        let b = parse_movetext_to_moves2(&start_fen(), clean).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn illegal_san_is_an_error() {
        let err = parse_movetext_to_moves2(&start_fen(), "1. e4 e5 2. Qh6").unwrap_err();
        assert_eq!(err.token, "Qh6");
        assert_eq!(err.ply, 2);
    }

    #[test]
    fn black_to_move_start_renders_ellipsis() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        // First move belongs to Black: renders with the "1..." prefix.
        let bytes = parse_movetext_to_moves2(fen, "1... e5").unwrap();
        let rendered = moves2_to_san_movetext(fen, &bytes, "").unwrap();
        assert_eq!(rendered, "1... e5");
    }

    #[test]
    fn hash_replay_matches_from_scratch_recompute() {
        let bytes = parse_movetext_to_moves2(&start_fen(), OPERA).unwrap();
        let words = to_words(&bytes);
        let stream = replay_moves2_hashes(&start_fen(), &words).unwrap();
        assert_eq!(stream.len(), words.len() + 1);
        // From-scratch recompute at every ply.
        let mut board = Board::startpos();
        assert_eq!(stream[0], (board.zobrist(), 0));
        for (i, &w) in words.iter().enumerate() {
            board.play(Move::from_word(w)).unwrap();
            assert_eq!(stream[i + 1], (board.zobrist(), (i + 1) as u32));
            assert_eq!(stream[i + 1].0, board.zobrist_full());
        }
    }

    #[test]
    fn hash_replay_rejects_illegal_moves() {
        let words = [Move::new(
            crate::types::Square(0),
            crate::types::Square(0),
            None,
        )
        .word()];
        assert!(replay_moves2_hashes(&start_fen(), &words).is_err());
    }

    #[test]
    fn position_stats_parallel_matches_sequential() {
        let games: Vec<Vec<u16>> = (0..64u32)
            .map(|g| {
                let mut board = Board::startpos();
                let mut state = 0xB00B_0000 ^ ((g as u64) * 0x9E37);
                let mut words = Vec::new();
                for _ in 0..40 {
                    let legal = board.legal_moves();
                    if legal.is_empty() {
                        break;
                    }
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    let mv = legal[(state % legal.len() as u64) as usize];
                    words.push(mv.word());
                    board.play(mv).unwrap();
                }
                words
            })
            .collect();
        let refs: Vec<&[u16]> = games.iter().map(|g| g.as_slice()).collect();

        let par = position_stats(&start_fen(), &refs);

        // Sequential reference.
        let mut seq: HashMap<u64, MoveStats> = HashMap::new();
        for (gi, game) in refs.iter().enumerate() {
            let mut board = Board::startpos();
            seq.entry(board.zobrist()).or_default().record(gi as u32, 0);
            for (ply, &w) in game.iter().enumerate() {
                board.play(Move::from_word(w)).unwrap();
                seq.entry(board.zobrist()).or_default().record(gi as u32, (ply + 1) as u32);
            }
        }
        assert_eq!(par, seq);
        let total: u64 = par.values().map(|s| s.count).sum();
        let plies: u64 = refs.iter().map(|g| g.len() as u64 + 1).sum();
        assert_eq!(total, plies);
    }
}
