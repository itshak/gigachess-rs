| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 66 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 82 | 8 | 87 |
| ubuntu-24.04-pext | 85 | 84 | 13 | 87 |
| ubuntu-24.04-portable | 77 | 74 | 12 | 87 |
| windows-2025-native | 77 | 76 | 13 | 87 |
| ALL | 473 | 461 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.002x | 1.223x | 33/36 | 0.917x |
| chess960_materialize | 0.989x | 1.460x | 12/12 | 1.215x |
| chess960_perft_d3 | 1.016x | 1.542x | 12/12 | 1.423x |
| corpus_count | 1.023x | 1.289x | 10/12 | 0.974x |
| corpus_materialize | 1.005x | 1.205x | 12/12 | 1.028x |
| corpus_uci_sorted | 0.980x | 1.478x | 11/12 | 0.991x |
| count | 0.995x | 1.478x | 36/36 | 1.130x |
| fen_parse | 0.972x | 1.143x | 33/36 | 0.932x |
| fen_write_native_ep | 1.015x | 1.711x | 36/36 | 1.293x |
| make_unmake_full | 1.002x | 1.313x | 35/36 | 0.985x |
| materialize | 0.999x | 1.292x | 34/36 | 0.880x |
| perft_bulk_d4 | 1.012x | 1.311x | 35/36 | 0.895x |
| perft_full_d3 | 1.017x | 1.323x | 36/36 | 1.179x |
| polyglot_key | 1.019x | 28.833x | 12/12 | 16.064x |
| polyglot_recompute | 1.023x | 1.022x | 8/12 | 0.890x |
| repetition_key | 1.012x | 20.566x | 12/12 | 13.980x |
| replay_checked_native | 1.018x | 1.065x | 9/12 | 0.987x |
| replay_trusted_native | 0.999x | 0.808x | 3/12 | 0.572x |
| san_parse_all | 1.014x | 1.954x | 36/36 | 1.511x |
| san_render_native | 0.988x | 2.478x | 36/36 | 1.654x |
| snapshot | 1.026x | 1.027x | 17/36 | 0.623x |
| startpos | 0.950x | 1.148x | 5/6 | 0.766x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.572x |
| macos-15-native | snapshot | middlegame | ferrum | 0.623x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.631x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.665x |
| macos-15-native | snapshot | tactical | ferrum | 0.680x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.683x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.693x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.700x |
| macos-15-native | startpos | start | shakmaty | 0.766x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.852x |
| macos-15-native | materialize | middlegame | ultra | 0.880x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.890x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.895x |
| macos-15-native | apply_children_full | tactical | chess | 0.917x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.929x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.929x |
| macos-15-native | fen_parse | kiwipete | shakmaty | 0.932x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.932x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.940x |
| macos-15-native | apply_children_full | kiwipete | cozy | 0.954x |
| macos-15-native | apply_children_full | start | chess | 0.957x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 0.969x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.974x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.981x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.984x |
| macos-15-native | materialize | tactical | ultra | 0.985x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.985x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.987x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.987x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.989x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.989x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.991x |
| macos-15-native | corpus_uci_sorted | seeded | ferrum | 0.991x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.991x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.992x |
| windows-2025-native | snapshot | start | ferrum | 0.992x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.992x |
| windows-2025-native | snapshot | tactical | cozy | 0.993x |
| windows-2025-native | snapshot | endgame | ferrum | 0.994x |
| windows-2025-native | snapshot | promotion | cozy | 0.994x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.998x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.998x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| windows-2025-native | fen_parse | middlegame | ultra | 1.000x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5209 | shakmaty | 8084 | 1.552x |
| ubuntu-24.04-arm-native | 5120 | shakmaty | 8375 | 1.636x |
| ubuntu-24.04-native | 5328 | shakmaty | 6528 | 1.225x |
| ubuntu-24.04-pext | 8636 | shakmaty | 10841 | 1.255x |
| ubuntu-24.04-portable | 7782 | shakmaty | 9314 | 1.197x |
| windows-2025-native | 18500 | shakmaty | 24200 | 1.308x |

Cold wins: 6/6 (separate from native cases).
