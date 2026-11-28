| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 75 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 85 | 83 | 16 | 87 |
| ubuntu-24.04-pext | 84 | 84 | 10 | 87 |
| ubuntu-24.04-portable | 84 | 79 | 11 | 87 |
| windows-2025-native | 86 | 86 | 13 | 87 |
| ALL | 494 | 486 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.991x | 1.260x | 35/36 | 0.925x |
| chess960_materialize | 1.030x | 1.480x | 12/12 | 1.313x |
| chess960_perft_d3 | 1.007x | 1.570x | 12/12 | 1.374x |
| corpus_count | 1.163x | 1.359x | 11/12 | 0.919x |
| corpus_materialize | 0.960x | 1.119x | 11/12 | 0.779x |
| corpus_uci_sorted | 1.012x | 1.560x | 12/12 | 1.414x |
| count | 1.041x | 1.524x | 36/36 | 1.073x |
| fen_parse | 0.946x | 1.163x | 34/36 | 0.850x |
| fen_write_native_ep | 0.965x | 1.688x | 36/36 | 1.277x |
| make_unmake_full | 1.003x | 1.300x | 36/36 | 1.034x |
| materialize | 1.039x | 1.333x | 36/36 | 1.050x |
| perft_bulk_d4 | 1.067x | 1.368x | 35/36 | 0.908x |
| perft_full_d3 | 1.224x | 1.610x | 36/36 | 1.328x |
| polyglot_key | 1.002x | 33.842x | 12/12 | 16.199x |
| polyglot_recompute | 1.485x | 1.540x | 12/12 | 1.259x |
| repetition_key | 0.959x | 21.940x | 12/12 | 14.563x |
| replay_checked_native | 1.019x | 1.225x | 12/12 | 1.102x |
| replay_trusted_native | 1.033x | 0.807x | 5/12 | 0.478x |
| san_parse_all | 0.996x | 1.847x | 36/36 | 1.419x |
| san_render_native | 1.009x | 2.544x | 36/36 | 1.838x |
| snapshot | 1.015x | 1.098x | 22/36 | 0.603x |
| startpos | 1.101x | 1.205x | 5/6 | 0.988x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.478x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.552x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.579x |
| macos-15-native | snapshot | endgame | ferrum | 0.603x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.674x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.696x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.704x |
| macos-15-native | corpus_materialize | seeded | ferrum | 0.779x |
| macos-15-native | snapshot | middlegame | chess | 0.790x |
| macos-15-native | snapshot | tactical | chess | 0.795x |
| macos-15-native | snapshot | start | cozy | 0.806x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.846x |
| macos-15-native | fen_parse | endgame | shakmaty | 0.850x |
| macos-15-native | snapshot | kiwipete | chess | 0.896x |
| macos-15-native | snapshot | promotion | chess | 0.906x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.908x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.919x |
| macos-15-native | apply_children_full | tactical | chess | 0.925x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.950x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.966x |
| windows-2025-native | startpos | start | shakmaty | 0.988x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5667 | shakmaty | 8625 | 1.522x |
| ubuntu-24.04-arm-native | 6639 | shakmaty | 8048 | 1.212x |
| ubuntu-24.04-native | 6894 | chess | 10246 | 1.486x |
| ubuntu-24.04-pext | 5869 | shakmaty | 8212 | 1.399x |
| ubuntu-24.04-portable | 5644 | chess | 8581 | 1.520x |
| windows-2025-native | 15600 | shakmaty | 18300 | 1.173x |

Cold wins: 6/6 (separate from native cases).
