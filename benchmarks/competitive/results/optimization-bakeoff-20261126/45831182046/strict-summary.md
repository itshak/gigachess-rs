| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 66 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 76 | 12 | 87 |
| ubuntu-24.04-native | 81 | 80 | 12 | 87 |
| ubuntu-24.04-pext | 82 | 79 | 14 | 87 |
| ubuntu-24.04-portable | 79 | 74 | 10 | 87 |
| windows-2025-native | 80 | 78 | 12 | 87 |
| ALL | 471 | 453 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.025x | 1.270x | 35/36 | 0.835x |
| chess960_materialize | 1.040x | 1.416x | 11/12 | 0.975x |
| chess960_perft_d3 | 1.036x | 1.431x | 12/12 | 1.173x |
| corpus_count | 0.997x | 1.085x | 6/12 | 0.881x |
| corpus_materialize | 1.012x | 1.144x | 12/12 | 1.023x |
| corpus_uci_sorted | 0.986x | 1.518x | 12/12 | 1.022x |
| count | 1.022x | 1.385x | 35/36 | 0.965x |
| fen_parse | 0.984x | 1.135x | 32/36 | 0.931x |
| fen_write_native_ep | 1.012x | 1.796x | 36/36 | 1.447x |
| make_unmake_full | 1.008x | 1.317x | 36/36 | 1.060x |
| materialize | 1.010x | 1.298x | 31/36 | 0.857x |
| perft_bulk_d4 | 1.028x | 1.203x | 33/36 | 0.886x |
| perft_full_d3 | 1.006x | 1.284x | 33/36 | 0.923x |
| polyglot_key | 0.987x | 30.571x | 12/12 | 16.367x |
| polyglot_recompute | 0.964x | 0.996x | 7/12 | 0.847x |
| repetition_key | 1.013x | 20.752x | 12/12 | 14.709x |
| replay_checked_native | 1.041x | 1.149x | 11/12 | 0.997x |
| replay_trusted_native | 1.012x | 0.824x | 4/12 | 0.582x |
| san_parse_all | 1.000x | 1.898x | 36/36 | 1.378x |
| san_render_native | 1.033x | 2.512x | 36/36 | 1.825x |
| snapshot | 1.010x | 1.065x | 23/36 | 0.717x |
| startpos | 1.035x | 1.194x | 6/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.582x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.646x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.655x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.695x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.696x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.717x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.717x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.800x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.835x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.847x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.855x |
| macos-15-native | materialize | tactical | ultra | 0.857x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.871x |
| macos-15-native | corpus_count | seeded | ultra | 0.881x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.886x |
| macos-15-native | snapshot | promotion | ferrum | 0.904x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.905x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.911x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.919x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.923x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.931x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.938x |
| macos-15-native | snapshot | start | ferrum | 0.939x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.939x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.939x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.941x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.944x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.962x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.964x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.965x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.966x |
| macos-15-native | perft_full_d3 | kiwipete | shakmaty | 0.969x |
| macos-15-native | chess960_materialize | 960_castle | cozy | 0.975x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.976x |
| macos-15-native | materialize | kiwipete | ultra | 0.979x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.983x |
| windows-2025-native | materialize | kiwipete | ultra | 0.983x |
| windows-2025-native | snapshot | start | ferrum | 0.984x |
| macos-15-native | materialize | middlegame | ultra | 0.985x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.991x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.991x |
| windows-2025-native | materialize | middlegame | ultra | 0.994x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.996x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.997x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.999x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4959 | shakmaty | 6125 | 1.235x |
| ubuntu-24.04-arm-native | 5280 | shakmaty | 9176 | 1.738x |
| ubuntu-24.04-native | 7775 | shakmaty | 11201 | 1.441x |
| ubuntu-24.04-pext | 4924 | shakmaty | 8372 | 1.700x |
| ubuntu-24.04-portable | 5204 | shakmaty | 7059 | 1.356x |
| windows-2025-native | 18700 | shakmaty | 22400 | 1.198x |

Cold wins: 6/6 (separate from native cases).
