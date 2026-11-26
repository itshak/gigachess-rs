| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 67 | 13 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 80 | 78 | 12 | 87 |
| ubuntu-24.04-pext | 78 | 71 | 9 | 87 |
| ubuntu-24.04-portable | 83 | 74 | 14 | 87 |
| windows-2025-native | 79 | 75 | 12 | 87 |
| ALL | 467 | 442 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.993x | 1.175x | 32/36 | 0.937x |
| chess960_materialize | 0.959x | 1.368x | 12/12 | 1.213x |
| chess960_perft_d3 | 0.953x | 1.315x | 12/12 | 1.059x |
| corpus_count | 0.930x | 1.029x | 5/12 | 0.849x |
| corpus_materialize | 0.976x | 1.084x | 11/12 | 0.968x |
| corpus_uci_sorted | 0.989x | 1.461x | 12/12 | 1.267x |
| count | 1.003x | 1.320x | 33/36 | 0.909x |
| fen_parse | 0.997x | 1.199x | 36/36 | 1.035x |
| fen_write_native_ep | 0.988x | 1.780x | 36/36 | 1.433x |
| make_unmake_full | 0.992x | 1.241x | 36/36 | 1.024x |
| materialize | 1.010x | 1.322x | 35/36 | 0.944x |
| perft_bulk_d4 | 1.004x | 1.117x | 30/36 | 0.834x |
| perft_full_d3 | 0.996x | 1.312x | 35/36 | 0.950x |
| polyglot_key | 0.980x | 34.557x | 12/12 | 16.683x |
| polyglot_recompute | 0.944x | 1.028x | 9/12 | 0.895x |
| repetition_key | 0.988x | 17.853x | 12/12 | 9.326x |
| replay_checked_native | 0.988x | 1.057x | 11/12 | 0.918x |
| replay_trusted_native | 1.004x | 0.703x | 1/12 | 0.508x |
| san_parse_all | 1.002x | 1.893x | 36/36 | 1.534x |
| san_render_native | 0.999x | 2.566x | 36/36 | 1.903x |
| snapshot | 1.013x | 1.013x | 19/36 | 0.797x |
| startpos | 0.971x | 1.195x | 6/6 | 1.003x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.508x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.529x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.567x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.619x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.664x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.675x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.725x |
| macos-15-native | snapshot | middlegame | ferrum | 0.797x |
| macos-15-native | snapshot | promotion | ferrum | 0.802x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.824x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.834x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.843x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.849x |
| macos-15-native | snapshot | kiwipete | chess | 0.869x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.870x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.885x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.895x |
| macos-15-native | count | promotion | cozy | 0.909x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.917x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.918x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.919x |
| macos-15-native | corpus_count | seeded | ultra | 0.925x |
| macos-15-native | count | middlegame | cozy | 0.928x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.929x |
| ubuntu-24.04-native | apply_children_full | promotion | chess | 0.937x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.937x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.944x |
| macos-15-native | materialize | middlegame | ultra | 0.944x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.948x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.950x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.951x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.952x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.955x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.959x |
| macos-15-native | perft_bulk_d4 | tactical | cozy | 0.961x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.962x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.967x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.968x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.971x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.972x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.975x |
| windows-2025-native | snapshot | kiwipete | chess | 0.976x |
| windows-2025-native | snapshot | tactical | chess | 0.981x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.983x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.988x |
| windows-2025-native | snapshot | promotion | chess | 0.989x |
| macos-15-native | snapshot | tactical | ferrum | 0.990x |
| windows-2025-native | snapshot | start | chess | 0.994x |
| macos-15-native | apply_children_full | start | chess | 0.995x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.996x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.997x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 8208 | ferrum | 11958 | 1.457x |
| ubuntu-24.04-arm-native | 7015 | chess | 9864 | 1.406x |
| ubuntu-24.04-native | 6973 | shakmaty | 10442 | 1.497x |
| ubuntu-24.04-pext | 6348 | cozy | 8676 | 1.367x |
| ubuntu-24.04-portable | 5589 | shakmaty | 7953 | 1.423x |
| windows-2025-native | 14900 | shakmaty | 19700 | 1.322x |

Cold wins: 6/6 (separate from native cases).
