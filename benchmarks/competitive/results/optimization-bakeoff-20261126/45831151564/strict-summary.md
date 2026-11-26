| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 78 | 74 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 76 | 11 | 87 |
| ubuntu-24.04-native | 81 | 79 | 9 | 87 |
| ubuntu-24.04-pext | 81 | 78 | 12 | 87 |
| ubuntu-24.04-portable | 77 | 76 | 14 | 87 |
| windows-2025-native | 81 | 78 | 12 | 87 |
| ALL | 476 | 461 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.014x | 1.250x | 35/36 | 0.982x |
| chess960_materialize | 0.997x | 1.381x | 12/12 | 1.243x |
| chess960_perft_d3 | 1.006x | 1.370x | 12/12 | 1.184x |
| corpus_count | 1.009x | 1.094x | 9/12 | 0.783x |
| corpus_materialize | 0.997x | 1.128x | 12/12 | 1.051x |
| corpus_uci_sorted | 1.007x | 1.504x | 12/12 | 1.330x |
| count | 0.994x | 1.284x | 34/36 | 0.823x |
| fen_parse | 1.064x | 1.237x | 36/36 | 1.074x |
| fen_write_native_ep | 1.035x | 1.712x | 36/36 | 1.326x |
| make_unmake_full | 0.992x | 1.302x | 36/36 | 1.023x |
| materialize | 1.005x | 1.224x | 34/36 | 0.788x |
| perft_bulk_d4 | 0.982x | 1.152x | 32/36 | 0.841x |
| perft_full_d3 | 0.989x | 1.251x | 35/36 | 0.925x |
| polyglot_key | 1.005x | 30.648x | 12/12 | 16.581x |
| polyglot_recompute | 1.003x | 0.991x | 7/12 | 0.781x |
| repetition_key | 1.001x | 21.280x | 12/12 | 14.307x |
| replay_checked_native | 1.074x | 1.137x | 11/12 | 0.952x |
| replay_trusted_native | 1.020x | 0.767x | 2/12 | 0.551x |
| san_parse_all | 0.990x | 1.865x | 36/36 | 1.301x |
| san_render_native | 0.987x | 2.559x | 36/36 | 1.601x |
| snapshot | 1.011x | 1.090x | 19/36 | 0.938x |
| startpos | 1.036x | 1.242x | 6/6 | 1.126x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.551x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.562x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.569x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.642x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.680x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.781x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.783x |
| macos-15-native | materialize | kiwipete | ultra | 0.788x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.797x |
| macos-15-native | count | endgame | cozy | 0.823x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.833x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.841x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.882x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.886x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.906x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.919x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.922x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.925x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.929x |
| ubuntu-24.04-arm-native | snapshot | middlegame | cozy | 0.938x |
| ubuntu-24.04-arm-native | snapshot | endgame | cozy | 0.940x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.942x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.942x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.943x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.945x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.952x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.952x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.954x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.966x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.967x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.976x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.978x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.979x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.982x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.982x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.982x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.985x |
| macos-15-native | snapshot | endgame | ferrum | 0.989x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| windows-2025-native | snapshot | start | cozy | 0.993x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.993x |
| windows-2025-native | snapshot | middlegame | cozy | 0.996x |
| windows-2025-native | snapshot | endgame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5166 | cozy | 10542 | 2.041x |
| ubuntu-24.04-arm-native | 6408 | chess | 8992 | 1.403x |
| ubuntu-24.04-native | 5989 | shakmaty | 8142 | 1.359x |
| ubuntu-24.04-pext | 7018 | shakmaty | 10178 | 1.450x |
| ubuntu-24.04-portable | 6580 | shakmaty | 8332 | 1.266x |
| windows-2025-native | 19100 | shakmaty | 26300 | 1.377x |

Cold wins: 6/6 (separate from native cases).
