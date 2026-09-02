// TurboChess-RS: ultra-high-performance, 100% MIT-licensed chess engine,
// PEXT/Fancy Magic move generator, 16-bit moves2 replay engine.
//
// SPDX-License-Identifier: MIT

pub mod attacks;
pub mod bitboard;
pub mod board;
pub mod database;
pub mod movegen;
pub mod fen;
pub mod moves;
pub mod replay;
pub mod san;
mod polyglot_keys;
pub mod types;
pub mod zobrist;

pub use board::{Board, IllegalMove, Undo};
pub use moves::Move;
pub use types::{Color, Piece, Role, Square};
