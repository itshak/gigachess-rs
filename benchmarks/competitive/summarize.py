#!/usr/bin/env python3
"""Convert matched medians to per-operation geometric mean throughput ratios."""
import collections
import csv
import math
import sys

groups = collections.defaultdict(dict)
with open(sys.argv[1], newline="") as f:
    for row in csv.DictReader(f, delimiter="\t"):
        key = (row["operation"], row["case"])
        assert row["library"] not in groups[key], key
        groups[key][row["library"]] = float(row["median_ns"])
libs = ["shakmaty", "cozy", "ultra", "chess", "ferrum"]
print("| Operation | " + " | ".join(libs) + " |")
print("|---|" + "---:|" * len(libs))
for op in dict.fromkeys(k[0] for k in groups):
    cases = [v for k, v in groups.items() if k[0] == op and "giga" in v]
    if not cases:
        continue
    vals = []
    for lib in libs:
        paired = [v[lib] / v["giga"] for v in cases if lib in v]
        vals.append(f"{math.exp(sum(map(math.log, paired))/len(paired)):.2f}x ({len(paired)})" if paired else "N/A")
    print("| " + op + " | " + " | ".join(vals) + " |")
print("\nHigher is better; parentheses show paired cases. Unsupported operations are N/A.")
