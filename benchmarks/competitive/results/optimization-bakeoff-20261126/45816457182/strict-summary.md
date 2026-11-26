| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 67 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 73 | 70 | 10 | 87 |
| ubuntu-24.04-pext | 85 | 80 | 12 | 87 |
| ubuntu-24.04-portable | 80 | 75 | 9 | 87 |
| windows-2025-native | 82 | 80 | 11 | 87 |
| ALL | 472 | 449 | 64 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.000x | 1.163x | 34/36 | 0.947x |
| chess960_materialize | 1.021x | 1.393x | 12/12 | 1.200x |
| chess960_perft_d3 | 0.997x | 1.354x | 12/12 | 1.149x |
| corpus_count | 1.023x | 1.071x | 7/12 | 0.803x |
| corpus_materialize | 1.004x | 1.101x | 11/12 | 0.987x |
| corpus_uci_sorted | 1.017x | 1.518x | 12/12 | 1.349x |
| count | 1.000x | 1.300x | 35/36 | 0.967x |
| fen_parse | 1.027x | 1.235x | 36/36 | 1.022x |
| fen_write_native_ep | 0.991x | 1.646x | 36/36 | 1.287x |
| make_unmake_full | 0.996x | 1.266x | 36/36 | 1.016x |
| materialize | 0.999x | 1.240x | 33/36 | 0.877x |
| perft_bulk_d4 | 1.005x | 1.165x | 31/36 | 0.836x |
| perft_full_d3 | 0.988x | 1.240x | 35/36 | 0.936x |
| polyglot_key | 0.984x | 32.986x | 12/12 | 16.106x |
| polyglot_recompute | 0.997x | 1.014x | 8/12 | 0.818x |
| repetition_key | 1.084x | 19.733x | 12/12 | 14.008x |
| replay_checked_native | 0.986x | 1.043x | 11/12 | 0.895x |
| replay_trusted_native | 0.995x | 0.665x | 1/12 | 0.467x |
| san_parse_all | 1.019x | 1.859x | 36/36 | 1.382x |
| san_render_native | 1.027x | 2.639x | 36/36 | 1.715x |
| snapshot | 0.998x | 1.104x | 20/36 | 0.953x |
| startpos | 1.001x | 1.261x | 6/6 | 1.198x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.467x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.495x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.526x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.530x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.554x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.651x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.660x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.724x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.777x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.803x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.816x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.818x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.836x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.842x |
| macos-15-native | materialize | kiwipete | ultra | 0.877x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.894x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.895x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.906x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.906x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.916x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.920x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.936x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.944x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.947x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.947x |
| macos-15-native | materialize | tactical | ultra | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.955x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.957x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.958x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.959x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.967x |
| macos-15-native | materialize | middlegame | ultra | 0.971x |
| macos-15-native | snapshot | start | ferrum | 0.978x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.979x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.987x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.988x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.993x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.994x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.995x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.996x |
| macos-15-native | snapshot | endgame | ferrum | 0.997x |
| macos-15-native | snapshot | promotion | ferrum | 0.998x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.998x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.998x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.999x |
| ubuntu-24.04-native | snapshot | promotion | chess | 1.000x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3541 | shakmaty | 5750 | 1.624x |
| ubuntu-24.04-arm-native | 6712 | chess | 8840 | 1.317x |
| ubuntu-24.04-native | 5230 | chess | 8479 | 1.621x |
| ubuntu-24.04-pext | 7574 | cozy | 11631 | 1.536x |
| ubuntu-24.04-portable | 6049 | shakmaty | 8473 | 1.401x |
| windows-2025-native | 11800 | shakmaty | 17900 | 1.517x |

Cold wins: 6/6 (separate from native cases).
