| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 70 | 15 | 87 |
| ubuntu-24.04-arm-native | 78 | 75 | 11 | 87 |
| ubuntu-24.04-native | 80 | 76 | 11 | 87 |
| ubuntu-24.04-pext | 83 | 83 | 9 | 87 |
| ubuntu-24.04-portable | 80 | 74 | 12 | 87 |
| windows-2025-native | 75 | 69 | 11 | 87 |
| ALL | 469 | 447 | 69 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.991x | 1.261x | 34/36 | 0.912x |
| chess960_materialize | 0.976x | 1.342x | 12/12 | 1.132x |
| chess960_perft_d3 | 0.989x | 1.364x | 12/12 | 1.141x |
| corpus_count | 0.960x | 1.008x | 5/12 | 0.744x |
| corpus_materialize | 0.982x | 1.073x | 11/12 | 0.990x |
| corpus_uci_sorted | 0.998x | 1.537x | 12/12 | 1.113x |
| count | 0.984x | 1.318x | 34/36 | 0.711x |
| fen_parse | 1.017x | 1.187x | 34/36 | 0.979x |
| fen_write_native_ep | 1.000x | 1.703x | 36/36 | 1.354x |
| make_unmake_full | 0.987x | 1.282x | 33/36 | 0.976x |
| materialize | 0.997x | 1.261x | 35/36 | 0.923x |
| perft_bulk_d4 | 0.992x | 1.114x | 31/36 | 0.766x |
| perft_full_d3 | 0.974x | 1.280x | 34/36 | 0.924x |
| polyglot_key | 0.967x | 31.962x | 12/12 | 16.055x |
| polyglot_recompute | 1.046x | 1.043x | 11/12 | 0.910x |
| repetition_key | 0.998x | 22.349x | 12/12 | 14.397x |
| replay_checked_native | 0.987x | 1.097x | 12/12 | 1.014x |
| replay_trusted_native | 1.017x | 0.765x | 3/12 | 0.515x |
| san_parse_all | 0.975x | 1.794x | 36/36 | 1.382x |
| san_render_native | 1.022x | 2.640x | 36/36 | 1.793x |
| snapshot | 0.906x | 1.002x | 18/36 | 0.676x |
| startpos | 1.056x | 1.285x | 6/6 | 1.002x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.515x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.528x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.566x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.626x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.659x |
| macos-15-native | snapshot | tactical | cozy | 0.676x |
| macos-15-native | count | middlegame | cozy | 0.711x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.713x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.716x |
| windows-2025-native | corpus_count | seeded | ultra | 0.744x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.766x |
| macos-15-native | snapshot | promotion | cozy | 0.793x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.811x |
| macos-15-native | snapshot | start | ferrum | 0.819x |
| macos-15-native | snapshot | endgame | cozy | 0.823x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.833x |
| windows-2025-native | snapshot | promotion | chess | 0.853x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.857x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.885x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.909x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.910x |
| windows-2025-native | apply_children_full | tactical | chess | 0.912x |
| windows-2025-native | perft_bulk_d4 | start | ultra | 0.921x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.922x |
| macos-15-native | materialize | middlegame | ultra | 0.923x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.924x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.925x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.955x |
| macos-15-native | corpus_count | seeded | ultra | 0.956x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.956x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.956x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.959x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.961x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.962x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.962x |
| windows-2025-native | perft_bulk_d4 | kiwipete | ultra | 0.963x |
| windows-2025-native | count | middlegame | cozy | 0.968x |
| windows-2025-native | snapshot | endgame | chess | 0.971x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.974x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.976x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 0.979x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.980x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.981x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.985x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.986x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.990x |
| macos-15-native | apply_children_full | start | ferrum | 0.990x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.996x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.997x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6375 | shakmaty | 10750 | 1.686x |
| ubuntu-24.04-arm-native | 7176 | shakmaty | 10103 | 1.408x |
| ubuntu-24.04-native | 5613 | shakmaty | 6908 | 1.231x |
| ubuntu-24.04-pext | 7291 | shakmaty | 10025 | 1.375x |
| ubuntu-24.04-portable | 7955 | shakmaty | 10129 | 1.273x |
| windows-2025-native | 15900 | shakmaty | 22600 | 1.421x |

Cold wins: 6/6 (separate from native cases).
