## Purpose

Defines the `turbochess_rs::compat::shakmaty` compatibility layer providing drop-in type and method parity for existing `shakmaty` 0.30 consumers.

## ADDED Requirements

### Requirement: Shakmaty API Facade SHALL Provide 1-Line Drop-in Compatibility

The system SHALL provide `turbochess_rs::compat::shakmaty` exporting `Chess`, `Position`, `Move`, `Role`, `Color`, `Square`, `Fen`, `San`, and `Zobrist64` matching `shakmaty` 0.30 method signatures.

#### Scenario: blind-base compiles with alias
- **WHEN** `blind-base` replaces `use shakmaty::*;` with `use turbochess_rs::compat::shakmaty::*;`
- **THEN** `cargo check` and `cargo test` in `blind-base/src-tauri` succeed with zero functional code changes
