| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 65 | 63 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 74 | 67 | 14 | 87 |
| ubuntu-24.04-pext | 85 | 80 | 12 | 87 |
| ubuntu-24.04-portable | 81 | 75 | 10 | 87 |
| windows-2025-native | 78 | 75 | 11 | 87 |
| ALL | 461 | 437 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.008x | 1.130x | 29/36 | 0.828x |
| chess960_materialize | 1.043x | 1.354x | 12/12 | 1.220x |
| chess960_perft_d3 | 0.984x | 1.391x | 12/12 | 1.179x |
| corpus_count | 1.024x | 1.095x | 8/12 | 0.738x |
| corpus_materialize | 1.001x | 1.081x | 11/12 | 0.872x |
| corpus_uci_sorted | 0.992x | 1.484x | 12/12 | 1.107x |
| count | 1.008x | 1.301x | 34/36 | 0.846x |
| fen_parse | 1.001x | 1.190x | 35/36 | 0.994x |
| fen_write_native_ep | 1.020x | 1.754x | 36/36 | 1.431x |
| make_unmake_full | 1.023x | 1.237x | 34/36 | 0.942x |
| materialize | 1.025x | 1.245x | 31/36 | 0.861x |
| perft_bulk_d4 | 1.019x | 1.144x | 30/36 | 0.799x |
| perft_full_d3 | 1.017x | 1.266x | 32/36 | 0.861x |
| polyglot_key | 1.008x | 31.678x | 12/12 | 16.345x |
| polyglot_recompute | 1.028x | 1.037x | 10/12 | 0.912x |
| repetition_key | 1.006x | 18.511x | 12/12 | 14.245x |
| replay_checked_native | 1.011x | 1.031x | 10/12 | 0.782x |
| replay_trusted_native | 1.016x | 0.726x | 2/12 | 0.518x |
| san_parse_all | 1.046x | 1.895x | 36/36 | 1.533x |
| san_render_native | 1.002x | 2.462x | 36/36 | 1.855x |
| snapshot | 1.013x | 1.043x | 21/36 | 0.813x |
| startpos | 1.006x | 1.251x | 6/6 | 1.196x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.518x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.528x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.549x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.664x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.680x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.690x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.730x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.738x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.782x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.799x |
| macos-15-native | snapshot | tactical | ferrum | 0.813x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.820x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.828x |
| macos-15-native | count | kiwipete | ultra | 0.846x |
| macos-15-native | materialize | kiwipete | ultra | 0.861x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.861x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.871x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.872x |
| windows-2025-native | materialize | kiwipete | ultra | 0.882x |
| macos-15-native | snapshot | endgame | ferrum | 0.891x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.893x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.901x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.907x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.907x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.912x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.915x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.916x |
| macos-15-native | apply_children_full | tactical | chess | 0.918x |
| macos-15-native | materialize | tactical | ultra | 0.927x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.933x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.942x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.942x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.942x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.945x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-native | snapshot | start | chess | 0.948x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.951x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.952x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.952x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.953x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.954x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.956x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.957x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.957x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.958x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.962x |
| windows-2025-native | materialize | middlegame | ultra | 0.962x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.963x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.967x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.968x |
| macos-15-native | apply_children_full | start | chess | 0.982x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.985x |
| macos-15-native | materialize | middlegame | ultra | 0.986x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.991x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.994x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.994x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.996x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5500 | shakmaty | 10583 | 1.924x |
| ubuntu-24.04-arm-native | 6543 | chess | 8680 | 1.327x |
| ubuntu-24.04-native | 7331 | shakmaty | 10519 | 1.435x |
| ubuntu-24.04-pext | 7945 | cozy | 13215 | 1.663x |
| ubuntu-24.04-portable | 5327 | shakmaty | 8398 | 1.576x |
| windows-2025-native | 15900 | shakmaty | 21200 | 1.333x |

Cold wins: 6/6 (separate from native cases).
