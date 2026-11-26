| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 75 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 9 | 87 |
| ubuntu-24.04-pext | 84 | 81 | 9 | 87 |
| ubuntu-24.04-portable | 77 | 77 | 14 | 87 |
| windows-2025-native | 76 | 68 | 12 | 87 |
| ALL | 476 | 462 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.022x | 1.250x | 35/36 | 0.980x |
| chess960_materialize | 1.012x | 1.531x | 12/12 | 1.340x |
| chess960_perft_d3 | 0.973x | 1.500x | 12/12 | 1.285x |
| corpus_count | 1.010x | 1.211x | 9/12 | 0.821x |
| corpus_materialize | 0.970x | 1.147x | 11/12 | 0.902x |
| corpus_uci_sorted | 1.025x | 1.556x | 12/12 | 1.237x |
| count | 0.996x | 1.449x | 36/36 | 1.055x |
| fen_parse | 0.961x | 1.122x | 32/36 | 0.912x |
| fen_write_native_ep | 1.004x | 1.695x | 36/36 | 1.340x |
| make_unmake_full | 0.983x | 1.290x | 35/36 | 0.987x |
| materialize | 1.001x | 1.339x | 35/36 | 0.855x |
| perft_bulk_d4 | 1.007x | 1.242x | 34/36 | 0.928x |
| perft_full_d3 | 1.001x | 1.330x | 36/36 | 1.028x |
| polyglot_key | 1.006x | 32.659x | 12/12 | 15.530x |
| polyglot_recompute | 1.015x | 1.020x | 10/12 | 0.801x |
| repetition_key | 1.027x | 23.336x | 12/12 | 13.650x |
| replay_checked_native | 0.988x | 1.065x | 11/12 | 0.960x |
| replay_trusted_native | 0.991x | 0.761x | 2/12 | 0.531x |
| san_parse_all | 0.993x | 1.834x | 36/36 | 1.380x |
| san_render_native | 1.031x | 2.586x | 36/36 | 1.797x |
| snapshot | 0.984x | 1.078x | 18/36 | 0.781x |
| startpos | 0.958x | 1.139x | 4/6 | 0.933x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.531x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.545x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.558x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.567x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.653x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.653x |
| macos-15-native | snapshot | tactical | ferrum | 0.781x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.801x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.821x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.852x |
| macos-15-native | materialize | tactical | ultra | 0.855x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.858x |
| windows-2025-native | corpus_count | seeded | ultra | 0.869x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.899x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.902x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.910x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.912x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.927x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.928x |
| macos-15-native | startpos | start | shakmaty | 0.933x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.946x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.947x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.949x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.950x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.957x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.960x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.966x |
| windows-2025-native | snapshot | tactical | ferrum | 0.968x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.969x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.970x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.971x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.976x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.977x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.980x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 0.985x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.985x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.987x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.989x |
| windows-2025-native | startpos | start | shakmaty | 0.990x |
| windows-2025-native | snapshot | endgame | chess | 0.991x |
| windows-2025-native | fen_parse | tactical | shakmaty | 0.996x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.997x |
| windows-2025-native | snapshot | middlegame | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4250 | shakmaty | 8125 | 1.912x |
| ubuntu-24.04-arm-native | 5513 | shakmaty | 7521 | 1.364x |
| ubuntu-24.04-native | 7211 | chess | 8603 | 1.193x |
| ubuntu-24.04-pext | 6273 | chess | 7956 | 1.268x |
| ubuntu-24.04-portable | 7261 | chess | 11418 | 1.573x |
| windows-2025-native | 18100 | shakmaty | 27100 | 1.497x |

Cold wins: 6/6 (separate from native cases).
