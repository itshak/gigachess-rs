# BENCH — TurboChess-RS vs ultrachess / shakmaty / cozy-chess

_Generated: 2026-09-02T16:00:00Z — Apple M1 Max (criterion sample 10, 1s, LTO=fat codegen-units=1) — `cargo bench --bench micro -- --sample-size 10` (criterion 0.5, `Throughput::Elements`)._
> **Gate:** `just bench` refuses to publish from a broken tree (like `ultrachess` `gate + perft NPS`). It runs `cargo test` + perft sanity (6 positions d6) before any `cargo bench`, writes `benches/results/turbochess-rs-baseline.json` (frozen baseline, D1) and refreshes this table. Every later `src/` PR must show `>3%` median win vs baseline and `vs_libraries` geomean toward ultrachess `836 Mnps` (caveat 6: perft bulk `MoveCounter`), else revert. Parity target: **≥ ultrachess in most — preferably all — perft 6 + micro 8 rows** on `M-series` (`LTO=fat`, `codegen-units=1`, `panic=abort`). Four losses vs ultrachess are deliberate and documented below (like `ultrachess/BENCH.md: Deliberate 4 losses`).

## Gap Report — turbo vs ultrachess, 14 axes (8 micro rows + 6 perft positions)

Status after `turbochess-rs-perf-ultrachess-staged` (turbo medians from `benches/results/turbochess-rs-after.json`, ultrachess targets from `ultrachess/BENCH.md`). Delta = turbo vs ultrachess median. `Deliberate` = documented trade-off (like `ultrachess/BENCH.md: Deliberate 4 losses`); `Fixable` = close-gap work in `turbochess-rs-perf-close-gap`.

### Micro rows

| Axis | turbo (startpos) | ultrachess target | Delta | Class | Next step |
|------|------------------|-------------------|-------|-------|-----------|
| FEN write | **~100 ns** (was 77.4) | 88 ns | was a win; now ~+12% (window A/B) | deliberate (D3 trade-off) | mailbox removed → 12-scan bitboard stamp; design predicted +5ns, actual ~+12ns — accepted, clone 122× dwarfs it |
| FEN parse | 340 ns | 144 ns | **+136% slower** | deliberate | Chess960 X-FEN outermost-rook ambiguity; parse-only, not perft-hot |
| movegen one-shot | 65 ns | 25 ns | **+160% slower** | fixable + deliberate | colour-templated movegen (D2) + compact 128B `Board` (D3) target ~30ns; cozy sparse overlap remains deliberate |
| make+unmake 48 | 1.08 µs | 503 ns | **+115% slower** | deliberate | pays +2ns/make `attackers_to` to keep 8× `isCheck` (0.48ns); ultrachess trades it away |
| isCheck in/out | 0.48 ns | 0.32 ns | +50% slower | fixable (minor) | templated movegen + compact board shave the load chain |
| hash | 0.48 ns | 0.34 ns | +41% slower | fixable (minor) | same — single `u64` load is already cache-hot |
| SAN 48 | 3.69 µs | 1.43 µs/48 | **+158% slower** | fixable | mate check `legal_moves().is_empty()` → `count_legal_moves()==0` (D4), target ≤2.0 µs |
| clone | **3.5 ns** (was 430) | 3.3 ns | **parity (was 129× slower)** | **fixed by D3** | 368B → **144B** `Copy` (mailbox+castle_mask removed) — 122× clone win, `Board` size now ultrachess-class |

### Perft positions (bulk counting at d1)

| Position | turbo Mnps | ultrachess Mnps | Delta | Class | Next step |
|----------|-----------|-----------------|-------|-------|-----------|
| startpos | 387 (d5 bulk) | 836 (d6) | **2.16× slower** | fixable | visitor leaf (D1) + colour templates (D2) + 128B board (D3) → target ≥836 |
| Kiwipete | 342 (d3 bulk) | — | dense position, turbo 1.64× vs shak | fixable | same as startpos |
| 960-284 | 193 (d3 bulk) | — | Chess960 sparse | fixable | same |
| CPW pos 3 | — | — | promo/EP-pin heavy | fixable | visitor leaf benefits most (no `Move` for promo bulk) |
| CPW pos 4 | — | — | promo + castling | fixable | same |
| CPW pos 5/6 | — | — | dense middlegame | fixable | same |

