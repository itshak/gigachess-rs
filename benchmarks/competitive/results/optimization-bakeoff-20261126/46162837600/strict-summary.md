| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 75 | 11 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 82 | 77 | 13 | 87 |
| ubuntu-24.04-pext | 85 | 84 | 15 | 87 |
| ubuntu-24.04-portable | 84 | 79 | 13 | 87 |
| windows-2025-native | 83 | 79 | 15 | 87 |
| ALL | 489 | 473 | 79 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.968x | 1.245x | 32/36 | 0.801x |
| chess960_materialize | 0.919x | 1.386x | 12/12 | 1.079x |
| chess960_perft_d3 | 1.001x | 1.521x | 12/12 | 1.302x |
| corpus_count | 1.156x | 1.356x | 12/12 | 1.084x |
| corpus_materialize | 1.020x | 1.145x | 12/12 | 1.053x |
| corpus_uci_sorted | 1.023x | 1.532x | 12/12 | 1.376x |
| count | 1.061x | 1.554x | 36/36 | 1.072x |
| fen_parse | 0.959x | 1.163x | 36/36 | 1.011x |
| fen_write_native_ep | 0.999x | 1.830x | 36/36 | 1.433x |
| make_unmake_full | 1.005x | 1.304x | 36/36 | 1.066x |
| materialize | 1.000x | 1.421x | 34/36 | 0.851x |
| perft_bulk_d4 | 1.049x | 1.298x | 34/36 | 0.947x |
| perft_full_d3 | 1.191x | 1.568x | 36/36 | 1.156x |
| polyglot_key | 1.000x | 33.604x | 12/12 | 16.014x |
| polyglot_recompute | 1.533x | 1.523x | 12/12 | 1.243x |
| repetition_key | 1.059x | 20.277x | 12/12 | 14.669x |
| replay_checked_native | 1.006x | 1.160x | 12/12 | 1.047x |
| replay_trusted_native | 1.063x | 0.849x | 4/12 | 0.597x |
| san_parse_all | 1.028x | 1.879x | 36/36 | 1.387x |
| san_render_native | 1.007x | 2.590x | 36/36 | 1.847x |
| snapshot | 1.014x | 1.021x | 19/36 | 0.890x |
| startpos | 1.004x | 1.202x | 6/6 | 1.003x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.597x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.651x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.725x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.734x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.755x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.796x |
| macos-15-native | apply_children_full | promotion | chess | 0.801x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.848x |
| macos-15-native | materialize | middlegame | ultra | 0.851x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.859x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.888x |
| macos-15-native | snapshot | endgame | ferrum | 0.890x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.930x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.946x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.947x |
| macos-15-native | materialize | endgame | ultra | 0.948x |
| macos-15-native | apply_children_full | middlegame | chess | 0.948x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.950x |
| windows-2025-native | snapshot | kiwipete | chess | 0.983x |
| macos-15-native | snapshot | middlegame | ferrum | 0.993x |
| windows-2025-native | snapshot | start | ferrum | 0.994x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.995x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 1.000x |
| ubuntu-24.04-native | snapshot | endgame | chess | 1.000x |
| ubuntu-24.04-native | snapshot | promotion | cozy | 1.000x |
| windows-2025-native | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3667 | shakmaty | 6583 | 1.795x |
| ubuntu-24.04-arm-native | 7144 | shakmaty | 9192 | 1.287x |
| ubuntu-24.04-native | 5804 | shakmaty | 8239 | 1.420x |
| ubuntu-24.04-pext | 7720 | shakmaty | 6728 | 0.872x |
| ubuntu-24.04-portable | 6496 | shakmaty | 7169 | 1.104x |
| windows-2025-native | 16300 | shakmaty | 26900 | 1.650x |

Cold wins: 5/6 (separate from native cases).
