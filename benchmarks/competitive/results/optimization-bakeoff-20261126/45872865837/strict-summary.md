| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 69 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 13 | 87 |
| ubuntu-24.04-native | 73 | 69 | 13 | 87 |
| ubuntu-24.04-pext | 76 | 73 | 9 | 87 |
| ubuntu-24.04-portable | 77 | 76 | 12 | 87 |
| windows-2025-native | 83 | 78 | 12 | 87 |
| ALL | 459 | 444 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.997x | 1.259x | 35/36 | 0.923x |
| chess960_materialize | 0.969x | 1.466x | 12/12 | 1.245x |
| chess960_perft_d3 | 1.004x | 1.494x | 12/12 | 1.186x |
| corpus_count | 0.965x | 1.216x | 10/12 | 0.956x |
| corpus_materialize | 1.021x | 1.167x | 12/12 | 1.031x |
| corpus_uci_sorted | 0.988x | 1.530x | 12/12 | 1.372x |
| count | 1.007x | 1.464x | 34/36 | 0.710x |
| fen_parse | 0.999x | 1.150x | 29/36 | 0.919x |
| fen_write_native_ep | 0.996x | 1.731x | 36/36 | 1.329x |
| make_unmake_full | 0.970x | 1.299x | 35/36 | 0.985x |
| materialize | 0.992x | 1.344x | 34/36 | 0.669x |
| perft_bulk_d4 | 1.002x | 1.279x | 34/36 | 0.902x |
| perft_full_d3 | 1.198x | 1.570x | 36/36 | 1.288x |
| polyglot_key | 0.980x | 31.839x | 12/12 | 16.395x |
| polyglot_recompute | 0.971x | 1.021x | 8/12 | 0.930x |
| repetition_key | 1.031x | 21.112x | 12/12 | 14.421x |
| replay_checked_native | 0.985x | 1.146x | 11/12 | 0.961x |
| replay_trusted_native | 0.975x | 0.767x | 3/12 | 0.598x |
| san_parse_all | 1.028x | 1.943x | 36/36 | 1.525x |
| san_render_native | 1.010x | 2.626x | 36/36 | 1.868x |
| snapshot | 0.927x | 0.916x | 5/36 | 0.460x |
| startpos | 0.906x | 1.187x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | promotion | ferrum | 0.460x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.569x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.598x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.603x |
| macos-15-native | snapshot | start | ferrum | 0.620x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.629x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.646x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.649x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.668x |
| macos-15-native | materialize | middlegame | ultra | 0.669x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.678x |
| macos-15-native | snapshot | tactical | cozy | 0.702x |
| macos-15-native | count | middlegame | cozy | 0.710x |
| macos-15-native | snapshot | middlegame | ferrum | 0.788x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.795x |
| macos-15-native | snapshot | endgame | ferrum | 0.804x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.858x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.902x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.919x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.921x |
| macos-15-native | apply_children_full | tactical | chess | 0.923x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.930x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.954x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.956x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.958x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.960x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.961x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.962x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.963x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.963x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.970x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.971x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.971x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.972x |
| windows-2025-native | snapshot | start | cozy | 0.972x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.978x |
| macos-15-native | count | endgame | cozy | 0.980x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| ubuntu-24.04-native | fen_parse | endgame | ultra | 0.982x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.982x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.983x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.983x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.985x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.986x |
| windows-2025-native | snapshot | tactical | ferrum | 0.987x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.988x |
| windows-2025-native | snapshot | promotion | cozy | 0.990x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.991x |
| macos-15-native | materialize | kiwipete | ultra | 0.993x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.993x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.994x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.995x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.996x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.996x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.997x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.997x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.997x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.998x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.999x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7458 | shakmaty | 10792 | 1.447x |
| ubuntu-24.04-arm-native | 5800 | shakmaty | 8273 | 1.426x |
| ubuntu-24.04-native | 5392 | shakmaty | 6794 | 1.260x |
| ubuntu-24.04-pext | 5382 | shakmaty | 7456 | 1.385x |
| ubuntu-24.04-portable | 7354 | shakmaty | 9869 | 1.342x |
| windows-2025-native | 18000 | shakmaty | 24600 | 1.367x |

Cold wins: 6/6 (separate from native cases).
