# Contributing to GigaChess

Thank you for your interest in contributing to **GigaChess**! We welcome contributions from the community to help make GigaChess the fastest, most reliable, and most developer-friendly chess library in Rust.

---

## Non-Negotiable Core Principles

Before writing code or opening a pull request, please review our core architectural rules:

### 1. 100% Permissive MIT Licensing
- Everything in this repository MUST be **100% MIT-licensed**.
- **NEVER** copy or adapt code from GPL/LGPL repositories (such as `shakmaty`, `chessops`, or `Stockfish` C++).
- Clean-room algorithmic implementations and techniques from permissive crates (e.g. `cozy-chess` under MIT / Apache-2.0) are welcome with explicit attribution.

### 2. Zero-Allocation Hot Paths
- Legal move generation (`board.legal_moves()`), pseudo-legal move generation, board inspection, and SAN parsing must never allocate on the heap (`malloc` / `Vec` / `String`).
- Always use stack buffers (e.g. `ArrayVec<Move, 256>`) or visitor patterns.
- Keep the `Board` struct cache-compact and strictly `Copy` (144 bytes).

### 3. Performance Regression Gate
- Any change touching core movegen, make/unmake, board state, hashing, or perft must demonstrate that it does not regress performance.
- We benchmark with Criterion on `release` profile (`lto = "fat" codegen-units = 1`). Run `cargo bench` to verify.

---

## Development Workflow

### Prerequisites
- Stable Rust toolchain (Rust 2021 edition).
- `cargo` and `git`.

### Building and Testing

```bash
# Fast compilation check
cargo check --all-targets --all-features

# Run the complete test suite (86+ unit, integration, and differential tests)
cargo test --all-features

# Lint check with clippy (must have zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --all -- --check
```

### Running Benchmarks

```bash
# Run head-to-head library comparisons vs Shakmaty & Cozy-Chess
cargo bench --bench vs_libraries

# Run perft throughput benchmarks (depth 1 to 5)
cargo bench --bench perft_bench

# Run database PGN & binary moves2 codec benchmarks
cargo bench --bench codec_bench

# Run micro-benchmarks
cargo bench --bench micro
```

---

## Submitting a Pull Request

1. **Fork & Branch**: Create a descriptive feature branch from `main` (e.g., `feature/neon-slider-opt` or `fix/fen-ep-parsing`).
2. **Commit Conventions**: Write clear, concise commit messages. If your change addresses a specific issue or optimization target, reference it in the message.
3. **Verify Locally**: Ensure `cargo test --all-features` and `cargo clippy` pass cleanly before pushing.
4. **Benchmark Proof**: If proposing an optimization, include before/after benchmark results in your PR description.
5. **Review**: A maintainer will review your pull request for correctness, safety, and performance impact.

---

## Code of Conduct

All contributors and participants agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).
