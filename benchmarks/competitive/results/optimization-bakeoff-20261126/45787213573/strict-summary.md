| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 71 | 13 | 87 |
| ubuntu-24.04-arm-native | 71 | 69 | 11 | 87 |
| ubuntu-24.04-native | 76 | 73 | 15 | 87 |
| ubuntu-24.04-pext | 76 | 72 | 9 | 87 |
| ubuntu-24.04-portable | 75 | 67 | 9 | 87 |
| windows-2025-native | 77 | 75 | 11 | 87 |
| ALL | 448 | 427 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.961x | 1.000x | 19/36 | 0.715x |
| chess960_materialize | 0.959x | 1.307x | 12/12 | 1.190x |
| chess960_perft_d3 | 0.977x | 1.317x | 12/12 | 1.236x |
| corpus_count | 1.004x | 1.339x | 12/12 | 1.012x |
| corpus_materialize | 0.966x | 1.136x | 11/12 | 0.948x |
| corpus_uci_sorted | 0.989x | 1.526x | 12/12 | 1.385x |
| count | 1.004x | 1.337x | 35/36 | 0.968x |
| fen_parse | 1.016x | 1.181x | 30/36 | 0.923x |
| fen_write_native_ep | 1.002x | 1.745x | 36/36 | 1.416x |
| make_unmake_full | 1.007x | 1.154x | 29/36 | 0.960x |
| materialize | 0.993x | 1.270x | 33/36 | 0.873x |
| perft_bulk_d4 | 1.001x | 1.268x | 36/36 | 1.042x |
| perft_full_d3 | 1.002x | 1.275x | 32/36 | 0.885x |
| polyglot_key | 0.983x | 32.872x | 12/12 | 16.448x |
| polyglot_recompute | 1.003x | 1.008x | 8/12 | 0.821x |
| repetition_key | 1.020x | 20.083x | 12/12 | 14.041x |
| replay_checked_native | 1.019x | 1.051x | 6/12 | 0.887x |
| replay_trusted_native | 0.903x | 0.559x | 0/12 | 0.427x |
| san_parse_all | 1.021x | 1.883x | 36/36 | 1.417x |
| san_render_native | 0.990x | 2.511x | 36/36 | 1.861x |
| snapshot | 0.877x | 1.017x | 23/36 | 0.886x |
| startpos | 0.956x | 1.261x | 6/6 | 1.105x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.427x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.439x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.443x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.444x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.451x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.525x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.576x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.591x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.715x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.728x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.744x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.749x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.780x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.821x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.854x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.860x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.865x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.871x |
| macos-15-native | materialize | kiwipete | ultra | 0.873x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.885x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.886x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.887x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.893x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.894x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.894x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.895x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.896x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.901x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.909x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.923x |
| macos-15-native | materialize | tactical | ultra | 0.929x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.932x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.940x |
| macos-15-native | materialize | middlegame | ultra | 0.947x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.948x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.949x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.951x |
| ubuntu-24.04-portable | apply_children_full | tactical | ferrum | 0.953x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.960x |
| ubuntu-24.04-portable | apply_children_full | endgame | chess | 0.961x |
| macos-15-native | snapshot | middlegame | ferrum | 0.965x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.966x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.968x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.968x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.968x |
| ubuntu-24.04-portable | count | start | ferrum | 0.968x |
| ubuntu-24.04-portable | make_unmake_full | tactical | ultra | 0.975x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.976x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.977x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.977x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.979x |
| ubuntu-24.04-native | fen_parse | endgame | ultra | 0.980x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.981x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | chess | 0.981x |
| ubuntu-24.04-portable | apply_children_full | start | chess | 0.981x |
| ubuntu-24.04-native | fen_parse | middlegame | ultra | 0.983x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.984x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.985x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.985x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.986x |
| ubuntu-24.04-portable | apply_children_full | kiwipete | ferrum | 0.986x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.986x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.986x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.989x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.992x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.993x |
| windows-2025-native | perft_full_d3 | kiwipete | shakmaty | 0.996x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.997x |
| ubuntu-24.04-pext | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 10833 | ferrum | 9916 | 0.915x |
| ubuntu-24.04-arm-native | 11288 | shakmaty | 7929 | 0.702x |
| ubuntu-24.04-native | 11465 | shakmaty | 6629 | 0.578x |
| ubuntu-24.04-pext | 8149 | shakmaty | 7858 | 0.964x |
| ubuntu-24.04-portable | 13811 | shakmaty | 9885 | 0.716x |
| windows-2025-native | 32000 | shakmaty | 24500 | 0.766x |

Cold wins: 0/6 (separate from native cases).
