| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 73 | 12 | 87 |
| ubuntu-24.04-arm-native | 77 | 77 | 12 | 87 |
| ubuntu-24.04-native | 82 | 82 | 13 | 87 |
| ubuntu-24.04-pext | 79 | 77 | 12 | 87 |
| ubuntu-24.04-portable | 79 | 77 | 12 | 87 |
| windows-2025-native | 84 | 83 | 12 | 87 |
| ALL | 476 | 469 | 73 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.000x | 1.270x | 36/36 | 1.020x |
| chess960_materialize | 1.008x | 1.471x | 12/12 | 1.267x |
| chess960_perft_d3 | 1.014x | 1.552x | 12/12 | 1.387x |
| corpus_count | 1.091x | 1.419x | 12/12 | 1.080x |
| corpus_materialize | 1.003x | 1.198x | 12/12 | 1.101x |
| corpus_uci_sorted | 0.993x | 1.569x | 12/12 | 1.390x |
| count | 1.047x | 1.526x | 36/36 | 1.077x |
| fen_parse | 0.982x | 1.161x | 34/36 | 0.940x |
| fen_write_native_ep | 0.995x | 1.728x | 36/36 | 1.414x |
| make_unmake_full | 1.006x | 1.315x | 36/36 | 1.010x |
| materialize | 1.024x | 1.282x | 34/36 | 0.968x |
| perft_bulk_d4 | 1.050x | 1.395x | 35/36 | 0.996x |
| perft_full_d3 | 1.216x | 1.581x | 36/36 | 1.298x |
| polyglot_key | 0.994x | 27.767x | 12/12 | 15.855x |
| polyglot_recompute | 0.834x | 0.848x | 0/12 | 0.718x |
| repetition_key | 0.968x | 19.274x | 12/12 | 13.889x |
| replay_checked_native | 0.995x | 1.139x | 12/12 | 1.044x |
| replay_trusted_native | 1.002x | 0.863x | 6/12 | 0.513x |
| san_parse_all | 0.984x | 1.875x | 36/36 | 1.372x |
| san_render_native | 1.003x | 2.517x | 36/36 | 1.645x |
| snapshot | 0.951x | 1.047x | 14/36 | 0.934x |
| startpos | 0.993x | 1.193x | 5/6 | 0.973x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.513x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.681x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.700x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.718x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.727x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.733x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.804x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.818x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.820x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.835x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.839x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.861x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.880x |
| ubuntu-24.04-arm-native | polyglot_recompute | lichess300 | shakmaty | 0.904x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.910x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.937x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.940x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.952x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.954x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.964x |
| ubuntu-24.04-pext | snapshot | endgame | ferrum | 0.964x |
| ubuntu-24.04-pext | snapshot | middlegame | ferrum | 0.967x |
| macos-15-native | materialize | kiwipete | ultra | 0.968x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.969x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.973x |
| windows-2025-native | startpos | start | shakmaty | 0.973x |
| ubuntu-24.04-pext | snapshot | start | ferrum | 0.973x |
| ubuntu-24.04-arm-native | polyglot_recompute | seeded | shakmaty | 0.974x |
| ubuntu-24.04-pext | snapshot | tactical | cozy | 0.974x |
| macos-15-native | snapshot | middlegame | ferrum | 0.980x |
| macos-15-native | snapshot | start | ferrum | 0.981x |
| macos-15-native | snapshot | promotion | ferrum | 0.984x |
| macos-15-native | snapshot | endgame | ferrum | 0.986x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.994x |
| macos-15-native | materialize | tactical | ultra | 0.995x |
| macos-15-native | snapshot | tactical | ferrum | 0.996x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.996x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3584 | shakmaty | 6000 | 1.674x |
| ubuntu-24.04-arm-native | 6712 | shakmaty | 9032 | 1.346x |
| ubuntu-24.04-native | 9188 | ferrum | 12914 | 1.406x |
| ubuntu-24.04-pext | 7802 | cozy | 12138 | 1.556x |
| ubuntu-24.04-portable | 4990 | shakmaty | 6911 | 1.385x |
| windows-2025-native | 17900 | shakmaty | 23300 | 1.302x |

Cold wins: 6/6 (separate from native cases).
