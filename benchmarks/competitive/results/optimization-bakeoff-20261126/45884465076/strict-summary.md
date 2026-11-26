| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 72 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 79 | 74 | 12 | 87 |
| ubuntu-24.04-pext | 82 | 82 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 74 | 12 | 87 |
| windows-2025-native | 84 | 82 | 12 | 87 |
| ALL | 479 | 463 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.998x | 1.275x | 35/36 | 0.978x |
| chess960_materialize | 0.941x | 1.472x | 12/12 | 1.276x |
| chess960_perft_d3 | 0.967x | 1.495x | 12/12 | 1.334x |
| corpus_count | 0.960x | 1.147x | 8/12 | 0.820x |
| corpus_materialize | 0.981x | 1.129x | 12/12 | 1.050x |
| corpus_uci_sorted | 0.987x | 1.464x | 12/12 | 1.225x |
| count | 1.023x | 1.458x | 35/36 | 0.988x |
| fen_parse | 0.998x | 1.168x | 36/36 | 1.008x |
| fen_write_native_ep | 1.007x | 1.736x | 36/36 | 1.372x |
| make_unmake_full | 1.017x | 1.294x | 36/36 | 1.041x |
| materialize | 1.041x | 1.372x | 34/36 | 0.768x |
| perft_bulk_d4 | 1.019x | 1.244x | 32/36 | 0.842x |
| perft_full_d3 | 1.029x | 1.334x | 36/36 | 1.157x |
| polyglot_key | 0.972x | 36.457x | 12/12 | 16.645x |
| polyglot_recompute | 0.953x | 0.993x | 7/12 | 0.848x |
| repetition_key | 0.975x | 23.166x | 12/12 | 14.500x |
| replay_checked_native | 1.003x | 1.177x | 12/12 | 1.096x |
| replay_trusted_native | 0.972x | 0.770x | 4/12 | 0.535x |
| san_parse_all | 1.018x | 1.827x | 36/36 | 1.442x |
| san_render_native | 1.017x | 2.613x | 36/36 | 1.894x |
| snapshot | 0.990x | 1.087x | 19/36 | 0.667x |
| startpos | 0.940x | 1.225x | 5/6 | 0.986x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.535x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.539x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.584x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.651x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.666x |
| macos-15-native | snapshot | middlegame | ferrum | 0.667x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.697x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.702x |
| macos-15-native | snapshot | endgame | ferrum | 0.753x |
| macos-15-native | materialize | kiwipete | ultra | 0.768x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.820x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.842x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.848x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.850x |
| macos-15-native | materialize | tactical | ultra | 0.860x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.876x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.877x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.884x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.911x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.925x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.929x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.941x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.941x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.951x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.951x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.968x |
| macos-15-native | snapshot | promotion | ferrum | 0.972x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.978x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.980x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.986x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.988x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.990x |
| macos-15-native | snapshot | start | ferrum | 0.991x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.994x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.994x |
| macos-15-native | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.999x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 1.000x |
| ubuntu-24.04-native | snapshot | promotion | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6333 | shakmaty | 9833 | 1.553x |
| ubuntu-24.04-arm-native | 6040 | shakmaty | 8064 | 1.335x |
| ubuntu-24.04-native | 5735 | shakmaty | 6343 | 1.106x |
| ubuntu-24.04-pext | 7823 | shakmaty | 8800 | 1.125x |
| ubuntu-24.04-portable | 5081 | shakmaty | 5827 | 1.147x |
| windows-2025-native | 8800 | shakmaty | 12500 | 1.420x |

Cold wins: 6/6 (separate from native cases).
