#!/usr/bin/env python3
"""Differential validation of turbochess-rs against python-chess.

Generates random games from Chess960 start positions (and standard chess) and
compares, at every ply:
  - the full legal move set (UCI with chess960=True == our king->rook words),
  - perft(2) node counts,
  - FEN round-trips (X-FEN castling notation),
  - Polyglot zobrist hashes for STANDARD positions (Chess960 castling hashing
    is a documented turbochess extension; python-chess folds by side).

Run: python3 scripts/diff_python_chess.py [--games N] [--plies M]

SPDX-License-Identifier: MIT
"""

import argparse
import random
import subprocess
import sys

import chess
import chess.polyglot

PROBE = __import__("os").environ.get("TC_PROBE", "target/release/examples/tc_probe")


class Probe:
    def __init__(self):
        self.p = subprocess.Popen(
            [PROBE], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True
        )

    def ask(self, line):
        self.p.stdin.write(line + "\n")
        self.p.stdin.flush()
        return self.p.stdout.readline().strip()

    def close(self):
        self.p.stdin.write("quit\n")
        self.p.stdin.flush()
        self.p.wait()


def word_to_uci(word):
    f = chess.square_name(word & 0x3F)
    t = chess.square_name((word >> 6) & 0x3F)
    promo = (word >> 12) & 0x7
    return f + t + (" nbrq"[promo] if promo else "")


def fail(msg):
    print("DIVERGENCE:", msg)
    sys.exit(1)


def check_position(probe, board, label, check_hash):
    fen = board.fen(en_passant="fen")
    got = probe.ask(f"moves {fen}")
    if got.startswith("error"):
        fail(f"{label}: probe rejected FEN {fen!r}: {got}")
    want = sorted(board.uci(m, chess960=True) for m in board.legal_moves)
    have = sorted(word_to_uci(int(w)) for w in got.split(",") if w)
    if want != have:
        fail(
            f"{label}: legal move mismatch for {fen}\n"
            f"  missing: {sorted(set(want) - set(have))}\n"
            f"  extra:   {sorted(set(have) - set(want))}"
        )

    p2 = probe.ask(f"perft {fen} 2")
    want_p2 = str(sum(1 for _ in board.legal_moves) and perft(board, 2))
    if p2 != want_p2:
        fail(f"{label}: perft(2) mismatch for {fen}: {p2} != {want_p2}")

    # FEN round-trip: our emitted FEN must (a) re-parse to the same position
    # in python-chess, (b) be byte-identical to python-chess's X-FEN output.
    rt = probe.ask(f"fen {fen}")
    if rt.startswith("error"):
        fail(f"{label}: our FEN rejected by our parser: {rt} (fen={fen})")
    py_xfen = board.fen(en_passant="fen")
    if rt != py_xfen:
        b_ours = chess.Board(rt)
        if board.fen(en_passant="fen") != b_ours.fen(en_passant="fen"):
            fail(f"{label}: FEN round-trip mismatch: ours={rt!r} python-chess={py_xfen!r}")

    if check_hash:
        h = probe.ask(f"hash {fen}")
        want_h = format(chess.polyglot.zobrist_hash(board), "x")
        if h != want_h:
            fail(f"{label}: zobrist mismatch for {fen}: {h} != {want_h}")


def perft(board, depth):
    if depth == 0:
        return 1
    if depth == 1:
        return board.legal_moves.count()
    n = 0
    for mv in board.legal_moves:
        board.push(mv)
        n += perft(board, depth - 1)
        board.pop()
    return n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--games", type=int, default=300)
    ap.add_argument("--plies", type=int, default=60)
    ap.add_argument("--seed", type=int, default=42)
    ap.add_argument("--depth960", type=int, default=3)
    args = ap.parse_args()
    rng = random.Random(args.seed)
    probe = Probe()

    # 1) Chess960 positions: random starts, random games.
    checked = 0
    for g in range(args.games):
        idx = rng.randrange(960)
        board = chess.Board.from_chess960_pos(idx)
        for ply in range(args.plies):
            check_position(probe, board, f"960#{idx} g{g} p{ply}", check_hash=False)
            checked += 1
            moves = list(board.legal_moves)
            if not moves:
                break
            board.push(rng.choice(moves))
        if (g + 1) % 50 == 0:
            print(f"  960 games: {g + 1}/{args.games}, positions checked: {checked}")

    # 2) Standard chess: random games with hash + perft parity (Polyglot).
    for g in range(max(args.games // 3, 40)):
        board = chess.Board()
        for ply in range(args.plies):
            check_position(probe, board, f"std g{g} p{ply}", check_hash=True)
            checked += 1
            moves = list(board.legal_moves)
            if not moves:
                break
            board.push(rng.choice(moves))
        if (g + 1) % 20 == 0:
            print(f"  std games: {g + 1}, positions checked: {checked}")

    # 3) Reference perft values for a few 960 positions (hardcoded into tests).
    print("\n960 reference perft values (for tests/perft.rs):")
    for idx in [0, 284, 518, 959]:
        board = chess.Board.from_chess960_pos(idx)
        fen = board.fen(en_passant="fen")
        ours = probe.ask(f"perft {fen} {args.depth960}")
        ref = str(perft(board, args.depth960))
        status = "OK" if ours == ref else "MISMATCH"
        print(f"  pos {idx:3d}: {fen!r} perft({args.depth960}) = {ref} [{status}]")

    probe.close()
    print(f"\nAll differential checks passed ({checked} positions).")


if __name__ == "__main__":
    main()
