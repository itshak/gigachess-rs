#!/usr/bin/env python3
"""Read the Lichess 2013-01 PGN (decompressed on stdin); retain 300 mainlines."""
import sys
import chess
import chess.pgn

n = 0
while n < 300:
    game = chess.pgn.read_game(sys.stdin)
    if game is None:
        raise RuntimeError("PGN ended before 300 games")
    assert not game.errors, game.errors
    assert game.board().fen() == chess.STARTING_FEN
    moves = list(game.mainline_moves())
    if not moves:
        continue
    print(" ".join(m.uci() for m in moves))
    n += 1
