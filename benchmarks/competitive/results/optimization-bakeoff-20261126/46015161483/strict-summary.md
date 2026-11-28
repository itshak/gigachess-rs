| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 80 | 76 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 87 | 87 | 13 | 87 |
| ubuntu-24.04-pext | 85 | 85 | 14 | 87 |
| ubuntu-24.04-portable | 80 | 77 | 10 | 87 |
| windows-2025-native | 84 | 80 | 15 | 87 |
| ALL | 495 | 484 | 77 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.012x | 1.268x | 36/36 | 1.014x |
| chess960_materialize | 1.018x | 1.480x | 12/12 | 1.238x |
| chess960_perft_d3 | 1.034x | 1.531x | 12/12 | 1.364x |
| corpus_count | 1.142x | 1.361x | 11/12 | 0.961x |
| corpus_materialize | 1.009x | 1.165x | 12/12 | 1.025x |
| corpus_uci_sorted | 1.017x | 1.526x | 12/12 | 1.398x |
| count | 1.063x | 1.528x | 36/36 | 1.157x |
| fen_parse | 0.991x | 1.208x | 35/36 | 0.999x |
| fen_write_native_ep | 1.025x | 1.682x | 36/36 | 1.323x |
| make_unmake_full | 1.024x | 1.323x | 36/36 | 1.055x |
| materialize | 1.024x | 1.283x | 33/36 | 0.867x |
| perft_bulk_d4 | 1.084x | 1.371x | 35/36 | 0.950x |
| perft_full_d3 | 1.247x | 1.593x | 36/36 | 1.248x |
| polyglot_key | 1.008x | 29.306x | 12/12 | 16.001x |
| polyglot_recompute | 1.322x | 1.360x | 12/12 | 1.130x |
| repetition_key | 0.972x | 20.008x | 12/12 | 14.823x |
| replay_checked_native | 1.010x | 1.167x | 12/12 | 1.064x |
| replay_trusted_native | 1.035x | 0.808x | 4/12 | 0.553x |
| san_parse_all | 1.009x | 1.847x | 36/36 | 1.335x |
| san_render_native | 1.015x | 2.603x | 36/36 | 1.605x |
| snapshot | 1.010x | 1.067x | 23/36 | 0.883x |
| startpos | 1.017x | 1.271x | 6/6 | 1.118x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.553x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.560x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.580x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.667x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.684x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.701x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.849x |
| macos-15-native | materialize | kiwipete | ultra | 0.867x |
| macos-15-native | snapshot | endgame | ferrum | 0.883x |
| macos-15-native | materialize | start | ultra | 0.909x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.931x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.939x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.950x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.961x |
| macos-15-native | materialize | tactical | ultra | 0.961x |
| macos-15-native | snapshot | tactical | ferrum | 0.966x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.978x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.980x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.992x |
| windows-2025-native | snapshot | tactical | cozy | 0.994x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.999x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5917 | shakmaty | 9833 | 1.662x |
| ubuntu-24.04-arm-native | 6952 | shakmaty | 8311 | 1.195x |
| ubuntu-24.04-native | 7681 | shakmaty | 10355 | 1.348x |
| ubuntu-24.04-pext | 5296 | shakmaty | 6877 | 1.299x |
| ubuntu-24.04-portable | 7601 | shakmaty | 9464 | 1.245x |
| windows-2025-native | 16400 | shakmaty | 23200 | 1.415x |

Cold wins: 6/6 (separate from native cases).
