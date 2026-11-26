| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 75 | 14 | 87 |
| ubuntu-24.04-arm-native | 84 | 79 | 12 | 87 |
| ubuntu-24.04-native | 79 | 78 | 11 | 87 |
| ubuntu-24.04-pext | 84 | 82 | 11 | 87 |
| ubuntu-24.04-portable | 79 | 78 | 12 | 87 |
| windows-2025-native | 81 | 75 | 12 | 87 |
| ALL | 484 | 467 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.000x | 1.270x | 35/36 | 0.895x |
| chess960_materialize | 0.970x | 1.482x | 12/12 | 1.258x |
| chess960_perft_d3 | 0.993x | 1.534x | 12/12 | 1.248x |
| corpus_count | 0.946x | 1.195x | 11/12 | 0.645x |
| corpus_materialize | 1.015x | 1.174x | 12/12 | 1.045x |
| corpus_uci_sorted | 1.006x | 1.543x | 12/12 | 1.291x |
| count | 0.988x | 1.482x | 36/36 | 1.112x |
| fen_parse | 0.966x | 1.179x | 35/36 | 0.990x |
| fen_write_native_ep | 0.968x | 1.707x | 36/36 | 1.435x |
| make_unmake_full | 1.006x | 1.321x | 36/36 | 1.030x |
| materialize | 0.968x | 1.295x | 34/36 | 0.751x |
| perft_bulk_d4 | 1.015x | 1.319x | 35/36 | 0.921x |
| perft_full_d3 | 0.939x | 1.243x | 35/36 | 0.931x |
| polyglot_key | 0.997x | 30.575x | 12/12 | 16.300x |
| polyglot_recompute | 0.991x | 1.046x | 8/12 | 0.963x |
| repetition_key | 0.917x | 19.092x | 12/12 | 14.342x |
| replay_checked_native | 0.994x | 1.063x | 12/12 | 1.002x |
| replay_trusted_native | 0.994x | 0.777x | 4/12 | 0.543x |
| san_parse_all | 1.030x | 1.942x | 36/36 | 1.538x |
| san_render_native | 0.978x | 2.477x | 36/36 | 1.622x |
| snapshot | 0.840x | 0.951x | 17/36 | 0.607x |
| startpos | 0.944x | 1.339x | 6/6 | 1.073x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.543x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| macos-15-native | snapshot | kiwipete | cozy | 0.607x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.634x |
| macos-15-native | corpus_count | seeded | ultra | 0.645x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.667x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.691x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.693x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.696x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.697x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.698x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.698x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.698x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.699x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.707x |
| macos-15-native | materialize | tactical | ultra | 0.751x |
| macos-15-native | apply_children_full | promotion | chess | 0.895x |
| macos-15-native | snapshot | middlegame | ferrum | 0.909x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.921x |
| ubuntu-24.04-portable | perft_full_d3 | start | shakmaty | 0.931x |
| macos-15-native | snapshot | endgame | ferrum | 0.942x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.962x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.963x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.971x |
| macos-15-native | materialize | kiwipete | ultra | 0.972x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.977x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.981x |
| windows-2025-native | snapshot | promotion | ferrum | 0.982x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.988x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.989x |
| macos-15-native | fen_parse | middlegame | cozy | 0.990x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.993x |
| windows-2025-native | snapshot | tactical | ferrum | 0.994x |
| windows-2025-native | snapshot | start | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | tactical | chess | 0.994x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.995x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.995x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6666 | shakmaty | 10208 | 1.531x |
| ubuntu-24.04-arm-native | 5679 | shakmaty | 8271 | 1.456x |
| ubuntu-24.04-native | 7865 | shakmaty | 11111 | 1.413x |
| ubuntu-24.04-pext | 5075 | shakmaty | 7867 | 1.550x |
| ubuntu-24.04-portable | 9017 | shakmaty | 10700 | 1.187x |
| windows-2025-native | 18000 | shakmaty | 21600 | 1.200x |

Cold wins: 6/6 (separate from native cases).
