| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 73 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 77 | 73 | 10 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 10 | 87 |
| ubuntu-24.04-portable | 77 | 73 | 11 | 87 |
| windows-2025-native | 82 | 82 | 14 | 87 |
| ALL | 474 | 461 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.996x | 1.264x | 34/36 | 0.922x |
| chess960_materialize | 0.992x | 1.445x | 12/12 | 1.282x |
| chess960_perft_d3 | 1.001x | 1.508x | 12/12 | 1.358x |
| corpus_count | 0.902x | 1.079x | 6/12 | 0.779x |
| corpus_materialize | 0.992x | 1.131x | 12/12 | 1.002x |
| corpus_uci_sorted | 1.001x | 1.525x | 12/12 | 1.267x |
| count | 0.981x | 1.429x | 35/36 | 0.990x |
| fen_parse | 0.998x | 1.213x | 35/36 | 0.966x |
| fen_write_native_ep | 1.000x | 1.718x | 36/36 | 1.164x |
| make_unmake_full | 1.009x | 1.314x | 36/36 | 1.060x |
| materialize | 0.986x | 1.315x | 34/36 | 0.983x |
| perft_bulk_d4 | 0.971x | 1.236x | 31/36 | 0.582x |
| perft_full_d3 | 1.212x | 1.587x | 36/36 | 1.224x |
| polyglot_key | 1.010x | 33.063x | 12/12 | 16.048x |
| polyglot_recompute | 0.997x | 0.990x | 6/12 | 0.855x |
| repetition_key | 1.075x | 21.905x | 12/12 | 14.072x |
| replay_checked_native | 0.982x | 1.161x | 12/12 | 1.057x |
| replay_trusted_native | 0.971x | 0.773x | 4/12 | 0.472x |
| san_parse_all | 0.990x | 1.866x | 36/36 | 1.335x |
| san_render_native | 0.989x | 2.497x | 36/36 | 1.765x |
| snapshot | 1.006x | 1.094x | 21/36 | 0.781x |
| startpos | 0.985x | 1.174x | 4/6 | 0.993x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.472x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.535x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.582x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.582x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.649x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.676x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.681x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.779x |
| macos-15-native | snapshot | kiwipete | chess | 0.781x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.806x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.821x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.825x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.855x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.886x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.892x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.892x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.896x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.905x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.914x |
| macos-15-native | apply_children_full | tactical | chess | 0.922x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.923x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.923x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.924x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.927x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.934x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.939x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.963x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.964x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.965x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.966x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.966x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.968x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.970x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.971x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.978x |
| windows-2025-native | materialize | kiwipete | ultra | 0.983x |
| macos-15-native | snapshot | promotion | ferrum | 0.983x |
| macos-15-native | materialize | tactical | ultra | 0.986x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.990x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.993x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.995x |
| ubuntu-24.04-native | snapshot | start | chess | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| windows-2025-native | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 1.000x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 1.000x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5417 | ferrum | 6708 | 1.238x |
| ubuntu-24.04-arm-native | 5664 | shakmaty | 7080 | 1.250x |
| ubuntu-24.04-native | 5529 | shakmaty | 9322 | 1.686x |
| ubuntu-24.04-pext | 5849 | shakmaty | 8814 | 1.507x |
| ubuntu-24.04-portable | 5405 | shakmaty | 7117 | 1.317x |
| windows-2025-native | 18200 | shakmaty | 21900 | 1.203x |

Cold wins: 6/6 (separate from native cases).
