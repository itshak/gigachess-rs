| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 79 | 75 | 11 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 9 | 87 |
| ubuntu-24.04-pext | 79 | 75 | 15 | 87 |
| ubuntu-24.04-portable | 77 | 77 | 12 | 87 |
| windows-2025-native | 76 | 75 | 12 | 87 |
| ALL | 473 | 464 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.874x | 1.117x | 30/36 | 0.906x |
| chess960_materialize | 1.016x | 1.476x | 12/12 | 1.291x |
| chess960_perft_d3 | 0.991x | 1.511x | 12/12 | 1.291x |
| corpus_count | 0.993x | 1.146x | 9/12 | 0.785x |
| corpus_materialize | 1.002x | 1.156x | 12/12 | 1.064x |
| corpus_uci_sorted | 0.993x | 1.594x | 12/12 | 1.455x |
| count | 0.972x | 1.435x | 35/36 | 0.851x |
| fen_parse | 1.023x | 1.231x | 36/36 | 1.002x |
| fen_write_native_ep | 0.989x | 1.668x | 36/36 | 1.243x |
| make_unmake_full | 0.904x | 1.183x | 32/36 | 0.922x |
| materialize | 1.018x | 1.336x | 36/36 | 1.020x |
| perft_bulk_d4 | 1.000x | 1.270x | 33/36 | 0.854x |
| perft_full_d3 | 0.990x | 1.312x | 36/36 | 1.068x |
| polyglot_key | 0.990x | 33.007x | 12/12 | 16.149x |
| polyglot_recompute | 0.961x | 1.004x | 6/12 | 0.866x |
| repetition_key | 1.067x | 23.437x | 12/12 | 14.037x |
| replay_checked_native | 0.920x | 1.124x | 12/12 | 1.041x |
| replay_trusted_native | 0.893x | 0.713x | 2/12 | 0.544x |
| san_parse_all | 0.991x | 1.835x | 36/36 | 1.367x |
| san_render_native | 0.984x | 2.636x | 36/36 | 1.478x |
| snapshot | 1.009x | 1.098x | 22/36 | 0.694x |
| startpos | 0.962x | 1.129x | 4/6 | 0.856x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.544x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.549x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.559x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.575x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.578x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.578x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.603x |
| macos-15-native | snapshot | endgame | ferrum | 0.694x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.785x |
| macos-15-native | snapshot | promotion | ferrum | 0.812x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.849x |
| macos-15-native | count | middlegame | cozy | 0.851x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.854x |
| macos-15-native | startpos | start | shakmaty | 0.856x |
| windows-2025-native | perft_bulk_d4 | kiwipete | ultra | 0.857x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.866x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.904x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.906x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.907x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.915x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.922x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.927x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.928x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.949x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.951x |
| ubuntu-24.04-pext | apply_children_full | promotion | chess | 0.957x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.958x |
| windows-2025-native | corpus_count | seeded | ultra | 0.961x |
| windows-2025-native | apply_children_full | promotion | chess | 0.965x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.967x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.974x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.976x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.978x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.980x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.980x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.981x |
| windows-2025-native | apply_children_full | tactical | chess | 0.985x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.986x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.987x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.996x |
| windows-2025-native | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6959 | shakmaty | 12125 | 1.742x |
| ubuntu-24.04-arm-native | 5824 | shakmaty | 7599 | 1.305x |
| ubuntu-24.04-native | 5819 | shakmaty | 8523 | 1.465x |
| ubuntu-24.04-pext | 5679 | cozy | 8278 | 1.458x |
| ubuntu-24.04-portable | 8453 | shakmaty | 9965 | 1.179x |
| windows-2025-native | 24300 | shakmaty | 26600 | 1.095x |

Cold wins: 6/6 (separate from native cases).
