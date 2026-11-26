| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 64 | 12 | 87 |
| ubuntu-24.04-arm-native | 73 | 72 | 11 | 87 |
| ubuntu-24.04-native | 79 | 76 | 11 | 87 |
| ubuntu-24.04-pext | 77 | 73 | 11 | 87 |
| ubuntu-24.04-portable | 80 | 76 | 15 | 87 |
| windows-2025-native | 77 | 70 | 12 | 87 |
| ALL | 453 | 431 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.919x | 1.067x | 23/36 | 0.840x |
| chess960_materialize | 0.925x | 1.296x | 12/12 | 1.074x |
| chess960_perft_d3 | 0.983x | 1.288x | 12/12 | 1.034x |
| corpus_count | 1.013x | 1.128x | 9/12 | 0.921x |
| corpus_materialize | 1.010x | 1.114x | 10/12 | 0.942x |
| corpus_uci_sorted | 1.049x | 1.551x | 12/12 | 1.355x |
| count | 0.971x | 1.290x | 35/36 | 0.986x |
| fen_parse | 0.984x | 1.184x | 35/36 | 0.942x |
| fen_write_native_ep | 0.998x | 1.727x | 36/36 | 1.449x |
| make_unmake_full | 0.908x | 1.148x | 30/36 | 0.885x |
| materialize | 0.980x | 1.252x | 34/36 | 0.753x |
| perft_bulk_d4 | 0.992x | 1.184x | 35/36 | 0.880x |
| perft_full_d3 | 0.912x | 1.143x | 30/36 | 0.915x |
| polyglot_key | 1.051x | 29.018x | 12/12 | 16.084x |
| polyglot_recompute | 0.983x | 1.020x | 8/12 | 0.899x |
| repetition_key | 1.032x | 18.498x | 12/12 | 14.268x |
| replay_checked_native | 0.960x | 0.990x | 4/12 | 0.908x |
| replay_trusted_native | 0.935x | 0.640x | 0/12 | 0.435x |
| san_parse_all | 0.998x | 1.942x | 36/36 | 1.229x |
| san_render_native | 0.953x | 2.496x | 36/36 | 1.660x |
| snapshot | 1.040x | 1.097x | 26/36 | 0.704x |
| startpos | 1.022x | 1.245x | 6/6 | 1.063x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.435x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.486x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.509x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.563x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.601x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.609x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.642x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.692x |
| macos-15-native | snapshot | middlegame | ferrum | 0.704x |
| macos-15-native | materialize | tactical | ultra | 0.753x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.760x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.825x |
| macos-15-native | apply_children_full | start | chess | 0.840x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.849x |
| macos-15-native | snapshot | start | ferrum | 0.852x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.880x |
| macos-15-native | apply_children_full | tactical | cozy | 0.884x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.885x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.893x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.894x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.899x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.905x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.908x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.915x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.921x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.924x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.924x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.924x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.926x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.928x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.932x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.933x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.937x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.937x |
| macos-15-native | materialize | kiwipete | ultra | 0.938x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.940x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.940x |
| macos-15-native | fen_parse | kiwipete | ferrum | 0.942x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.942x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.944x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.947x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.948x |
| macos-15-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.953x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.958x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.958x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.959x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.961x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.962x |
| ubuntu-24.04-pext | apply_children_full | promotion | chess | 0.963x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.963x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.967x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.978x |
| ubuntu-24.04-pext | apply_children_full | tactical | chess | 0.985x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.985x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.985x |
| macos-15-native | count | promotion | cozy | 0.986x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.988x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | chess | 0.988x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | shakmaty | 0.989x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.989x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.990x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.991x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.992x |
| windows-2025-native | snapshot | endgame | ferrum | 0.993x |
| ubuntu-24.04-pext | make_unmake_full | kiwipete | ultra | 0.996x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.998x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4417 | ferrum | 11125 | 2.519x |
| ubuntu-24.04-arm-native | 5256 | shakmaty | 7976 | 1.518x |
| ubuntu-24.04-native | 9017 | shakmaty | 11411 | 1.265x |
| ubuntu-24.04-pext | 5566 | shakmaty | 7827 | 1.406x |
| ubuntu-24.04-portable | 6760 | shakmaty | 9575 | 1.416x |
| windows-2025-native | 17700 | shakmaty | 24500 | 1.384x |

Cold wins: 6/6 (separate from native cases).
