| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 73 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 82 | 80 | 12 | 87 |
| ubuntu-24.04-pext | 77 | 77 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 77 | 12 | 87 |
| windows-2025-native | 81 | 76 | 12 | 87 |
| ALL | 475 | 462 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.963x | 1.215x | 34/36 | 0.794x |
| chess960_materialize | 0.995x | 1.442x | 12/12 | 1.160x |
| chess960_perft_d3 | 0.992x | 1.536x | 12/12 | 1.371x |
| corpus_count | 0.960x | 1.127x | 7/12 | 0.933x |
| corpus_materialize | 1.004x | 1.133x | 12/12 | 1.041x |
| corpus_uci_sorted | 1.018x | 1.475x | 12/12 | 1.186x |
| count | 0.986x | 1.462x | 36/36 | 1.022x |
| fen_parse | 1.008x | 1.170x | 31/36 | 0.934x |
| fen_write_native_ep | 0.982x | 1.774x | 36/36 | 1.345x |
| make_unmake_full | 0.938x | 1.215x | 35/36 | 0.940x |
| materialize | 1.006x | 1.405x | 36/36 | 1.093x |
| perft_bulk_d4 | 0.992x | 1.231x | 32/36 | 0.800x |
| perft_full_d3 | 1.005x | 1.335x | 35/36 | 0.967x |
| polyglot_key | 0.998x | 35.093x | 12/12 | 16.502x |
| polyglot_recompute | 1.028x | 1.050x | 9/12 | 0.945x |
| repetition_key | 0.937x | 22.176x | 12/12 | 14.467x |
| replay_checked_native | 1.001x | 1.052x | 10/12 | 0.903x |
| replay_trusted_native | 0.973x | 0.754x | 3/12 | 0.478x |
| san_parse_all | 0.996x | 1.823x | 36/36 | 1.242x |
| san_render_native | 1.017x | 2.522x | 36/36 | 1.873x |
| snapshot | 0.986x | 1.019x | 21/36 | 0.643x |
| startpos | 1.018x | 1.211x | 6/6 | 1.115x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.478x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.563x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.626x |
| macos-15-native | snapshot | tactical | ferrum | 0.643x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.644x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.674x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.684x |
| macos-15-native | snapshot | middlegame | ferrum | 0.713x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.732x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.791x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.794x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.800x |
| macos-15-native | snapshot | promotion | ferrum | 0.833x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.858x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.878x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.903x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.933x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.934x |
| ubuntu-24.04-pext | fen_parse | middlegame | ultra | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.939x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.940x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.940x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.945x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.949x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.951x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.952x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.953x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.953x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.955x |
| macos-15-native | apply_children_full | start | chess | 0.955x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.956x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.957x |
| macos-15-native | perft_full_d3 | tactical | ultra | 0.967x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.969x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| windows-2025-native | snapshot | middlegame | cozy | 0.988x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.991x |
| windows-2025-native | snapshot | endgame | chess | 0.991x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.997x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.997x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.999x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6625 | shakmaty | 11959 | 1.805x |
| ubuntu-24.04-arm-native | 6048 | shakmaty | 7456 | 1.233x |
| ubuntu-24.04-native | 6722 | shakmaty | 9467 | 1.408x |
| ubuntu-24.04-pext | 6139 | cozy | 10545 | 1.718x |
| ubuntu-24.04-portable | 5805 | shakmaty | 6995 | 1.205x |
| windows-2025-native | 18800 | shakmaty | 24200 | 1.287x |

Cold wins: 6/6 (separate from native cases).
