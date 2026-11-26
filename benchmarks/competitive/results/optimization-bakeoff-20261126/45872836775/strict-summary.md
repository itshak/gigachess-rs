| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 71 | 14 | 87 |
| ubuntu-24.04-arm-native | 78 | 78 | 12 | 87 |
| ubuntu-24.04-native | 84 | 83 | 15 | 87 |
| ubuntu-24.04-pext | 79 | 75 | 11 | 87 |
| ubuntu-24.04-portable | 80 | 78 | 11 | 87 |
| windows-2025-native | 83 | 80 | 12 | 87 |
| ALL | 476 | 465 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.982x | 1.268x | 35/36 | 0.957x |
| chess960_materialize | 0.998x | 1.456x | 12/12 | 1.246x |
| chess960_perft_d3 | 0.971x | 1.515x | 12/12 | 1.315x |
| corpus_count | 1.014x | 1.282x | 9/12 | 0.950x |
| corpus_materialize | 1.033x | 1.181x | 12/12 | 1.058x |
| corpus_uci_sorted | 1.016x | 1.577x | 12/12 | 1.451x |
| count | 0.976x | 1.478x | 36/36 | 1.113x |
| fen_parse | 0.990x | 1.215x | 36/36 | 1.041x |
| fen_write_native_ep | 0.979x | 1.738x | 36/36 | 1.428x |
| make_unmake_full | 1.000x | 1.328x | 36/36 | 1.032x |
| materialize | 0.981x | 1.263x | 33/36 | 0.831x |
| perft_bulk_d4 | 0.985x | 1.316x | 34/36 | 0.922x |
| perft_full_d3 | 0.978x | 1.277x | 35/36 | 0.991x |
| polyglot_key | 1.016x | 26.784x | 12/12 | 14.598x |
| polyglot_recompute | 0.850x | 0.865x | 2/12 | 0.731x |
| repetition_key | 0.949x | 18.919x | 12/12 | 13.871x |
| replay_checked_native | 1.035x | 1.168x | 12/12 | 1.078x |
| replay_trusted_native | 1.054x | 0.863x | 5/12 | 0.531x |
| san_parse_all | 0.990x | 1.928x | 36/36 | 1.385x |
| san_render_native | 0.987x | 2.498x | 36/36 | 1.559x |
| snapshot | 0.958x | 1.026x | 19/36 | 0.554x |
| startpos | 0.955x | 1.123x | 4/6 | 0.956x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.531x |
| macos-15-native | snapshot | start | ferrum | 0.554x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.582x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.671x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.708x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.723x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.731x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.751x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.759x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.769x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.772x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.784x |
| macos-15-native | materialize | tactical | ultra | 0.831x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.844x |
| macos-15-native | snapshot | endgame | cozy | 0.858x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.869x |
| macos-15-native | materialize | kiwipete | ultra | 0.872x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.911x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.915x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.922x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.948x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.949x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.950x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.953x |
| macos-15-native | startpos | start | shakmaty | 0.956x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.957x |
| macos-15-native | apply_children_full | tactical | chess | 0.957x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.966x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.980x |
| ubuntu-24.04-arm-native | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| macos-15-native | materialize | promotion | ultra | 0.982x |
| macos-15-native | snapshot | promotion | ferrum | 0.984x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.990x |
| macos-15-native | perft_full_d3 | kiwipete | ultra | 0.991x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.996x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | chess | 0.999x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.999x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.999x |
| ubuntu-24.04-portable | snapshot | middlegame | chess | 1.000x |
| ubuntu-24.04-portable | snapshot | kiwipete | chess | 1.000x |
| ubuntu-24.04-portable | snapshot | endgame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5667 | shakmaty | 9667 | 1.706x |
| ubuntu-24.04-arm-native | 5761 | shakmaty | 8560 | 1.486x |
| ubuntu-24.04-native | 8112 | shakmaty | 7581 | 0.935x |
| ubuntu-24.04-pext | 5776 | chess | 8972 | 1.553x |
| ubuntu-24.04-portable | 7955 | shakmaty | 10259 | 1.290x |
| windows-2025-native | 16600 | shakmaty | 20800 | 1.253x |

Cold wins: 5/6 (separate from native cases).
