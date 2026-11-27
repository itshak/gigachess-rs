| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 79 | 74 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 14 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 13 | 87 |
| ubuntu-24.04-portable | 80 | 80 | 13 | 87 |
| windows-2025-native | 81 | 78 | 13 | 87 |
| ALL | 485 | 475 | 77 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.011x | 1.291x | 36/36 | 1.004x |
| chess960_materialize | 1.043x | 1.514x | 12/12 | 1.277x |
| chess960_perft_d3 | 1.011x | 1.532x | 12/12 | 1.362x |
| corpus_count | 1.031x | 1.237x | 11/12 | 0.994x |
| corpus_materialize | 1.010x | 1.150x | 10/12 | 0.934x |
| corpus_uci_sorted | 1.022x | 1.489x | 12/12 | 1.327x |
| count | 1.019x | 1.491x | 36/36 | 1.091x |
| fen_parse | 0.992x | 1.197x | 35/36 | 0.987x |
| fen_write_native_ep | 1.003x | 1.800x | 36/36 | 1.424x |
| make_unmake_full | 0.995x | 1.305x | 36/36 | 1.030x |
| materialize | 1.017x | 1.333x | 34/36 | 0.710x |
| perft_bulk_d4 | 1.031x | 1.274x | 34/36 | 0.893x |
| perft_full_d3 | 1.025x | 1.328x | 36/36 | 1.120x |
| polyglot_key | 0.994x | 30.179x | 12/12 | 15.956x |
| polyglot_recompute | 1.013x | 1.012x | 7/12 | 0.886x |
| repetition_key | 1.053x | 20.518x | 12/12 | 14.369x |
| replay_checked_native | 0.997x | 1.137x | 12/12 | 1.034x |
| replay_trusted_native | 1.000x | 0.827x | 5/12 | 0.526x |
| san_parse_all | 1.022x | 1.929x | 36/36 | 1.412x |
| san_render_native | 1.027x | 2.607x | 36/36 | 1.552x |
| snapshot | 1.040x | 1.034x | 19/36 | 0.899x |
| startpos | 1.000x | 1.221x | 6/6 | 1.115x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.526x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.580x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.659x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.664x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.692x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.708x |
| macos-15-native | materialize | kiwipete | ultra | 0.710x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.798x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.886x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.893x |
| macos-15-native | snapshot | endgame | ferrum | 0.899x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.924x |
| windows-2025-native | snapshot | endgame | ferrum | 0.928x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.929x |
| windows-2025-native | corpus_materialize | seeded | ultra | 0.934x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.935x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.936x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.938x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.945x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.946x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.950x |
| macos-15-native | materialize | tactical | ultra | 0.957x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.974x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.974x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.974x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.975x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.976x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.977x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.979x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.980x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.987x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| windows-2025-native | snapshot | promotion | cozy | 0.993x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.994x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.995x |
| macos-15-native | snapshot | promotion | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5916 | shakmaty | 8625 | 1.458x |
| ubuntu-24.04-arm-native | 5520 | shakmaty | 8369 | 1.516x |
| ubuntu-24.04-native | 5795 | shakmaty | 7518 | 1.297x |
| ubuntu-24.04-pext | 5817 | shakmaty | 8171 | 1.405x |
| ubuntu-24.04-portable | 5929 | shakmaty | 7861 | 1.326x |
| windows-2025-native | 19700 | shakmaty | 25300 | 1.284x |

Cold wins: 6/6 (separate from native cases).
