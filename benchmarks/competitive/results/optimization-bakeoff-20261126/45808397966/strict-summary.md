| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 68 | 13 | 87 |
| ubuntu-24.04-arm-native | 77 | 76 | 12 | 87 |
| ubuntu-24.04-native | 82 | 78 | 13 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 9 | 87 |
| ubuntu-24.04-portable | 81 | 76 | 12 | 87 |
| windows-2025-native | 77 | 73 | 12 | 87 |
| ALL | 473 | 452 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.038x | 1.163x | 31/36 | 0.926x |
| chess960_materialize | 1.104x | 1.459x | 12/12 | 1.257x |
| chess960_perft_d3 | 1.066x | 1.467x | 12/12 | 1.297x |
| corpus_count | 0.791x | 1.105x | 7/12 | 0.795x |
| corpus_materialize | 1.000x | 1.156x | 12/12 | 1.048x |
| corpus_uci_sorted | 0.982x | 1.498x | 12/12 | 1.147x |
| count | 1.035x | 1.387x | 36/36 | 1.027x |
| fen_parse | 0.980x | 1.189x | 34/36 | 0.953x |
| fen_write_native_ep | 1.031x | 1.787x | 36/36 | 1.377x |
| make_unmake_full | 1.021x | 1.240x | 36/36 | 1.023x |
| materialize | 1.024x | 1.244x | 32/36 | 0.890x |
| perft_bulk_d4 | 0.948x | 1.185x | 32/36 | 0.754x |
| perft_full_d3 | 1.023x | 1.284x | 33/36 | 0.908x |
| polyglot_key | 1.003x | 31.302x | 12/12 | 15.447x |
| polyglot_recompute | 0.992x | 1.011x | 8/12 | 0.893x |
| repetition_key | 0.949x | 17.895x | 12/12 | 13.966x |
| replay_checked_native | 0.993x | 1.039x | 9/12 | 0.864x |
| replay_trusted_native | 0.989x | 0.702x | 3/12 | 0.469x |
| san_parse_all | 1.000x | 1.922x | 36/36 | 1.287x |
| san_render_native | 1.004x | 2.466x | 36/36 | 1.842x |
| snapshot | 1.114x | 1.064x | 26/36 | 0.731x |
| startpos | 1.141x | 1.217x | 6/6 | 1.111x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.469x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.500x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.539x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.607x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.628x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.635x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.642x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.714x |
| macos-15-native | snapshot | tactical | ferrum | 0.731x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.754x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.795x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.855x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.864x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.883x |
| macos-15-native | snapshot | middlegame | ferrum | 0.884x |
| ubuntu-24.04-portable | materialize | promotion | ultra | 0.890x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.893x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.902x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.908x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.926x |
| macos-15-native | materialize | promotion | ultra | 0.934x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.937x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| macos-15-native | snapshot | endgame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.944x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.945x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.946x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.951x |
| macos-15-native | fen_parse | start | cozy | 0.953x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.957x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.957x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.963x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.963x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.967x |
| ubuntu-24.04-portable | materialize | kiwipete | ultra | 0.978x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.978x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.979x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.981x |
| macos-15-native | perft_bulk_d4 | middlegame | ultra | 0.988x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.989x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.991x |
| windows-2025-native | materialize | kiwipete | ultra | 0.992x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.993x |
| ubuntu-24.04-portable | snapshot | middlegame | chess | 0.995x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6542 | shakmaty | 9708 | 1.484x |
| ubuntu-24.04-arm-native | 5232 | shakmaty | 7727 | 1.477x |
| ubuntu-24.04-native | 5424 | shakmaty | 7391 | 1.363x |
| ubuntu-24.04-pext | 4879 | cozy | 8407 | 1.723x |
| ubuntu-24.04-portable | 7935 | shakmaty | 11160 | 1.406x |
| windows-2025-native | 12800 | shakmaty | 16100 | 1.258x |

Cold wins: 6/6 (separate from native cases).
