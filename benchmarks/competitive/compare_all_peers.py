#!/usr/bin/env python3
"""Summarize a complete paired candidate benchmark. No missing rows treated as wins."""
import collections
import csv
import math
import pathlib
import statistics
import sys

root=pathlib.Path(sys.argv[1])
libs=["shakmaty","cozy","ultra","chess","ferrum"]
def geo(values):
    return math.exp(statistics.mean(map(math.log,values)))
def read(side,rep):
    with (root/f"{side}-{rep}.tsv").open(newline="") as f:
        return {(r["operation"],r["case"],r["library"]):float(r["median_ns"]) for r in csv.DictReader(f,delimiter="\t")}
b1,b2,c1,c2=(read(s,r) for s,r in (("baseline",1),("baseline",2),("candidate",1),("candidate",2)))
assert b1.keys()==b2.keys()==c1.keys()==c2.keys()
b={k:geo([b1[k],b2[k]]) for k in b1}
c={k:geo([c1[k],c2[k]]) for k in c1}
ops=collections.defaultdict(list)
for op,case,lib in b:
    if lib=="giga":ops[op].append(case)
print("| Operation | Own baseline | "+" | ".join(libs)+" | All-peer wins |")
print("|---|---:|"+"---:|"*(len(libs)+1))
for op,cases in ops.items():
    own=[b[op,x,"giga"]/c[op,x,"giga"] for x in cases]
    def fmt(lib):
        xs=[c[op,x,lib]/c[op,x,"giga"] for x in cases if (op,x,lib) in c]
        return f"{geo(xs):.2f}x" if xs else "N/A"
    wins=total=0
    for x in cases:
        for lib in libs:
            if (op,x,lib) not in c:continue
            total+=1
            wins+= c[op,x,lib] > c[op,x,"giga"]
    print(f"| {op} | {geo(own):.2f}x | "+" | ".join(map(fmt,libs))+f" | {wins}/{total} |")
print("\nNative output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.")
