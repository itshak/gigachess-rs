| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 66 | 63 | 13 | 87 |
| ubuntu-24.04-arm-native | 78 | 74 | 11 | 87 |
| ubuntu-24.04-native | 75 | 72 | 9 | 87 |
| ubuntu-24.04-pext | 73 | 71 | 9 | 87 |
| ubuntu-24.04-portable | 77 | 77 | 10 | 87 |
| windows-2025-native | 77 | 75 | 12 | 87 |
| ALL | 446 | 432 | 64 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.968x | 1.045x | 24/36 | 0.779x |
| chess960_materialize | 1.002x | 1.354x | 12/12 | 1.087x |
| chess960_perft_d3 | 0.967x | 1.390x | 12/12 | 1.159x |
| corpus_count | 0.970x | 1.335x | 12/12 | 1.002x |
| corpus_materialize | 0.965x | 1.129x | 12/12 | 1.005x |
| corpus_uci_sorted | 0.979x | 1.477x | 12/12 | 1.082x |
| count | 0.973x | 1.283x | 33/36 | 0.802x |
| fen_parse | 0.998x | 1.170x | 34/36 | 0.936x |
| fen_write_native_ep | 1.473x | 1.648x | 36/36 | 1.123x |
| make_unmake_full | 0.994x | 1.246x | 34/36 | 0.882x |
| materialize | 0.991x | 1.282x | 33/36 | 0.769x |
| perft_bulk_d4 | 0.998x | 1.276x | 36/36 | 1.044x |
| perft_full_d3 | 1.029x | 1.282x | 34/36 | 0.932x |
| polyglot_key | 0.994x | 34.329x | 12/12 | 16.134x |
| polyglot_recompute | 0.984x | 1.017x | 7/12 | 0.929x |
| repetition_key | 0.993x | 22.020x | 12/12 | 14.107x |
| replay_checked_native | 0.984x | 1.009x | 7/12 | 0.807x |
| replay_trusted_native | 0.989x | 0.622x | 0/12 | 0.457x |
| san_parse_all | 0.989x | 1.889x | 36/36 | 1.410x |
| san_render_native | 0.986x | 2.547x | 36/36 | 1.891x |
| snapshot | 0.937x | 0.943x | 6/36 | 0.657x |
| startpos | 0.914x | 1.245x | 6/6 | 1.008x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.457x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.487x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.489x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.519x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.545x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.633x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.643x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.646x |
| macos-15-native | snapshot | promotion | ferrum | 0.657x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.723x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.747x |
| macos-15-native | materialize | tactical | ultra | 0.769x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.779x |
| macos-15-native | snapshot | start | ferrum | 0.793x |
| macos-15-native | count | middlegame | cozy | 0.802x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.807x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.812x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.821x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.838x |
| ubuntu-24.04-pext | apply_children_full | kiwipete | cozy | 0.842x |
| macos-15-native | apply_children_full | tactical | cozy | 0.844x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.848x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.851x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.854x |
| windows-2025-native | snapshot | tactical | ferrum | 0.856x |
| windows-2025-native | snapshot | endgame | ferrum | 0.856x |
| windows-2025-native | snapshot | start | ferrum | 0.859x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.860x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.861x |
| windows-2025-native | snapshot | promotion | ferrum | 0.862x |
| ubuntu-24.04-pext | apply_children_full | start | cozy | 0.868x |
| ubuntu-24.04-pext | apply_children_full | middlegame | cozy | 0.881x |
| macos-15-native | make_unmake_full | endgame | ultra | 0.882x |
| ubuntu-24.04-pext | apply_children_full | endgame | chess | 0.903x |
| macos-15-native | materialize | middlegame | ultra | 0.910x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.924x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.929x |
| macos-15-native | materialize | promotion | ultra | 0.929x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.932x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.934x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.936x |
| macos-15-native | fen_parse | middlegame | cozy | 0.936x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.938x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.939x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.943x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.954x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.954x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.956x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.956x |
| macos-15-native | fen_parse | promotion | shakmaty | 0.959x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.963x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.967x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.969x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.971x |
| ubuntu-24.04-portable | count | start | ferrum | 0.972x |
| ubuntu-24.04-native | snapshot | promotion | cozy | 0.975x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.976x |
| macos-15-native | snapshot | middlegame | ferrum | 0.979x |
| macos-15-native | snapshot | tactical | ferrum | 0.982x |
| ubuntu-24.04-portable | snapshot | start | cozy | 0.986x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.989x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.990x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 0.991x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.991x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.993x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | cozy | 0.995x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.996x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | chess | 0.997x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | cozy | 0.997x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.997x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.998x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 14375 | ferrum | 14583 | 1.014x |
| ubuntu-24.04-arm-native | 10625 | shakmaty | 7048 | 0.663x |
| ubuntu-24.04-native | 13650 | shakmaty | 8633 | 0.632x |
| ubuntu-24.04-pext | 9849 | cozy | 8403 | 0.853x |
| ubuntu-24.04-portable | 9386 | shakmaty | 7217 | 0.769x |
| windows-2025-native | 34400 | shakmaty | 27100 | 0.788x |

Cold wins: 1/6 (separate from native cases).
