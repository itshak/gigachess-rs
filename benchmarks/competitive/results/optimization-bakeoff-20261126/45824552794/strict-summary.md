| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 68 | 13 | 87 |
| ubuntu-24.04-arm-native | 78 | 78 | 11 | 87 |
| ubuntu-24.04-native | 80 | 78 | 14 | 87 |
| ubuntu-24.04-pext | 78 | 72 | 12 | 87 |
| ubuntu-24.04-portable | 79 | 74 | 10 | 87 |
| windows-2025-native | 80 | 77 | 14 | 87 |
| ALL | 466 | 447 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.981x | 1.243x | 34/36 | 0.900x |
| chess960_materialize | 0.976x | 1.355x | 12/12 | 1.189x |
| chess960_perft_d3 | 1.003x | 1.396x | 12/12 | 1.231x |
| corpus_count | 1.022x | 1.129x | 8/12 | 0.949x |
| corpus_materialize | 1.029x | 1.109x | 12/12 | 1.016x |
| corpus_uci_sorted | 1.009x | 1.507x | 12/12 | 1.278x |
| count | 0.995x | 1.351x | 34/36 | 0.956x |
| fen_parse | 0.991x | 1.189x | 34/36 | 0.929x |
| fen_write_native_ep | 0.992x | 1.810x | 36/36 | 1.452x |
| make_unmake_full | 0.987x | 1.295x | 36/36 | 1.019x |
| materialize | 0.987x | 1.272x | 31/36 | 0.793x |
| perft_bulk_d4 | 0.984x | 1.139x | 31/36 | 0.850x |
| perft_full_d3 | 1.010x | 1.244x | 30/36 | 0.849x |
| polyglot_key | 1.025x | 32.460x | 12/12 | 16.504x |
| polyglot_recompute | 1.016x | 1.029x | 8/12 | 0.868x |
| repetition_key | 0.984x | 18.314x | 12/12 | 13.055x |
| replay_checked_native | 1.030x | 1.076x | 10/12 | 0.988x |
| replay_trusted_native | 0.975x | 0.802x | 5/12 | 0.423x |
| san_parse_all | 0.985x | 1.953x | 36/36 | 1.313x |
| san_render_native | 0.996x | 2.491x | 36/36 | 1.644x |
| snapshot | 1.009x | 1.049x | 19/36 | 0.648x |
| startpos | 0.975x | 1.232x | 6/6 | 1.129x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.423x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.564x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.648x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.653x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.664x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.701x |
| macos-15-native | snapshot | start | ferrum | 0.708x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.773x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.781x |
| macos-15-native | materialize | promotion | ultra | 0.793x |
| macos-15-native | materialize | kiwipete | ultra | 0.817x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.849x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.850x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.851x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.867x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.868x |
| macos-15-native | snapshot | middlegame | ferrum | 0.877x |
| macos-15-native | perft_full_d3 | kiwipete | ultra | 0.893x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.893x |
| macos-15-native | apply_children_full | tactical | chess | 0.900x |
| macos-15-native | materialize | tactical | ultra | 0.917x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.922x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.925x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.929x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.929x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.932x |
| macos-15-native | apply_children_full | promotion | chess | 0.932x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.932x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.939x |
| macos-15-native | materialize | middlegame | ultra | 0.941x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.941x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.943x |
| macos-15-native | snapshot | endgame | cozy | 0.949x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.949x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.950x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.951x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.956x |
| macos-15-native | snapshot | tactical | ferrum | 0.961x |
| windows-2025-native | snapshot | tactical | chess | 0.971x |
| ubuntu-24.04-pext | fen_parse | endgame | ultra | 0.975x |
| macos-15-native | count | promotion | cozy | 0.975x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.981x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.982x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.982x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.984x |
| windows-2025-native | snapshot | promotion | cozy | 0.986x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.988x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.990x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.992x |
| windows-2025-native | snapshot | start | chess | 0.993x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.995x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.996x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 10625 | shakmaty | 13125 | 1.235x |
| ubuntu-24.04-arm-native | 6680 | chess | 8944 | 1.339x |
| ubuntu-24.04-native | 5929 | shakmaty | 9224 | 1.556x |
| ubuntu-24.04-pext | 6064 | shakmaty | 7588 | 1.251x |
| ubuntu-24.04-portable | 6070 | shakmaty | 6065 | 0.999x |
| windows-2025-native | 17500 | shakmaty | 30900 | 1.766x |

Cold wins: 5/6 (separate from native cases).
