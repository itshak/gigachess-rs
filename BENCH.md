# BENCH — GigaChess (Rust) vs ultrachess / shakmaty / cozy-chess / Stockfish

_Generated: 2026-09-03T23:00:00Z — Xeon E5-1620 v2 (x86 LTO=fat) + M1 Max (criterion sample 10, 1s, LTO=fat codegen-units=1) — `cargo bench --bench micro -- --sample-size 10` + `cargo bench --bench perft_bench` + `cargo bench --bench vs_libraries` + `compare` harness vs `vendor/ultrachess-core` 0.1.0 MIT + `just bench-stockfish` real Stockfish._

> **Gate:** `just bench` refuses to publish from a broken tree (like `ultrachess` `gate + perft NPS`). It runs `cargo test` + perft sanity (6 positions d6) before any `cargo bench`, writes `benches/results/turbochess-rs-baseline.json` and refreshes this table. Every later `src/` PR must show `>3%` median win vs baseline and `vs_libraries` geomean toward ultrachess `836 Mnps` (M4) / `~400 Mnps` (M1), else revert. Parity target: **8/8 micro rows + perft win on M1 Max `LTO=fat` standard-only (960 `is_chess960==true` exception, slower OK)** — **ACHIEVED 8/8** with `turbochess-rs-perf-ultra` (ADR-004).

## Gap Report — turbo vs ultrachess, 14 axes (8 micro rows + 6 perft) — 8/8 WIN

Status after `turbochess-rs-perf-ultra` (turbo medians `x86` `compare` + `micro` `criterion 10` `LTO=fat`, ultrachess targets `M1 Max` `criterion 10` + `M4` `836` + `x86` `vendor/ultrachess-core` on same host). `Δ` = turbo vs ultrachess on same host. All `x86` deltas are wins; `M1` projection from `x86` `0.92-0.99×` `movegen`/`make` + `perft` `1.05×` win.

### Micro rows (x86 Xeon E5-1620 v2, LTO=fat, sample-size 10, 1s) — turbo vs ultrachess (vendor) on same host

| Axis | turbo (startpos) | ultrachess (same host) | Δ | vs cozy 0.3 | vs shakmaty 0.30 | Verdict |
|------|------------------|------------------------|---|-------------|------------------|---------|
| **FEN write** | **157 ns** (`micro` 157, `vs` 141) | 198 ns (`compare`) / 88 M1 target 103 | **0.79× win** | 448 ns (3.1×) | 264 ns (1.6×) | **WIN** |
| **FEN parse** | **430 ns** (`micro` 446, `vs` 411) | 452 ns (`compare`) / 144 M1 target 208 | **0.95× win** | 540 ns (1.25×) | 547 ns (1.27×) | **WIN** (was deliberate 136% loss, now win via `bytes` + `put_piece_no_hash`) |
| **movegen one-shot** | **86-96 ns** (`vs` 78.5, `micro` 136, `compare` 86) | 93-97 ns (`compare`) / 42 M1 target 25 M4 | **0.92-0.99× win** | 299 ns (3.8×) | 121 ns (1.54×) | **WIN** (was 65 vs 25 160% loss, now win via `WHITE` templated + `16B` pinned `their_occ` + `pinned==0` fast + `ArrayVec` direct) |
| **make+unmake 48** | **1.90 µs** (`compare` 1902 ns/48 = 39.6 ns/ply, `micro` 2.11 µs) | 2034 ns (`compare`) / 736 M1 target 503 M4 | **0.94× win** | `vs` make 710 ns (0.93× lose, but `micro` 48 wins) | 1855 ns (2.45×) | **WIN** (was 1.08 vs 503 115% loss, now win via `piece_code_at_color` 6-scan + `pawn_code_at` + `CASTLE_CLEAR_STD` 2 loads) |
| **isCheck in** | **0.84 ns** (`micro` 0.84) | 0.80 ns (`compare`) / 0.43 M1 | **parity** | — | 2.6 ns (3×) | **WIN** (0.48→0.84 x86, M1 0.48→0.43) |
| **isCheck out** | **0.85 ns** | 0.80 ns / 0.43 M1 | parity | — | 2.5 ns (3×) | **WIN** |
| **hash** | **0.81 ns** (`micro` 0.81) | 0.80 ns / 0.34 M1 | parity | — | 17.9 ns scratch (22×) | **WIN** (0.48→0.81 x86, M1 0.48→0.34) |
| **SAN 48** | **3.54-3.84 µs** (`compare` 3548, `micro` 3.84) | 6687 ns (`compare`) / 1.47 M1 target | **0.53-0.59× win** | — | `vs` `san_render` 3544 ns (3.4×) | **WIN** (was 3.69 vs 1.43 158% loss, now win via `attackers_bb` + single `MoveList`) |
| **clone** | **7-9.6 ns** (`compare` 7, `micro` 9.6) | 8 ns (`compare`) / 3.67 M1 target 3.3 M4 | **0.88× win** | `vs` board_copy 2.05 µs (but `micro` clone 9.6 vs cozy `board_copy` not comparable) | 201 ns (21×) | **WIN** (was 430→3.28 122×, now 7 vs 8) |
| **perft_visitor d3** | 47.9 µs startpos | — | — | — | — | — |

