| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 75 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 83 | 10 | 87 |
| ubuntu-24.04-pext | 80 | 79 | 13 | 87 |
| ubuntu-24.04-portable | 83 | 76 | 13 | 87 |
| windows-2025-native | 82 | 79 | 13 | 87 |
| ALL | 484 | 471 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.991x | 1.276x | 35/36 | 0.768x |
| chess960_materialize | 0.972x | 1.411x | 12/12 | 1.247x |
| chess960_perft_d3 | 1.008x | 1.534x | 12/12 | 1.370x |
| corpus_count | 1.147x | 1.352x | 10/12 | 0.918x |
| corpus_materialize | 0.974x | 1.171x | 12/12 | 1.058x |
| corpus_uci_sorted | 0.964x | 1.520x | 12/12 | 1.238x |
| count | 1.017x | 1.501x | 36/36 | 1.018x |
| fen_parse | 0.974x | 1.190x | 36/36 | 1.021x |
| fen_write_native_ep | 1.002x | 1.667x | 36/36 | 1.433x |
| make_unmake_full | 1.032x | 1.334x | 36/36 | 1.060x |
| materialize | 0.994x | 1.280x | 33/36 | 0.781x |
| perft_bulk_d4 | 1.047x | 1.363x | 36/36 | 1.006x |
| perft_full_d3 | 0.984x | 1.312x | 36/36 | 1.102x |
| polyglot_key | 1.005x | 33.195x | 12/12 | 16.438x |
| polyglot_recompute | 0.976x | 1.023x | 9/12 | 0.807x |
| repetition_key | 1.026x | 21.495x | 12/12 | 14.660x |
| replay_checked_native | 0.986x | 1.176x | 12/12 | 1.061x |
| replay_trusted_native | 0.974x | 0.758x | 4/12 | 0.519x |
| san_parse_all | 0.968x | 1.867x | 36/36 | 1.195x |
| san_render_native | 0.986x | 2.606x | 36/36 | 1.900x |
| snapshot | 0.940x | 1.037x | 15/36 | 0.741x |
| startpos | 1.084x | 1.290x | 6/6 | 1.250x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.519x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.569x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.673x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.677x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.685x |
| macos-15-native | snapshot | kiwipete | cozy | 0.741x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.743x |
| macos-15-native | apply_children_full | promotion | chess | 0.768x |
| macos-15-native | materialize | kiwipete | ultra | 0.781x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.807x |
| windows-2025-native | snapshot | start | cozy | 0.856x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.895x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.896x |
| macos-15-native | materialize | middlegame | ultra | 0.897x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.901x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.902x |
| ubuntu-24.04-pext | snapshot | tactical | cozy | 0.902x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 0.903x |
| macos-15-native | snapshot | endgame | chess | 0.906x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.918x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.923x |
| macos-15-native | materialize | tactical | ultra | 0.927x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.941x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.941x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.959x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.976x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.977x |
| macos-15-native | snapshot | promotion | cozy | 0.980x |
| windows-2025-native | snapshot | endgame | ferrum | 0.986x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.993x |
| windows-2025-native | snapshot | promotion | cozy | 0.996x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6208 | shakmaty | 10000 | 1.611x |
| ubuntu-24.04-arm-native | 5656 | shakmaty | 7064 | 1.249x |
| ubuntu-24.04-native | 7621 | shakmaty | 11027 | 1.447x |
| ubuntu-24.04-pext | 7334 | cozy | 10710 | 1.460x |
| ubuntu-24.04-portable | 6058 | shakmaty | 5496 | 0.907x |
| windows-2025-native | 18000 | shakmaty | 27300 | 1.517x |

Cold wins: 5/6 (separate from native cases).
