| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 66 | 10 | 87 |
| ubuntu-24.04-arm-native | 75 | 73 | 11 | 87 |
| ubuntu-24.04-native | 83 | 79 | 12 | 87 |
| ubuntu-24.04-pext | 82 | 80 | 13 | 87 |
| ubuntu-24.04-portable | 75 | 70 | 9 | 87 |
| windows-2025-native | 80 | 77 | 11 | 87 |
| ALL | 462 | 445 | 66 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.991x | 1.189x | 34/36 | 0.855x |
| chess960_materialize | 0.972x | 1.311x | 12/12 | 1.142x |
| chess960_perft_d3 | 1.000x | 1.331x | 12/12 | 1.254x |
| corpus_count | 1.006x | 1.134x | 8/12 | 0.868x |
| corpus_materialize | 0.999x | 1.123x | 12/12 | 1.054x |
| corpus_uci_sorted | 1.066x | 1.662x | 12/12 | 1.427x |
| count | 0.952x | 1.242x | 33/36 | 0.962x |
| fen_parse | 1.061x | 1.264x | 36/36 | 1.019x |
| fen_write_native_ep | 0.950x | 1.577x | 36/36 | 1.154x |
| make_unmake_full | 0.928x | 1.228x | 35/36 | 0.840x |
| materialize | 1.013x | 1.141x | 32/36 | 0.812x |
| perft_bulk_d4 | 0.971x | 1.135x | 33/36 | 0.842x |
| perft_full_d3 | 0.941x | 1.102x | 29/36 | 0.910x |
| polyglot_key | 1.000x | 26.507x | 12/12 | 16.210x |
| polyglot_recompute | 0.986x | 0.994x | 6/12 | 0.870x |
| repetition_key | 1.048x | 19.891x | 12/12 | 14.048x |
| replay_checked_native | 1.004x | 1.066x | 8/12 | 0.950x |
| replay_trusted_native | 0.936x | 0.772x | 2/12 | 0.554x |
| san_parse_all | 1.000x | 1.919x | 36/36 | 1.458x |
| san_render_native | 0.948x | 2.399x | 36/36 | 1.514x |
| snapshot | 1.019x | 1.086x | 21/36 | 0.915x |
| startpos | 1.255x | 1.527x | 5/6 | 0.999x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.554x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.566x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.634x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.641x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.660x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.786x |
| macos-15-native | materialize | kiwipete | ultra | 0.812x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.840x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.842x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.842x |
| macos-15-native | apply_children_full | tactical | chess | 0.855x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.864x |
| macos-15-native | materialize | middlegame | ultra | 0.868x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.868x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.870x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.879x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.891x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.891x |
| macos-15-native | materialize | tactical | ultra | 0.900x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.905x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.907x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.910x |
| macos-15-native | apply_children_full | promotion | chess | 0.912x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.914x |
| macos-15-native | snapshot | tactical | ferrum | 0.915x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.922x |
| windows-2025-native | materialize | kiwipete | ultra | 0.940x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.943x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.947x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.950x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.955x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.956x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.956x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.958x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.960x |
| macos-15-native | snapshot | endgame | ferrum | 0.960x |
| macos-15-native | count | middlegame | ferrum | 0.962x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.962x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.963x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.965x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.966x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.966x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.969x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.970x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.973x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.977x |
| macos-15-native | snapshot | promotion | ferrum | 0.979x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.979x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.980x |
| macos-15-native | count | kiwipete | ultra | 0.986x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.989x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.990x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.992x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.994x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.997x |
| windows-2025-native | startpos | start | shakmaty | 0.999x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4209 | shakmaty | 7125 | 1.693x |
| ubuntu-24.04-arm-native | 6928 | shakmaty | 8647 | 1.248x |
| ubuntu-24.04-native | 8837 | shakmaty | 12112 | 1.371x |
| ubuntu-24.04-pext | 7744 | cozy | 11111 | 1.435x |
| ubuntu-24.04-portable | 7942 | shakmaty | 10316 | 1.299x |
| windows-2025-native | 16400 | shakmaty | 25500 | 1.555x |

Cold wins: 6/6 (separate from native cases).
