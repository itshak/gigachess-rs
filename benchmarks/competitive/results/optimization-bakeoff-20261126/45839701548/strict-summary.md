| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 72 | 16 | 87 |
| ubuntu-24.04-arm-native | 72 | 69 | 12 | 87 |
| ubuntu-24.04-native | 72 | 72 | 15 | 87 |
| ubuntu-24.04-pext | 82 | 80 | 13 | 87 |
| ubuntu-24.04-portable | 76 | 75 | 10 | 87 |
| windows-2025-native | 77 | 74 | 14 | 87 |
| ALL | 451 | 442 | 80 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.970x | 1.233x | 35/36 | 0.942x |
| chess960_materialize | 1.045x | 1.471x | 12/12 | 1.051x |
| chess960_perft_d3 | 1.040x | 1.537x | 12/12 | 1.375x |
| corpus_count | 1.045x | 1.188x | 8/12 | 0.859x |
| corpus_materialize | 1.055x | 1.177x | 11/12 | 0.998x |
| corpus_uci_sorted | 1.009x | 1.465x | 12/12 | 1.102x |
| count | 1.024x | 1.472x | 35/36 | 0.983x |
| fen_parse | 0.983x | 1.118x | 32/36 | 0.575x |
| fen_write_native_ep | 0.979x | 1.696x | 36/36 | 1.044x |
| make_unmake_full | 0.941x | 1.263x | 36/36 | 1.039x |
| materialize | 0.998x | 1.286x | 36/36 | 1.048x |
| perft_bulk_d4 | 0.992x | 1.282x | 35/36 | 0.911x |
| perft_full_d3 | 1.023x | 1.086x | 23/36 | 0.765x |
| polyglot_key | 1.029x | 30.314x | 12/12 | 15.848x |
| polyglot_recompute | 0.684x | 0.690x | 0/12 | 0.531x |
| repetition_key | 1.018x | 20.167x | 12/12 | 13.761x |
| replay_checked_native | 1.004x | 1.068x | 9/12 | 0.963x |
| replay_trusted_native | 1.008x | 0.757x | 4/12 | 0.480x |
| san_parse_all | 0.993x | 1.922x | 36/36 | 1.389x |
| san_render_native | 0.993x | 2.552x | 36/36 | 1.611x |
| snapshot | 0.923x | 0.940x | 13/36 | 0.649x |
| startpos | 1.050x | 1.223x | 6/6 | 1.124x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.480x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.481x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.531x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.553x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.568x |
| macos-15-native | fen_parse | start | ultra | 0.575x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.582x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.587x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.631x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.648x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.648x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.649x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.655x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.658x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.659x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.662x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.662x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.663x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.668x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.675x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.765x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.774x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.775x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.795x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.800x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.809x |
| ubuntu-24.04-arm-native | polyglot_recompute | lichess300 | shakmaty | 0.811x |
| ubuntu-24.04-arm-native | polyglot_recompute | seeded | shakmaty | 0.815x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.818x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.831x |
| windows-2025-native | snapshot | endgame | cozy | 0.834x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.850x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.851x |
| ubuntu-24.04-arm-native | perft_full_d3 | endgame | ultra | 0.852x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.859x |
| macos-15-native | snapshot | kiwipete | chess | 0.874x |
| macos-15-native | snapshot | endgame | ferrum | 0.895x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.903x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.907x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.911x |
| macos-15-native | fen_parse | tactical | shakmaty | 0.938x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.940x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.942x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.948x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| macos-15-native | snapshot | promotion | ferrum | 0.959x |
| macos-15-native | perft_full_d3 | kiwipete | ultra | 0.962x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.962x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.963x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.967x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.976x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.979x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.980x |
| macos-15-native | count | middlegame | cozy | 0.983x |
| windows-2025-native | snapshot | kiwipete | chess | 0.983x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.983x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.984x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | shakmaty | 0.986x |
| macos-15-native | fen_parse | kiwipete | ultra | 0.987x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.990x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.992x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.993x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.994x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.997x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.998x |
| windows-2025-native | snapshot | promotion | cozy | 0.999x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4333 | cozy | 13083 | 3.019x |
| ubuntu-24.04-arm-native | 6281 | chess | 8793 | 1.400x |
| ubuntu-24.04-native | 7291 | shakmaty | 9464 | 1.298x |
| ubuntu-24.04-pext | 6654 | shakmaty | 8753 | 1.315x |
| ubuntu-24.04-portable | 6700 | shakmaty | 9795 | 1.462x |
| windows-2025-native | 19500 | shakmaty | 24300 | 1.246x |

Cold wins: 6/6 (separate from native cases).
