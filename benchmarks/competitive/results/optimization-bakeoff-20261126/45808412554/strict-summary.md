| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 73 | 14 | 87 |
| ubuntu-24.04-arm-native | 76 | 72 | 11 | 87 |
| ubuntu-24.04-native | 80 | 78 | 11 | 87 |
| ubuntu-24.04-pext | 80 | 78 | 14 | 87 |
| ubuntu-24.04-portable | 76 | 75 | 13 | 87 |
| windows-2025-native | 84 | 81 | 11 | 87 |
| ALL | 472 | 457 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.990x | 1.180x | 31/36 | 0.806x |
| chess960_materialize | 1.068x | 1.435x | 12/12 | 1.187x |
| chess960_perft_d3 | 1.114x | 1.498x | 12/12 | 1.370x |
| corpus_count | 1.058x | 1.407x | 12/12 | 1.094x |
| corpus_materialize | 1.033x | 1.191x | 12/12 | 1.080x |
| corpus_uci_sorted | 0.997x | 1.530x | 12/12 | 1.368x |
| count | 1.123x | 1.482x | 36/36 | 1.080x |
| fen_parse | 1.017x | 1.225x | 36/36 | 1.042x |
| fen_write_native_ep | 0.991x | 1.735x | 36/36 | 1.190x |
| make_unmake_full | 0.996x | 1.237x | 35/36 | 0.809x |
| materialize | 1.047x | 1.290x | 36/36 | 1.023x |
| perft_bulk_d4 | 1.076x | 1.356x | 36/36 | 1.140x |
| perft_full_d3 | 1.025x | 1.254x | 30/36 | 0.873x |
| polyglot_key | 1.023x | 29.585x | 12/12 | 16.167x |
| polyglot_recompute | 1.002x | 1.015x | 9/12 | 0.883x |
| repetition_key | 0.994x | 18.556x | 12/12 | 9.395x |
| replay_checked_native | 1.000x | 1.009x | 8/12 | 0.840x |
| replay_trusted_native | 0.997x | 0.730x | 3/12 | 0.476x |
| san_parse_all | 0.990x | 1.919x | 36/36 | 1.466x |
| san_render_native | 0.994x | 2.590x | 36/36 | 1.611x |
| snapshot | 0.894x | 1.001x | 15/36 | 0.875x |
| startpos | 1.097x | 1.189x | 5/6 | 0.990x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.476x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.496x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.561x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.617x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.648x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.740x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.741x |
| macos-15-native | apply_children_full | start | chess | 0.806x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.809x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.840x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.866x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.873x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.875x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.879x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.882x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.883x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.895x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.897x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.898x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.911x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.918x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.922x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.928x |
| windows-2025-native | snapshot | promotion | ferrum | 0.929x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.932x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.940x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.954x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.956x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.957x |
| macos-15-native | apply_children_full | tactical | chess | 0.958x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.960x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.960x |
| macos-15-native | snapshot | promotion | ferrum | 0.962x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.963x |
| macos-15-native | snapshot | middlegame | ferrum | 0.975x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.979x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.979x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.984x |
| windows-2025-native | startpos | start | shakmaty | 0.990x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.992x |
| ubuntu-24.04-pext | snapshot | promotion | ferrum | 0.993x |
| ubuntu-24.04-pext | snapshot | middlegame | ferrum | 0.994x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.997x |
| ubuntu-24.04-pext | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | endgame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 10333 | shakmaty | 9042 | 0.875x |
| ubuntu-24.04-arm-native | 10704 | shakmaty | 8903 | 0.832x |
| ubuntu-24.04-native | 11277 | shakmaty | 6991 | 0.620x |
| ubuntu-24.04-pext | 8332 | cozy | 7704 | 0.925x |
| ubuntu-24.04-portable | 12118 | shakmaty | 7602 | 0.627x |
| windows-2025-native | 28500 | shakmaty | 24800 | 0.870x |

Cold wins: 0/6 (separate from native cases).
