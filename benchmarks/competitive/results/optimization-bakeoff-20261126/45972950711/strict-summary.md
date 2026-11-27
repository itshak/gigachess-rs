| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 71 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 77 | 12 | 87 |
| ubuntu-24.04-native | 81 | 74 | 13 | 87 |
| ubuntu-24.04-pext | 81 | 81 | 11 | 87 |
| ubuntu-24.04-portable | 81 | 78 | 11 | 87 |
| windows-2025-native | 83 | 81 | 12 | 87 |
| ALL | 480 | 462 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.955x | 1.221x | 34/36 | 0.942x |
| chess960_materialize | 1.013x | 1.479x | 12/12 | 1.220x |
| chess960_perft_d3 | 1.022x | 1.548x | 12/12 | 1.369x |
| corpus_count | 1.004x | 1.212x | 10/12 | 0.820x |
| corpus_materialize | 1.007x | 1.174x | 12/12 | 1.029x |
| corpus_uci_sorted | 1.014x | 1.500x | 12/12 | 1.200x |
| count | 0.997x | 1.488x | 36/36 | 1.008x |
| fen_parse | 0.987x | 1.177x | 33/36 | 0.973x |
| fen_write_native_ep | 1.030x | 1.740x | 36/36 | 1.359x |
| make_unmake_full | 1.013x | 1.296x | 35/36 | 1.000x |
| materialize | 0.994x | 1.317x | 34/36 | 0.814x |
| perft_bulk_d4 | 1.024x | 1.291x | 34/36 | 0.899x |
| perft_full_d3 | 0.986x | 1.281x | 36/36 | 1.015x |
| polyglot_key | 0.987x | 31.891x | 12/12 | 15.912x |
| polyglot_recompute | 1.003x | 1.061x | 10/12 | 0.894x |
| repetition_key | 0.969x | 21.085x | 12/12 | 14.245x |
| replay_checked_native | 0.976x | 1.119x | 11/12 | 0.889x |
| replay_trusted_native | 0.985x | 0.773x | 3/12 | 0.444x |
| san_parse_all | 1.004x | 1.869x | 36/36 | 1.404x |
| san_render_native | 1.000x | 2.537x | 36/36 | 1.802x |
| snapshot | 0.997x | 1.079x | 19/36 | 0.645x |
| startpos | 0.884x | 1.273x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.444x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.553x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.554x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.617x |
| macos-15-native | snapshot | endgame | ferrum | 0.645x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.668x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.696x |
| macos-15-native | materialize | kiwipete | ultra | 0.814x |
| macos-15-native | snapshot | promotion | cozy | 0.814x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.820x |
| macos-15-native | materialize | tactical | ultra | 0.847x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.889x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.894x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.899x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.902x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.907x |
| macos-15-native | snapshot | middlegame | chess | 0.911x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.916x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.928x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.938x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.942x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.942x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.948x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.970x |
| macos-15-native | fen_parse | tactical | shakmaty | 0.973x |
| macos-15-native | snapshot | start | cozy | 0.975x |
| ubuntu-24.04-pext | apply_children_full | promotion | chess | 0.984x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.989x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.989x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.990x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.992x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.994x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.996x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| windows-2025-native | startpos | start | shakmaty | 1.000x |
| windows-2025-native | make_unmake_full | tactical | ultra | 1.000x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 11292 | shakmaty | 11083 | 0.981x |
| ubuntu-24.04-arm-native | 5776 | shakmaty | 6936 | 1.201x |
| ubuntu-24.04-native | 5937 | shakmaty | 6609 | 1.113x |
| ubuntu-24.04-pext | 7090 | shakmaty | 11057 | 1.560x |
| ubuntu-24.04-portable | 4977 | shakmaty | 6574 | 1.321x |
| windows-2025-native | 16600 | shakmaty | 16900 | 1.018x |

Cold wins: 5/6 (separate from native cases).
