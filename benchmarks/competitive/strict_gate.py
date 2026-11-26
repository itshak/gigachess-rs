"""Report strict case/platform wins from paired ABBA benchmark artifacts."""
import argparse
import csv
import gzip
import math
import pathlib
from statistics import median
from collections import defaultdict

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("run", type=pathlib.Path)
p.add_argument("--losses", action="store_true")
p.add_argument("--cold", action="store_true", help="Report cold first-use separately")
p.add_argument("--require-complete", action="store_true", help="Require all six platforms and 87 native cases each")
p.add_argument("--require-wins", action="store_true", help="Release gate: require all native cases and cold medians to win")
a = p.parse_args()

def read(folder, name):
    path = folder / (name + ".tsv")
    if not path.exists():
        path = path.with_suffix(".tsv.gz")
    opener = gzip.open if path.suffix == ".gz" else open
    with opener(path, "rt") as f:
        rows = list(csv.DictReader(f, delimiter="\t"))
    result = {(r["operation"], r["case"], r["library"]): float(r["median_ns"]) for r in rows}
    assert len(result) == len(rows), f"Duplicate rows: {path}"
    assert all(v > 0 and math.isfinite(v) for v in result.values()), path
    return result

def mean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))

results = []
for folder in sorted(a.run.glob("combined-*")):
    base1, base2, cand1, cand2 = [read(folder, n) for n in ("baseline-1", "baseline-2", "candidate-1", "candidate-2")]
    assert base1.keys() == base2.keys() == cand1.keys() == cand2.keys(), folder
    b = {k: mean([base1[k], base2[k]]) for k in base1}
    c = {k: mean([cand1[k], cand2[k]]) for k in cand1}
    platform = folder.name.removeprefix("combined-")
    for op, case, lib in c:
        if lib != "giga":
            continue
        peers = [(v, k[2]) for k, v in c.items() if k[:2] == (op, case) and k[2] != "giga" and not k[2].startswith("giga_")]
        if not peers:
            continue
        fastest, peer = min(peers)
        results.append((platform, op, case, fastest / c[op, case, lib], b[op, case, lib] / c[op, case, lib], peer))
assert results, "No comparable results"
expected_platforms={"macos-15-native","ubuntu-24.04-arm-native","ubuntu-24.04-native","ubuntu-24.04-pext","ubuntu-24.04-portable","windows-2025-native"}
if a.require_complete or a.require_wins:
    assert {r[0] for r in results}==expected_platforms, "Incomplete platform set"
    for platform in expected_platforms:
        rows=[r for r in results if r[0]==platform]
        assert len(rows)==87, f"Incomplete native case set: {platform}: {len(rows)}"
        assert len({(r[1],r[2]) for r in rows})==87, f"Duplicate native cases: {platform}"

print("| Platform | Wins | >=1.03x | >=2x | Total |")
print("|---|---:|---:|---:|---:|")
for platform in sorted({r[0] for r in results}) + ["ALL"]:
    rows = [r for r in results if r[0] == platform or platform == "ALL"]
    print(f"| {platform} | {sum(r[3]>1 for r in rows)} | {sum(r[3]>=1.03 for r in rows)} | {sum(r[3]>=2 for r in rows)} | {len(rows)} |")
print("\n| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |")
print("|---|---:|---:|---:|---:|")
groups = defaultdict(list)
for r in results:
    groups[r[1]].append(r)
for op, rows in sorted(groups.items()):
    print(f"| {op} | {mean([r[4] for r in rows]):.3f}x | {mean([r[3] for r in rows]):.3f}x | {sum(r[3]>1 for r in rows)}/{len(rows)} | {min(r[3] for r in rows):.3f}x |")
if a.losses:
    print("\n| Platform | Operation | Case | Fastest peer | Ratio |")
    print("|---|---|---|---|---:|")
    for platform, op, case, ratio, own, peer in sorted(results, key=lambda r:r[3]):
        if ratio <= 1:
            print(f"| {platform} | {op} | {case} | {peer} | {ratio:.3f}x |")

cold_rows=[]
if a.cold or a.require_wins:
    print("\n| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |")
    print("|---|---:|---|---:|---:|")
    for platform in sorted({r[0] for r in results}):
        folder=a.run/("combined-"+platform)
        def samples(name):
            path=folder/name
            values=list(map(int,path.read_text().split()))
            assert len(values)==7 and all(v>0 for v in values), path
            return median(values)
        giga=samples("candidate-cold-ns.txt")
        peers=[(samples(f"candidate-{lib}-cold-ns.txt"),lib) for lib in ("shakmaty","cozy","ultra","chess","ferrum")]
        fast,peer=min(peers)
        ratio=fast/giga
        cold_rows.append(ratio)
        print(f"| {platform} | {giga} | {peer} | {fast} | {ratio:.3f}x |")
    print(f"\nCold wins: {sum(v>1 for v in cold_rows)}/{len(cold_rows)} (separate from native cases).")
if a.require_wins:
    import sys
    sys.exit(0 if all(r[3]>1 for r in results) and all(v>1 for v in cold_rows) else 1)
