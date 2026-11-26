| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 71 | 13 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 79 | 76 | 9 | 87 |
| ubuntu-24.04-pext | 81 | 79 | 15 | 87 |
| ubuntu-24.04-portable | 79 | 76 | 11 | 87 |
| windows-2025-native | 80 | 75 | 12 | 87 |
| ALL | 471 | 454 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.006x | 1.265x | 34/36 | 0.924x |
| chess960_materialize | 0.972x | 1.392x | 12/12 | 1.247x |
| chess960_perft_d3 | 0.980x | 1.368x | 11/12 | 0.997x |
| corpus_count | 0.977x | 1.106x | 8/12 | 0.811x |
| corpus_materialize | 0.986x | 1.157x | 11/12 | 0.989x |
| corpus_uci_sorted | 0.972x | 1.520x | 12/12 | 1.101x |
| count | 0.995x | 1.322x | 34/36 | 0.945x |
| fen_parse | 1.062x | 1.272x | 36/36 | 1.041x |
| fen_write_native_ep | 0.994x | 1.727x | 36/36 | 1.423x |
| make_unmake_full | 0.995x | 1.307x | 35/36 | 0.993x |
| materialize | 0.994x | 1.236x | 35/36 | 0.953x |
| perft_bulk_d4 | 0.991x | 1.162x | 31/36 | 0.833x |
| perft_full_d3 | 0.992x | 1.189x | 31/36 | 0.912x |
| polyglot_key | 0.969x | 28.345x | 12/12 | 16.578x |
| polyglot_recompute | 0.997x | 1.039x | 10/12 | 0.895x |
| repetition_key | 0.955x | 17.604x | 12/12 | 14.367x |
| replay_checked_native | 0.959x | 1.065x | 10/12 | 0.974x |
| replay_trusted_native | 0.992x | 0.841x | 5/12 | 0.560x |
| san_parse_all | 0.995x | 1.948x | 36/36 | 1.383x |
| san_render_native | 1.006x | 2.544x | 36/36 | 1.643x |
| snapshot | 1.009x | 1.045x | 18/36 | 0.827x |
| startpos | 0.990x | 1.201x | 6/6 | 1.008x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.560x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.578x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.660x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.697x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.755x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.811x |
| macos-15-native | snapshot | start | chess | 0.827x |
| windows-2025-native | snapshot | tactical | ferrum | 0.831x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.833x |
| macos-15-native | snapshot | tactical | ferrum | 0.849x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.889x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.895x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.903x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.912x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.912x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.915x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.917x |
| macos-15-native | apply_children_full | tactical | chess | 0.924x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.938x |
| macos-15-native | count | middlegame | cozy | 0.945x |
| macos-15-native | materialize | tactical | ultra | 0.953x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.956x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.957x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.961x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.961x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.967x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.967x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.972x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.974x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.974x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.975x |
| windows-2025-native | snapshot | start | cozy | 0.977x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.979x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.980x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.980x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.988x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.989x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.989x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.993x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.994x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.996x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.996x |
| macos-15-native | chess960_perft_d3 | 960_start | cozy | 0.997x |
| macos-15-native | apply_children_full | promotion | chess | 0.999x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 1.000x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 2500 | shakmaty | 5625 | 2.250x |
| ubuntu-24.04-arm-native | 6704 | shakmaty | 8512 | 1.270x |
| ubuntu-24.04-native | 4719 | shakmaty | 7379 | 1.564x |
| ubuntu-24.04-pext | 7211 | shakmaty | 9264 | 1.285x |
| ubuntu-24.04-portable | 8562 | shakmaty | 11317 | 1.322x |
| windows-2025-native | 16800 | shakmaty | 25200 | 1.500x |

Cold wins: 6/6 (separate from native cases).
