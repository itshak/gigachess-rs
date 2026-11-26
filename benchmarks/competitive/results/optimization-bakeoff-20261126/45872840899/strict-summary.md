| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 79 | 75 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 86 | 84 | 12 | 87 |
| ubuntu-24.04-pext | 84 | 82 | 8 | 87 |
| ubuntu-24.04-portable | 82 | 78 | 11 | 87 |
| windows-2025-native | 82 | 82 | 12 | 87 |
| ALL | 492 | 480 | 67 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.012x | 1.277x | 36/36 | 1.031x |
| chess960_materialize | 0.982x | 1.505x | 12/12 | 1.311x |
| chess960_perft_d3 | 0.992x | 1.502x | 12/12 | 1.384x |
| corpus_count | 0.981x | 1.209x | 10/12 | 0.818x |
| corpus_materialize | 0.990x | 1.148x | 12/12 | 1.040x |
| corpus_uci_sorted | 0.996x | 1.519x | 12/12 | 1.383x |
| count | 0.989x | 1.447x | 35/36 | 0.966x |
| fen_parse | 0.987x | 1.187x | 36/36 | 1.046x |
| fen_write_native_ep | 0.996x | 1.664x | 36/36 | 1.309x |
| make_unmake_full | 0.991x | 1.326x | 36/36 | 1.010x |
| materialize | 0.981x | 1.271x | 33/36 | 0.938x |
| perft_bulk_d4 | 0.995x | 1.297x | 35/36 | 0.913x |
| perft_full_d3 | 0.995x | 1.324x | 36/36 | 1.156x |
| polyglot_key | 0.995x | 33.426x | 12/12 | 15.929x |
| polyglot_recompute | 0.991x | 1.028x | 9/12 | 0.872x |
| repetition_key | 0.992x | 21.915x | 12/12 | 13.849x |
| replay_checked_native | 0.960x | 1.136x | 12/12 | 1.004x |
| replay_trusted_native | 0.966x | 0.732x | 4/12 | 0.481x |
| san_parse_all | 0.984x | 1.822x | 36/36 | 1.375x |
| san_render_native | 1.005x | 2.633x | 36/36 | 1.825x |
| snapshot | 1.001x | 1.118x | 25/36 | 0.941x |
| startpos | 1.023x | 1.186x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.481x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.486x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.517x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.548x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.602x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.659x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.675x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.806x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.818x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.866x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.872x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.884x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.913x |
| macos-15-native | materialize | kiwipete | ultra | 0.938x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.941x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.951x |
| macos-15-native | snapshot | promotion | ferrum | 0.957x |
| macos-15-native | materialize | middlegame | ultra | 0.960x |
| windows-2025-native | count | middlegame | cozy | 0.966x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.980x |
| macos-15-native | materialize | tactical | ultra | 0.981x |
| macos-15-native | snapshot | endgame | ferrum | 0.987x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.993x |
| windows-2025-native | startpos | start | shakmaty | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3417 | shakmaty | 5792 | 1.695x |
| ubuntu-24.04-arm-native | 5496 | shakmaty | 8584 | 1.562x |
| ubuntu-24.04-native | 7565 | chess | 11933 | 1.577x |
| ubuntu-24.04-pext | 6064 | cozy | 7741 | 1.277x |
| ubuntu-24.04-portable | 6730 | shakmaty | 7762 | 1.153x |
| windows-2025-native | 16800 | shakmaty | 20100 | 1.196x |

Cold wins: 6/6 (separate from native cases).
