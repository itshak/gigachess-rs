| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 73 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 10 | 87 |
| ubuntu-24.04-pext | 77 | 73 | 11 | 87 |
| ubuntu-24.04-portable | 79 | 76 | 12 | 87 |
| windows-2025-native | 83 | 80 | 12 | 87 |
| ALL | 477 | 463 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.983x | 1.236x | 35/36 | 0.807x |
| chess960_materialize | 0.980x | 1.517x | 12/12 | 1.303x |
| chess960_perft_d3 | 0.989x | 1.498x | 12/12 | 1.365x |
| corpus_count | 1.007x | 1.199x | 9/12 | 0.810x |
| corpus_materialize | 1.001x | 1.152x | 12/12 | 1.059x |
| corpus_uci_sorted | 0.992x | 1.513x | 12/12 | 1.363x |
| count | 1.003x | 1.484x | 36/36 | 1.021x |
| fen_parse | 1.006x | 1.219x | 35/36 | 0.999x |
| fen_write_native_ep | 0.986x | 1.753x | 36/36 | 1.242x |
| make_unmake_full | 1.009x | 1.291x | 36/36 | 1.058x |
| materialize | 0.980x | 1.318x | 33/36 | 0.891x |
| perft_bulk_d4 | 0.998x | 1.282x | 34/36 | 0.911x |
| perft_full_d3 | 1.076x | 1.405x | 36/36 | 1.070x |
| polyglot_key | 0.983x | 32.023x | 12/12 | 16.076x |
| polyglot_recompute | 1.003x | 0.998x | 6/12 | 0.888x |
| repetition_key | 0.944x | 20.427x | 12/12 | 14.654x |
| replay_checked_native | 1.032x | 1.080x | 11/12 | 0.972x |
| replay_trusted_native | 1.010x | 0.773x | 3/12 | 0.514x |
| san_parse_all | 1.009x | 1.802x | 36/36 | 1.030x |
| san_render_native | 1.002x | 2.487x | 36/36 | 1.756x |
| snapshot | 0.944x | 1.018x | 18/36 | 0.933x |
| startpos | 0.985x | 1.184x | 5/6 | 0.988x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.514x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.537x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.673x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.704x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.725x |
| macos-15-native | apply_children_full | endgame | chess | 0.807x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.810x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.823x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.835x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.888x |
| macos-15-native | materialize | middlegame | ultra | 0.891x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.895x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.911x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.926x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.929x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.936x |
| macos-15-native | materialize | kiwipete | ultra | 0.937x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.938x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.939x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.939x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.939x |
| macos-15-native | materialize | tactical | ultra | 0.969x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.971x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.972x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.975x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.978x |
| macos-15-native | snapshot | promotion | ferrum | 0.984x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.985x |
| macos-15-native | snapshot | start | ferrum | 0.986x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.988x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.991x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 0.992x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.992x |
| macos-15-native | snapshot | middlegame | ferrum | 0.994x |
| ubuntu-24.04-pext | snapshot | tactical | cozy | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| ubuntu-24.04-pext | fen_parse | start | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.999x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4250 | shakmaty | 5375 | 1.265x |
| ubuntu-24.04-arm-native | 6655 | shakmaty | 8696 | 1.307x |
| ubuntu-24.04-native | 6139 | shakmaty | 8323 | 1.356x |
| ubuntu-24.04-pext | 5771 | shakmaty | 7969 | 1.381x |
| ubuntu-24.04-portable | 5194 | shakmaty | 7089 | 1.365x |
| windows-2025-native | 16000 | shakmaty | 23400 | 1.462x |

Cold wins: 6/6 (separate from native cases).
