#!/bin/bash
set -euo pipefail
source ~/.cargo/env
cd ~/giga-rs-round3-20261126
out=~/giga-rs-evidence-20261126/round3
base=exp-static-sliders,exp-ray-prefilter,exp-san,exp-uci,exp-fused-perft,exp-copy-perft,exp-mutation,exp-compact-board,exp-direct-legal,exp-early-san,exp-perft-kernel,exp-quiet-fast,exp-fused-geometry,exp-uci-direct,exp-ep-candidates
e="$base,exp-checker-rays,exp-full-visitor,exp-color-make,exp-checked-known,exp-unmake-xor,exp-known-role,exp-static-unchecked,exp-uci-sort-key,exp-compact-attacks,exp-bool-attacks,exp-promotion-batch,exp-perft-xor"
extras=(exp-inline-make exp-split-quiet exp-role-select exp-attack-descriptor exp-fast-castle-keys exp-hash-role-scan exp-fen-byte-copy)
all=$(IFS=,;echo "${extras[*]}")
export RUSTFLAGS="-C target-cpu=native"
export CARGO_TARGET_DIR=~/giga-rs-regression-local-20261126/target
git rev-parse HEAD > "$out/commit.txt"
echo "CORRECTNESS ALL $(date -u)"
cargo test --locked --release --features "$e,$all" > "$out/tests.log" 2>&1
echo "PASS ALL $(date -u)"
export CARGO_TARGET_DIR=~/giga-rs-bench-20261126/benchmarks/competitive/target
export BENCH_MS=15 BENCH_LIB=giga
export BENCH_FILTER=apply_children_full,make_unmake_full,replay_trusted_native,replay_checked_native,perft_full_d3,perft_bulk_d4,corpus_uci_sorted,polyglot_recompute,fen_parse,fen_write_native_ep,corpus_count,corpus_materialize,snapshot
build(){
 cargo build --locked --release --manifest-path benchmarks/competitive/Cargo.toml --bin gigachess-competitive-bench --features "$2" > "$out/$1-build.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/gigachess-competitive-bench" "$out/$1-bin"
}
build E "$e"
for x in "${extras[@]}";do
 echo "BUILD $x $(date -u)"
 build "$x" "$e,$x"
 mkdir -p "$out/$x"
 for pair in baseline:1 candidate:1 candidate:2 baseline:2;do
  side=${pair%:*};rep=${pair#*:};bin=E;[[ "$side" == candidate ]]&&bin="$x"
  "$out/$bin-bin" > "$out/$x/$side-$rep.tsv" 2> "$out/$x/$side-$rep-validation.txt"
 done
 python3 - "$out/$x" <<'PY'
import csv,sys,pathlib,statistics,math
p=pathlib.Path(sys.argv[1])
def load(s,i):
 with (p/f'{s}-{i}.tsv').open() as f:return {(r['operation'],r['case']):float(r['median_ns']) for r in csv.DictReader(f,delimiter='\t')}
a,b,c,d=[load(s,i) for s,i in [('baseline',1),('candidate',1),('candidate',2),('baseline',2)]]
assert a.keys()==b.keys()==c.keys()==d.keys()
g={}
for k in a:g.setdefault(k[0],[]).append(math.sqrt(a[k]*d[k]/(b[k]*c[k])))
for k,v in g.items():print(f'{k:26} {math.exp(statistics.mean(map(math.log,v))):.3f}x worst {min(v):.3f}x')
PY
 echo "DONE $x $(date -u)"
done
