| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 78 | 76 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 85 | 80 | 11 | 87 |
| ubuntu-24.04-pext | 84 | 82 | 17 | 87 |
| ubuntu-24.04-portable | 86 | 80 | 11 | 87 |
| windows-2025-native | 81 | 80 | 14 | 87 |
| ALL | 493 | 476 | 77 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.037x | 1.305x | 35/36 | 0.971x |
| chess960_materialize | 0.991x | 1.467x | 12/12 | 1.207x |
| chess960_perft_d3 | 1.008x | 1.509x | 12/12 | 1.199x |
| corpus_count | 1.121x | 1.333x | 12/12 | 1.101x |
| corpus_materialize | 0.995x | 1.133x | 12/12 | 1.018x |
| corpus_uci_sorted | 0.978x | 1.516x | 12/12 | 1.381x |
| count | 1.059x | 1.555x | 36/36 | 1.078x |
| fen_parse | 1.017x | 1.238x | 36/36 | 1.063x |
| fen_write_native_ep | 1.033x | 1.771x | 36/36 | 1.433x |
| make_unmake_full | 1.002x | 1.309x | 36/36 | 1.000x |
| materialize | 1.035x | 1.374x | 34/36 | 0.890x |
| perft_bulk_d4 | 1.070x | 1.341x | 34/36 | 0.950x |
| perft_full_d3 | 1.203x | 1.543x | 36/36 | 1.193x |
| polyglot_key | 1.015x | 31.377x | 12/12 | 15.998x |
| polyglot_recompute | 1.455x | 1.459x | 12/12 | 1.166x |
| repetition_key | 1.040x | 20.146x | 12/12 | 14.013x |
| replay_checked_native | 0.970x | 1.101x | 11/12 | 0.994x |
| replay_trusted_native | 0.919x | 0.757x | 4/12 | 0.528x |
| san_parse_all | 1.026x | 1.897x | 36/36 | 1.393x |
| san_render_native | 1.019x | 2.599x | 36/36 | 1.819x |
| snapshot | 0.996x | 1.018x | 21/36 | 0.614x |
| startpos | 1.000x | 1.211x | 6/6 | 1.115x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.528x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.577x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.602x |
| macos-15-native | snapshot | start | ferrum | 0.614x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.637x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.648x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.659x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.690x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.860x |
| macos-15-native | snapshot | middlegame | ferrum | 0.880x |
| macos-15-native | snapshot | endgame | ferrum | 0.887x |
| macos-15-native | materialize | tactical | ultra | 0.890x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.942x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.950x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.950x |
| macos-15-native | apply_children_full | tactical | chess | 0.971x |
| windows-2025-native | snapshot | endgame | chess | 0.974x |
| windows-2025-native | snapshot | promotion | chess | 0.992x |
| windows-2025-native | snapshot | start | chess | 0.993x |
| windows-2025-native | snapshot | kiwipete | chess | 0.993x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.994x |
| macos-15-native | snapshot | promotion | ferrum | 0.996x |
| macos-15-native | materialize | middlegame | ultra | 0.999x |
| windows-2025-native | snapshot | tactical | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5083 | shakmaty | 9375 | 1.844x |
| ubuntu-24.04-arm-native | 5848 | shakmaty | 7671 | 1.312x |
| ubuntu-24.04-native | 6098 | shakmaty | 7935 | 1.301x |
| ubuntu-24.04-pext | 5251 | shakmaty | 6903 | 1.315x |
| ubuntu-24.04-portable | 8215 | shakmaty | 11792 | 1.435x |
| windows-2025-native | 14000 | shakmaty | 21400 | 1.529x |

Cold wins: 6/6 (separate from native cases).
