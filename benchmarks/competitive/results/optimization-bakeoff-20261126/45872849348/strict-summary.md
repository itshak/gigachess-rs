| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 66 | 66 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 78 | 78 | 12 | 87 |
| ubuntu-24.04-pext | 83 | 79 | 9 | 87 |
| ubuntu-24.04-portable | 85 | 80 | 12 | 87 |
| windows-2025-native | 83 | 80 | 12 | 87 |
| ALL | 474 | 461 | 69 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.874x | 1.129x | 31/36 | 0.720x |
| chess960_materialize | 0.994x | 1.419x | 12/12 | 1.154x |
| chess960_perft_d3 | 0.949x | 1.517x | 12/12 | 1.298x |
| corpus_count | 0.994x | 1.229x | 10/12 | 0.925x |
| corpus_materialize | 1.002x | 1.153x | 11/12 | 0.947x |
| corpus_uci_sorted | 1.028x | 1.559x | 12/12 | 1.311x |
| count | 0.984x | 1.479x | 35/36 | 0.947x |
| fen_parse | 1.018x | 1.220x | 36/36 | 1.032x |
| fen_write_native_ep | 0.978x | 1.678x | 36/36 | 1.395x |
| make_unmake_full | 0.902x | 1.256x | 33/36 | 0.898x |
| materialize | 0.979x | 1.264x | 33/36 | 0.770x |
| perft_bulk_d4 | 0.938x | 1.248x | 34/36 | 0.907x |
| perft_full_d3 | 0.963x | 1.289x | 35/36 | 0.999x |
| polyglot_key | 0.977x | 28.582x | 12/12 | 16.030x |
| polyglot_recompute | 0.989x | 1.044x | 10/12 | 0.859x |
| repetition_key | 0.985x | 20.547x | 12/12 | 14.793x |
| replay_checked_native | 1.011x | 1.178x | 12/12 | 1.059x |
| replay_trusted_native | 1.006x | 0.837x | 1/12 | 0.659x |
| san_parse_all | 0.961x | 1.865x | 36/36 | 1.465x |
| san_render_native | 0.966x | 2.543x | 36/36 | 1.748x |
| snapshot | 0.930x | 1.023x | 19/36 | 0.679x |
| startpos | 0.955x | 1.197x | 6/6 | 1.001x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.659x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.674x |
| macos-15-native | snapshot | endgame | chess | 0.679x |
| macos-15-native | snapshot | tactical | ferrum | 0.693x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.720x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.755x |
| macos-15-native | materialize | kiwipete | ultra | 0.770x |
| macos-15-native | snapshot | middlegame | ferrum | 0.771x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.775x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.778x |
| macos-15-native | materialize | middlegame | ultra | 0.795x |
| macos-15-native | materialize | tactical | ultra | 0.804x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.813x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.828x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.851x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.858x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.859x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.863x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.873x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.876x |
| macos-15-native | snapshot | promotion | chess | 0.878x |
| macos-15-native | apply_children_full | middlegame | chess | 0.882x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.883x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.898x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.907x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.920x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.925x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.944x |
| macos-15-native | count | promotion | cozy | 0.947x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.947x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.949x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.955x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.955x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.966x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.967x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.971x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.972x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.973x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.977x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.977x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.979x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.980x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-pext | make_unmake_full | kiwipete | ultra | 0.998x |
| macos-15-native | perft_full_d3 | middlegame | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6416 | shakmaty | 7083 | 1.104x |
| ubuntu-24.04-arm-native | 5768 | shakmaty | 7833 | 1.358x |
| ubuntu-24.04-native | 8466 | shakmaty | 10520 | 1.243x |
| ubuntu-24.04-pext | 5673 | cozy | 8077 | 1.424x |
| ubuntu-24.04-portable | 7584 | shakmaty | 11061 | 1.458x |
| windows-2025-native | 16100 | shakmaty | 18100 | 1.124x |

Cold wins: 6/6 (separate from native cases).
