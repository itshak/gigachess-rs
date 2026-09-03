## Why

Following the successful migration of the JavaScript/TypeScript library from `turbochess` to `gigachess` on npm and GitHub, the Rust engine (`turbochess-rs`) must be aligned under the unified **GigaChess** brand identity.

The crate name `gigachess` is currently available and unclaimed on crates.io. Renaming the Rust crate from `turbochess-rs` to `gigachess` (with repository at `https://github.com/itshak/gigachess-rs`) ensures seamless cross-language parity, secures the prime `gigachess` crate namespace on crates.io before third-party squatting, sets up automated GitHub Actions for CI and crates.io publishing, adopts the high-res transparent cybernetic logo and social preview card, and prepares repository privacy hygiene.

## What Changes

- **Package & Crate Identity**: Rename crate in `Cargo.toml` to `gigachess`, update repository URL to `https://github.com/itshak/gigachess-rs`, set package metadata, keywords, documentation links, and crate description.
- **Brand Assets**: Import the newly created transparent high-resolution cybernetic knight logo (`assets/logo.png`) and social preview card (`assets/social-preview.png`).
- **GitHub Workflows**:
  - Add `.github/workflows/ci.yml` running `cargo test`, `cargo check --all-targets`, `cargo clippy`, and benchmark integrity on push/PR.
  - Add `.github/workflows/release.yml` triggering on git tags (`v*`), validating version match with `Cargo.toml`, generating GitHub release notes, and publishing to crates.io via `cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}`.
- **Documentation & Repository Preparation**:
  - Update `README.md`, `AGENTS.md`, `BENCH.md`, `MIGRATION.md`, and `justfile` to `gigachess`.
  - Add `.github/SECURITY.md`, issue templates, and ensure clean repository posture (ready for private/public toggle).
- **Backward Compatibility & Deprecation**: Add guidance/notes for `turbochess-rs` consumers transitioning to `gigachess`.

## Capabilities

### New Capabilities
- `gigachess-rs-crate-rename`: Crate renaming to `gigachess`, crates.io metadata, cargo release workflows, brand asset deployment, and documentation overhaul.

### Modified Capabilities

## Impact

- **Crate Name**: Consumers will import `gigachess = "0.1"` instead of `turbochess-rs`.
- **APIs & Internals**: No internal engine logic breaks; crate re-exports and benchmarks seamlessly adapt to the `gigachess` namespace.
- **CI/CD**: Adds automated continuous integration and release workflows targeting crates.io.
