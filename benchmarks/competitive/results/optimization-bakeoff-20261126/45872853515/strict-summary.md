| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 75 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 80 | 78 | 14 | 87 |
| ubuntu-24.04-pext | 83 | 82 | 16 | 87 |
| ubuntu-24.04-portable | 79 | 78 | 12 | 87 |
| windows-2025-native | 83 | 81 | 12 | 87 |
| ALL | 480 | 473 | 79 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.006x | 1.246x | 35/36 | 0.785x |
| chess960_materialize | 0.923x | 1.474x | 12/12 | 1.268x |
| chess960_perft_d3 | 0.946x | 1.488x | 12/12 | 1.160x |
| corpus_count | 0.995x | 1.226x | 9/12 | 0.973x |
| corpus_materialize | 0.952x | 1.129x | 11/12 | 0.943x |
| corpus_uci_sorted | 0.954x | 1.529x | 12/12 | 1.265x |
| count | 0.989x | 1.472x | 36/36 | 1.133x |
| fen_parse | 0.989x | 1.171x | 33/36 | 0.929x |
| fen_write_native_ep | 1.011x | 1.803x | 36/36 | 1.317x |
| make_unmake_full | 1.001x | 1.309x | 35/36 | 0.872x |
| materialize | 0.997x | 1.341x | 35/36 | 0.963x |
| perft_bulk_d4 | 1.012x | 1.272x | 34/36 | 0.894x |
| perft_full_d3 | 1.220x | 1.584x | 36/36 | 1.305x |
| polyglot_key | 0.941x | 29.322x | 12/12 | 15.955x |
| polyglot_recompute | 0.973x | 1.038x | 8/12 | 0.875x |
| repetition_key | 0.981x | 19.425x | 12/12 | 14.616x |
| replay_checked_native | 0.945x | 1.105x | 10/12 | 0.991x |
| replay_trusted_native | 0.957x | 0.813x | 4/12 | 0.565x |
| san_parse_all | 1.015x | 1.897x | 36/36 | 1.528x |
| san_render_native | 1.064x | 2.574x | 36/36 | 1.904x |
| snapshot | 1.028x | 1.074x | 20/36 | 0.742x |
| startpos | 1.052x | 1.189x | 6/6 | 1.001x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.565x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.584x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.663x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.696x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.705x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.733x |
| macos-15-native | snapshot | middlegame | ferrum | 0.742x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.751x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.785x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.872x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.875x |
| macos-15-native | snapshot | endgame | ferrum | 0.878x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.892x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.894x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.922x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.929x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| macos-15-native | materialize | tactical | ultra | 0.963x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.967x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.969x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.970x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.971x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.971x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.972x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.973x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.977x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.977x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.979x |
| ubuntu-24.04-native | fen_parse | endgame | ultra | 0.979x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.987x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.988x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.991x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.991x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.991x |
| ubuntu-24.04-native | fen_parse | promotion | ferrum | 0.992x |
| macos-15-native | snapshot | promotion | ferrum | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6541 | shakmaty | 11042 | 1.688x |
| ubuntu-24.04-arm-native | 6265 | shakmaty | 9633 | 1.538x |
| ubuntu-24.04-native | 6022 | shakmaty | 7912 | 1.314x |
| ubuntu-24.04-pext | 5567 | shakmaty | 7288 | 1.309x |
| ubuntu-24.04-portable | 7294 | shakmaty | 10249 | 1.405x |
| windows-2025-native | 13500 | ferrum | 23700 | 1.756x |

Cold wins: 6/6 (separate from native cases).
