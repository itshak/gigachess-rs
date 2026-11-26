| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 72 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 83 | 76 | 12 | 87 |
| ubuntu-24.04-pext | 82 | 81 | 9 | 87 |
| ubuntu-24.04-portable | 82 | 75 | 12 | 87 |
| windows-2025-native | 83 | 83 | 12 | 87 |
| ALL | 482 | 465 | 69 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.005x | 1.264x | 35/36 | 0.988x |
| chess960_materialize | 1.022x | 1.489x | 12/12 | 1.289x |
| chess960_perft_d3 | 0.996x | 1.505x | 12/12 | 1.357x |
| corpus_count | 0.994x | 1.150x | 9/12 | 0.816x |
| corpus_materialize | 0.986x | 1.129x | 12/12 | 1.051x |
| corpus_uci_sorted | 0.980x | 1.477x | 12/12 | 1.257x |
| count | 1.004x | 1.468x | 36/36 | 1.018x |
| fen_parse | 1.030x | 1.170x | 34/36 | 0.989x |
| fen_write_native_ep | 1.013x | 1.731x | 36/36 | 1.455x |
| make_unmake_full | 1.019x | 1.304x | 36/36 | 1.040x |
| materialize | 1.012x | 1.329x | 33/36 | 0.765x |
| perft_bulk_d4 | 1.025x | 1.257x | 32/36 | 0.913x |
| perft_full_d3 | 1.019x | 1.310x | 36/36 | 1.102x |
| polyglot_key | 0.968x | 34.271x | 12/12 | 16.645x |
| polyglot_recompute | 0.992x | 1.018x | 8/12 | 0.873x |
| repetition_key | 0.961x | 20.840x | 12/12 | 14.477x |
| replay_checked_native | 0.977x | 1.079x | 11/12 | 0.993x |
| replay_trusted_native | 0.991x | 0.769x | 4/12 | 0.548x |
| san_parse_all | 1.041x | 1.871x | 36/36 | 1.481x |
| san_render_native | 1.007x | 2.553x | 36/36 | 1.863x |
| snapshot | 1.019x | 1.127x | 23/36 | 0.944x |
| startpos | 1.073x | 1.180x | 5/6 | 0.989x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.548x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.568x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.590x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.661x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.683x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.689x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.691x |
| macos-15-native | materialize | start | ultra | 0.765x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.816x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.873x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.878x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.906x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.913x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.918x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.924x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.932x |
| macos-15-native | materialize | kiwipete | ultra | 0.936x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.942x |
| macos-15-native | snapshot | middlegame | ferrum | 0.944x |
| macos-15-native | snapshot | endgame | ferrum | 0.948x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.950x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.954x |
| macos-15-native | materialize | tactical | ultra | 0.955x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.956x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.963x |
| macos-15-native | snapshot | tactical | ferrum | 0.964x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.965x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.966x |
| macos-15-native | snapshot | start | ferrum | 0.977x |
| macos-15-native | apply_children_full | tactical | chess | 0.988x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.989x |
| windows-2025-native | startpos | start | shakmaty | 0.989x |
| macos-15-native | snapshot | promotion | ferrum | 0.989x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.992x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.993x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4959 | shakmaty | 9250 | 1.865x |
| ubuntu-24.04-arm-native | 5312 | shakmaty | 6984 | 1.315x |
| ubuntu-24.04-native | 7469 | shakmaty | 9338 | 1.250x |
| ubuntu-24.04-pext | 4908 | shakmaty | 7593 | 1.547x |
| ubuntu-24.04-portable | 6540 | shakmaty | 6608 | 1.010x |
| windows-2025-native | 9900 | shakmaty | 11900 | 1.202x |

Cold wins: 6/6 (separate from native cases).
