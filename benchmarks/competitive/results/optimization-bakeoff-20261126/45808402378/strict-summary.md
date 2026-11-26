| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 68 | 65 | 13 | 87 |
| ubuntu-24.04-arm-native | 76 | 75 | 11 | 87 |
| ubuntu-24.04-native | 80 | 75 | 11 | 87 |
| ubuntu-24.04-pext | 79 | 77 | 15 | 87 |
| ubuntu-24.04-portable | 79 | 73 | 12 | 87 |
| windows-2025-native | 79 | 74 | 12 | 87 |
| ALL | 461 | 439 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.045x | 1.167x | 32/36 | 0.844x |
| chess960_materialize | 1.027x | 1.399x | 12/12 | 1.179x |
| chess960_perft_d3 | 0.987x | 1.381x | 12/12 | 1.107x |
| corpus_count | 0.803x | 1.086x | 7/12 | 0.815x |
| corpus_materialize | 0.980x | 1.156x | 11/12 | 0.993x |
| corpus_uci_sorted | 0.992x | 1.454x | 12/12 | 1.042x |
| count | 0.985x | 1.336x | 34/36 | 0.953x |
| fen_parse | 1.004x | 1.179x | 36/36 | 1.016x |
| fen_write_native_ep | 1.001x | 1.835x | 36/36 | 1.433x |
| make_unmake_full | 1.001x | 1.224x | 36/36 | 1.006x |
| materialize | 1.003x | 1.266x | 33/36 | 0.699x |
| perft_bulk_d4 | 0.899x | 1.154x | 30/36 | 0.838x |
| perft_full_d3 | 1.005x | 1.232x | 30/36 | 0.845x |
| polyglot_key | 0.985x | 31.502x | 12/12 | 16.607x |
| polyglot_recompute | 0.991x | 1.020x | 7/12 | 0.877x |
| repetition_key | 0.913x | 18.077x | 12/12 | 14.321x |
| replay_checked_native | 0.998x | 1.027x | 7/12 | 0.938x |
| replay_trusted_native | 1.013x | 0.684x | 2/12 | 0.467x |
| san_parse_all | 0.972x | 1.937x | 36/36 | 1.507x |
| san_render_native | 0.992x | 2.462x | 36/36 | 1.659x |
| snapshot | 1.207x | 1.011x | 22/36 | 0.477x |
| startpos | 1.079x | 1.272x | 6/6 | 1.109x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.467x |
| macos-15-native | snapshot | endgame | ferrum | 0.477x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.500x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.539x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.552x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.572x |
| macos-15-native | snapshot | tactical | ferrum | 0.616x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.621x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.657x |
| macos-15-native | snapshot | start | ferrum | 0.665x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.672x |
| macos-15-native | materialize | tactical | ultra | 0.699x |
| macos-15-native | materialize | kiwipete | ultra | 0.784x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.815x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.838x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.844x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.845x |
| macos-15-native | apply_children_full | tactical | chess | 0.853x |
| macos-15-native | apply_children_full | promotion | chess | 0.856x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.874x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.877x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.895x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.905x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.906x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.914x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.916x |
| macos-15-native | corpus_count | seeded | ultra | 0.919x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.923x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.925x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.937x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.938x |
| ubuntu-24.04-arm-native | snapshot | endgame | cozy | 0.943x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.946x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.949x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.951x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.951x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.953x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.954x |
| macos-15-native | perft_bulk_d4 | middlegame | ultra | 0.958x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.958x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.959x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.966x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.968x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| macos-15-native | materialize | endgame | ultra | 0.970x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.972x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.979x |
| windows-2025-native | snapshot | tactical | cozy | 0.981x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.983x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.983x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.985x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.985x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.986x |
| windows-2025-native | snapshot | middlegame | cozy | 0.987x |
| macos-15-native | count | middlegame | cozy | 0.989x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.990x |
| macos-15-native | snapshot | kiwipete | cozy | 0.993x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.993x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.994x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.995x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6458 | shakmaty | 14250 | 2.207x |
| ubuntu-24.04-arm-native | 5592 | shakmaty | 8208 | 1.468x |
| ubuntu-24.04-native | 5446 | shakmaty | 7290 | 1.339x |
| ubuntu-24.04-pext | 5939 | shakmaty | 9234 | 1.555x |
| ubuntu-24.04-portable | 6198 | shakmaty | 7975 | 1.287x |
| windows-2025-native | 21700 | cozy | 27200 | 1.253x |

Cold wins: 6/6 (separate from native cases).
