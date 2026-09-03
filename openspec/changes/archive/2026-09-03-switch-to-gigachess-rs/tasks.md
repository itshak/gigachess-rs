## 1. Crate Configuration & Metadata

- [x] 1.1 Update `Cargo.toml` crate name to `gigachess`, repository to `https://github.com/itshak/gigachess-rs`, version to `0.1.0`, description, keywords, and verify with `cargo check --all-targets`
- [x] 1.2 Update crate imports in `tests/`, `benches/`, and `examples/` from `turbochess_rs` to `gigachess` and verify `cargo check --tests --benches --examples` succeeds cleanly
- [x] 1.3 Update git remote `origin` to `git@github.com:itshak/gigachess-rs.git` and test connectivity

## 2. Brand Identity & Visual Assets

- [x] 2.1 Copy the 800x800 transparent GigaChess cybernetic knight logo from `gigachess/assets/logo.png` to `assets/logo.png` and verify file size is < 1MB
- [x] 2.2 Copy the optimized 16:9 social preview banner from `gigachess/assets/social-preview.png` to `assets/social-preview.png` and verify file size is < 1MB

## 3. GitHub Actions & Crates.io Release Pipeline

- [x] 3.1 Create `.github/workflows/ci.yml` to run `cargo check`, `cargo test --all-features`, `cargo clippy`, and benchmark compile verification on push and PR
- [x] 3.2 Create `.github/workflows/release.yml` to validate version against `Cargo.toml`, create GitHub releases on `v*` tag push, and publish to crates.io via `CARGO_REGISTRY_TOKEN`
- [x] 3.3 Add `.github/SECURITY.md` and repository issue templates

## 4. Documentation & Privacy Preparation

- [x] 4.1 Update `README.md` with GigaChess Rust crate branding, installation snippet (`gigachess = "0.1"`), code examples, and badge links
- [x] 4.2 Update `AGENTS.md`, `BENCH.md`, `MIGRATION.md`, and `justfile` to reflect the `gigachess` project identity
- [x] 4.3 Audit `.gitignore` and ensure no secret tokens, cache files, or private environment variables can leak when switching repo visibility

## 5. Verification & Dry-Run Publishing

- [x] 5.1 Run `cargo test --all-features` and verify 100% of unit, parity, and integration test suites pass
- [x] 5.2 Run `cargo publish --dry-run` to verify crates.io package structure, files list, and metadata validation
