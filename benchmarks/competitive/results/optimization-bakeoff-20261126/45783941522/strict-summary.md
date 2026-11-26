| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 67 | 11 | 87 |
| ubuntu-24.04-arm-native | 76 | 73 | 11 | 87 |
| ubuntu-24.04-native | 78 | 76 | 14 | 87 |
| ubuntu-24.04-pext | 82 | 79 | 12 | 87 |
| ubuntu-24.04-portable | 79 | 74 | 11 | 87 |
| windows-2025-native | 77 | 75 | 11 | 87 |
| ALL | 463 | 444 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.024x | 1.079x | 27/36 | 0.612x |
| chess960_materialize | 1.060x | 1.342x | 12/12 | 1.131x |
| chess960_perft_d3 | 1.046x | 1.384x | 12/12 | 1.253x |
| corpus_count | 1.020x | 1.343x | 12/12 | 1.012x |
| corpus_materialize | 1.055x | 1.205x | 12/12 | 1.074x |
| corpus_uci_sorted | 1.055x | 1.564x | 12/12 | 1.322x |
| count | 0.966x | 1.313x | 36/36 | 1.012x |
| fen_parse | 1.024x | 1.199x | 35/36 | 0.985x |
| fen_write_native_ep | 0.992x | 1.676x | 36/36 | 1.397x |
| make_unmake_full | 0.983x | 1.271x | 36/36 | 1.026x |
| materialize | 0.963x | 1.156x | 33/36 | 0.782x |
| perft_bulk_d4 | 0.997x | 1.278x | 36/36 | 1.099x |
| perft_full_d3 | 0.984x | 1.144x | 27/36 | 0.743x |
| polyglot_key | 1.046x | 29.398x | 12/12 | 16.512x |
| polyglot_recompute | 1.050x | 1.023x | 8/12 | 0.887x |
| repetition_key | 1.035x | 20.688x | 12/12 | 14.602x |
| replay_checked_native | 1.040x | 0.992x | 8/12 | 0.846x |
| replay_trusted_native | 1.016x | 0.632x | 0/12 | 0.442x |
| san_parse_all | 0.977x | 1.953x | 36/36 | 1.365x |
| san_render_native | 1.014x | 2.614x | 36/36 | 1.587x |
| snapshot | 1.065x | 1.094x | 21/36 | 0.660x |
| startpos | 0.890x | 0.946x | 4/6 | 0.500x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.442x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.466x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.487x |
| windows-2025-native | startpos | start | shakmaty | 0.500x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.529x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.575x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.612x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.614x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.631x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.660x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.715x |
| macos-15-native | startpos | start | shakmaty | 0.732x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.743x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.758x |
| macos-15-native | snapshot | start | ferrum | 0.761x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.782x |
| macos-15-native | materialize | start | ultra | 0.782x |
| macos-15-native | materialize | middlegame | ultra | 0.790x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.803x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.812x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.827x |
| macos-15-native | materialize | kiwipete | ultra | 0.829x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.840x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.846x |
| macos-15-native | snapshot | middlegame | cozy | 0.860x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.880x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.887x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.891x |
| macos-15-native | apply_children_full | middlegame | chess | 0.896x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.898x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.901x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.906x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.910x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.912x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.913x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.915x |
| macos-15-native | snapshot | tactical | ferrum | 0.916x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.916x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.921x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.929x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.929x |
| macos-15-native | apply_children_full | kiwipete | cozy | 0.935x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.942x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.947x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.947x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.958x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.962x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.963x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.966x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.967x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.974x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.975x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.978x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.979x |
| macos-15-native | fen_parse | endgame | shakmaty | 0.985x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.985x |
| windows-2025-native | perft_full_d3 | endgame | ultra | 0.994x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.994x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 14209 | shakmaty | 9666 | 0.680x |
| ubuntu-24.04-arm-native | 10473 | shakmaty | 8784 | 0.839x |
| ubuntu-24.04-native | 12468 | shakmaty | 7612 | 0.611x |
| ubuntu-24.04-pext | 11481 | cozy | 11131 | 0.970x |
| ubuntu-24.04-portable | 14256 | shakmaty | 10229 | 0.718x |
| windows-2025-native | 22400 | shakmaty | 20500 | 0.915x |

Cold wins: 0/6 (separate from native cases).
