# Releasing GigaChess-RS

The crate is published on [crates.io](https://crates.io/crates/gigachess) by
the GitHub workflow `.github/workflows/release.yml` when a maintainer pushes
a new Git tag matching `v*`. Routine GitHub CI is deliberately manual-only:
tests and performance benchmarks run on our aiserver.

## v0.1.11 release checklist

- The source on `main` contains `Cargo.toml` and `Cargo.lock` at 0.1.11,
  the matching `CHANGELOG.md`, completed correctness fixes, and the SOTA
  research in `docs/research/rust-movegen-sota-2026-11-24.md`.
- Verified on aiserver: `cargo test --all-features --locked` passes.
- Verified on aiserver: `cargo publish --dry-run --allow-dirty` packages and
  compiles `gigachess 0.1.11`; it does not upload.
- The release workflow verifies that tag and manifest versions match, runs
  its own required publish checks, invokes `cargo publish`, and creates a
  GitHub Release; `CARGO_REGISTRY_TOKEN` must be configured in Actions secrets.
- **Tag the current HEAD of main**, not its squash-merge parent. The parent
  contains a `[skip ci]` instruction; GitHub may suppress push-triggered
  publishing for tags pointing to that parent. This documentation commit
  intentionally contains no CI-skipping instruction.
- Do not rewrite an already published tag or package version; crates.io
  release versions are immutable.

### Publish when ready

From an authenticated maintainer checkout:

```sh
git fetch origin main --tags
git checkout main
git pull --ff-only origin main
git tag v0.1.11
git push origin v0.1.11
```

Alternatively, create the GitHub release `v0.1.11`, targeting current
`main` at GitHub Releases. Confirm the Release workflow completed and
crates.io lists `gigachess 0.1.11`.

For subsequent patch versions, update the manifest and lockfile together
with the changelog, run the same server tests and dry run, merge to main,
then push only the next matching version tag.
