# gigachess-rs-crate-rename Specification

## Purpose
Establishes the GigaChess Rust engine crate branding, crates.io publishing pipeline, repository asset configuration, and cross-platform verification workflows.

## Requirements

### Requirement: Cargo Package Identity and Crate Naming
The crate SHALL be named `gigachess` in `Cargo.toml`, with repository pointing to `https://github.com/itshak/gigachess-rs`, version `0.1.0`, MIT licensing, and appropriate chess engine keywords/categories for crates.io indexing.

#### Scenario: Cargo metadata inspection
- **WHEN** running `cargo read-manifest` or inspecting package metadata
- **THEN** package name MUST be `gigachess` and repository MUST be `https://github.com/itshak/gigachess-rs`

### Requirement: Brand Assets and Transparency
The repository SHALL contain high-resolution brand assets matching the GigaChess ecosystem in `assets/logo.png` (800x800 transparent PNG under 1MB) and `assets/social-preview.png` (16:9 widescreen card under 1MB).

#### Scenario: Asset resolution and size verification
- **WHEN** checking `assets/logo.png` and `assets/social-preview.png`
- **THEN** both files MUST exist, be formatted as valid PNGs, and each file size MUST be strictly less than 1 megabyte (1,048,576 bytes)

### Requirement: Continuous Integration Workflow
The repository SHALL include a GitHub Actions CI workflow in `.github/workflows/ci.yml` that builds and tests the crate across standard and release profiles.

#### Scenario: CI workflow execution
- **WHEN** code is pushed or a pull request is opened
- **THEN** GitHub Actions MUST execute `cargo check --all-targets`, `cargo test --all-features`, and verify formatting/clippy

### Requirement: Automated Crates.io Release Workflow
The repository SHALL include an automated release workflow in `.github/workflows/release.yml` triggered on version tag pushes (`v*`) that publishes `gigachess` to crates.io.

#### Scenario: Tag push triggers crates.io release
- **WHEN** a git tag matching `v*` (e.g. `v0.1.0`) is pushed to GitHub
- **THEN** the workflow validates that the tag version matches `Cargo.toml`, creates a GitHub release, and executes `cargo publish` using the `CARGO_REGISTRY_TOKEN` secret

### Requirement: Clean Room Repository Privacy Posture
The repository SHALL be prepared for seamless toggling between public and private visibility without leaking sensitive keys, temporary artifacts, or broken absolute paths.

#### Scenario: Security and git hygiene check
- **WHEN** git status and ignored files are audited
- **THEN** all `.env`, tokens, target artifacts, and temporary benchmark dumps MUST be excluded by `.gitignore` and `.github/SECURITY.md` MUST define responsible vulnerability reporting
