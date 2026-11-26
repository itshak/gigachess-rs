| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 68 | 11 | 87 |
| ubuntu-24.04-arm-native | 77 | 76 | 11 | 87 |
| ubuntu-24.04-native | 82 | 80 | 9 | 87 |
| ubuntu-24.04-pext | 84 | 82 | 9 | 87 |
| ubuntu-24.04-portable | 81 | 76 | 11 | 87 |
| windows-2025-native | 76 | 74 | 11 | 87 |
| ALL | 470 | 456 | 62 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.048x | 1.080x | 30/36 | 0.778x |
| chess960_materialize | 1.011x | 1.357x | 12/12 | 1.135x |
| chess960_perft_d3 | 0.921x | 1.302x | 11/12 | 0.713x |
| corpus_count | 0.987x | 1.280x | 12/12 | 1.012x |
| corpus_materialize | 1.041x | 1.157x | 12/12 | 1.064x |
| corpus_uci_sorted | 1.009x | 1.513x | 12/12 | 1.230x |
| count | 1.000x | 1.324x | 35/36 | 0.977x |
| fen_parse | 1.039x | 1.217x | 35/36 | 0.811x |
| fen_write_native_ep | 0.969x | 1.671x | 36/36 | 1.383x |
| make_unmake_full | 0.984x | 1.247x | 36/36 | 1.050x |
| materialize | 1.015x | 1.234x | 32/36 | 0.723x |
| perft_bulk_d4 | 1.020x | 1.296x | 36/36 | 1.050x |
| perft_full_d3 | 1.085x | 1.273x | 32/36 | 0.944x |
| polyglot_key | 1.034x | 33.351x | 12/12 | 16.456x |
| polyglot_recompute | 1.018x | 1.017x | 8/12 | 0.869x |
| repetition_key | 0.910x | 18.271x | 12/12 | 14.142x |
| replay_checked_native | 1.015x | 0.989x | 6/12 | 0.799x |
| replay_trusted_native | 1.009x | 0.643x | 0/12 | 0.465x |
| san_parse_all | 0.962x | 1.849x | 36/36 | 1.389x |
| san_render_native | 0.993x | 2.496x | 36/36 | 1.924x |
| snapshot | 1.124x | 1.101x | 23/36 | 0.707x |
| startpos | 1.070x | 1.222x | 6/6 | 1.119x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.465x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.485x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.509x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.530x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.585x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.647x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.653x |
| macos-15-native | snapshot | kiwipete | chess | 0.707x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.712x |
| ubuntu-24.04-native | chess960_perft_d3 | 960_start | cozy | 0.713x |
| macos-15-native | materialize | start | ultra | 0.723x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.764x |
| macos-15-native | snapshot | middlegame | ferrum | 0.773x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.778x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.799x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.808x |
| macos-15-native | fen_parse | start | cozy | 0.811x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.816x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.819x |
| macos-15-native | materialize | tactical | ultra | 0.828x |
| macos-15-native | materialize | kiwipete | ultra | 0.844x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.869x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.901x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.910x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.919x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.920x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.921x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.944x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.944x |
| macos-15-native | snapshot | start | ferrum | 0.947x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.952x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.954x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.954x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.954x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.960x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.962x |
| windows-2025-native | materialize | kiwipete | ultra | 0.966x |
| macos-15-native | snapshot | endgame | ferrum | 0.966x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.975x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.977x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.978x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.978x |
| macos-15-native | apply_children_full | start | chess | 0.980x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.982x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.990x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 1.000x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 13792 | shakmaty | 11167 | 0.810x |
| ubuntu-24.04-arm-native | 10808 | shakmaty | 8441 | 0.781x |
| ubuntu-24.04-native | 14722 | shakmaty | 8433 | 0.573x |
| ubuntu-24.04-pext | 8060 | cozy | 7533 | 0.935x |
| ubuntu-24.04-portable | 9422 | shakmaty | 5793 | 0.615x |
| windows-2025-native | 29300 | cozy | 21700 | 0.741x |

Cold wins: 0/6 (separate from native cases).
