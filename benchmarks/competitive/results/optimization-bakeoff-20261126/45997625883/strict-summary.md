| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 71 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 13 | 87 |
| ubuntu-24.04-native | 83 | 82 | 12 | 87 |
| ubuntu-24.04-pext | 84 | 83 | 13 | 87 |
| ubuntu-24.04-portable | 81 | 75 | 12 | 87 |
| windows-2025-native | 80 | 79 | 12 | 87 |
| ALL | 481 | 469 | 76 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.009x | 1.257x | 35/36 | 0.941x |
| chess960_materialize | 1.023x | 1.487x | 12/12 | 1.261x |
| chess960_perft_d3 | 1.011x | 1.529x | 12/12 | 1.280x |
| corpus_count | 1.013x | 1.245x | 11/12 | 0.981x |
| corpus_materialize | 1.032x | 1.165x | 12/12 | 1.010x |
| corpus_uci_sorted | 1.021x | 1.516x | 12/12 | 1.405x |
| count | 1.002x | 1.493x | 35/36 | 0.921x |
| fen_parse | 0.989x | 1.183x | 35/36 | 0.934x |
| fen_write_native_ep | 1.006x | 1.804x | 36/36 | 1.177x |
| make_unmake_full | 1.006x | 1.348x | 36/36 | 1.062x |
| materialize | 0.986x | 1.366x | 34/36 | 0.446x |
| perft_bulk_d4 | 1.003x | 1.262x | 32/36 | 0.919x |
| perft_full_d3 | 1.008x | 1.332x | 36/36 | 1.000x |
| polyglot_key | 1.028x | 33.002x | 12/12 | 16.170x |
| polyglot_recompute | 0.998x | 1.037x | 8/12 | 0.967x |
| repetition_key | 0.990x | 19.785x | 12/12 | 14.031x |
| replay_checked_native | 1.007x | 1.117x | 11/12 | 0.832x |
| replay_trusted_native | 1.007x | 0.782x | 4/12 | 0.551x |
| san_parse_all | 0.997x | 1.900x | 36/36 | 1.496x |
| san_render_native | 1.007x | 2.659x | 36/36 | 1.912x |
| snapshot | 1.013x | 1.059x | 18/36 | 0.909x |
| startpos | 0.938x | 1.227x | 6/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | materialize | endgame | ultra | 0.446x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.551x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.578x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.658x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.679x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.681x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.724x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.757x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.832x |
| macos-15-native | snapshot | tactical | ferrum | 0.909x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.919x |
| macos-15-native | count | middlegame | cozy | 0.921x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.934x |
| macos-15-native | fen_parse | start | shakmaty | 0.934x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.936x |
| macos-15-native | materialize | tactical | ultra | 0.940x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.941x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.941x |
| macos-15-native | snapshot | endgame | ferrum | 0.947x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.951x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.956x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.967x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.974x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.977x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.981x |
| macos-15-native | snapshot | kiwipete | cozy | 0.982x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.983x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.986x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.987x |
| windows-2025-native | snapshot | endgame | chess | 0.988x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.988x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.989x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.993x |
| windows-2025-native | snapshot | promotion | chess | 0.995x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.995x |
| windows-2025-native | snapshot | start | cozy | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7708 | shakmaty | 11834 | 1.535x |
| ubuntu-24.04-arm-native | 5720 | shakmaty | 7631 | 1.334x |
| ubuntu-24.04-native | 4869 | shakmaty | 7643 | 1.570x |
| ubuntu-24.04-pext | 7382 | shakmaty | 10085 | 1.366x |
| ubuntu-24.04-portable | 6793 | shakmaty | 7218 | 1.063x |
| windows-2025-native | 14700 | shakmaty | 19900 | 1.354x |

Cold wins: 6/6 (separate from native cases).
