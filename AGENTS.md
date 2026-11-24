# AGENTS.md — GigaChess (Rust) AI Agent Instructions

> Canonical guide for AI coding assistants working on the `gigachess` Rust crate.

## Project Overview

**GigaChess** is an ultra-high-performance, 100% MIT-licensed chess engine, PEXT/Fancy Magic move generator, 16-bit binary replay engine, and Shakmaty drop-in compatibility crate in Rust.

- **License:** MIT (Permissive, unrestricted commercial and proprietary reuse)
- **Primary Consumers:** Chess engines, position search tools, repertoire engines, master game indexers, and the broader Rust chess ecosystem.

## Tech Stack & Architecture

| Layer | Technology & Design |
|---|---|
| **Language** | Rust 2021 edition (100% safe public APIs with isolated, verified intrinsics) |
| **Bitboards** | Native `u64` bitboards with PEXT (BMI2) and Fancy Magic slider tables |
| **Move Encoding** | 16-bit packed `u16` (`moves2` wire format: `from | (to << 6) | (promo << 12)`) |
| **Hashing** | Incremental 64-bit Polyglot/Shakmaty Zobrist hashes with zero allocations |
| **Compatibility** | Drop-in `gigachess::compat::shakmaty` facade replacing `shakmaty` 0.30 |
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
   - Always prefix commit messages with the change ID in brackets: `[switch-to-gigachess-rs] Update crate branding`.

---

## Shared GigaChess JS / Rust Contract (MANDATORY)

Before changing FEN parsing, Chess960, castling, hashes, repetition, packed
moves2, null moves, or SAN: read `openspec/changes/dual-engine-hardening-20261123/`
including the common `specs/mirrored-core-contract/spec.md`; its identical
counterpart is in `itshak/gigachess` JS. Test golden fixtures in both repos:

- Rust `tests/data/position-keys-v1.tsv` and `tests/data/moves2-v1.tsv`
- JS `tests/position-keys-v1.tsv` and `tests/moves2-v1.tsv`

Rules are the same across language boundaries: a1=0, White=0, Black=1,
packed `from | (to << 6) | (promo << 12)`, canonical castling king-source to
INITIAL ROOK square, `0xffff` null move, SAN `--`/UCI `0000`, Polyglot
vectors and Chess960 castling-key seed `0x00C0_FFEE_DABA_D00D`.
Polyglot EP hashing is pseudo-legal; FIDE repetition hashes must use legal EP.
This Rust crate remains independently optimized; the JS library must remain
pure TypeScript without mandatory WASM loading. Any wire/rules change needs
joint design, golden vectors and tests in BOTH repositories.