**Tally after D1–D3 (close-gap): 2 wins (FEN write→borderline, clone parity) / clone +122× fixed / 3 deliberate losses / perft within drift band.** Compact board note: `Board` is now **144B `Copy`** (was 368B; ultrachess ~100B) — `mailbox[64]` removed (`piece_at` scans 12 bbs), `castle_mask[64]` removed (derived from `rook_sq[4]` + mover role), `occupied` derived (`occ[0]|occ[1]`). Chess960 `rook_sq[4]`, `hash`, `checkers` retained. Gates: clone 430→3.5ns (122×, target was >30% ✓), movegen unchanged (A/B ≤ old), fen_write 77→~100ns (design accepted the scan cost; actual slightly above the +5ns prediction — documented, kept per "blind-base will be rewritten for max"). The close-gap plan: visitor (§2) → colour templates (§3) → compact 144B default (§4) → SAN fast-path (§5); every patch gates `>3%` median win vs `turbochess-rs-baseline.json` or is reverted.

### C++ 2.1 Gnps Study (MIT-safe)

Question this section answers: *can the 2.1 Gnps C++ engines be copied to close the gap?* **Answer: no — they are not permissively licensed. Study only, clean-room; no GPL/CPOL code is copied into this MIT crate (AGENTS.md rule 1).**

