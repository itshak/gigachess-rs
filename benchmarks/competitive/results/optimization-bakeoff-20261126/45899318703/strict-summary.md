| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 74 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 86 | 85 | 12 | 87 |
| ubuntu-24.04-pext | 80 | 76 | 8 | 87 |
| ubuntu-24.04-portable | 79 | 78 | 13 | 87 |
| windows-2025-native | 84 | 82 | 12 | 87 |
| ALL | 483 | 474 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.033x | 1.273x | 34/36 | 0.952x |
| chess960_materialize | 1.038x | 1.468x | 12/12 | 1.255x |
| chess960_perft_d3 | 1.054x | 1.526x | 12/12 | 1.365x |
| corpus_count | 1.030x | 1.275x | 11/12 | 0.998x |
| corpus_materialize | 1.033x | 1.179x | 12/12 | 1.065x |
| corpus_uci_sorted | 1.035x | 1.535x | 12/12 | 1.245x |
| count | 1.025x | 1.473x | 36/36 | 1.042x |
| fen_parse | 1.079x | 1.240x | 34/36 | 0.991x |
| fen_write_native_ep | 1.042x | 1.731x | 36/36 | 1.477x |
| make_unmake_full | 1.035x | 1.332x | 36/36 | 1.063x |
| materialize | 1.032x | 1.291x | 33/36 | 0.891x |
| perft_bulk_d4 | 1.037x | 1.305x | 35/36 | 0.926x |
| perft_full_d3 | 1.063x | 1.297x | 36/36 | 1.102x |
| polyglot_key | 1.055x | 27.547x | 12/12 | 16.213x |
| polyglot_recompute | 1.049x | 1.011x | 8/12 | 0.882x |
| repetition_key | 1.020x | 19.955x | 12/12 | 15.059x |
| replay_checked_native | 1.005x | 1.151x | 12/12 | 1.063x |
| replay_trusted_native | 1.017x | 0.849x | 5/12 | 0.567x |
| san_parse_all | 1.025x | 1.929x | 36/36 | 1.527x |
| san_render_native | 1.041x | 2.523x | 36/36 | 1.574x |
| snapshot | 1.024x | 1.043x | 18/36 | 0.803x |
| startpos | 1.062x | 1.272x | 5/6 | 0.993x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.567x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.584x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.675x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.703x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.709x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.787x |
| macos-15-native | snapshot | start | ferrum | 0.803x |
| macos-15-native | snapshot | kiwipete | chess | 0.829x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.882x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.890x |
| macos-15-native | materialize | kiwipete | ultra | 0.891x |
| macos-15-native | snapshot | tactical | chess | 0.910x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.916x |
| macos-15-native | materialize | tactical | ultra | 0.923x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.926x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.932x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.940x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.945x |
| macos-15-native | apply_children_full | tactical | chess | 0.952x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.954x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.956x |
| macos-15-native | materialize | middlegame | ultra | 0.969x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.972x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.972x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.980x |
| macos-15-native | snapshot | promotion | ferrum | 0.983x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.984x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.989x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 0.991x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.993x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.996x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.998x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.998x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4583 | shakmaty | 5833 | 1.273x |
| ubuntu-24.04-arm-native | 5176 | shakmaty | 7463 | 1.442x |
| ubuntu-24.04-native | 7464 | shakmaty | 10089 | 1.352x |
| ubuntu-24.04-pext | 5950 | shakmaty | 7605 | 1.278x |
| ubuntu-24.04-portable | 5859 | shakmaty | 7832 | 1.337x |
| windows-2025-native | 15900 | shakmaty | 18500 | 1.164x |

Cold wins: 6/6 (separate from native cases).
