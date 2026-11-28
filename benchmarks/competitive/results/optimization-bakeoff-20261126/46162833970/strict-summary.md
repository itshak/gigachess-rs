| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 75 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 81 | 14 | 87 |
| ubuntu-24.04-pext | 85 | 85 | 10 | 87 |
| ubuntu-24.04-portable | 84 | 79 | 15 | 87 |
| windows-2025-native | 84 | 84 | 14 | 87 |
| ALL | 491 | 483 | 80 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.980x | 1.263x | 34/36 | 0.893x |
| chess960_materialize | 1.011x | 1.503x | 12/12 | 1.349x |
| chess960_perft_d3 | 1.028x | 1.535x | 12/12 | 1.363x |
| corpus_count | 1.107x | 1.298x | 11/12 | 0.918x |
| corpus_materialize | 1.001x | 1.191x | 12/12 | 1.087x |
| corpus_uci_sorted | 0.981x | 1.518x | 12/12 | 1.195x |
| count | 0.991x | 1.505x | 36/36 | 1.017x |
| fen_parse | 0.971x | 1.163x | 32/36 | 0.938x |
| fen_write_native_ep | 0.982x | 1.695x | 36/36 | 1.246x |
| make_unmake_full | 1.024x | 1.338x | 36/36 | 1.072x |
| materialize | 0.978x | 1.333x | 36/36 | 1.026x |
| perft_bulk_d4 | 1.013x | 1.317x | 34/36 | 0.793x |
| perft_full_d3 | 1.173x | 1.539x | 36/36 | 1.035x |
| polyglot_key | 1.011x | 33.307x | 12/12 | 16.675x |
| polyglot_recompute | 1.493x | 1.508x | 12/12 | 1.184x |
| repetition_key | 0.962x | 23.209x | 12/12 | 15.092x |
| replay_checked_native | 0.962x | 1.209x | 12/12 | 1.102x |
| replay_trusted_native | 0.992x | 0.806x | 5/12 | 0.562x |
| san_parse_all | 1.017x | 1.903x | 36/36 | 1.415x |
| san_render_native | 0.997x | 2.664x | 36/36 | 1.650x |
| snapshot | 0.945x | 1.081x | 23/36 | 0.612x |
| startpos | 0.990x | 1.126x | 4/6 | 0.771x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.562x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.575x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.593x |
| macos-15-native | snapshot | tactical | cozy | 0.612x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.612x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.685x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.692x |
| macos-15-native | snapshot | kiwipete | cozy | 0.705x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.717x |
| macos-15-native | snapshot | promotion | ferrum | 0.766x |
| macos-15-native | startpos | start | shakmaty | 0.771x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.793x |
| macos-15-native | snapshot | start | chess | 0.832x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.893x |
| macos-15-native | apply_children_full | tactical | chess | 0.908x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.918x |
| ubuntu-24.04-native | fen_parse | kiwipete | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.948x |
| macos-15-native | snapshot | endgame | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.950x |
| ubuntu-24.04-native | fen_parse | middlegame | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.952x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.970x |
| ubuntu-24.04-native | fen_parse | promotion | ferrum | 0.983x |
| windows-2025-native | startpos | start | shakmaty | 0.987x |
| ubuntu-24.04-native | fen_parse | start | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7084 | shakmaty | 10333 | 1.459x |
| ubuntu-24.04-arm-native | 6640 | ferrum | 8808 | 1.327x |
| ubuntu-24.04-native | 6400 | ferrum | 10355 | 1.618x |
| ubuntu-24.04-pext | 6229 | cozy | 9324 | 1.497x |
| ubuntu-24.04-portable | 5829 | shakmaty | 8096 | 1.389x |
| windows-2025-native | 16400 | shakmaty | 20500 | 1.250x |

Cold wins: 6/6 (separate from native cases).
