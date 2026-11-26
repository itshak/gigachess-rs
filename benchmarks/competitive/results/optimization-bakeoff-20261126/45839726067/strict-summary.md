| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 73 | 12 | 87 |
| ubuntu-24.04-arm-native | 73 | 73 | 12 | 87 |
| ubuntu-24.04-native | 80 | 79 | 16 | 87 |
| ubuntu-24.04-pext | 77 | 73 | 12 | 87 |
| ubuntu-24.04-portable | 74 | 73 | 14 | 87 |
| windows-2025-native | 84 | 79 | 13 | 87 |
| ALL | 461 | 450 | 79 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.027x | 1.251x | 33/36 | 0.908x |
| chess960_materialize | 1.037x | 1.477x | 12/12 | 1.267x |
| chess960_perft_d3 | 1.013x | 1.552x | 12/12 | 1.364x |
| corpus_count | 1.024x | 1.248x | 11/12 | 0.965x |
| corpus_materialize | 1.017x | 1.208x | 12/12 | 1.061x |
| corpus_uci_sorted | 0.986x | 1.489x | 12/12 | 1.275x |
| count | 0.989x | 1.462x | 36/36 | 1.101x |
| fen_parse | 1.139x | 1.311x | 34/36 | 0.787x |
| fen_write_native_ep | 1.023x | 1.772x | 36/36 | 1.433x |
| make_unmake_full | 0.969x | 1.297x | 34/36 | 0.961x |
| materialize | 1.027x | 1.278x | 34/36 | 0.942x |
| perft_bulk_d4 | 1.028x | 1.303x | 35/36 | 0.915x |
| perft_full_d3 | 1.019x | 1.037x | 18/36 | 0.754x |
| polyglot_key | 0.992x | 28.692x | 12/12 | 16.632x |
| polyglot_recompute | 1.042x | 1.049x | 11/12 | 0.765x |
| repetition_key | 1.006x | 20.277x | 12/12 | 14.704x |
| replay_checked_native | 1.068x | 1.153x | 12/12 | 1.021x |
| replay_trusted_native | 0.939x | 0.800x | 3/12 | 0.539x |
| san_parse_all | 1.005x | 1.959x | 36/36 | 1.531x |
| san_render_native | 1.036x | 2.631x | 36/36 | 1.608x |
| snapshot | 1.018x | 1.033x | 15/36 | 0.938x |
| startpos | 0.944x | 1.159x | 5/6 | 0.930x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.539x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.563x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.639x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.661x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.674x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.739x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.754x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.765x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.766x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.787x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.792x |
| ubuntu-24.04-portable | perft_full_d3 | endgame | ultra | 0.798x |
| ubuntu-24.04-portable | perft_full_d3 | middlegame | ultra | 0.809x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.822x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.833x |
| ubuntu-24.04-arm-native | perft_full_d3 | endgame | ultra | 0.841x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.842x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.844x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.846x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.858x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.872x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.876x |
| macos-15-native | apply_children_full | tactical | cozy | 0.908x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.915x |
| ubuntu-24.04-portable | perft_full_d3 | promotion | ultra | 0.921x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.930x |
| macos-15-native | startpos | start | shakmaty | 0.930x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.938x |
| macos-15-native | materialize | kiwipete | ultra | 0.942x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.948x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.953x |
| macos-15-native | snapshot | tactical | chess | 0.958x |
| ubuntu-24.04-portable | perft_full_d3 | start | shakmaty | 0.960x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.961x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.962x |
| macos-15-native | perft_full_d3 | middlegame | ultra | 0.962x |
| macos-15-native | materialize | tactical | ultra | 0.965x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.965x |
| ubuntu-24.04-arm-native | perft_full_d3 | start | ultra | 0.966x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.968x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.970x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.970x |
| ubuntu-24.04-native | perft_full_d3 | endgame | ultra | 0.971x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.977x |
| windows-2025-native | snapshot | middlegame | chess | 0.980x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.982x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.986x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.986x |
| macos-15-native | apply_children_full | middlegame | chess | 0.989x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.989x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.989x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.991x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.993x |
| windows-2025-native | snapshot | kiwipete | chess | 0.994x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6291 | shakmaty | 11000 | 1.749x |
| ubuntu-24.04-arm-native | 6504 | shakmaty | 7367 | 1.133x |
| ubuntu-24.04-native | 6439 | shakmaty | 9554 | 1.484x |
| ubuntu-24.04-pext | 6389 | shakmaty | 7366 | 1.153x |
| ubuntu-24.04-portable | 6590 | shakmaty | 8733 | 1.325x |
| windows-2025-native | 17600 | shakmaty | 29500 | 1.676x |

Cold wins: 6/6 (separate from native cases).
