| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 69 | 13 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 12 | 87 |
| ubuntu-24.04-native | 80 | 75 | 9 | 87 |
| ubuntu-24.04-pext | 79 | 76 | 13 | 87 |
| ubuntu-24.04-portable | 76 | 74 | 12 | 87 |
| windows-2025-native | 81 | 80 | 12 | 87 |
| ALL | 467 | 451 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.011x | 1.171x | 34/36 | 0.916x |
| chess960_materialize | 1.064x | 1.413x | 12/12 | 1.211x |
| chess960_perft_d3 | 1.041x | 1.465x | 12/12 | 1.287x |
| corpus_count | 0.829x | 1.152x | 9/12 | 0.786x |
| corpus_materialize | 0.967x | 1.120x | 12/12 | 1.025x |
| corpus_uci_sorted | 0.992x | 1.541x | 12/12 | 1.224x |
| count | 1.020x | 1.361x | 35/36 | 0.755x |
| fen_parse | 0.985x | 1.207x | 35/36 | 0.989x |
| fen_write_native_ep | 1.003x | 1.688x | 36/36 | 1.383x |
| make_unmake_full | 0.956x | 1.221x | 35/36 | 0.891x |
| materialize | 1.020x | 1.217x | 31/36 | 0.884x |
| perft_bulk_d4 | 0.941x | 1.211x | 33/36 | 0.843x |
| perft_full_d3 | 1.044x | 1.268x | 35/36 | 0.955x |
| polyglot_key | 0.967x | 30.871x | 12/12 | 16.807x |
| polyglot_recompute | 1.005x | 1.025x | 8/12 | 0.885x |
| repetition_key | 1.022x | 19.281x | 12/12 | 14.256x |
| replay_checked_native | 1.042x | 1.065x | 11/12 | 0.919x |
| replay_trusted_native | 1.012x | 0.695x | 0/12 | 0.464x |
| san_parse_all | 0.967x | 1.920x | 36/36 | 1.341x |
| san_render_native | 0.975x | 2.540x | 36/36 | 1.778x |
| snapshot | 1.081x | 1.026x | 15/36 | 0.588x |
| startpos | 1.092x | 1.242x | 6/6 | 1.116x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.464x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.532x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.563x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.567x |
| macos-15-native | snapshot | endgame | ferrum | 0.588x |
| macos-15-native | snapshot | promotion | ferrum | 0.611x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.637x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.640x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.658x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.663x |
| macos-15-native | count | middlegame | cozy | 0.755x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.763x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.786x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.824x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.843x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.850x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.854x |
| ubuntu-24.04-portable | materialize | promotion | ultra | 0.884x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.885x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.891x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.894x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.897x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.899x |
| ubuntu-24.04-pext | snapshot | promotion | ferrum | 0.901x |
| macos-15-native | materialize | endgame | ultra | 0.901x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.901x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.902x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.902x |
| macos-15-native | materialize | kiwipete | ultra | 0.905x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.916x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.919x |
| macos-15-native | apply_children_full | promotion | chess | 0.924x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.943x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.952x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.955x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.957x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.958x |
| windows-2025-native | materialize | kiwipete | ultra | 0.972x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.972x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.973x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.976x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.977x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.979x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.979x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.980x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.980x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.983x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.988x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.989x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.990x |
| ubuntu-24.04-portable | materialize | kiwipete | ultra | 0.994x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 10416 | shakmaty | 11083 | 1.064x |
| ubuntu-24.04-arm-native | 7257 | shakmaty | 9289 | 1.280x |
| ubuntu-24.04-native | 5045 | shakmaty | 7201 | 1.427x |
| ubuntu-24.04-pext | 7704 | shakmaty | 10219 | 1.326x |
| ubuntu-24.04-portable | 7835 | shakmaty | 9237 | 1.179x |
| windows-2025-native | 13600 | shakmaty | 16900 | 1.243x |

Cold wins: 6/6 (separate from native cases).
