| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 68 | 65 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 73 | 11 | 87 |
| ubuntu-24.04-native | 81 | 78 | 9 | 87 |
| ubuntu-24.04-pext | 78 | 71 | 9 | 87 |
| ubuntu-24.04-portable | 78 | 74 | 10 | 87 |
| windows-2025-native | 81 | 81 | 11 | 87 |
| ALL | 465 | 442 | 63 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.023x | 1.218x | 34/36 | 0.946x |
| chess960_materialize | 1.053x | 1.391x | 12/12 | 1.269x |
| chess960_perft_d3 | 1.039x | 1.378x | 12/12 | 1.042x |
| corpus_count | 0.757x | 1.019x | 5/12 | 0.824x |
| corpus_materialize | 0.931x | 1.065x | 10/12 | 0.888x |
| corpus_uci_sorted | 0.989x | 1.458x | 12/12 | 1.243x |
| count | 1.002x | 1.311x | 34/36 | 0.891x |
| fen_parse | 1.001x | 1.146x | 35/36 | 0.979x |
| fen_write_native_ep | 0.985x | 1.768x | 36/36 | 1.373x |
| make_unmake_full | 1.014x | 1.216x | 36/36 | 1.023x |
| materialize | 0.998x | 1.311x | 32/36 | 0.784x |
| perft_bulk_d4 | 0.897x | 1.117x | 31/36 | 0.731x |
| perft_full_d3 | 1.014x | 1.354x | 35/36 | 0.917x |
| polyglot_key | 1.032x | 35.858x | 12/12 | 16.796x |
| polyglot_recompute | 1.010x | 1.004x | 9/12 | 0.769x |
| repetition_key | 1.050x | 21.401x | 12/12 | 14.893x |
| replay_checked_native | 0.964x | 1.048x | 8/12 | 0.778x |
| replay_trusted_native | 1.023x | 0.716x | 2/12 | 0.501x |
| san_parse_all | 1.004x | 1.903x | 36/36 | 1.527x |
| san_render_native | 1.013x | 2.491x | 36/36 | 1.886x |
| snapshot | 0.907x | 1.002x | 20/36 | 0.751x |
| startpos | 1.014x | 1.226x | 6/6 | 1.152x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.501x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.562x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.603x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.604x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.605x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.617x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.657x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.714x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.731x |
| macos-15-native | snapshot | promotion | ferrum | 0.751x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.769x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.778x |
| macos-15-native | materialize | kiwipete | ultra | 0.784x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.824x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.845x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.868x |
| macos-15-native | snapshot | endgame | ferrum | 0.886x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.888x |
| macos-15-native | count | middlegame | cozy | 0.891x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.894x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.895x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.909x |
| macos-15-native | materialize | tactical | ultra | 0.915x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.917x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.918x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.918x |
| macos-15-native | materialize | middlegame | ultra | 0.922x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.923x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.932x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.934x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.937x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.941x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.942x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.943x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.943x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.946x |
| macos-15-native | snapshot | middlegame | ferrum | 0.955x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.958x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.960x |
| macos-15-native | materialize | endgame | ultra | 0.962x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.964x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.966x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.968x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| macos-15-native | fen_parse | tactical | ultra | 0.979x |
| macos-15-native | corpus_count | seeded | ultra | 0.980x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.987x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.990x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.993x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.995x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.997x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7167 | shakmaty | 8958 | 1.250x |
| ubuntu-24.04-arm-native | 5160 | shakmaty | 8672 | 1.681x |
| ubuntu-24.04-native | 5638 | shakmaty | 7960 | 1.412x |
| ubuntu-24.04-pext | 5113 | chess | 8545 | 1.671x |
| ubuntu-24.04-portable | 5024 | shakmaty | 6249 | 1.244x |
| windows-2025-native | 16700 | shakmaty | 22900 | 1.371x |

Cold wins: 6/6 (separate from native cases).
