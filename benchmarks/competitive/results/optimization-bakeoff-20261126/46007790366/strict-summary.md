| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 69 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 85 | 85 | 13 | 87 |
| ubuntu-24.04-pext | 83 | 82 | 10 | 87 |
| ubuntu-24.04-portable | 85 | 78 | 12 | 87 |
| windows-2025-native | 85 | 83 | 12 | 87 |
| ALL | 491 | 476 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.029x | 1.265x | 35/36 | 0.970x |
| chess960_materialize | 1.056x | 1.469x | 12/12 | 1.265x |
| chess960_perft_d3 | 1.052x | 1.520x | 12/12 | 1.404x |
| corpus_count | 1.142x | 1.371x | 10/12 | 0.918x |
| corpus_materialize | 1.069x | 1.210x | 12/12 | 1.072x |
| corpus_uci_sorted | 0.984x | 1.529x | 12/12 | 1.151x |
| count | 1.041x | 1.499x | 34/36 | 0.919x |
| fen_parse | 0.978x | 1.191x | 35/36 | 0.916x |
| fen_write_native_ep | 0.988x | 1.633x | 36/36 | 1.207x |
| make_unmake_full | 1.013x | 1.332x | 35/36 | 1.000x |
| materialize | 1.008x | 1.245x | 33/36 | 0.931x |
| perft_bulk_d4 | 1.052x | 1.365x | 36/36 | 1.014x |
| perft_full_d3 | 1.233x | 1.553x | 36/36 | 1.078x |
| polyglot_key | 1.053x | 30.601x | 12/12 | 16.850x |
| polyglot_recompute | 1.069x | 1.076x | 8/12 | 0.865x |
| repetition_key | 1.024x | 21.181x | 12/12 | 14.553x |
| replay_checked_native | 1.033x | 1.168x | 12/12 | 1.065x |
| replay_trusted_native | 1.007x | 0.753x | 4/12 | 0.497x |
| san_parse_all | 1.031x | 1.927x | 36/36 | 1.362x |
| san_render_native | 1.012x | 2.692x | 36/36 | 1.823x |
| snapshot | 0.992x | 1.113x | 27/36 | 0.639x |
| startpos | 1.104x | 1.240x | 6/6 | 1.159x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.497x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.578x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.631x |
| macos-15-native | snapshot | start | cozy | 0.639x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.672x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.689x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.758x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.760x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.865x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.877x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.916x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.918x |
| macos-15-native | count | middlegame | cozy | 0.919x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.922x |
| macos-15-native | materialize | start | ultra | 0.931x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.939x |
| macos-15-native | materialize | tactical | ultra | 0.960x |
| macos-15-native | snapshot | promotion | ferrum | 0.962x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.964x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.967x |
| macos-15-native | apply_children_full | tactical | chess | 0.970x |
| macos-15-native | count | endgame | cozy | 0.993x |
| macos-15-native | snapshot | middlegame | ferrum | 0.993x |
| macos-15-native | materialize | middlegame | ultra | 0.999x |
| ubuntu-24.04-pext | make_unmake_full | kiwipete | ultra | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6292 | ferrum | 7166 | 1.139x |
| ubuntu-24.04-arm-native | 6655 | shakmaty | 7832 | 1.177x |
| ubuntu-24.04-native | 8266 | shakmaty | 12283 | 1.486x |
| ubuntu-24.04-pext | 5989 | shakmaty | 10526 | 1.758x |
| ubuntu-24.04-portable | 7514 | shakmaty | 11862 | 1.579x |
| windows-2025-native | 20200 | shakmaty | 26800 | 1.327x |

Cold wins: 6/6 (separate from native cases).
