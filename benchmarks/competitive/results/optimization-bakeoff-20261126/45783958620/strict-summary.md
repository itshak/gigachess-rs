| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 69 | 14 | 87 |
| ubuntu-24.04-arm-native | 72 | 70 | 11 | 87 |
| ubuntu-24.04-native | 81 | 78 | 15 | 87 |
| ubuntu-24.04-pext | 79 | 74 | 9 | 87 |
| ubuntu-24.04-portable | 75 | 69 | 9 | 87 |
| windows-2025-native | 67 | 63 | 12 | 87 |
| ALL | 444 | 423 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.956x | 0.982x | 17/36 | 0.832x |
| chess960_materialize | 0.972x | 1.337x | 12/12 | 1.102x |
| chess960_perft_d3 | 0.948x | 1.319x | 12/12 | 1.183x |
| corpus_count | 0.937x | 1.371x | 12/12 | 1.027x |
| corpus_materialize | 0.944x | 1.163x | 12/12 | 1.010x |
| corpus_uci_sorted | 0.973x | 1.522x | 12/12 | 1.360x |
| count | 1.012x | 1.337x | 35/36 | 0.966x |
| fen_parse | 1.029x | 1.171x | 34/36 | 0.993x |
| fen_write_native_ep | 1.017x | 1.749x | 36/36 | 1.367x |
| make_unmake_full | 0.955x | 1.142x | 31/36 | 0.929x |
| materialize | 0.987x | 1.261x | 32/36 | 0.847x |
| perft_bulk_d4 | 1.010x | 1.259x | 36/36 | 1.065x |
| perft_full_d3 | 0.961x | 1.256x | 29/36 | 0.845x |
| polyglot_key | 1.000x | 32.485x | 12/12 | 16.655x |
| polyglot_recompute | 1.000x | 1.035x | 8/12 | 0.884x |
| repetition_key | 1.030x | 20.145x | 12/12 | 14.336x |
| replay_checked_native | 0.967x | 1.006x | 7/12 | 0.816x |
| replay_trusted_native | 0.922x | 0.564x | 0/12 | 0.424x |
| san_parse_all | 1.000x | 1.895x | 36/36 | 1.280x |
| san_render_native | 1.013x | 2.493x | 36/36 | 1.790x |
| snapshot | 0.822x | 0.978x | 17/36 | 0.520x |
| startpos | 1.097x | 1.378x | 6/6 | 1.155x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.424x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.431x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.459x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.469x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.484x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.506x |
| macos-15-native | snapshot | tactical | chess | 0.520x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.579x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.709x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.737x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.743x |
| windows-2025-native | snapshot | endgame | ultra | 0.762x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.816x |
| macos-15-native | apply_children_full | tactical | chess | 0.832x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.837x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.844x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.845x |
| macos-15-native | materialize | middlegame | ultra | 0.847x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.852x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.853x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.866x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.870x |
| macos-15-native | materialize | kiwipete | ultra | 0.870x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.871x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.872x |
| macos-15-native | snapshot | endgame | cozy | 0.874x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.884x |
| macos-15-native | apply_children_full | middlegame | chess | 0.886x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.888x |
| windows-2025-native | snapshot | start | ultra | 0.888x |
| windows-2025-native | snapshot | promotion | ultra | 0.890x |
| macos-15-native | snapshot | start | ferrum | 0.890x |
| macos-15-native | materialize | promotion | ultra | 0.891x |
| windows-2025-native | snapshot | middlegame | ultra | 0.894x |
| windows-2025-native | snapshot | tactical | ultra | 0.895x |
| windows-2025-native | snapshot | kiwipete | ultra | 0.897x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.909x |
| macos-15-native | snapshot | middlegame | ferrum | 0.917x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.919x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.921x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.929x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.934x |
| ubuntu-24.04-portable | apply_children_full | endgame | chess | 0.939x |
| ubuntu-24.04-portable | apply_children_full | tactical | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.941x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.942x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.946x |
| ubuntu-24.04-portable | apply_children_full | kiwipete | ferrum | 0.947x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.952x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.961x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.964x |
| ubuntu-24.04-portable | count | start | ferrum | 0.966x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.969x |
| ubuntu-24.04-native | apply_children_full | promotion | chess | 0.972x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.972x |
| ubuntu-24.04-pext | apply_children_full | promotion | chess | 0.973x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.976x |
| macos-15-native | apply_children_full | start | shakmaty | 0.977x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.978x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.979x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.981x |
| ubuntu-24.04-portable | apply_children_full | start | chess | 0.982x |
| macos-15-native | snapshot | promotion | ferrum | 0.984x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.984x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.985x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.987x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.989x |
| windows-2025-native | materialize | kiwipete | ultra | 0.989x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.993x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.993x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.994x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.995x |
| ubuntu-24.04-native | apply_children_full | endgame | chess | 0.997x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 14334 | shakmaty | 11167 | 0.779x |
| ubuntu-24.04-arm-native | 9088 | shakmaty | 6953 | 0.765x |
| ubuntu-24.04-native | 12305 | shakmaty | 6012 | 0.489x |
| ubuntu-24.04-pext | 7648 | shakmaty | 6276 | 0.821x |
| ubuntu-24.04-portable | 12559 | shakmaty | 8813 | 0.702x |
| windows-2025-native | 27900 | ferrum | 22800 | 0.817x |

Cold wins: 0/6 (separate from native cases).
