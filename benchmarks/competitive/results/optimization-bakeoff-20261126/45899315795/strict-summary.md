| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 69 | 12 | 87 |
| ubuntu-24.04-arm-native | 82 | 79 | 12 | 87 |
| ubuntu-24.04-native | 78 | 76 | 9 | 87 |
| ubuntu-24.04-pext | 74 | 73 | 9 | 87 |
| ubuntu-24.04-portable | 80 | 75 | 9 | 87 |
| windows-2025-native | 77 | 76 | 11 | 87 |
| ALL | 461 | 448 | 62 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.001x | 1.296x | 35/36 | 0.914x |
| chess960_materialize | 0.955x | 1.438x | 12/12 | 1.222x |
| chess960_perft_d3 | 0.958x | 1.440x | 12/12 | 1.280x |
| corpus_count | 0.962x | 1.141x | 8/12 | 0.822x |
| corpus_materialize | 0.999x | 1.145x | 12/12 | 1.040x |
| corpus_uci_sorted | 1.010x | 1.524x | 12/12 | 1.436x |
| count | 0.948x | 1.373x | 36/36 | 1.010x |
| fen_parse | 1.024x | 1.199x | 35/36 | 0.962x |
| fen_write_native_ep | 0.920x | 1.586x | 36/36 | 1.152x |
| make_unmake_full | 0.945x | 1.232x | 35/36 | 0.994x |
| materialize | 1.004x | 1.305x | 33/36 | 0.878x |
| perft_bulk_d4 | 0.975x | 1.222x | 33/36 | 0.871x |
| perft_full_d3 | 0.915x | 1.213x | 35/36 | 0.960x |
| polyglot_key | 1.012x | 32.878x | 12/12 | 16.822x |
| polyglot_recompute | 0.975x | 1.000x | 7/12 | 0.847x |
| repetition_key | 1.043x | 21.513x | 12/12 | 15.612x |
| replay_checked_native | 0.907x | 1.070x | 9/12 | 0.868x |
| replay_trusted_native | 0.944x | 0.728x | 3/12 | 0.533x |
| san_parse_all | 1.011x | 1.814x | 36/36 | 1.375x |
| san_render_native | 0.970x | 2.482x | 36/36 | 1.396x |
| snapshot | 0.859x | 0.931x | 8/36 | 0.639x |
| startpos | 0.895x | 1.112x | 4/6 | 0.897x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.533x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.558x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.562x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.579x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.612x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.620x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.623x |
| macos-15-native | snapshot | tactical | ferrum | 0.639x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.816x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.822x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.847x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 0.858x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.864x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.866x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.866x |
| ubuntu-24.04-pext | snapshot | kiwipete | chess | 0.866x |
| ubuntu-24.04-native | snapshot | promotion | cozy | 0.866x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.867x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.867x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.867x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.868x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.868x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.871x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.871x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.874x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.878x |
| macos-15-native | materialize | tactical | ultra | 0.878x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.881x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.884x |
| macos-15-native | materialize | middlegame | ultra | 0.890x |
| windows-2025-native | startpos | start | shakmaty | 0.897x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.906x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.913x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.913x |
| macos-15-native | apply_children_full | tactical | chess | 0.914x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.919x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.922x |
| macos-15-native | materialize | kiwipete | ultra | 0.926x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.928x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.929x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.932x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.946x |
| macos-15-native | snapshot | endgame | ferrum | 0.951x |
| macos-15-native | snapshot | promotion | ferrum | 0.956x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.960x |
| macos-15-native | fen_parse | middlegame | ultra | 0.962x |
| macos-15-native | snapshot | start | ferrum | 0.969x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.970x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.976x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.982x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.985x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.993x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.994x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.995x |
| windows-2025-native | snapshot | promotion | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.997x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.998x |
| windows-2025-native | snapshot | start | ferrum | 0.999x |
| windows-2025-native | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.999x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 2625 | shakmaty | 4625 | 1.762x |
| ubuntu-24.04-arm-native | 5392 | chess | 9951 | 1.846x |
| ubuntu-24.04-native | 6211 | shakmaty | 7115 | 1.146x |
| ubuntu-24.04-pext | 5309 | cozy | 8094 | 1.525x |
| ubuntu-24.04-portable | 8773 | shakmaty | 9675 | 1.103x |
| windows-2025-native | 21900 | shakmaty | 27500 | 1.256x |

Cold wins: 6/6 (separate from native cases).
