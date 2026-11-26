| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 62 | 60 | 13 | 87 |
| ubuntu-24.04-arm-native | 77 | 76 | 11 | 87 |
| ubuntu-24.04-native | 74 | 72 | 9 | 87 |
| ubuntu-24.04-pext | 81 | 78 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 80 | 10 | 87 |
| windows-2025-native | 73 | 73 | 11 | 87 |
| ALL | 449 | 439 | 64 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.883x | 1.191x | 29/36 | 0.728x |
| chess960_materialize | 0.923x | 1.422x | 12/12 | 1.218x |
| chess960_perft_d3 | 0.830x | 1.267x | 11/12 | 0.721x |
| corpus_count | 0.956x | 1.199x | 10/12 | 0.794x |
| corpus_materialize | 1.004x | 1.112x | 11/12 | 0.870x |
| corpus_uci_sorted | 1.031x | 1.537x | 12/12 | 1.278x |
| count | 0.926x | 1.380x | 35/36 | 0.966x |
| fen_parse | 0.939x | 1.154x | 34/36 | 0.951x |
| fen_write_native_ep | 0.922x | 1.591x | 36/36 | 1.169x |
| make_unmake_full | 0.853x | 1.130x | 30/36 | 0.785x |
| materialize | 0.980x | 1.316x | 34/36 | 0.788x |
| perft_bulk_d4 | 0.844x | 1.088x | 26/36 | 0.803x |
| perft_full_d3 | 0.811x | 1.087x | 25/36 | 0.683x |
| polyglot_key | 1.047x | 35.356x | 12/12 | 18.276x |
| polyglot_recompute | 1.029x | 1.002x | 6/12 | 0.869x |
| repetition_key | 1.075x | 22.104x | 12/12 | 16.952x |
| replay_checked_native | 0.904x | 1.027x | 7/12 | 0.900x |
| replay_trusted_native | 0.968x | 0.770x | 4/12 | 0.520x |
| san_parse_all | 0.944x | 1.802x | 36/36 | 1.417x |
| san_render_native | 0.940x | 2.462x | 36/36 | 1.697x |
| snapshot | 1.009x | 1.111x | 26/36 | 0.682x |
| startpos | 1.263x | 1.517x | 5/6 | 0.907x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.520x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.521x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.542x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.547x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.668x |
| macos-15-native | snapshot | middlegame | ferrum | 0.682x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.683x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.716x |
| windows-2025-native | chess960_perft_d3 | 960_castle | cozy | 0.721x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.722x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.724x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.728x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.785x |
| macos-15-native | materialize | kiwipete | ultra | 0.788x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.794x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.803x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.806x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.812x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.813x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.827x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.850x |
| ubuntu-24.04-native | snapshot | start | chess | 0.857x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.861x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.863x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.866x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.867x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.869x |
| macos-15-native | corpus_materialize | lichess300 | ferrum | 0.870x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.871x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.880x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.900x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | chess | 0.901x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.907x |
| windows-2025-native | startpos | start | shakmaty | 0.907x |
| macos-15-native | perft_full_d3 | tactical | ultra | 0.916x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.920x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.921x |
| macos-15-native | snapshot | tactical | cozy | 0.921x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.924x |
| windows-2025-native | perft_full_d3 | tactical | shakmaty | 0.934x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.938x |
| windows-2025-native | perft_bulk_d4 | kiwipete | ultra | 0.941x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.941x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.945x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.945x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.951x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.951x |
| macos-15-native | perft_bulk_d4 | endgame | ultra | 0.952x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.953x |
| macos-15-native | apply_children_full | tactical | chess | 0.955x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.957x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.959x |
| macos-15-native | perft_bulk_d4 | middlegame | cozy | 0.960x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.961x |
| windows-2025-native | perft_full_d3 | endgame | ultra | 0.964x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.965x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.966x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.968x |
| macos-15-native | make_unmake_full | middlegame | ultra | 0.968x |
| ubuntu-24.04-arm-native | apply_children_full | middlegame | ferrum | 0.968x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.969x |
| macos-15-native | snapshot | start | ferrum | 0.975x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.978x |
| macos-15-native | materialize | middlegame | ultra | 0.979x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.980x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.981x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.985x |
| macos-15-native | perft_full_d3 | middlegame | ultra | 0.986x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.987x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.994x |
| macos-15-native | apply_children_full | start | chess | 0.995x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.995x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 9667 | shakmaty | 11250 | 1.164x |
| ubuntu-24.04-arm-native | 6504 | shakmaty | 10351 | 1.591x |
| ubuntu-24.04-native | 5766 | shakmaty | 6851 | 1.188x |
| ubuntu-24.04-pext | 8276 | cozy | 10520 | 1.271x |
| ubuntu-24.04-portable | 6228 | shakmaty | 7739 | 1.243x |
| windows-2025-native | 18000 | shakmaty | 21300 | 1.183x |

Cold wins: 6/6 (separate from native cases).
