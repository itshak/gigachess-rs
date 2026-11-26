| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 73 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 82 | 11 | 87 |
| ubuntu-24.04-pext | 83 | 79 | 15 | 87 |
| ubuntu-24.04-portable | 81 | 76 | 10 | 87 |
| windows-2025-native | 80 | 79 | 13 | 87 |
| ALL | 482 | 468 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.996x | 1.268x | 34/36 | 0.966x |
| chess960_materialize | 1.024x | 1.480x | 12/12 | 1.296x |
| chess960_perft_d3 | 1.012x | 1.502x | 12/12 | 1.367x |
| corpus_count | 1.066x | 1.122x | 9/12 | 0.971x |
| corpus_materialize | 1.006x | 1.112x | 12/12 | 1.042x |
| corpus_uci_sorted | 1.019x | 1.498x | 12/12 | 1.206x |
| count | 1.001x | 1.508x | 36/36 | 1.038x |
| fen_parse | 0.990x | 1.170x | 35/36 | 0.995x |
| fen_write_native_ep | 1.001x | 1.809x | 36/36 | 1.290x |
| make_unmake_full | 0.997x | 1.280x | 36/36 | 1.047x |
| materialize | 0.993x | 1.417x | 32/36 | 0.906x |
| perft_bulk_d4 | 0.993x | 1.225x | 32/36 | 0.854x |
| perft_full_d3 | 1.194x | 1.614x | 36/36 | 1.225x |
| polyglot_key | 0.996x | 37.254x | 12/12 | 16.917x |
| polyglot_recompute | 1.006x | 0.998x | 7/12 | 0.921x |
| repetition_key | 1.021x | 23.474x | 12/12 | 14.629x |
| replay_checked_native | 1.059x | 1.079x | 12/12 | 1.023x |
| replay_trusted_native | 1.002x | 0.780x | 4/12 | 0.503x |
| san_parse_all | 1.053x | 1.857x | 36/36 | 1.341x |
| san_render_native | 0.993x | 2.533x | 36/36 | 1.845x |
| snapshot | 0.976x | 1.062x | 24/36 | 0.902x |
| startpos | 0.994x | 1.177x | 5/6 | 0.991x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.503x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.574x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.659x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.667x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.668x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.692x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.710x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.854x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.868x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.901x |
| windows-2025-native | snapshot | endgame | ferrum | 0.902x |
| macos-15-native | materialize | kiwipete | ultra | 0.906x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.920x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.921x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.926x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.930x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.930x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.932x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.935x |
| macos-15-native | materialize | tactical | ultra | 0.936x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.944x |
| macos-15-native | materialize | middlegame | ultra | 0.945x |
| macos-15-native | materialize | promotion | ultra | 0.959x |
| macos-15-native | apply_children_full | tactical | chess | 0.966x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.971x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.981x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.983x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.983x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.986x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.986x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.987x |
| macos-15-native | snapshot | promotion | ferrum | 0.988x |
| windows-2025-native | corpus_count | seeded | ultra | 0.990x |
| windows-2025-native | startpos | start | shakmaty | 0.991x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5792 | ferrum | 6417 | 1.108x |
| ubuntu-24.04-arm-native | 5671 | cozy | 8688 | 1.532x |
| ubuntu-24.04-native | 5621 | shakmaty | 8782 | 1.562x |
| ubuntu-24.04-pext | 5620 | shakmaty | 8674 | 1.543x |
| ubuntu-24.04-portable | 6946 | shakmaty | 8897 | 1.281x |
| windows-2025-native | 18800 | shakmaty | 24400 | 1.298x |

Cold wins: 6/6 (separate from native cases).
