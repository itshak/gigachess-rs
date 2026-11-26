| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 70 | 11 | 87 |
| ubuntu-24.04-arm-native | 73 | 69 | 10 | 87 |
| ubuntu-24.04-native | 80 | 79 | 8 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 8 | 87 |
| ubuntu-24.04-portable | 73 | 68 | 13 | 87 |
| windows-2025-native | 69 | 65 | 11 | 87 |
| ALL | 452 | 432 | 61 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.985x | 0.986x | 21/36 | 0.641x |
| chess960_materialize | 1.003x | 1.355x | 12/12 | 1.154x |
| chess960_perft_d3 | 0.996x | 1.393x | 12/12 | 1.272x |
| corpus_count | 0.995x | 1.346x | 12/12 | 1.145x |
| corpus_materialize | 1.003x | 1.171x | 12/12 | 1.053x |
| corpus_uci_sorted | 0.986x | 1.509x | 12/12 | 1.362x |
| count | 0.994x | 1.327x | 35/36 | 0.946x |
| fen_parse | 0.999x | 1.178x | 32/36 | 0.986x |
| fen_write_native_ep | 0.999x | 1.243x | 35/36 | 0.970x |
| make_unmake_full | 0.997x | 1.241x | 36/36 | 1.046x |
| materialize | 1.007x | 1.261x | 32/36 | 0.870x |
| perft_bulk_d4 | 0.996x | 1.257x | 36/36 | 1.044x |
| perft_full_d3 | 1.004x | 1.219x | 30/36 | 0.725x |
| polyglot_key | 1.005x | 33.363x | 12/12 | 16.775x |
| polyglot_recompute | 0.976x | 0.999x | 7/12 | 0.871x |
| repetition_key | 0.988x | 21.085x | 12/12 | 14.869x |
| replay_checked_native | 0.997x | 1.009x | 8/12 | 0.830x |
| replay_trusted_native | 0.984x | 0.644x | 0/12 | 0.473x |
| san_parse_all | 0.991x | 1.930x | 36/36 | 1.526x |
| san_render_native | 1.003x | 2.513x | 36/36 | 1.868x |
| snapshot | 1.073x | 1.043x | 18/36 | 0.913x |
| startpos | 1.061x | 1.234x | 6/6 | 1.184x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.473x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.487x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.513x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.521x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.567x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.615x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.632x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.641x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.711x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.712x |
| windows-2025-native | apply_children_full | endgame | cozy | 0.716x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.724x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.725x |
| windows-2025-native | apply_children_full | start | cozy | 0.750x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.769x |
| windows-2025-native | apply_children_full | middlegame | cozy | 0.796x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.805x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.810x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.827x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.828x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.830x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.831x |
| macos-15-native | materialize | tactical | ultra | 0.870x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.871x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.877x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.889x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.911x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.911x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.913x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.916x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.916x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.917x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.917x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.918x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.921x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.927x |
| macos-15-native | apply_children_full | tactical | chess | 0.930x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.931x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.935x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.936x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.938x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.941x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.942x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.944x |
| windows-2025-native | perft_full_d3 | endgame | ultra | 0.945x |
| ubuntu-24.04-portable | count | start | ferrum | 0.946x |
| ubuntu-24.04-portable | apply_children_full | promotion | ferrum | 0.948x |
| windows-2025-native | materialize | middlegame | ultra | 0.951x |
| macos-15-native | materialize | kiwipete | ultra | 0.956x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.960x |
| macos-15-native | materialize | middlegame | ultra | 0.960x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.963x |
| macos-15-native | snapshot | tactical | ferrum | 0.967x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.967x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.970x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.970x |
| macos-15-native | snapshot | promotion | ferrum | 0.981x |
| ubuntu-24.04-pext | apply_children_full | tactical | chess | 0.982x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.983x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.986x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.988x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.988x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.991x |
| macos-15-native | snapshot | middlegame | ferrum | 0.994x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.995x |
| windows-2025-native | snapshot | promotion | ferrum | 0.996x |
| ubuntu-24.04-native | fen_parse | middlegame | ultra | 0.996x |
| windows-2025-native | snapshot | endgame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6625 | shakmaty | 5334 | 0.805x |
| ubuntu-24.04-arm-native | 10504 | shakmaty | 8016 | 0.763x |
| ubuntu-24.04-native | 8913 | shakmaty | 6812 | 0.764x |
| ubuntu-24.04-pext | 9734 | shakmaty | 8486 | 0.872x |
| ubuntu-24.04-portable | 11418 | shakmaty | 8363 | 0.732x |
| windows-2025-native | 20300 | shakmaty | 16900 | 0.833x |

Cold wins: 0/6 (separate from native cases).
