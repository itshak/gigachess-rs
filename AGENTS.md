# AGENTS.md — TurboChess-RS AI Agent Instructions

> Canonical guide for AI coding assistants working on the `turbochess-rs` Rust crate.

## Project Overview

**TurboChess-RS** is an ultra-high-performance, 100% MIT-licensed chess engine, PEXT/Fancy Magic move generator, 16-bit binary replay engine, and Shakmaty drop-in compatibility crate in Rust.

- **License:** MIT (Permissive, unrestricted commercial and proprietary reuse)
- **Primary Consumers:** `blind-base` (GigaBase position search, repertoire engine, master game indexing) and the broader Rust chess ecosystem.

## Tech Stack & Architecture

| Layer | Technology & Design |
|---|---|
| **Language** | Rust 2021 edition (100% safe public APIs with isolated, verified intrinsics) |
| **Bitboards** | Native `u64` bitboards with PEXT (BMI2) and Fancy Magic slider tables |
| **Move Encoding** | 16-bit packed `u16` (`moves2` wire format: `from | (to << 6) | (promo << 12)`) |
| **Hashing** | Incremental 64-bit Polyglot/Shakmaty Zobrist hashes with zero allocations |
| **Compatibility** | Drop-in `turbochess_rs::compat::shakmaty` facade replacing `shakmaty` 0.30 |
| **Memory Policy** | Zero heap allocations in hot movegen loops (`ArrayVec<Move, 256>`) |

## Build & Test Commands

```bash
cargo check                        # fast typecheck
cargo test                         # run unit and parity tests
cargo clippy                       # lint check
cargo bench                        # run Criterion performance benchmarks
cargo test --features pext         # test with hardware PEXT enabled
```

## Core Rules for AI Agents

1. **Licensing & Hygiene:**
   - Everything in this repository MUST be **100% MIT licensed**.
   - Reusing algorithms/code from MIT crates (e.g. `cozy-chess`) is encouraged with proper attribution.
   - **NEVER** copy code from GPL repositories (`shakmaty`, `chessops`, `stockfish` C++).
2. **Performance First:**
   - Keep move generation zero-allocation. Use stack arrays (`ArrayVec`) rather than `Vec`.
   - In hot loops, prefer bitwise operations and precomputed tables.
3. **OpenSpec Traceability:**
   - Always prefix commit messages with the change ID in brackets: `[turbochess-rs-core-engine] Add PEXT sliding attacks`.
