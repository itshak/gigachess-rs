| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 73 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 85 | 85 | 16 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 9 | 87 |
| ubuntu-24.04-portable | 80 | 77 | 12 | 87 |
| windows-2025-native | 82 | 79 | 12 | 87 |
| ALL | 484 | 474 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.995x | 1.263x | 34/36 | 0.938x |
| chess960_materialize | 1.079x | 1.509x | 12/12 | 1.262x |
| chess960_perft_d3 | 1.034x | 1.545x | 12/12 | 1.383x |
| corpus_count | 1.052x | 1.228x | 10/12 | 0.825x |
| corpus_materialize | 1.035x | 1.173x | 12/12 | 1.059x |
| corpus_uci_sorted | 1.009x | 1.513x | 12/12 | 1.265x |
| count | 1.012x | 1.497x | 36/36 | 1.030x |
| fen_parse | 1.027x | 1.206x | 35/36 | 0.934x |
| fen_write_native_ep | 1.003x | 1.767x | 36/36 | 1.284x |
| make_unmake_full | 1.009x | 1.323x | 36/36 | 1.041x |
| materialize | 1.053x | 1.353x | 34/36 | 0.772x |
| perft_bulk_d4 | 1.023x | 1.293x | 33/36 | 0.924x |
| perft_full_d3 | 1.013x | 1.304x | 36/36 | 1.100x |
| polyglot_key | 1.008x | 30.306x | 12/12 | 16.949x |
| polyglot_recompute | 1.036x | 1.004x | 7/12 | 0.829x |
| repetition_key | 1.015x | 20.040x | 12/12 | 14.613x |
| replay_checked_native | 1.048x | 1.171x | 12/12 | 1.057x |
| replay_trusted_native | 1.044x | 0.829x | 5/12 | 0.558x |
| san_parse_all | 1.018x | 1.940x | 36/36 | 1.393x |
| san_render_native | 1.020x | 2.573x | 36/36 | 1.641x |
| snapshot | 0.995x | 1.046x | 20/36 | 0.754x |
| startpos | 1.041x | 1.345x | 6/6 | 1.121x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.558x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.579x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.659x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.681x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.751x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.754x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.764x |
| macos-15-native | materialize | kiwipete | ultra | 0.772x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.825x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.829x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.920x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.924x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.934x |
| macos-15-native | apply_children_full | start | chess | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | cozy | 0.939x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.943x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.944x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.950x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.950x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.957x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.967x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.974x |
| macos-15-native | materialize | tactical | ultra | 0.977x |
| windows-2025-native | snapshot | promotion | chess | 0.985x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.985x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| macos-15-native | snapshot | endgame | ferrum | 0.989x |
| windows-2025-native | snapshot | start | chess | 0.990x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.992x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.994x |
| windows-2025-native | snapshot | endgame | chess | 0.994x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5750 | shakmaty | 11333 | 1.971x |
| ubuntu-24.04-arm-native | 5712 | shakmaty | 7616 | 1.333x |
| ubuntu-24.04-native | 6439 | shakmaty | 7100 | 1.103x |
| ubuntu-24.04-pext | 5374 | shakmaty | 6962 | 1.295x |
| ubuntu-24.04-portable | 6267 | shakmaty | 7702 | 1.229x |
| windows-2025-native | 14600 | shakmaty | 21500 | 1.473x |

Cold wins: 6/6 (separate from native cases).
