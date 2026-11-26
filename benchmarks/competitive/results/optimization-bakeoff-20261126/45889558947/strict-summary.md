| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 80 | 73 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 85 | 85 | 15 | 87 |
| ubuntu-24.04-pext | 76 | 71 | 13 | 87 |
| ubuntu-24.04-portable | 79 | 76 | 9 | 87 |
| windows-2025-native | 83 | 79 | 11 | 87 |
| ALL | 482 | 462 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.005x | 1.263x | 33/36 | 0.866x |
| chess960_materialize | 0.980x | 1.488x | 12/12 | 1.177x |
| chess960_perft_d3 | 0.959x | 1.508x | 12/12 | 1.140x |
| corpus_count | 0.984x | 1.225x | 10/12 | 0.854x |
| corpus_materialize | 0.998x | 1.177x | 12/12 | 1.044x |
| corpus_uci_sorted | 0.984x | 1.524x | 12/12 | 1.253x |
| count | 1.008x | 1.495x | 36/36 | 1.013x |
| fen_parse | 0.992x | 1.153x | 33/36 | 0.991x |
| fen_write_native_ep | 1.006x | 1.717x | 36/36 | 1.363x |
| make_unmake_full | 0.995x | 1.304x | 36/36 | 1.024x |
| materialize | 1.001x | 1.303x | 36/36 | 1.000x |
| perft_bulk_d4 | 0.981x | 1.298x | 35/36 | 0.915x |
| perft_full_d3 | 0.998x | 1.318x | 36/36 | 1.096x |
| polyglot_key | 1.013x | 30.079x | 12/12 | 16.526x |
| polyglot_recompute | 1.013x | 1.046x | 8/12 | 0.895x |
| repetition_key | 0.986x | 20.766x | 12/12 | 14.712x |
| replay_checked_native | 1.005x | 1.169x | 12/12 | 1.050x |
| replay_trusted_native | 0.984x | 0.810x | 4/12 | 0.503x |
| san_parse_all | 0.986x | 1.894x | 36/36 | 1.364x |
| san_render_native | 0.993x | 2.577x | 36/36 | 1.683x |
| snapshot | 0.989x | 1.020x | 17/36 | 0.716x |
| startpos | 1.077x | 1.200x | 6/6 | 1.003x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.503x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.546x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.651x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.674x |
| macos-15-native | snapshot | endgame | ferrum | 0.716x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.753x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.826x |
| windows-2025-native | snapshot | kiwipete | chess | 0.843x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.854x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.866x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.895x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.902x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.915x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.920x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.954x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.962x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.962x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.965x |
| windows-2025-native | snapshot | start | chess | 0.970x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.971x |
| macos-15-native | apply_children_full | tactical | chess | 0.977x |
| macos-15-native | snapshot | tactical | ferrum | 0.982x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.990x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.991x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.994x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.995x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.996x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.996x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.998x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5292 | shakmaty | 10000 | 1.890x |
| ubuntu-24.04-arm-native | 5296 | shakmaty | 7263 | 1.371x |
| ubuntu-24.04-native | 7100 | shakmaty | 9093 | 1.281x |
| ubuntu-24.04-pext | 6313 | shakmaty | 7821 | 1.239x |
| ubuntu-24.04-portable | 6259 | shakmaty | 8683 | 1.387x |
| windows-2025-native | 15000 | shakmaty | 19700 | 1.313x |

Cold wins: 6/6 (separate from native cases).
