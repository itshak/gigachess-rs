| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 71 | 11 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 85 | 82 | 16 | 87 |
| ubuntu-24.04-pext | 80 | 75 | 10 | 87 |
| ubuntu-24.04-portable | 84 | 78 | 13 | 87 |
| windows-2025-native | 82 | 78 | 13 | 87 |
| ALL | 483 | 463 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.981x | 1.241x | 31/36 | 0.896x |
| chess960_materialize | 0.971x | 1.423x | 12/12 | 1.266x |
| chess960_perft_d3 | 1.008x | 1.518x | 12/12 | 1.231x |
| corpus_count | 1.162x | 1.364x | 12/12 | 1.036x |
| corpus_materialize | 1.013x | 1.131x | 12/12 | 1.055x |
| corpus_uci_sorted | 0.991x | 1.486x | 12/12 | 1.306x |
| count | 1.063x | 1.542x | 36/36 | 1.074x |
| fen_parse | 0.985x | 1.203x | 36/36 | 1.008x |
| fen_write_native_ep | 1.012x | 1.821x | 36/36 | 1.422x |
| make_unmake_full | 0.976x | 1.266x | 36/36 | 1.022x |
| materialize | 0.989x | 1.368x | 33/36 | 0.835x |
| perft_bulk_d4 | 1.056x | 1.311x | 34/36 | 0.913x |
| perft_full_d3 | 1.169x | 1.550x | 36/36 | 1.276x |
| polyglot_key | 1.004x | 36.088x | 12/12 | 17.487x |
| polyglot_recompute | 1.479x | 1.508x | 12/12 | 1.029x |
| repetition_key | 1.054x | 22.781x | 12/12 | 15.082x |
| replay_checked_native | 0.931x | 1.077x | 11/12 | 0.991x |
| replay_trusted_native | 0.979x | 0.769x | 4/12 | 0.519x |
| san_parse_all | 1.027x | 1.885x | 36/36 | 1.338x |
| san_render_native | 1.008x | 2.547x | 36/36 | 1.790x |
| snapshot | 1.010x | 1.011x | 17/36 | 0.944x |
| startpos | 0.982x | 1.201x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.519x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.588x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.635x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.646x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.654x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.666x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.702x |
| macos-15-native | materialize | kiwipete | ultra | 0.835x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.896x |
| macos-15-native | materialize | middlegame | ultra | 0.910x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.913x |
| macos-15-native | apply_children_full | tactical | chess | 0.913x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.920x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.933x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| macos-15-native | materialize | tactical | ultra | 0.968x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.968x |
| macos-15-native | snapshot | endgame | ferrum | 0.970x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.972x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.975x |
| windows-2025-native | snapshot | promotion | ferrum | 0.986x |
| windows-2025-native | snapshot | tactical | cozy | 0.988x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.991x |
| macos-15-native | snapshot | start | ferrum | 0.991x |
| macos-15-native | snapshot | promotion | ferrum | 0.991x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | tactical | cozy | 0.999x |
| windows-2025-native | snapshot | middlegame | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.999x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 1.000x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 1.000x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7208 | shakmaty | 8708 | 1.208x |
| ubuntu-24.04-arm-native | 4928 | chess | 8065 | 1.637x |
| ubuntu-24.04-native | 7580 | shakmaty | 8048 | 1.062x |
| ubuntu-24.04-pext | 5924 | chess | 8870 | 1.497x |
| ubuntu-24.04-portable | 5084 | shakmaty | 6097 | 1.199x |
| windows-2025-native | 14700 | shakmaty | 20800 | 1.415x |

Cold wins: 6/6 (separate from native cases).
