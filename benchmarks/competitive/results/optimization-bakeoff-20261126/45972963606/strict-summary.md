| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 71 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 11 | 87 |
| ubuntu-24.04-native | 80 | 80 | 16 | 87 |
| ubuntu-24.04-pext | 79 | 77 | 15 | 87 |
| ubuntu-24.04-portable | 80 | 80 | 15 | 87 |
| windows-2025-native | 81 | 75 | 13 | 87 |
| ALL | 472 | 462 | 82 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.003x | 1.251x | 33/36 | 0.870x |
| chess960_materialize | 1.022x | 1.497x | 12/12 | 1.271x |
| chess960_perft_d3 | 0.996x | 1.517x | 12/12 | 1.347x |
| corpus_count | 1.081x | 1.337x | 11/12 | 0.969x |
| corpus_materialize | 1.014x | 1.209x | 12/12 | 1.027x |
| corpus_uci_sorted | 1.011x | 1.496x | 12/12 | 1.208x |
| count | 1.037x | 1.499x | 36/36 | 1.066x |
| fen_parse | 0.939x | 1.136x | 31/36 | 0.859x |
| fen_write_native_ep | 0.987x | 1.747x | 36/36 | 1.422x |
| make_unmake_full | 1.004x | 1.327x | 36/36 | 1.006x |
| materialize | 1.025x | 1.296x | 34/36 | 0.779x |
| perft_bulk_d4 | 1.026x | 1.364x | 34/36 | 0.912x |
| perft_full_d3 | 1.206x | 1.540x | 36/36 | 1.153x |
| polyglot_key | 0.987x | 28.210x | 12/12 | 16.626x |
| polyglot_recompute | 0.986x | 0.996x | 5/12 | 0.847x |
| repetition_key | 0.978x | 20.479x | 12/12 | 14.475x |
| replay_checked_native | 0.929x | 1.115x | 12/12 | 1.051x |
| replay_trusted_native | 0.972x | 0.821x | 5/12 | 0.510x |
| san_parse_all | 0.966x | 1.962x | 36/36 | 1.384x |
| san_render_native | 1.014x | 2.599x | 36/36 | 1.678x |
| snapshot | 1.024x | 0.999x | 14/36 | 0.630x |
| startpos | 0.959x | 1.193x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.510x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.542x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.606x |
| macos-15-native | snapshot | tactical | ferrum | 0.630x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.661x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.663x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.671x |
| macos-15-native | snapshot | promotion | cozy | 0.695x |
| macos-15-native | materialize | middlegame | ultra | 0.779x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.836x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.847x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.859x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.865x |
| ubuntu-24.04-native | fen_parse | middlegame | ultra | 0.867x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.870x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.871x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.888x |
| macos-15-native | apply_children_full | tactical | chess | 0.890x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.909x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.912x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.932x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.938x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.938x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.950x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.951x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.956x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.968x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.969x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.973x |
| macos-15-native | materialize | endgame | ultra | 0.975x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.976x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| windows-2025-native | snapshot | endgame | ferrum | 0.979x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.980x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.982x |
| windows-2025-native | snapshot | promotion | ferrum | 0.988x |
| windows-2025-native | snapshot | middlegame | cozy | 0.993x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.994x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.998x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.999x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 1.000x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 1.000x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6417 | shakmaty | 13250 | 2.065x |
| ubuntu-24.04-arm-native | 6416 | shakmaty | 8359 | 1.303x |
| ubuntu-24.04-native | 6410 | shakmaty | 8423 | 1.314x |
| ubuntu-24.04-pext | 6086 | cozy | 9010 | 1.480x |
| ubuntu-24.04-portable | 6390 | shakmaty | 7532 | 1.179x |
| windows-2025-native | 16100 | shakmaty | 21900 | 1.360x |

Cold wins: 6/6 (separate from native cases).
