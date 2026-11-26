| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 70 | 11 | 87 |
| ubuntu-24.04-arm-native | 73 | 72 | 12 | 87 |
| ubuntu-24.04-native | 81 | 72 | 12 | 87 |
| ubuntu-24.04-pext | 75 | 72 | 12 | 87 |
| ubuntu-24.04-portable | 81 | 74 | 12 | 87 |
| windows-2025-native | 78 | 75 | 14 | 87 |
| ALL | 461 | 435 | 73 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.890x | 1.115x | 31/36 | 0.681x |
| chess960_materialize | 0.988x | 1.478x | 12/12 | 1.314x |
| chess960_perft_d3 | 0.951x | 1.478x | 12/12 | 1.338x |
| corpus_count | 1.005x | 1.268x | 9/12 | 0.984x |
| corpus_materialize | 1.005x | 1.183x | 12/12 | 1.007x |
| corpus_uci_sorted | 1.018x | 1.530x | 12/12 | 1.371x |
| count | 0.996x | 1.475x | 36/36 | 1.050x |
| fen_parse | 0.974x | 1.158x | 34/36 | 0.977x |
| fen_write_native_ep | 1.010x | 1.753x | 36/36 | 1.411x |
| make_unmake_full | 0.897x | 1.204x | 32/36 | 0.897x |
| materialize | 0.991x | 1.281x | 33/36 | 0.907x |
| perft_bulk_d4 | 0.956x | 1.245x | 35/36 | 0.879x |
| perft_full_d3 | 0.981x | 1.044x | 23/36 | 0.750x |
| polyglot_key | 0.946x | 27.754x | 12/12 | 15.756x |
| polyglot_recompute | 0.996x | 1.011x | 8/12 | 0.827x |
| repetition_key | 0.975x | 19.697x | 12/12 | 13.706x |
| replay_checked_native | 0.998x | 1.042x | 9/12 | 0.944x |
| replay_trusted_native | 0.997x | 0.792x | 1/12 | 0.591x |
| san_parse_all | 0.991x | 1.995x | 36/36 | 1.521x |
| san_render_native | 0.987x | 2.400x | 36/36 | 1.433x |
| snapshot | 0.998x | 1.079x | 24/36 | 0.860x |
| startpos | 1.016x | 1.224x | 6/6 | 1.054x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.591x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.640x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.645x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.679x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.681x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.708x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.750x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.778x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.781x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.786x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.798x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.820x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.827x |
| ubuntu-24.04-arm-native | perft_full_d3 | endgame | ultra | 0.832x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.838x |
| windows-2025-native | snapshot | kiwipete | chess | 0.860x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.865x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.865x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.867x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.879x |
| macos-15-native | apply_children_full | tactical | chess | 0.882x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.897x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.897x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.904x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.905x |
| macos-15-native | materialize | kiwipete | ultra | 0.907x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.915x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.930x |
| macos-15-native | materialize | tactical | ultra | 0.933x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.937x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.943x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.944x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.949x |
| ubuntu-24.04-pext | make_unmake_full | kiwipete | ultra | 0.953x |
| macos-15-native | snapshot | middlegame | ferrum | 0.955x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.961x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.962x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.964x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.970x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.973x |
| ubuntu-24.04-pext | perft_full_d3 | endgame | ultra | 0.973x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.977x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.979x |
| ubuntu-24.04-native | apply_children_full | tactical | chess | 0.983x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.984x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.984x |
| macos-15-native | materialize | middlegame | ultra | 0.985x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.985x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.986x |
| ubuntu-24.04-arm-native | perft_full_d3 | start | ultra | 0.986x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.988x |
| macos-15-native | snapshot | endgame | ferrum | 0.989x |
| windows-2025-native | snapshot | endgame | cozy | 0.991x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 0.992x |
| windows-2025-native | snapshot | start | chess | 0.993x |
| windows-2025-native | snapshot | tactical | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4250 | shakmaty | 5000 | 1.176x |
| ubuntu-24.04-arm-native | 6816 | shakmaty | 8368 | 1.228x |
| ubuntu-24.04-native | 5828 | shakmaty | 6729 | 1.155x |
| ubuntu-24.04-pext | 6659 | shakmaty | 8963 | 1.346x |
| ubuntu-24.04-portable | 7485 | shakmaty | 9057 | 1.210x |
| windows-2025-native | 15400 | shakmaty | 20400 | 1.325x |

Cold wins: 6/6 (separate from native cases).
