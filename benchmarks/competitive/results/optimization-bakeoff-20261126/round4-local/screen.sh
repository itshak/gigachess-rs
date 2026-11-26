#!/bin/bash
set -euo pipefail
source ~/.cargo/env
cd ~/giga-rs-round4-copy-20261126
out=~/giga-rs-evidence-20261126/round4
e=exp-static-sliders,exp-ray-prefilter,exp-san,exp-uci,exp-fused-perft,exp-copy-perft,exp-mutation,exp-compact-board,exp-direct-legal,exp-early-san,exp-perft-kernel,exp-quiet-fast,exp-fused-geometry,exp-uci-direct,exp-ep-candidates,exp-checker-rays,exp-full-visitor,exp-color-make,exp-checked-known,exp-unmake-xor,exp-known-role,exp-static-unchecked,exp-uci-sort-key,exp-compact-attacks,exp-bool-attacks,exp-promotion-batch,exp-perft-xor,exp-fen-rank
candidate=exp-ordinary-kernel,exp-pawn-kernel,exp-child-copy,exp-color-gen,exp-bulk-write,exp-checker-first,exp-hash-pawn-bytes,exp-align16
export RUSTFLAGS="-C target-cpu=native"
export CARGO_TARGET_DIR=~/giga-rs-regression-local-20261126/target
git rev-parse HEAD > "$out/commit.txt"
echo "FULL RELEASE TEST $(date -u)"
cargo test --locked --release --features "$e,$candidate" > "$out/tests.log" 2>&1
echo "PASS FULL RELEASE $(date -u)"
cargo test --locked --release --features "$e,$candidate" --test perft -- --ignored > "$out/deep-perft.log" 2>&1 || { cat "$out/deep-perft.log"; exit 1; }
echo "PASS DEEP PERFT $(date -u)"
export CARGO_TARGET_DIR=~/giga-rs-bench-20261126/benchmarks/competitive/target
build(){
 cargo build --locked --release --manifest-path benchmarks/competitive/Cargo.toml --bin gigachess-competitive-bench --features "$2" > "$out/$1-build.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/gigachess-competitive-bench" "$out/$1-bin"
 "$out/$1-bin" --sizes > "$out/$1-sizes.txt"
}
echo "BUILD F $(date -u)"
build F "$e"
echo "BUILD G $(date -u)"
build G "$e,$candidate"
export BENCH_MS=25
for pair in baseline:1 candidate:1 candidate:2 baseline:2;do
 side=${pair%:*};rep=${pair#*:};bin=F;[[ "$side" == candidate ]]&&bin=G
 "$out/$bin-bin" > "$out/$side-$rep.tsv" 2>"$out/$side-$rep-validation.txt"
done
python3 benchmarks/competitive/compare_ab.py "$out" > "$out/ab-summary.md"
echo "DONE $(date -u)"
