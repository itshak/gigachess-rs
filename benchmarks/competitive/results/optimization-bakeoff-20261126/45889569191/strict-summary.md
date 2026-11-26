| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 73 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 84 | 82 | 11 | 87 |
| ubuntu-24.04-pext | 84 | 82 | 14 | 87 |
| ubuntu-24.04-portable | 80 | 78 | 11 | 87 |
| windows-2025-native | 81 | 79 | 12 | 87 |
| ALL | 484 | 472 | 73 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.003x | 1.251x | 34/36 | 0.942x |
| chess960_materialize | 0.973x | 1.489x | 12/12 | 1.341x |
| chess960_perft_d3 | 0.964x | 1.498x | 12/12 | 1.333x |
| corpus_count | 0.962x | 1.125x | 10/12 | 0.903x |
| corpus_materialize | 0.988x | 1.096x | 12/12 | 1.003x |
| corpus_uci_sorted | 0.967x | 1.520x | 12/12 | 1.257x |
| count | 1.024x | 1.526x | 36/36 | 1.031x |
| fen_parse | 1.005x | 1.179x | 36/36 | 1.023x |
| fen_write_native_ep | 0.988x | 1.772x | 36/36 | 1.148x |
| make_unmake_full | 0.999x | 1.288x | 36/36 | 1.055x |
| materialize | 1.008x | 1.453x | 35/36 | 0.800x |
| perft_bulk_d4 | 1.018x | 1.234x | 31/36 | 0.866x |
| perft_full_d3 | 1.201x | 1.627x | 36/36 | 1.292x |
| polyglot_key | 0.988x | 37.654x | 12/12 | 16.483x |
| polyglot_recompute | 1.095x | 1.158x | 8/12 | 0.921x |
| repetition_key | 1.058x | 23.317x | 12/12 | 13.930x |
| replay_checked_native | 0.996x | 1.189x | 12/12 | 1.096x |
| replay_trusted_native | 0.963x | 0.777x | 4/12 | 0.473x |
| san_parse_all | 0.967x | 1.778x | 36/36 | 1.371x |
| san_render_native | 0.996x | 2.583x | 36/36 | 1.893x |
| snapshot | 0.990x | 1.059x | 20/36 | 0.623x |
| startpos | 1.030x | 1.274x | 6/6 | 1.001x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.473x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.585x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.595x |
| macos-15-native | snapshot | tactical | ferrum | 0.623x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.672x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.673x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.673x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.732x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.762x |
| macos-15-native | materialize | kiwipete | ultra | 0.800x |
| macos-15-native | snapshot | middlegame | ferrum | 0.852x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.866x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.900x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.903x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.919x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.921x |
| macos-15-native | snapshot | start | ferrum | 0.923x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.941x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.942x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.946x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.946x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.950x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.951x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.954x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.964x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.983x |
| windows-2025-native | snapshot | tactical | chess | 0.985x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.996x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.997x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |
| macos-15-native | snapshot | endgame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5875 | ferrum | 7916 | 1.347x |
| ubuntu-24.04-arm-native | 5544 | shakmaty | 8351 | 1.506x |
| ubuntu-24.04-native | 6046 | shakmaty | 7710 | 1.275x |
| ubuntu-24.04-pext | 5661 | cozy | 7434 | 1.313x |
| ubuntu-24.04-portable | 5417 | shakmaty | 7465 | 1.378x |
| windows-2025-native | 15500 | shakmaty | 22500 | 1.452x |

Cold wins: 6/6 (separate from native cases).
