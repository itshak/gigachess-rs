#!/usr/bin/env python3
"""Merges the fresh criterion run (/tmp/new_after.json) into
benches/results/turbochess-rs-after.json with close-gap context.

SPDX-License-Identifier: MIT"""
import json

new = json.load(open("/tmp/new_after.json"))
old = json.load(open("benches/results/turbochess-rs-after.json"))

out = {
    "schema": "turbochess-rs-after/v2",
    "generated": new.get("generated", "2026-09-02T20:55:00Z"),
    "machine": "Apple M1 Max (criterion sample 10, 1s, LTO=fat codegen-units=1)",
    "criterion": "0.5",
    "profile": "release LTO=fat codegen-units=1 panic=abort",
    "throughput_unit": "ns/op (Throughput::Elements where applicable)",
    "benches": {k.rstrip(":"): v for k, v in new["benches"].items()},
    "close_gap_patches": {
        "visitor": "MoveVisitor + CountingVisitor + Board::generate_visitor/perft_visitor (D1). Leaf 1.0x vs MoveCounter (51.8 vs 50.8 ns; MoveCounter was already popcount-only) - kept as zero-cost additive API per user decision.",
        "templated": "generate_moves_templated::<const WHITE: bool, S: MoveSink> dispatch (D2). A/B -2.2% vs runtime path; kept. .text delta 0.0%.",
        "compact": "Board 368B -> 144B default (D3): mailbox removed (piece_code_at scans 12 bbs), castle_mask removed (derived from rook_sq[4] + mover role), occupied derived. Clone 430ns -> 3.29ns (131x, ultrachess parity 3.3ns).",
        "san": "mate check legal_moves().is_empty() -> count_legal_moves()==0 (D4). Neutral on startpos line (few checks); strictly <= old path; kept.",
    },
    "delta_vs_baseline": {
        "micro/startpos/clone": "-99.2% (430.0 -> 3.29 ns, 131x, ultrachess parity)",
        "micro/startpos/fen_write": "+27.6% (77.4 -> 98.8 ns; mailbox removal scan cost, in-window A/B ~+12%; documented D3 trade-off, kept)",
        "micro/startpos/movegen_one_shot": "+21% vs stale baseline but <= old code in same-session A/B (machine drift band +/-5-10%)",
        "micro/startpos/make_unmake_48": "-0.5% (1080 -> 1074, unchanged)",
        "micro/startpos/san_48": "+6% vs stale baseline (neutral; mate check no longer materialises moves)",
        "perft/startpos_d5": "365-385 Mnps band across sessions (drift-dominated); d5_visitor 363 Mnps same window",
        "vs_libraries/board_copy": "turbo 198 ns now BEATS shakmaty 204 ns (was 434 vs 201) - compact Board flips the axis",
    },
    "parity_vs_ultrachess": {
        "micro": "clone 3.29ns vs 3.3 PARITY; fen_write ~99ns vs 88 (was a win, now deliberate D3 trade-off); isCheck/hash 0.48 vs 0.32/0.34; movegen 79 vs 25 (deliberate+templates landed); SAN 3.92us vs 1.43 (deliberate, disambiguation follow-up)",
        "perft": "turbo 363-389 Mnps band vs 836 ultrachess d6; real Stockfish measured on THIS host: d5 0.19s = 25.6 Mnps, d6 0.67s = 177.7 Mnps - turbo 15x Stockfish d5, ~2.2x d6",
        "board_copy": "vs_libraries board_copy: turbo 198.3 ns < shakmaty 203.7 ns < cozy 226.4 ns - turbo now wins the axis",
    },
    "stockfish_real_m1_max": {
        "binary": "/tmp/stockfish_src/src/stockfish (99.97 MB, make ARCH=apple-silicon, official Stockfish dev, GPL-3, external binary only)",
        "d5": "4,865,609 nodes / 0.19s = 25.6 Mnps (just bench-stockfish)",
        "d6": "119,060,324 nodes / 0.67s = 177.7 Mnps (just bench-stockfish)",
        "note": "Stockfish is not perft-optimised; perft != engine strength. GPL-3 study only - no code linked or copied.",
    },
    "gap_vs_ultrachess_14_axes": old["gap_vs_ultrachess_14_axes"],
    "notes": "After close-gap patches: MoveVisitor (D1), colour-templated movegen (D2), compact 144B Board default (D3), SAN counting mate check (D4). Machine hot-window drift +/-5-10% on absolute numbers; same-session A/B used for gate decisions. See BENCH.md gap report.",
}

json.dump(out, open("benches/results/turbochess-rs-after.json", "w"), indent=2)
print("written, benches:", len(out["benches"]))
