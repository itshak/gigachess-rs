| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 71 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 80 | 77 | 14 | 87 |
| ubuntu-24.04-pext | 84 | 80 | 11 | 87 |
| ubuntu-24.04-portable | 80 | 79 | 12 | 87 |
| windows-2025-native | 80 | 78 | 14 | 87 |
| ALL | 475 | 464 | 78 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.996x | 1.278x | 33/36 | 0.893x |
| chess960_materialize | 0.986x | 1.480x | 12/12 | 1.249x |
| chess960_perft_d3 | 1.033x | 1.538x | 12/12 | 1.168x |
| corpus_count | 1.112x | 1.348x | 12/12 | 1.066x |
| corpus_materialize | 1.016x | 1.154x | 12/12 | 1.043x |
| corpus_uci_sorted | 1.015x | 1.503x | 12/12 | 1.225x |
| count | 1.067x | 1.545x | 36/36 | 1.085x |
| fen_parse | 0.988x | 1.136x | 31/36 | 0.931x |
| fen_write_native_ep | 1.011x | 1.730x | 36/36 | 1.128x |
| make_unmake_full | 0.997x | 1.313x | 36/36 | 1.064x |
| materialize | 1.003x | 1.344x | 34/36 | 0.850x |
| perft_bulk_d4 | 1.067x | 1.341x | 34/36 | 0.911x |
| perft_full_d3 | 1.214x | 1.586x | 36/36 | 1.325x |
| polyglot_key | 1.003x | 31.509x | 12/12 | 16.743x |
| polyglot_recompute | 0.978x | 1.000x | 7/12 | 0.716x |
| repetition_key | 1.066x | 21.795x | 12/12 | 14.434x |
| replay_checked_native | 1.044x | 1.173x | 12/12 | 1.055x |
| replay_trusted_native | 0.996x | 0.788x | 3/12 | 0.524x |
| san_parse_all | 1.012x | 1.892x | 36/36 | 1.287x |
| san_render_native | 1.026x | 2.615x | 36/36 | 1.834x |
| snapshot | 0.988x | 1.020x | 16/36 | 0.627x |
| startpos | 0.909x | 1.115x | 5/6 | 0.783x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.524x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| macos-15-native | snapshot | tactical | ferrum | 0.627x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.674x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.675x |
| macos-15-native | snapshot | promotion | chess | 0.683x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.688x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.705x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.716x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.737x |
| macos-15-native | snapshot | start | ferrum | 0.750x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.758x |
| macos-15-native | startpos | start | shakmaty | 0.783x |
| macos-15-native | materialize | start | ultra | 0.850x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.870x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.882x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.893x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.911x |
| macos-15-native | snapshot | middlegame | ferrum | 0.930x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.931x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.935x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.940x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.942x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.945x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.946x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.954x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.957x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.959x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.960x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.962x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.971x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.972x |
| macos-15-native | fen_parse | middlegame | cozy | 0.975x |
| windows-2025-native | snapshot | middlegame | cozy | 0.976x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.978x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.979x |
| macos-15-native | apply_children_full | start | chess | 0.980x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.981x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.983x |
| windows-2025-native | snapshot | tactical | cozy | 0.985x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.985x |
| windows-2025-native | snapshot | endgame | ferrum | 0.987x |
| macos-15-native | materialize | tactical | ultra | 0.993x |
| windows-2025-native | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6250 | shakmaty | 7333 | 1.173x |
| ubuntu-24.04-arm-native | 6559 | chess | 8879 | 1.354x |
| ubuntu-24.04-native | 6453 | ferrum | 10722 | 1.662x |
| ubuntu-24.04-pext | 5897 | shakmaty | 8285 | 1.405x |
| ubuntu-24.04-portable | 7073 | shakmaty | 9727 | 1.375x |
| windows-2025-native | 14700 | shakmaty | 22200 | 1.510x |

Cold wins: 6/6 (separate from native cases).
