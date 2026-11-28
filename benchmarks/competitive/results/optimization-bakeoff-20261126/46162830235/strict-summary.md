| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 74 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 85 | 83 | 10 | 87 |
| ubuntu-24.04-pext | 84 | 83 | 11 | 87 |
| ubuntu-24.04-portable | 82 | 79 | 14 | 87 |
| windows-2025-native | 84 | 82 | 12 | 87 |
| ALL | 491 | 480 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.982x | 1.242x | 36/36 | 1.007x |
| chess960_materialize | 0.928x | 1.426x | 12/12 | 1.091x |
| chess960_perft_d3 | 0.993x | 1.539x | 12/12 | 1.349x |
| corpus_count | 1.078x | 1.291x | 10/12 | 0.903x |
| corpus_materialize | 1.009x | 1.147x | 12/12 | 1.048x |
| corpus_uci_sorted | 0.929x | 1.421x | 12/12 | 1.020x |
| count | 1.038x | 1.522x | 36/36 | 1.078x |
| fen_parse | 0.988x | 1.177x | 36/36 | 1.006x |
| fen_write_native_ep | 1.003x | 1.662x | 36/36 | 1.287x |
| make_unmake_full | 1.020x | 1.321x | 36/36 | 1.033x |
| materialize | 1.009x | 1.313x | 34/36 | 0.916x |
| perft_bulk_d4 | 1.067x | 1.341x | 33/36 | 0.907x |
| perft_full_d3 | 1.176x | 1.518x | 36/36 | 1.121x |
| polyglot_key | 0.980x | 35.635x | 12/12 | 15.927x |
| polyglot_recompute | 1.538x | 1.525x | 12/12 | 1.205x |
| repetition_key | 1.009x | 23.289x | 12/12 | 14.322x |
| replay_checked_native | 1.015x | 1.208x | 12/12 | 1.095x |
| replay_trusted_native | 0.997x | 0.739x | 4/12 | 0.524x |
| san_parse_all | 1.011x | 1.803x | 36/36 | 1.270x |
| san_render_native | 1.003x | 2.647x | 36/36 | 1.872x |
| snapshot | 0.992x | 1.164x | 22/36 | 0.646x |
| startpos | 0.993x | 1.247x | 4/6 | 0.989x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.524x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.527x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.578x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.578x |
| macos-15-native | snapshot | endgame | ferrum | 0.646x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.669x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.678x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.705x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.721x |
| macos-15-native | snapshot | tactical | ferrum | 0.824x |
| macos-15-native | snapshot | kiwipete | cozy | 0.840x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.903x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.907x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.907x |
| macos-15-native | materialize | tactical | ultra | 0.916x |
| macos-15-native | materialize | kiwipete | ultra | 0.932x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.952x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.953x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.958x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.960x |
| macos-15-native | snapshot | start | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.962x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.962x |
| macos-15-native | snapshot | middlegame | ferrum | 0.985x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.989x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.994x |
| windows-2025-native | startpos | start | shakmaty | 0.996x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.997x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7584 | shakmaty | 12291 | 1.621x |
| ubuntu-24.04-arm-native | 6600 | shakmaty | 8279 | 1.254x |
| ubuntu-24.04-native | 9063 | shakmaty | 9133 | 1.008x |
| ubuntu-24.04-pext | 5695 | shakmaty | 7272 | 1.277x |
| ubuntu-24.04-portable | 6486 | shakmaty | 8262 | 1.274x |
| windows-2025-native | 15000 | shakmaty | 18800 | 1.253x |

Cold wins: 6/6 (separate from native cases).