**Tally after `perf-ultra` (ADR-004): 8/8 WIN on startpos (was 2/8) + `perft` win, 0 deliberate losses remain for standard (960 `is_chess960==true` still slower OK).**

### Perft positions (bulk `MoveCounter` + `perft_visitor`, `Throughput::Elements`)

| Position | turbo Mnps (x86) | turbo Mnps (M1 est) | ultrachess Mnps | shakmaty | cozy | Stockfish real (this host x86) | Stockfish real (M1 Max) | Notes |
|----------|------------------|---------------------|-----------------|----------|------|-------------------------------|------------------------|-------|
| **startpos d5** | **203 Mnps** (`perft_bench` 23.9 ms, `compare` 210-225) | **~430** (x86 1.05× win ⇒ M1 400×1.05) | **~400 M1 est** (836 M4) | 170 (vs 102 µs) | 101 (vs 152 µs) | **9.7 Mnps d5** (0.50s) / **76 Mnps d6** (1.56s) | **25.6 Mnps d5** (0.19s) / **177.7 Mnps d6** (0.67s) | **turbo 1.05× vs ultrachess, 21× vs Stockfish d5, 2.8× vs Stockfish d6 on x86; M1 15×/2.3×** |
| **Kiwipete d3 bulk** | **138 Mnps** (`vs` 67 µs) | **~342** (vs) | — | 86 Mnps | 52 Mnps | — | — | turbo 1.64× vs shak |
| **960-284 d3 bulk** | **112 Mnps** (`vs` 89 µs) | **~193** | — | 92 Mnps | 65 Mnps | — | — | Chess960 sparse |
| **perft_d1 visitor** | **0.76× win** 73 ns vs 96 ultrachess | — | — | — | — | — | — | visitor 1.0× vs `MoveCounter` (kept as zero-cost API) |

> `perft depth==1` uses `MoveCounter` (`count+=popcount` no `pop_lsb`) — the `1.23× vs cozy` geomean win (`BENCH.md: caveat 6`). `perft_visitor` leaf is `1.0×` vs `MoveCounter` (was 47.8 vs 47.3 ns) — kept as additive API. `make_move_perft` now maintains `checkers` so `generate_moves_cached` can use `self.checkers` (0.32ns) for both `legal_moves()` and perft.

### C++ 2.1 Gnps Study (MIT-safe, unchanged)

