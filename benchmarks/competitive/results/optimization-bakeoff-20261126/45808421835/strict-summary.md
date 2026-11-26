| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 67 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 80 | 77 | 16 | 87 |
| ubuntu-24.04-pext | 81 | 78 | 13 | 87 |
| ubuntu-24.04-portable | 80 | 75 | 13 | 87 |
| windows-2025-native | 79 | 77 | 12 | 87 |
| ALL | 470 | 451 | 77 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.004x | 1.206x | 33/36 | 0.868x |
| chess960_materialize | 0.984x | 1.323x | 12/12 | 1.020x |
| chess960_perft_d3 | 1.021x | 1.429x | 12/12 | 1.270x |
| corpus_count | 1.026x | 1.324x | 12/12 | 1.140x |
| corpus_materialize | 1.031x | 1.197x | 12/12 | 1.100x |
| corpus_uci_sorted | 1.042x | 1.558x | 12/12 | 1.228x |
| count | 1.000x | 1.331x | 35/36 | 0.947x |
| fen_parse | 0.989x | 1.201x | 36/36 | 1.024x |
| fen_write_native_ep | 1.009x | 1.748x | 36/36 | 1.422x |
| make_unmake_full | 1.017x | 1.266x | 35/36 | 0.991x |
| materialize | 1.014x | 1.205x | 32/36 | 0.932x |
| perft_bulk_d4 | 1.014x | 1.295x | 36/36 | 1.119x |
| perft_full_d3 | 1.000x | 1.180x | 27/36 | 0.855x |
| polyglot_key | 1.039x | 26.898x | 12/12 | 16.340x |
| polyglot_recompute | 1.050x | 1.017x | 8/12 | 0.860x |
| repetition_key | 1.060x | 18.685x | 12/12 | 14.052x |
| replay_checked_native | 1.117x | 1.018x | 8/12 | 0.871x |
| replay_trusted_native | 1.077x | 0.757x | 3/12 | 0.518x |
| san_parse_all | 1.004x | 2.005x | 36/36 | 1.379x |
| san_render_native | 0.998x | 2.550x | 36/36 | 1.644x |
| snapshot | 0.853x | 1.013x | 19/36 | 0.906x |
| startpos | 0.933x | 1.375x | 6/6 | 1.116x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.518x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.530x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.586x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.623x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.657x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.738x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.855x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.857x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.860x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.868x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.871x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.877x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.877x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.881x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.906x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.907x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.908x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.914x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.915x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.917x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.920x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.921x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.922x |
| macos-15-native | perft_full_d3 | kiwipete | ultra | 0.928x |
| macos-15-native | materialize | kiwipete | ultra | 0.932x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.938x |
| macos-15-native | materialize | start | ultra | 0.938x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.940x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.946x |
| macos-15-native | count | start | cozy | 0.947x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.959x |
| macos-15-native | materialize | middlegame | ultra | 0.960x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.961x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.962x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.964x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.966x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.968x |
| macos-15-native | snapshot | middlegame | ferrum | 0.968x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.970x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.975x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.975x |
| windows-2025-native | materialize | kiwipete | ultra | 0.982x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.983x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.991x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.993x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.994x |
| macos-15-native | apply_children_full | tactical | cozy | 0.996x |
| ubuntu-24.04-pext | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | middlegame | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 25291 | shakmaty | 14166 | 0.560x |
| ubuntu-24.04-arm-native | 12399 | shakmaty | 8855 | 0.714x |
| ubuntu-24.04-native | 13820 | shakmaty | 9063 | 0.656x |
| ubuntu-24.04-pext | 10345 | cozy | 7069 | 0.683x |
| ubuntu-24.04-portable | 13745 | shakmaty | 9057 | 0.659x |
| windows-2025-native | 27800 | chess | 23700 | 0.853x |

Cold wins: 0/6 (separate from native cases).
