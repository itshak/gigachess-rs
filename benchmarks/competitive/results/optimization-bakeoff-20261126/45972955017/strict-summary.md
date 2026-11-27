| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 73 | 11 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 84 | 9 | 87 |
| ubuntu-24.04-pext | 79 | 75 | 8 | 87 |
| ubuntu-24.04-portable | 80 | 77 | 12 | 87 |
| windows-2025-native | 85 | 82 | 13 | 87 |
| ALL | 484 | 470 | 65 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.043x | 1.295x | 35/36 | 0.981x |
| chess960_materialize | 0.967x | 1.440x | 12/12 | 1.232x |
| chess960_perft_d3 | 1.005x | 1.525x | 12/12 | 1.373x |
| corpus_count | 1.016x | 1.228x | 10/12 | 0.996x |
| corpus_materialize | 1.014x | 1.156x | 12/12 | 1.037x |
| corpus_uci_sorted | 1.024x | 1.516x | 12/12 | 1.398x |
| count | 0.997x | 1.517x | 36/36 | 1.154x |
| fen_parse | 0.989x | 1.186x | 35/36 | 0.947x |
| fen_write_native_ep | 1.037x | 1.722x | 36/36 | 1.424x |
| make_unmake_full | 0.994x | 1.296x | 36/36 | 1.032x |
| materialize | 0.981x | 1.311x | 33/36 | 0.887x |
| perft_bulk_d4 | 1.017x | 1.290x | 34/36 | 0.888x |
| perft_full_d3 | 1.004x | 1.308x | 36/36 | 1.116x |
| polyglot_key | 1.020x | 30.746x | 12/12 | 15.926x |
| polyglot_recompute | 1.014x | 1.030x | 9/12 | 0.901x |
| repetition_key | 1.034x | 20.887x | 12/12 | 14.024x |
| replay_checked_native | 1.002x | 1.171x | 12/12 | 1.072x |
| replay_trusted_native | 1.027x | 0.824x | 5/12 | 0.498x |
| san_parse_all | 1.027x | 1.886x | 36/36 | 1.392x |
| san_render_native | 1.050x | 2.551x | 36/36 | 1.888x |
| snapshot | 1.012x | 1.035x | 18/36 | 0.906x |
| startpos | 0.974x | 1.155x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.498x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.665x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.677x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.695x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.779x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.841x |
| macos-15-native | materialize | kiwipete | ultra | 0.887x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.888x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.901x |
| macos-15-native | snapshot | promotion | ferrum | 0.906x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.917x |
| macos-15-native | snapshot | endgame | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.945x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.945x |
| macos-15-native | fen_parse | endgame | shakmaty | 0.947x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.967x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.972x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.974x |
| macos-15-native | apply_children_full | tactical | chess | 0.981x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.988x |
| macos-15-native | materialize | middlegame | ultra | 0.990x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.991x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.994x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.996x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.996x |
| macos-15-native | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.997x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |
| macos-15-native | materialize | tactical | ultra | 1.000x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 1.000x |
| ubuntu-24.04-pext | snapshot | start | cozy | 1.000x |
| ubuntu-24.04-pext | snapshot | kiwipete | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5958 | shakmaty | 11292 | 1.895x |
| ubuntu-24.04-arm-native | 5704 | shakmaty | 7168 | 1.257x |
| ubuntu-24.04-native | 4927 | shakmaty | 8177 | 1.660x |
| ubuntu-24.04-pext | 4949 | shakmaty | 8444 | 1.706x |
| ubuntu-24.04-portable | 8275 | shakmaty | 9027 | 1.091x |
| windows-2025-native | 12500 | shakmaty | 17000 | 1.360x |

Cold wins: 6/6 (separate from native cases).
