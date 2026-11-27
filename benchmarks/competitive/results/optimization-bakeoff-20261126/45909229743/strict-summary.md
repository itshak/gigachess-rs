| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 72 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 81 | 76 | 14 | 87 |
| ubuntu-24.04-pext | 86 | 83 | 13 | 87 |
| ubuntu-24.04-portable | 80 | 80 | 13 | 87 |
| windows-2025-native | 80 | 79 | 13 | 87 |
| ALL | 481 | 469 | 78 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.001x | 1.285x | 35/36 | 0.977x |
| chess960_materialize | 1.029x | 1.497x | 12/12 | 1.193x |
| chess960_perft_d3 | 0.999x | 1.522x | 12/12 | 1.316x |
| corpus_count | 1.100x | 1.402x | 12/12 | 1.025x |
| corpus_materialize | 1.004x | 1.199x | 12/12 | 1.024x |
| corpus_uci_sorted | 0.992x | 1.534x | 12/12 | 1.341x |
| count | 1.056x | 1.539x | 36/36 | 1.134x |
| fen_parse | 0.961x | 1.155x | 34/36 | 0.923x |
| fen_write_native_ep | 0.982x | 1.752x | 36/36 | 1.375x |
| make_unmake_full | 0.992x | 1.328x | 36/36 | 1.084x |
| materialize | 1.029x | 1.322x | 33/36 | 0.956x |
| perft_bulk_d4 | 1.051x | 1.369x | 35/36 | 0.957x |
| perft_full_d3 | 1.186x | 1.581x | 36/36 | 1.321x |
| polyglot_key | 0.991x | 29.513x | 12/12 | 16.725x |
| polyglot_recompute | 0.972x | 0.992x | 6/12 | 0.859x |
| repetition_key | 1.022x | 20.675x | 12/12 | 14.843x |
| replay_checked_native | 1.006x | 1.160x | 12/12 | 1.056x |
| replay_trusted_native | 0.997x | 0.843x | 5/12 | 0.581x |
| san_parse_all | 0.982x | 1.939x | 36/36 | 1.240x |
| san_render_native | 1.005x | 2.618x | 36/36 | 1.747x |
| snapshot | 0.973x | 1.036x | 15/36 | 0.624x |
| startpos | 0.995x | 1.259x | 6/6 | 1.128x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| macos-15-native | snapshot | promotion | ferrum | 0.624x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.672x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.695x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.699x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.700x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.701x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.844x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.859x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.875x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.910x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.915x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.923x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.926x |
| macos-15-native | snapshot | start | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.955x |
| macos-15-native | materialize | kiwipete | ultra | 0.956x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.957x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.961x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.962x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.965x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.965x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.967x |
| macos-15-native | materialize | middlegame | ultra | 0.967x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.972x |
| macos-15-native | materialize | tactical | ultra | 0.973x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.977x |
| macos-15-native | apply_children_full | tactical | chess | 0.977x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.977x |
| windows-2025-native | snapshot | middlegame | chess | 0.983x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.988x |
| windows-2025-native | snapshot | start | ferrum | 0.988x |
| windows-2025-native | snapshot | tactical | ferrum | 0.989x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.989x |
| macos-15-native | snapshot | tactical | ferrum | 0.991x |
| windows-2025-native | snapshot | endgame | cozy | 0.991x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.993x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.994x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 2709 | shakmaty | 5083 | 1.876x |
| ubuntu-24.04-arm-native | 6520 | shakmaty | 8609 | 1.320x |
| ubuntu-24.04-native | 6145 | shakmaty | 6192 | 1.008x |
| ubuntu-24.04-pext | 7464 | cozy | 11772 | 1.577x |
| ubuntu-24.04-portable | 6439 | shakmaty | 10165 | 1.579x |
| windows-2025-native | 17100 | shakmaty | 23300 | 1.363x |

Cold wins: 6/6 (separate from native cases).
