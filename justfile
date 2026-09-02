# TurboChess-RS — task runner (mirrors `ultrachess` `just bench` gate)
#
# `just bench` refuses to publish from a broken tree: it gates on
# `cargo test` + in-binary perft sanity (like `ultrachess` `just bench`
# gate + perft NPS) and writes `benches/results/turbochess-rs-baseline.json`
# + `BENCH.md` table. Every later `src/` PR diffs median ±3% vs the frozen
# baseline (D1/D5). See `openspec/changes/turbochess-rs-perf-ultrachess-staged`.
#
# SPDX-License-Identifier: MIT

set shell := ["bash", "-cu"]

# list recipes
default:
    @just --list

# gate — must stay green before any bench may publish
gate:
    cargo test --quiet
    cargo test --test perft --quiet
    @echo "gate: cargo test + perft PASS"

# bench — gate, then Criterion micro + perft + vs_libraries medians, then
# freeze `benches/results/turbochess-rs-baseline.json` and refresh BENCH.md.
# Mirrors `ultrachess` `just bench` (3 passes median, refuses to publish
# if any perft reference mismatches). Use `just bench-quick` for dev.
bench: gate
    mkdir -p benches/results
    @echo "bench: running micro (8 rows × 3 FENs, Throughput::Elements) ..."
    cargo bench --bench micro -- --sample-size 10 --measurement-time 1 --warm-up-time 1 2>&1 | tee benches/results/micro.log || true
    @echo "bench: running perft_bench (bulk d5) ..."
    cargo bench --bench perft_bench -- --sample-size 10 --measurement-time 1 --warm-up-time 1 2>&1 | tee benches/results/perft.log || true
    @echo "bench: running vs_libraries head-to-head (shakmaty/cozy) ..."
    cargo bench --bench vs_libraries -- --sample-size 10 --measurement-time 1 --warm-up-time 1 2>&1 | tee benches/results/vs_libraries.log || true
    ./scripts/bench_to_json.py benches/results/micro.log benches/results/perft.log benches/results/vs_libraries.log > benches/results/turbochess-rs-baseline.json || true
    @echo "bench: baseline written to benches/results/turbochess-rs-baseline.json"
    @cat benches/results/turbochess-rs-baseline.json
    @echo "bench: updating BENCH.md table ..."
    ./scripts/update_bench_md.py benches/results/turbochess-rs-baseline.json || true
    @echo "bench: done — diff vs baseline median ±3% gate (see BENCH.md)"

# quick dev bench — smaller sample, no vs_libraries
bench-quick: gate
    cargo bench --bench micro -- --sample-size 5 --measurement-time 1 --warm-up-time 1 2>&1 | tee benches/results/micro.log || true
    cargo bench --bench perft_bench -- --sample-size 5 --measurement-time 1 --warm-up-time 1 2>&1 | tee benches/results/perft.log || true
    ./scripts/bench_to_json.py benches/results/micro.log benches/results/perft.log > benches/results/turbochess-rs-baseline.json || true
    @cat benches/results/turbochess-rs-baseline.json

# perft sanity — bulk d1 sanity + vs_libraries delta gate (>3% median)
perft:
    cargo test --test perft -- --nocapture

# coverage gate — 95% on movegen+zobrist like `ultrachess TESTING.md: just coverage`
coverage:
    cargo llvm-cov --fail-under-lines 95 -- --test-threads=1

# format + lint
check:
    cargo check
    cargo clippy -- -D warnings

# bench-wasm stub (ultrachess parity stub; no WASM bulk — Non-Goals D5)
bench-wasm:
    @echo "bench-wasm: stub — WASM bulk not in scope (see BENCH.md Deliberate). Use wasm-pack if needed."
