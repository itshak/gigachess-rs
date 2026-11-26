| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 71 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 80 | 79 | 16 | 87 |
| ubuntu-24.04-pext | 85 | 82 | 13 | 87 |
| ubuntu-24.04-portable | 85 | 80 | 12 | 87 |
| windows-2025-native | 81 | 73 | 13 | 87 |
| ALL | 484 | 464 | 78 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.981x | 1.287x | 34/36 | 0.962x |
| chess960_materialize | 1.009x | 1.447x | 12/12 | 1.082x |
| chess960_perft_d3 | 1.041x | 1.520x | 12/12 | 1.340x |
| corpus_count | 1.037x | 1.253x | 10/12 | 0.880x |
| corpus_materialize | 1.087x | 1.189x | 12/12 | 1.059x |
| corpus_uci_sorted | 1.035x | 1.568x | 12/12 | 1.396x |
| count | 0.970x | 1.494x | 36/36 | 1.076x |
| fen_parse | 0.940x | 1.112x | 30/36 | 0.868x |
| fen_write_native_ep | 0.989x | 1.723x | 36/36 | 1.303x |
| make_unmake_full | 0.965x | 1.304x | 36/36 | 1.011x |
| materialize | 0.987x | 1.276x | 34/36 | 0.912x |
| perft_bulk_d4 | 0.992x | 1.299x | 33/36 | 0.916x |
| perft_full_d3 | 1.161x | 1.552x | 36/36 | 1.345x |
| polyglot_key | 1.029x | 28.619x | 12/12 | 16.441x |
| polyglot_recompute | 1.108x | 1.063x | 10/12 | 0.928x |
| repetition_key | 1.026x | 19.931x | 12/12 | 14.628x |
| replay_checked_native | 1.009x | 1.113x | 12/12 | 1.009x |
| replay_trusted_native | 1.012x | 0.783x | 3/12 | 0.475x |
| san_parse_all | 0.980x | 1.951x | 36/36 | 1.218x |
| san_render_native | 1.008x | 2.581x | 36/36 | 1.596x |
| snapshot | 1.045x | 1.056x | 24/36 | 0.813x |
| startpos | 1.117x | 1.290x | 6/6 | 1.113x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.475x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.646x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.658x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.684x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.699x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.751x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.813x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.868x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.880x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.882x |
| macos-15-native | fen_parse | promotion | shakmaty | 0.899x |
| macos-15-native | materialize | middlegame | ultra | 0.912x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.916x |
| ubuntu-24.04-portable | snapshot | start | chess | 0.925x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.928x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.930x |
| macos-15-native | snapshot | middlegame | ferrum | 0.933x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.955x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.955x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.958x |
| macos-15-native | apply_children_full | tactical | cozy | 0.962x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.963x |
| macos-15-native | snapshot | endgame | chess | 0.964x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.964x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.964x |
| ubuntu-24.04-native | fen_parse | middlegame | ultra | 0.966x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.966x |
| macos-15-native | materialize | promotion | ultra | 0.973x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.977x |
| windows-2025-native | snapshot | tactical | cozy | 0.983x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.985x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.987x |
| macos-15-native | apply_children_full | promotion | chess | 0.990x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 8709 | shakmaty | 9333 | 1.072x |
| ubuntu-24.04-arm-native | 5688 | shakmaty | 7361 | 1.294x |
| ubuntu-24.04-native | 6650 | shakmaty | 10135 | 1.524x |
| ubuntu-24.04-pext | 5691 | cozy | 7278 | 1.279x |
| ubuntu-24.04-portable | 8536 | shakmaty | 12163 | 1.425x |
| windows-2025-native | 17600 | shakmaty | 25500 | 1.449x |

Cold wins: 6/6 (separate from native cases).