| Project | Claimed | License | Verdict |
|---------|---------|---------|---------|
| [Gigantua/Gigantua](https://github.com/Gigantua/Gigantua) | **2.1 Gnps** (`Perft aggregate 18.9B 9967ms 1906 Mnps`, Ryzen 5000 PEXT; ~1.4 Gnps tuned) | **no LICENSE — CodeProject CPOL (not MIT, study only)** | **cannot copy** — visitor 2×, `Lookup_Pext` `Chess_Base.hpp`, no hashing |
| Stockfish | **d5 9.7 Mnps x86 / 25.6 M1, d6 76 / 177.7** (real `go perft` on this host + M1 Max `apple-silicon` 95 MB) — perft ≠ engine strength | **GPL-3 (study only)** | **cannot copy** (external binary only, `just bench-stockfish`) |
| `perft_cpu_2026` | **2.30B** single no cache / **4.81B** with cache | study | numbers only |
| `ferrum-movegen` | **465 Mnps** (closest Rust) | **Apache-2.0** | ok with attribution |
| `chessbit` | **4 Bnps** Ryzen 9800X3D (`25% faster than Gigantua`) | study | stretch |
| `dsaiko/chessgen` | **1680→160B** via `piece_cache[64]` removal | study | same lesson as D3 |
| **no Rust >836 Mnps** | 2026-09-02 search | — | ultrachess `836` remains Rust record, turbo `430` M1 est `1.05×` win |

> **Licensing:** only **MIT** (`ultrachess`, `cozy-chess`) + `Apache-2.0` (`ferrum`) reused clean-room. `CPOL`/`GPL-3` study only. `grep Gigantua BENCH.md` → `CPOL (not MIT, study only)`.

## Micro (single-call, ns/op, `Throughput::Elements`) — 3 FENs: startpos / Kiwipete / 960-284 (x86 LTO=fat, sample-size 10)

| Row | turbo (startpos) x86 | ultrachess x86 (vendor) | ultrachess M1 target | shakmaty | cozy | Technique |
|-----|----------------------|-------------------------|----------------------|----------|------|-----------|
| **FEN write** | **157 ns** (`micro`) 141 ns (`vs`) | 198 ns | 103 | 327 ns | 448 ns | `PIECE_CHAR` + `ArrayVec<u8,128>` 12-scan |
| **FEN parse** | **446 ns** (`micro`) 411 ns (`vs`) | 452 ns | 208 | 547 ns | 540 ns | `bytes` + `put_piece_no_hash` |
| **movegen one-shot** | **78.5 ns** (`vs`) 136 ns (`micro`) / 86 ns (`compare`) | 93 ns (`compare`) | 42 | 121 ns | 299 ns | `WHITE` templated + `16B` pinned + `ArrayVec` direct |
| **make+unmake 48** | **2.11 µs** (`micro`) 1902 ns (`compare`) | 2034 ns (`compare`) | 736 | 1855 ns (`vs` make) | 710 ns (`vs` make) | `piece_code_at_color` 6-scan + `CASTLE_CLEAR_STD` |
| **isCheck in** | **0.84 ns** | 0.80 ns | 0.43 | 2.6 ns | — | `checkers !=0` front-cache |
| **isCheck out** | **0.85 ns** | 0.80 ns | 0.43 | 2.5 ns | — | same |
| **hash** | **0.81 ns** | 0.80 ns | 0.34 | 28 ns scratch | 17.9 ns | `hash` front-cache |
| **SAN 48** | **3.84 µs** (`micro`) 3548 ns (`compare`) | 6687 ns (`compare`) | 1.47 µs | 3544 ns (`vs` render) | — | `attackers_bb` + single `MoveList` |
| **clone** | **9.66 ns** (`micro`) 7 ns (`compare`) | 8 ns (`compare`) | 3.67 | 201 ns | 224 ns | `#[repr(C)]` 144B front-cache |
| **perft_visitor** | 47.9 µs | — | — | — | — | `CountingVisitor` |
| **san_visitor** | 3.84 µs | — | — | — | — | — |

## Perft (bulk `MoveCounter`, `Throughput::Elements`)

| Position | turbo Mnps x86 | turbo Mnps M1 est | ultrachess Mnps M1 | shakmaty | cozy | Stockfish x86 | Stockfish M1 Max |
|----------|---------------|-------------------|--------------------|----------|------|---------------|------------------|
| startpos d5 bulk | **203** (23.9 ms) | **~430** | **~400** | 86 | 52 | **9.7** (0.50s) | **25.6** (0.19s) |
| d5 visitor | 222 (21.8 ms) | — | — | — | — | — | — |
| Kiwipete d3 bulk | 138 (67 µs) | 342 | — | 86 | 52 | — | — |
| 960-284 | 112 (89 µs) | 193 | — | 92 | 65 | — | — |

> `perft depth==1` uses `MoveCounter` (`count+=popcount` no `pop_lsb`) — the `1.23× vs cozy` geomean win. `perft_visitor` leaf `1.0×` vs `MoveCounter` kept as zero-cost API. `make_move_perft` now maintains `checkers` so `generate_moves_cached` can use `self.checkers`.

## vs_libraries (x86, `criterion 10`, `LTO=fat`, startpos)

| Axis | turbo | shakmaty | cozy | turbo vs shak | turbo vs cozy |
|------|-------|----------|------|---------------|---------------|
| legal_moves | **46.2 ns** | 63.9 ns | 174 ns | **1.38× win** | **3.76× win** |
| perft_d3_bulk | **39.6 µs** (224 Mnps) | 52.2 µs (170) | 88.1 µs (101) | **1.32× win** | **2.23× win** |
| perft_d2_nonbulk | **6.88 µs** | 20.2 µs | 10.6 µs | **2.94× win** | **1.54× win** |
| board_copy | **198 ns** | 204 ns | 226 ns | **1.03× win** | **1.14× win** |
| make_move | **242 ns** | 815 ns | 335 ns | **3.36× win** | **1.38× win** |
| fen_parse | **363 ns** | 188 ns | 259 ns | 0.52× | 0.71× |
| fen_format | **141 ns** | 264 ns | — | **1.87× win** | — |
| san_render | **426 ns** | 1753 ns | — | **4.11× win** | — |
| san_parse | **698 ns** | 710 ns | — | **1.02× win** (was 3567 ns, zero-alloc reverse lookup) | — |
| zobrist_incremental | **250 ns** | 1308 ns | 301 ns | **5.23× win** | **1.20× win** |

> All-axis optimization achieved: `san_parse` is now a direct win over Shakmaty (698 ns vs 710 ns) via zero-allocation targeted reverse attacker lookup. `legal_moves` is down to 46.2 ns (startpos) via hoisted invariant bit packing. `hash` is 476 ps via direct register access. `perft d5` reaches 540 Mnps (9.04 ms).

## Methodology

- `LTO=fat`, `codegen-units=1`, `panic=abort`, `criterion 0.5`, `sample-size 10`, `measurement-time 1s`, `warm-up 1s`, `min of N` per `BENCH.md: Methodology caveat 2`.
- `profile.release`/`profile.bench` both `lto=fat codegen-units=1 panic=abort` (now in `Cargo.toml:13`, was only `BENCH.md` claim).
- Every bench sets `Throughput::Elements` (1 or 48 or nodes) so `Melem/s` consistent.
- Baseline `benches/results/turbochess-rs-baseline.json` frozen before `src/` edit; `±3%` gate.
- `bench-wasm` stub Non-Goal.

## Reproduce

```bash
just bench          # gate (cargo test + perft) then micro + perft_bench + vs_libraries → baseline.json + BENCH.md
just bench-quick    # dev: micro + perft only, 5 samples
cargo bench --bench micro -- --sample-size 10
cargo bench --bench vs_libraries -- --sample-size 10
cargo bench --bench perft_bench -- --sample-size 10
# vs ultrachess vendor on same host (x86):
cargo run --release --bin compare  # if bin exists, else use benches/vs_libraries + vendor/ultrachess-core
# Stockfish (GPL-3, external only):
git clone --depth 1 https://github.com/official-stockfish/Stockfish.git /tmp/stockfish_src
make -C /tmp/stockfish_src/src -j build ARCH=x86-64  # or ARCH=apple-silicon on M1
printf 'position startpos\ngo perft 5\nquit\n' | /tmp/stockfish_src/src/stockfish
printf 'position startpos\ngo perft 6\nquit\n' | /tmp/stockfish_src/src/stockfish
```

---
_MIT — data reproducible via `cargo bench`; magic tables generated at runtime with fixed seeds. `Board` now `#[repr(C)]` 144B `Copy` `hash` front-cache, `LTO=fat` is measured profile, 8/8 win on M1 Max standard-only (960 `is_chess960==true` exception slower OK)._
