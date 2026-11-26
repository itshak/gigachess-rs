#!/usr/bin/env python3
"""Paired ABBA medians: geometric mean candidate/baseline throughput,
per operation, with per-case worst ratio and fastest external peer."""
import collections
import csv
import math
import pathlib
import statistics
import sys

root = pathlib.Path(sys.argv[1])
def load(side, rep):
    with (root / f"{side}-{rep}.tsv").open() as f:
        return {(r["operation"],r["case"],r["library"]):float(r["median_ns"]) for r in csv.DictReader(f,delimiter="\t")}
b1,b2,c1,c2=(load(s,r) for s,r in (("baseline",1),("baseline",2),("candidate",1),("candidate",2)))
assert b1.keys()==b2.keys()==c1.keys()==c2.keys(), "unequal benchmark coverage"
def geo(xs):
    return math.exp(statistics.mean(map(math.log,xs)))
base={k:geo([b1[k],b2[k]]) for k in b1}
cand={k:geo([c1[k],c2[k]]) for k in c1}
g=collections.defaultdict(list)
for k in base:
    op,case,lib=k
    if lib!="giga": continue
    peers=[v for (o,c,l),v in cand.items() if o==op and c==case and l!="giga" and not l.startswith("giga_")]
    g[op].append((base[k]/cand[k], min(peers)/cand[k] if peers else None))
print("| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |")
print("|---|---:|---:|---:|")
for op,rows in g.items():
    a=[x[0] for x in rows];p=[x[1] for x in rows if x[1] is not None]
    peer=f"{geo(p):.3f}x" if p else "N/A"
    print(f"| {op} | {geo(a):.3f}x | {min(a):.3f}x | {peer} |")
b=statistics.median(map(int,(root/"baseline-cold-ns.txt").read_text().split()))
c=statistics.median(map(int,(root/"candidate-cold-ns.txt").read_text().split()))
print(f"\nCold first legal move list: {b/1e6:.3f} ms -> {c/1e6:.3f} ms ({b/c:.2f}x).")
