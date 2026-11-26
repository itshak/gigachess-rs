| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 71 | 17 | 87 |
| ubuntu-24.04-arm-native | 73 | 72 | 12 | 87 |
| ubuntu-24.04-native | 86 | 82 | 12 | 87 |
| ubuntu-24.04-pext | 82 | 80 | 17 | 87 |
| ubuntu-24.04-portable | 80 | 73 | 10 | 87 |
| windows-2025-native | 79 | 77 | 12 | 87 |
| ALL | 474 | 455 | 80 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.006x | 1.201x | 34/36 | 0.947x |
| chess960_materialize | 0.995x | 1.519x | 12/12 | 1.280x |
| chess960_perft_d3 | 0.980x | 1.488x | 12/12 | 1.267x |
| corpus_count | 0.968x | 1.233x | 10/12 | 0.862x |
| corpus_materialize | 1.042x | 1.170x | 11/12 | 0.974x |
| corpus_uci_sorted | 1.010x | 1.563x | 12/12 | 1.412x |
| count | 0.980x | 1.440x | 35/36 | 0.760x |
| fen_parse | 1.037x | 1.198x | 35/36 | 0.996x |
| fen_write_native_ep | 1.005x | 1.702x | 36/36 | 1.061x |
| make_unmake_full | 1.015x | 1.321x | 36/36 | 1.055x |
| materialize | 1.033x | 1.311x | 35/36 | 0.902x |
| perft_bulk_d4 | 1.009x | 1.308x | 34/36 | 0.918x |
| perft_full_d3 | 1.013x | 1.066x | 24/36 | 0.759x |
| polyglot_key | 0.956x | 29.479x | 12/12 | 16.267x |
| polyglot_recompute | 0.976x | 1.012x | 7/12 | 0.826x |
| repetition_key | 1.008x | 20.210x | 12/12 | 14.244x |
| replay_checked_native | 0.965x | 1.092x | 11/12 | 0.983x |
| replay_trusted_native | 0.994x | 0.791x | 4/12 | 0.515x |
| san_parse_all | 1.012x | 1.868x | 36/36 | 1.081x |
| san_render_native | 1.008x | 2.487x | 36/36 | 1.835x |
| snapshot | 0.988x | 1.077x | 24/36 | 0.546x |
| startpos | 1.081x | 1.218x | 6/6 | 1.001x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.515x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.521x |
| macos-15-native | snapshot | middlegame | cozy | 0.546x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.569x |
| macos-15-native | snapshot | endgame | ferrum | 0.642x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.662x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.688x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.710x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.759x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.760x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.790x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.791x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.826x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.837x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.842x |
| ubuntu-24.04-arm-native | perft_full_d3 | endgame | ultra | 0.851x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.858x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.859x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.862x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.862x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.871x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.883x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.902x |
| macos-15-native | materialize | middlegame | ultra | 0.902x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.916x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.918x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.927x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.931x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.939x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.947x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.947x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.959x |
| macos-15-native | snapshot | promotion | ferrum | 0.968x |
| macos-15-native | snapshot | tactical | ferrum | 0.972x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.972x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.974x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.982x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.983x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.993x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.995x |
| ubuntu-24.04-pext | fen_parse | start | ferrum | 0.996x |
| ubuntu-24.04-arm-native | perft_full_d3 | start | ultra | 0.998x |
| windows-2025-native | perft_full_d3 | endgame | ultra | 0.999x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7541 | shakmaty | 9375 | 1.243x |
| ubuntu-24.04-arm-native | 7383 | shakmaty | 8271 | 1.120x |
| ubuntu-24.04-native | 7414 | shakmaty | 11252 | 1.518x |
| ubuntu-24.04-pext | 5871 | shakmaty | 8358 | 1.424x |
| ubuntu-24.04-portable | 5658 | shakmaty | 8102 | 1.432x |
| windows-2025-native | 14100 | shakmaty | 20500 | 1.454x |

Cold wins: 6/6 (separate from native cases).
