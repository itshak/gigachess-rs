| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 69 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 81 | 10 | 87 |
| ubuntu-24.04-pext | 80 | 72 | 13 | 87 |
| ubuntu-24.04-portable | 82 | 76 | 12 | 87 |
| windows-2025-native | 84 | 80 | 13 | 87 |
| ALL | 480 | 457 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.015x | 1.276x | 35/36 | 0.849x |
| chess960_materialize | 1.071x | 1.481x | 12/12 | 1.285x |
| chess960_perft_d3 | 1.054x | 1.552x | 12/12 | 1.363x |
| corpus_count | 0.949x | 1.115x | 9/12 | 0.755x |
| corpus_materialize | 1.024x | 1.159x | 12/12 | 1.036x |
| corpus_uci_sorted | 1.009x | 1.506x | 12/12 | 1.295x |
| count | 0.995x | 1.484x | 36/36 | 1.027x |
| fen_parse | 1.019x | 1.187x | 35/36 | 0.990x |
| fen_write_native_ep | 1.015x | 1.771x | 36/36 | 1.391x |
| make_unmake_full | 0.989x | 1.288x | 35/36 | 0.984x |
| materialize | 1.018x | 1.419x | 34/36 | 0.913x |
| perft_bulk_d4 | 1.013x | 1.266x | 33/36 | 0.895x |
| perft_full_d3 | 1.015x | 1.354x | 36/36 | 1.185x |
| polyglot_key | 0.990x | 35.103x | 12/12 | 16.232x |
| polyglot_recompute | 1.007x | 1.011x | 8/12 | 0.844x |
| repetition_key | 0.984x | 20.168x | 12/12 | 14.223x |
| replay_checked_native | 0.966x | 1.123x | 11/12 | 0.911x |
| replay_trusted_native | 0.975x | 0.760x | 4/12 | 0.562x |
| san_parse_all | 1.004x | 1.895x | 36/36 | 1.525x |
| san_render_native | 0.997x | 2.595x | 36/36 | 1.857x |
| snapshot | 0.996x | 0.989x | 19/36 | 0.663x |
| startpos | 0.989x | 1.157x | 5/6 | 0.929x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.562x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.592x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.635x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.654x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.657x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.661x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.663x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.665x |
| macos-15-native | snapshot | tactical | cozy | 0.700x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.755x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.762x |
| macos-15-native | corpus_count | seeded | ultra | 0.764x |
| windows-2025-native | snapshot | middlegame | cozy | 0.825x |
| macos-15-native | snapshot | middlegame | ferrum | 0.830x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.844x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.849x |
| macos-15-native | snapshot | start | ferrum | 0.871x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.895x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.911x |
| macos-15-native | materialize | kiwipete | ultra | 0.913x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.917x |
| macos-15-native | snapshot | endgame | ferrum | 0.927x |
| macos-15-native | startpos | start | shakmaty | 0.929x |
| macos-15-native | materialize | middlegame | ultra | 0.936x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.947x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.963x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.975x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.978x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.984x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.987x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.988x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.989x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.990x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 10667 | ferrum | 14833 | 1.391x |
| ubuntu-24.04-arm-native | 5520 | shakmaty | 8688 | 1.574x |
| ubuntu-24.04-native | 4913 | shakmaty | 7508 | 1.528x |
| ubuntu-24.04-pext | 5331 | shakmaty | 7662 | 1.437x |
| ubuntu-24.04-portable | 5368 | shakmaty | 6487 | 1.208x |
| windows-2025-native | 15200 | shakmaty | 21100 | 1.388x |

Cold wins: 6/6 (separate from native cases).
