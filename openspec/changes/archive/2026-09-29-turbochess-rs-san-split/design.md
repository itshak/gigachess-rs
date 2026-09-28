# turbochess-rs-san-split — Design

## Context

`move_to_san` composes two pieces of work with different inputs:

| | needs | gigachess's cost today |
|---|---|---|
| **body** — piece letter, minimal disambiguation, `x`, target, `=Q` | the position **before** the move | `attackers_bb` pre-filter + at most one `generate_moves_into` (the `1.43µs/48` path) |
| **suffix** — `+` / `#` | the position **after** the move | 144B `Board` copy + `make_move_unchecked` (recomputes `checkers`, +2 ns) + `in_check()` (0.32 ns) + `count_legal_moves()` only when in check + `unmake_move` |

The body is already the cheap half. The suffix is cheap *in principle* and expensive only because the function has to manufacture the after-position to read it.

## Decision

Expose the seam; change no rule.

```rust
pub fn move_to_san(board: &Board, mv: Move) -> Option<San> {   // unchanged signature
    let mut out = move_to_san_body(board, mv)?;
    if let Some(c) = check_mate_suffix_after_make(board, mv) { out.push(c); }
    Some(out)
}

pub fn move_to_san_body(board: &Board, mv: Move) -> Option<San>   // today's lines 28-135
pub fn check_mate_suffix(after: &Board) -> Option<char>          // in_check() → '+' / '#'
fn check_mate_suffix_after_make(board: &Board, mv: Move) -> Option<char>  // today's copy/make/read/unmake
```

Both suffix entry points funnel through the single `check_mate_suffix` expression, so the rule — "in check, and no legal reply, is mate" — has exactly one implementation and the two routes cannot drift. `move_to_san`'s output is unchanged by construction: same body, same suffix, same order.

## The caller's contract

A caller that wants the split must have reached the after-position through a make that **maintains `checkers`**:

| make | maintains `checkers`? | usable for `check_mate_suffix`? |
|---|---|---|
| `make_move_unchecked`, `make_move_perft` | yes (`attackers_to` after the turn flip) | **yes** |
| `play` (built on `make_move_unchecked`) | yes | **yes** |
| `make_move_fast`, `play_fast` (v0.1.4) | **no** — the field is left stale | no: a stale `checkers` reads as "not in check" |

This is the one sharp edge, it is why the doc comment on `check_mate_suffix` says so in bold, and `a_stale_checkers_cache_would_be_a_stale_suffix` pins the behaviour so the trade-off cannot be quietly lost. The consumer (`cbh-parser`) therefore pays +2 ns/make on its walk and saves a board copy, a `make_move_unchecked` and an `unmake_move` per ply — a net win, and the hash it then maintains is the Polyglot one it already wanted for other consumers.

## Alternatives rejected

- **Return the suffix from `move_to_san` as a struct / `(San, Option<char>)`** — breaks every existing caller for no gain; the caller that needs the split cannot use it anyway (it has no after-position at that point).
- **A `Board` method `san_with_after(&Board, Move)`** — puts notation on the board, which is where `san` is not; `san.rs` is the right home.
- **Let the caller do the check/mate test itself** — that is chess logic in a consumer, duplicating a rule gigachess already owns and can get wrong. Rejected under the "one chess core" rule.
- **A feature flag** — a build-time switch on a 15-line refactor is a maintenance surface with no buyer.

## Verification

1. `tests/san_split_property.rs`: 100,000 positions from random playouts across the standard start and two Chess960 starts (284, 518). For every legal move asserts `move_to_san == move_to_san_body + check_mate_suffix(after)`, that the body never ends in `+`/`#`, and that make/unmake is clean.
2. `src/san.rs` unit tests: body carries no suffix; the composition equals the monolith; quiet / check / mate / promotion-with-check / disambiguation shapes; the stale-`checkers` edge.
3. The pre-existing suites are the regression net and must stay green untouched: `tests/san_parity.rs` (byte parity against `shakmaty::SanPlus`), `tests/san_disambiguation.rs`, `tests/perft.rs`, `tests/chess960.rs`, `tests/zobrist.rs`, `tests/replay.rs`, `tests/null_move.rs`, `tests/play_fast_property.rs`, and the `src` unit tests (49 in the lib).
4. `cargo clippy --all-targets --all-features` and `cargo check --all-features` clean.

`cargo test --features pext` is CI-only: the M1 Max host this was written on has no BMI2, so the PEXT path is not exercised locally and is left to CI.
