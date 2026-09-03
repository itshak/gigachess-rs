## Context

`turbochess-rs` is a high-performance Rust chess engine designed as the native/WASM companion to `gigachess` (JS/TS). With `gigachess` now released on npm and GitHub, the Rust crate must claim the `gigachess` package name on crates.io, link with `https://github.com/itshak/gigachess-rs`, establish automated GitHub Actions for CI and crates.io publishing, and align documentation and branding assets.

## Goals / Non-Goals

**Goals:**
- Rename crate to `gigachess` in `Cargo.toml` and verify all internal tests, benches, and examples build cleanly.
- Set up GitHub Actions CI (`.github/workflows/ci.yml`) testing `cargo check`, `cargo test`, and `cargo clippy`.
- Set up GitHub Actions Release (`.github/workflows/release.yml`) automated deployment to crates.io via `CARGO_REGISTRY_TOKEN`.
- Import and deploy high-resolution brand assets (`assets/logo.png` transparent 800x800 and `assets/social-preview.png`).
- Update all documentation (`README.md`, `AGENTS.md`, `BENCH.md`, `MIGRATION.md`, `justfile`) and issue templates.
- Ensure clean repository state with `.gitignore` and security policy to support making the repository private or public seamlessly.

**Non-Goals:**
- Rewriting core move generation or bitboard algorithms (PEXT, ultrachess-core integration remains untouched).
- Changing public Rust API signatures beyond the crate name import path (`turbochess_rs` -> `gigachess`).

## Decisions

### Decision 1: Crate Name `gigachess` vs `gigachess-rs`
- **Choice:** Crate name in `Cargo.toml` is `gigachess`, repository name is `gigachess-rs`.
- **Rationale:** `gigachess` is currently unclaimed on crates.io. In the Rust ecosystem, prime library names (e.g. `serde`, `tokio`, `shakmaty`) are preferred over `-rs` suffixes for crates, while the GitHub repository is named `gigachess-rs` to distinguish it from the JS repository (`gigachess`).
- **Alternatives considered:** Naming the crate `gigachess-rs`. Rejected because `gigachess` is available and much cleaner for downstream consumers (`gigachess = "0.1"`).

### Decision 2: Automated Crates.io Publishing via GitHub Actions
- **Choice:** Dedicated `.github/workflows/release.yml` that triggers only on git tag push (`v*`), validates that the git tag version matches `Cargo.toml`, generates release notes, and runs `cargo publish`.
- **Rationale:** Prevents accidental publishing from local machines and guarantees that every crates.io release corresponds to a tagged, verified commit on GitHub.

### Decision 3: Shared Visual Identity with JS Ecosystem
- **Choice:** Reuse the exact transparent 800x800 cybernetic knight logo (`assets/logo.png`, 645 KB) and 16:9 social card (`assets/social-preview.png`, 499 KB).
- **Rationale:** Ensures immediate brand recognition across both JavaScript/TypeScript and Rust libraries, while keeping asset sizes under GitHub's 1 MB limit.

## Risks / Trade-offs

- [Crates.io Namespace Squatting] → Mitigation: Publishing or reserving `gigachess` 0.1.0 on crates.io via the release action immediately locks the namespace.
- [Private/Public Repo Switching] → Mitigation: Ensure no sensitive tokens, local test files, or hardcoded personal paths exist in tracked files; configure `.gitignore` comprehensively.
