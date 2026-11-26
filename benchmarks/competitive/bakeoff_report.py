#!/usr/bin/env python3
"""Extract per-candidate independent, paired performance evidence."""
import collections
import csv
import json
import math
import pathlib
import statistics
import sys

root=pathlib.Path(sys.argv[1])
primary={
 "mutation":"make_unmake_full", "undo-role":"perft_full_d3",
 "ray-prefilter":"replay_checked_native", "king-targets":"corpus_count",
 "copy-perft":"perft_full_d3", "fused-perft":"perft_bulk_d4",
 "san":"san_parse_all", "uci":"corpus_uci_sorted", "standard-castle":"apply_children_full",
 "role-lookup":"perft_full_d3", "static-sliders":"corpus_count",
 "compact-board":"snapshot", "mailbox":"apply_children_full",
 "direct-legal":"san_parse_all", "early-san":"san_render_native",
 "standard-movegen":"corpus_count", "ep-candidates":"corpus_count",
 "tagged-perft":"perft_bulk_d4", "perft-kernel":"perft_full_d3",
 "direct-hash":"replay_trusted_native", "quiet-fast":"replay_trusted_native",
 "standard-no-special":"corpus_count", "king-local":"corpus_count",
 "pin-rays":"corpus_count", "fused-geometry":"perft_bulk_d4",
 "uci-direct":"corpus_uci_sorted"
}
def geo(x):return math.exp(statistics.mean(map(math.log,x)))
def load(path):
    with path.open() as f:
        return {(r["operation"],r["case"],r["library"]):float(r["median_ns"]) for r in csv.DictReader(f,delimiter="\t")}
def artifact(d):
    b1,b2,c1,c2=[load(d/f"{s}-{i}.tsv") for s,i in (("baseline",1),("baseline",2),("candidate",1),("candidate",2))]
    assert b1.keys()==b2.keys()==c1.keys()==c2.keys(),str(d)
    grouped=collections.defaultdict(list)
    for k in b1:
        if k[2]!="giga":continue
        grouped[k[0]].append((b1[k]/c1[k],b2[k]/c2[k]))
    ops={}
    for op,pairs in grouped.items():
        first=geo([x[0] for x in pairs]);second=geo([x[1] for x in pairs])
        ops[op]={"ratio":geo([first,second]),"repeat_1":first,"repeat_2":second,
                 "worst_case":min(geo(pair) for pair in pairs),"cases":len(pairs)}
    cold_b=statistics.median(map(int,(d/"baseline-cold-ns.txt").read_text().split()))
    cold_c=statistics.median(map(int,(d/"candidate-cold-ns.txt").read_text().split()))
    return {"ops":ops,"cold_baseline_ns":cold_b,"cold_candidate_ns":cold_c,"cold_ratio":cold_b/cold_c}
results=collections.defaultdict(dict)
for d in sorted(root.glob("*/bakeoff-*")):
    if not (d/"candidate-2.tsv").exists() or not (d/"baseline-cold-ns.txt").exists():continue
    prefix="bakeoff-ubuntu-24.04-"
    name=d.name.removeprefix(prefix)
    arch="arm64" if name.startswith("arm-") else "x64"
    candidate=name.removeprefix("arm-")
    results[candidate][arch]={"run":d.parent.name,**artifact(d)}
print("| Candidate | Primary workload | x64 ratio | ARM64 ratio | Evidence |")
print("|---|---|---:|---:|---|")
for name,op in primary.items():
    data=results.get(name,{})
    def f(arch):
        if arch not in data:return "pending"
        value=data[arch]["ops"][op]
        return f'{value["ratio"]:.2f}x'
    print(f"| {name} | {op} | {f('x64')} | {f('arm64')} | "+("both" if len(data)==2 else "partial" if data else "pending")+" |")
print("\nRatios compare candidate to same-runner baseline. Two ABBA repeats; values within ~3% should not be treated as conclusive. See JSON for all operations and worst-case ratios.")
(root/"candidate-metrics.json").write_text(json.dumps(results,indent=2)+"\n")