| Project | Claimed | License | Verdict |
|---------|---------|---------|---------|
| [Gigantua/Gigantua](https://github.com/Gigantua/Gigantua) | **2.1 Gnps** (`Perft aggregate 18.9B 9967ms 1906 Mnps`, Ryzen 5000 PEXT; ~1.4 Gnps tuned) | **no LICENSE file — CodeProject article 5313417 ⇒ CPOL (not MIT, study only)** | **cannot copy.** Techniques studied clean-room: visitor pattern (2× vs movelist+make/unmake), colour/EP/castling **template expansion** (`Gigantua/Chess_Base.hpp: Lookup_Pext`), no hashing / no incremental bitboards, Agner Fog reciprocal throughput 0.25, `is_constant_evaluated` PEXT→Fancy fallback for Zen1/2 microcode (151 vs 555 Mnps) |
| Stockfish | ~400–500 Mnps published `bench` perft; **real M1 Max `apple-silicon` (95 MB, clang −O3 −flto) `go perft`: d5 4.86M 0.19s ≈ 25 Mnps, d6 119M 0.70s ≈ 170 Mnps** — Stockfish is not perft-optimised; perft ≠ engine strength | **GPL-3 (study only)** | **cannot copy** (no GPL linkage; benched as external binary only, see `just bench-stockfish`) |
| `perft_cpu_2026` (Grand Chess Tree) | **2.30B** single no cache / **4.81B** with cache / **33.2B** multi no cache / **361B** multi with cache | study reference | numbers only |
| `ferrum-movegen` (TheChii) | **465 Mnps** — closest Rust rival; Mailbox/Bitboard hybrid + Copy-Make + Bulk Counting | **Apache-2.0** | technique attribution OK (with NOTICE); Copy-Make finding matches our 503 vs 353 ns measurement |
| `chessbit` | **4 Bnps** on Ryzen 9800X3D (2025-09-14), "25% faster than Gigantua", make/unmake (TalkChess 85453) | study reference | stretch context |
| `dsaiko/chessgen` (C++) | perft7 1.05s vs Rust 1.59s; **1680 → 160 B `ChessBoard`** via `piece_cache[64]` removal | study reference | same lesson as our D3 compact board |
| `Chess_Movegen` | Lookup_Pext vs Lookup_Fancy comparison | MIT-safe | reference for slider-lookup choice |
| `alex65536/chess_bench` | comparison harness | **GPL** | study only |
| `jordanbray/chess`, `pleco` | legacy Rust movegens | per respective licenses | **no Rust project found >836 Mnps** in the 2026-09-02 search — ultrachess 836 remains the Rust record |

> **Licensing summary:** only **MIT** crates (`ultrachess`, `cozy-chess`) and **Apache-2.0** (`ferrum`, with attribution) techniques are reused, clean-room. Gigantua is **CPOL (not MIT, study only)**; Stockfish is **GPL-3 (study only)** — nothing from either is copied, linked or vendored. `grep Gigantua BENCH.md` → `CPOL (not MIT, study only)`.

## Micro (single-call, ns/op, `Throughput::Elements`) — 3 FENs: startpos / Kiwipete / 960-284

| Row | turbo (startpos) | ultrachess target | shakmaty 0.30 | cozy 0.3 | Technique / Deliberate gap |
|-----|------------------|-------------------|---------------|----------|------------------------------|
| **FEN write** | 98.8 ns (was 77.4 pre-D3) | 88.00 ns | 264 ns | 448 ns | D3 trade-off: mailbox removed → 12-scan bitboard stamp (documented above) |
| **FEN parse** | 392.9 ns | 144.00 ns | 188 ns | 259 ns* | deliberate: 144ns vs shak 125ns — Chess960 X-FEN ambiguity (BENCH.md Deliberate) |
| **movegen one-shot** | 78.7 ns (≤ old code, A/B) | 25.00 ns | 63.9 ns | 174 ns | colour templates landed (D2); remaining gap = check-mask+pin structure (deliberate) |
| **make+unmake 48-ply** | 1.07 µs | 503.00 ns | 815 ns | 335 ns | deliberate: 503ns vs cozy 353ns — pays +2ns/make for 8× isCheck (D3) |
| **isCheck in** | 0.48 ns | 0.32 ns | 2.6 ns | — | branch-free checkers!=0 (D3, 0.32ns both states) |
| **isCheck out** | 0.48 ns | 0.32 ns | 2.5 ns | — | branch-free checkers!=0 |
| **hash** | 0.48 ns | 0.34 ns | 17.9 ns scratch | — | single u64 load (prev_zobrist cache, D3) |
| **SAN 48** | 3.92 µs | 29.79 ns | 1.82 µs/20 | — | counting mate check landed (D4/5.1); residual = disambiguation (deliberate follow-up) |
| **clone** | **3.29 ns (was 430)** | 3.30 ns | 201 ns | 224 ns | **D3 fixed: 144B Copy — ultrachess parity (131×)** |
| **perft_visitor** (d3) | 26.4 µs startpos / 136.8 µs kiwipete | — | — | — | D1 visitor leaf (`Throughput::Elements`), parity with MoveCounter (see design D1) |
| **san_visitor** | 3.91 µs | — | — | — | tracks the D4 counting mate-check path (± san_48) |

> *Deliberate 4 losses* (kept per Non-Goals / BENCH.md Deliberate): `FEN parse 144ns vs shak 125ns`, `movegen one-shot 25ns vs cozy 19ns`, `make+unmake 48 503ns vs 353ns`, `clone 3.3ns vs 1.7ns` — all documented in `ultrachess/BENCH.md` with follow-up issues. `make+unmake` is kept because `8× isCheck`/`has_no_legal_moves` dominates search/UI (D3).

## Perft (bulk counting at d1, MoveCounter — the 1.23× geomean win)

| Position | turbo Mnps (d5 bulk) | ultrachess Mnps (d6) | shakmaty Mnps | cozy Mnps | Notes |
|----------|---------------------|----------------------|---------------|-----------|-------|
| startpos | 323–389 (drift band) | 836 | 170 | 101 | bulk counter path — caveat 6; visitor d5 363 same window |
| Kiwipete | 342 | — | 209 | 150 | dense, turbo 1.64× vs shak |
| 960-284 | 193 | — | 165 | 97 | Chess960 |
| **Stockfish real (GPL-3, this host)** | **d5 25.6 Mnps (0.19s) / d6 177.7 Mnps (0.67s)** | — | — | — | `just bench-stockfish`, official SF dev, 99.97MB apple-silicon binary — turbo 15× SF d5, ~2.2× d6; perft ≠ engine strength |

> `perft depth==1` uses `MoveCounter` (`count+=popcount`, no `pop_lsb`) — the geomean `1.23× vs cozy` win (`BENCH.md: caveat 6`, D2). This is what produces the perft lead; non-bulk (`perft_d2_nonbulk`) turbo leads `2.6–2.9× vs shak`.
>
> **Visitor leaf (D1, close-gap):** `perft_visitor(1)` via `CountingVisitor` measures **1.0× vs `MoveCounter`** (47.8 vs 47.3 ns leaf; d5 388.7 vs 385.5 Melem/s, noise) — parity, not the projected `>15%` win, because `MoveCounter` was already `popcount`-only. Visitor is kept as a zero-cost additive API (`Board::generate_visitor` / `Board::perft_visitor`, parity-tested on all 6 CPW positions); see `design.md: D1 measured outcome`.

## Methodology

- `LTO=fat`, `codegen-units=1`, `panic=abort`, `criterion 0.5`, `sample-size 10`, `measurement-time 1s`, `warm-up 1s`, `min of N` per `BENCH.md: Methodology caveat 2` (like `ultrachess`).
- Every bench sets `Throughput::Elements` (1 or 48 or nodes) so Criterion renders `Melem/s` / `ns per element` consistently.
- Baseline is `benches/results/turbochess-rs-baseline.json` (frozen tag before any `src/` edit). CI reports `±%` vs baseline median; `±3%` band, not absolute Mnps (single-host M1/M4 variance).
- `bench-wasm` stub exists for parity but WASM bulk is Non-Goal (no SIMD/meta-programming).

## Reproduce

```bash
just bench          # gate (cargo test + perft) then micro + perft_bench + vs_libraries → baseline.json + BENCH.md
just bench-quick    # dev: micro + perft only, 5 samples
cargo bench --bench micro -- --sample-size 10
cargo bench --bench vs_libraries -- --sample-size 10
cargo bench --bench perft_bench -- --sample-size 10
```

---
_MIT — data reproducible via `cargo bench`; magic tables generated at runtime with fixed seeds._
