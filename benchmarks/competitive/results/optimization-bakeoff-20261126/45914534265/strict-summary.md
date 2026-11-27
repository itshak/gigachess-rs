| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 71 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 82 | 15 | 87 |
| ubuntu-24.04-pext | 84 | 83 | 10 | 87 |
| ubuntu-24.04-portable | 81 | 78 | 10 | 87 |
| windows-2025-native | 82 | 79 | 13 | 87 |
| ALL | 484 | 472 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.022x | 1.321x | 34/36 | 0.905x |
| chess960_materialize | 1.004x | 1.496x | 12/12 | 1.211x |
| chess960_perft_d3 | 0.985x | 1.570x | 12/12 | 1.373x |
| corpus_count | 0.974x | 1.243x | 10/12 | 0.888x |
| corpus_materialize | 0.955x | 1.185x | 11/12 | 0.979x |
| corpus_uci_sorted | 0.953x | 1.519x | 12/12 | 1.301x |
| count | 1.010x | 1.466x | 36/36 | 1.033x |
| fen_parse | 1.026x | 1.201x | 36/36 | 1.019x |
| fen_write_native_ep | 1.002x | 1.726x | 36/36 | 1.325x |
| make_unmake_full | 0.999x | 1.295x | 35/36 | 0.979x |
| materialize | 1.014x | 1.272x | 34/36 | 0.747x |
| perft_bulk_d4 | 1.005x | 1.303x | 35/36 | 0.920x |
| perft_full_d3 | 0.999x | 1.303x | 36/36 | 1.017x |
| polyglot_key | 0.991x | 29.779x | 12/12 | 16.624x |
| polyglot_recompute | 0.963x | 1.010x | 6/12 | 0.873x |
| repetition_key | 0.978x | 20.623x | 12/12 | 14.594x |
| replay_checked_native | 0.933x | 1.145x | 12/12 | 1.060x |
| replay_trusted_native | 1.028x | 0.804x | 5/12 | 0.522x |
| san_parse_all | 0.998x | 1.936x | 36/36 | 1.370x |
| san_render_native | 0.986x | 2.525x | 36/36 | 1.578x |
| snapshot | 1.115x | 1.047x | 20/36 | 0.783x |
| startpos | 1.007x | 1.223x | 6/6 | 1.118x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.522x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.569x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.664x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.691x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.702x |
| macos-15-native | materialize | kiwipete | ultra | 0.747x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.783x |
| macos-15-native | materialize | tactical | ultra | 0.861x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.873x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.888x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.903x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.905x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.920x |
| macos-15-native | snapshot | endgame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | endgame | cozy | 0.943x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.948x |
| macos-15-native | snapshot | start | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.952x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.955x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.956x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.963x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.967x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.969x |
| macos-15-native | apply_children_full | tactical | chess | 0.973x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.979x |
| ubuntu-24.04-native | make_unmake_full | kiwipete | ultra | 0.979x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.983x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.984x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.990x |
| windows-2025-native | snapshot | promotion | cozy | 0.993x |
| macos-15-native | snapshot | tactical | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.999x |
| windows-2025-native | snapshot | tactical | chess | 1.000x |
| windows-2025-native | snapshot | middlegame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5958 | shakmaty | 9500 | 1.594x |
| ubuntu-24.04-arm-native | 5593 | shakmaty | 7857 | 1.405x |
| ubuntu-24.04-native | 7011 | shakmaty | 8423 | 1.201x |
| ubuntu-24.04-pext | 5141 | ferrum | 8648 | 1.682x |
| ubuntu-24.04-portable | 6840 | shakmaty | 7491 | 1.095x |
| windows-2025-native | 17700 | shakmaty | 21700 | 1.226x |

Cold wins: 6/6 (separate from native cases).
