| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 72 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 85 | 85 | 13 | 87 |
| ubuntu-24.04-pext | 86 | 84 | 12 | 87 |
| ubuntu-24.04-portable | 81 | 78 | 10 | 87 |
| windows-2025-native | 83 | 81 | 13 | 87 |
| ALL | 486 | 479 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.024x | 1.259x | 35/36 | 0.922x |
| chess960_materialize | 0.990x | 1.456x | 12/12 | 1.264x |
| chess960_perft_d3 | 1.014x | 1.522x | 12/12 | 1.432x |
| corpus_count | 1.103x | 1.378x | 11/12 | 0.988x |
| corpus_materialize | 0.984x | 1.146x | 11/12 | 0.956x |
| corpus_uci_sorted | 1.010x | 1.596x | 12/12 | 1.171x |
| count | 1.039x | 1.519x | 35/36 | 0.847x |
| fen_parse | 0.993x | 1.168x | 36/36 | 1.024x |
| fen_write_native_ep | 1.039x | 1.665x | 36/36 | 1.318x |
| make_unmake_full | 0.987x | 1.309x | 36/36 | 1.041x |
| materialize | 0.994x | 1.285x | 34/36 | 0.897x |
| perft_bulk_d4 | 1.054x | 1.353x | 34/36 | 0.889x |
| perft_full_d3 | 1.208x | 1.572x | 36/36 | 1.286x |
| polyglot_key | 0.987x | 30.178x | 12/12 | 15.617x |
| polyglot_recompute | 0.996x | 1.031x | 8/12 | 0.914x |
| repetition_key | 1.019x | 20.800x | 12/12 | 13.687x |
| replay_checked_native | 0.970x | 1.114x | 11/12 | 0.936x |
| replay_trusted_native | 1.015x | 0.760x | 4/12 | 0.514x |
| san_parse_all | 0.998x | 1.873x | 36/36 | 1.393x |
| san_render_native | 1.015x | 2.665x | 36/36 | 1.814x |
| snapshot | 1.019x | 1.086x | 23/36 | 0.578x |
| startpos | 0.970x | 1.163x | 4/6 | 0.932x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.514x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.577x |
| macos-15-native | snapshot | start | ferrum | 0.578x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.580x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.597x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.667x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.670x |
| macos-15-native | snapshot | middlegame | ferrum | 0.676x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.687x |
| macos-15-native | count | endgame | cozy | 0.847x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.878x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.889x |
| macos-15-native | materialize | tactical | ultra | 0.897x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.914x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.922x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.922x |
| macos-15-native | startpos | start | shakmaty | 0.932x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.936x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.956x |
| macos-15-native | corpus_materialize | seeded | ferrum | 0.956x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.958x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.960x |
| macos-15-native | snapshot | endgame | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.960x |
| macos-15-native | snapshot | promotion | cozy | 0.961x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.990x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.992x |
| windows-2025-native | startpos | start | shakmaty | 0.997x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.997x |
| macos-15-native | materialize | kiwipete | ultra | 0.999x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4292 | shakmaty | 7750 | 1.806x |
| ubuntu-24.04-arm-native | 8112 | chess | 10248 | 1.263x |
| ubuntu-24.04-native | 9237 | shakmaty | 11130 | 1.205x |
| ubuntu-24.04-pext | 7424 | shakmaty | 11301 | 1.522x |
| ubuntu-24.04-portable | 5628 | shakmaty | 7392 | 1.313x |
| windows-2025-native | 16600 | shakmaty | 22300 | 1.343x |

Cold wins: 6/6 (separate from native cases).
