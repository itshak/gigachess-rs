#!/bin/bash
set -euo pipefail
source ~/.cargo/env
cd ~/giga-rs-round2-prepared-20261126
out=~/giga-rs-evidence-20261126/round2-final
base=exp-static-sliders,exp-ray-prefilter,exp-san,exp-uci,exp-fused-perft,exp-copy-perft,exp-mutation,exp-compact-board,exp-direct-legal,exp-early-san,exp-perft-kernel,exp-quiet-fast,exp-fused-geometry,exp-uci-direct,exp-ep-candidates
d="$base,exp-normal-kernel,exp-checker-rays,exp-checked-direct,exp-uci-bytes,exp-hash-unroll,exp-full-visitor"
e="$base,exp-checker-rays,exp-full-visitor,exp-color-make,exp-checked-known,exp-unmake-xor,exp-known-role,exp-static-unchecked,exp-uci-sort-key,exp-compact-attacks,exp-bool-attacks,exp-promotion-batch,exp-perft-xor"
extras=exp-prepared-move,exp-kingless-board,exp-nibble-role,exp-role-planes,exp-fen-rank,exp-leaf-reuse,exp-castle-hash-table
git rev-parse HEAD > "$out/commit.txt"
echo "TEST E plus all additional layout/prepared experiments $(date -u)"
export RUSTFLAGS="-C target-cpu=native"
export CARGO_TARGET_DIR=~/giga-rs-regression-local-20261126/target
cargo test --locked --release --features "$e,$extras,pext" > "$out/tests.log" 2>&1
echo "PASS $(date -u)"
# Same commit and compiler for D/E. Only run target rows and all competing
# libraries, so timing remains a preliminary screen on a busy machine.
export CARGO_TARGET_DIR=~/giga-rs-bench-20261126/benchmarks/competitive/target
build() {
  cargo build --locked --release --manifest-path benchmarks/competitive/Cargo.toml --bin gigachess-competitive-bench --features "$2" > "$out/$1-build.log" 2>&1
  cp "$CARGO_TARGET_DIR/release/gigachess-competitive-bench" "$out/$1-bin"
}
echo "BUILD E $(date -u)"
build E "$e,exp-prepared-move"
echo "METADATA BENCH $(date -u)"
BENCH_MS=80 "$out/E-bin" --prepared-replay > "$out/prepared-replay.tsv" 2>"$out/prepared-validation.txt"
# Build separate E without prepared flag so the comparison is exact.
build E-native "$e"
build D "$d"
export BENCH_MS=20
export BENCH_FILTER=perft_full_d3,perft_bulk_d4,corpus_uci_sorted,apply_children_full,make_unmake_full,replay_trusted_native,replay_checked_native,polyglot_recompute,snapshot
for pair in baseline:1 candidate:1 candidate:2 baseline:2;do
 side=${pair%:*};rep=${pair#*:};bin=D;[[ "$side" == candidate ]]&&bin=E-native
 "$out/$bin-bin" > "$out/$side-$rep.tsv" 2>"$out/$side-$rep-validation.txt"
done
echo "DONE $(date -u)"
