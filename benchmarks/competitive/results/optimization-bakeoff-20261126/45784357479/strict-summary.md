| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 67 | 13 | 87 |
| ubuntu-24.04-arm-native | 74 | 71 | 10 | 87 |
| ubuntu-24.04-native | 80 | 75 | 9 | 87 |
| ubuntu-24.04-pext | 74 | 71 | 12 | 87 |
| ubuntu-24.04-portable | 81 | 75 | 11 | 87 |
| windows-2025-native | 75 | 72 | 11 | 87 |
| ALL | 454 | 431 | 66 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.002x | 1.048x | 28/36 | 0.763x |
| chess960_materialize | 0.958x | 1.320x | 12/12 | 1.173x |
| chess960_perft_d3 | 0.973x | 1.400x | 12/12 | 1.276x |
| corpus_count | 0.961x | 1.325x | 12/12 | 1.022x |
| corpus_materialize | 0.960x | 1.138x | 11/12 | 0.992x |
| corpus_uci_sorted | 0.950x | 1.486x | 12/12 | 1.194x |
| count | 1.017x | 1.350x | 34/36 | 0.978x |
| fen_parse | 1.059x | 1.175x | 33/36 | 0.939x |
| fen_write_native_ep | 1.004x | 1.765x | 36/36 | 1.360x |
| make_unmake_full | 1.030x | 1.224x | 36/36 | 1.024x |
| materialize | 1.013x | 1.322x | 32/36 | 0.878x |
| perft_bulk_d4 | 0.996x | 1.251x | 35/36 | 0.959x |
| perft_full_d3 | 1.013x | 1.316x | 32/36 | 0.931x |
| polyglot_key | 0.985x | 33.027x | 12/12 | 16.352x |
| polyglot_recompute | 0.910x | 0.922x | 1/12 | 0.794x |
| repetition_key | 1.048x | 20.702x | 12/12 | 14.075x |
| replay_checked_native | 0.982x | 1.040x | 9/12 | 0.878x |
| replay_trusted_native | 0.957x | 0.653x | 0/12 | 0.484x |
| san_parse_all | 1.017x | 1.885x | 36/36 | 1.508x |
| san_render_native | 1.014x | 2.461x | 36/36 | 1.915x |
| snapshot | 0.999x | 0.991x | 17/36 | 0.713x |
| startpos | 1.098x | 1.142x | 6/6 | 1.003x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.484x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.511x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.521x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.598x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.600x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.631x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.636x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.681x |
| macos-15-native | snapshot | middlegame | ferrum | 0.713x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.762x |
| macos-15-native | apply_children_full | middlegame | chess | 0.763x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.794x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.810x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.812x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.832x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.838x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.847x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.852x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.859x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.861x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.863x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.878x |
| macos-15-native | materialize | kiwipete | ultra | 0.878x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.880x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.891x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.899x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.901x |
| macos-15-native | materialize | tactical | ultra | 0.903x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.931x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.931x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.933x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.936x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.939x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.939x |
| ubuntu-24.04-pext | fen_parse | endgame | ultra | 0.946x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.949x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.949x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.951x |
| macos-15-native | snapshot | start | ferrum | 0.954x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.959x |
| ubuntu-24.04-arm-native | polyglot_recompute | lichess300 | shakmaty | 0.959x |
| macos-15-native | perft_bulk_d4 | middlegame | cozy | 0.959x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.961x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.962x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.962x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.964x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.970x |
| macos-15-native | apply_children_full | promotion | ferrum | 0.975x |
| windows-2025-native | materialize | kiwipete | ultra | 0.976x |
| macos-15-native | count | promotion | cozy | 0.978x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.981x |
| macos-15-native | materialize | start | ultra | 0.981x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.985x |
| ubuntu-24.04-pext | snapshot | kiwipete | chess | 0.987x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.992x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.993x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.993x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.993x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.994x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.995x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.995x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.996x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.997x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.997x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.999x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.999x |
| ubuntu-24.04-pext | snapshot | start | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 13834 | shakmaty | 10250 | 0.741x |
| ubuntu-24.04-arm-native | 10055 | shakmaty | 8848 | 0.880x |
| ubuntu-24.04-native | 8939 | shakmaty | 6414 | 0.718x |
| ubuntu-24.04-pext | 8302 | cozy | 7964 | 0.959x |
| ubuntu-24.04-portable | 8886 | shakmaty | 6123 | 0.689x |
| windows-2025-native | 28100 | shakmaty | 22600 | 0.804x |

Cold wins: 0/6 (separate from native cases).
