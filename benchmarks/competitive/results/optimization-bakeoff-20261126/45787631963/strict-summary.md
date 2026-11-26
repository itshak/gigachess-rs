| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 66 | 12 | 87 |
| ubuntu-24.04-arm-native | 80 | 74 | 11 | 87 |
| ubuntu-24.04-native | 80 | 76 | 12 | 87 |
| ubuntu-24.04-pext | 76 | 71 | 12 | 87 |
| ubuntu-24.04-portable | 84 | 77 | 12 | 87 |
| windows-2025-native | 70 | 69 | 11 | 87 |
| ALL | 460 | 433 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.031x | 1.109x | 30/36 | 0.796x |
| chess960_materialize | 1.029x | 1.356x | 12/12 | 1.103x |
| chess960_perft_d3 | 0.976x | 1.359x | 12/12 | 1.004x |
| corpus_count | 0.754x | 1.011x | 5/12 | 0.759x |
| corpus_materialize | 0.992x | 1.113x | 11/12 | 0.998x |
| corpus_uci_sorted | 1.013x | 1.543x | 12/12 | 1.354x |
| count | 0.997x | 1.322x | 35/36 | 0.890x |
| fen_parse | 1.018x | 1.197x | 35/36 | 0.967x |
| fen_write_native_ep | 0.991x | 1.732x | 36/36 | 1.392x |
| make_unmake_full | 1.004x | 1.245x | 36/36 | 1.020x |
| materialize | 1.015x | 1.303x | 34/36 | 0.876x |
| perft_bulk_d4 | 0.899x | 1.136x | 31/36 | 0.847x |
| perft_full_d3 | 0.993x | 1.219x | 30/36 | 0.705x |
| polyglot_key | 1.002x | 33.735x | 12/12 | 16.209x |
| polyglot_recompute | 0.988x | 1.024x | 8/12 | 0.867x |
| repetition_key | 1.061x | 20.296x | 12/12 | 14.640x |
| replay_checked_native | 1.023x | 1.016x | 9/12 | 0.844x |
| replay_trusted_native | 0.987x | 0.640x | 0/12 | 0.475x |
| san_parse_all | 1.038x | 1.925x | 36/36 | 1.454x |
| san_render_native | 1.001x | 2.577x | 36/36 | 1.903x |
| snapshot | 0.977x | 0.969x | 23/36 | 0.734x |
| startpos | 0.863x | 1.088x | 5/6 | 0.607x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.475x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.485x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.490x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.514x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.590x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.607x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.614x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.623x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.628x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.705x |
| windows-2025-native | snapshot | start | ferrum | 0.734x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.737x |
| windows-2025-native | snapshot | tactical | ferrum | 0.740x |
| windows-2025-native | snapshot | promotion | ferrum | 0.752x |
| windows-2025-native | snapshot | middlegame | chess | 0.753x |
| windows-2025-native | snapshot | endgame | ferrum | 0.757x |
| macos-15-native | corpus_count | seeded | ultra | 0.759x |
| macos-15-native | snapshot | tactical | cozy | 0.785x |
| macos-15-native | snapshot | middlegame | ferrum | 0.790x |
| macos-15-native | apply_children_full | tactical | chess | 0.796x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.800x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.811x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.819x |
| macos-15-native | snapshot | promotion | ferrum | 0.825x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.830x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.833x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.840x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.840x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.844x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.847x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.850x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.862x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.865x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.867x |
| macos-15-native | materialize | kiwipete | ultra | 0.876x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.880x |
| macos-15-native | count | middlegame | cozy | 0.890x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.891x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.908x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.919x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.940x |
| windows-2025-native | perft_full_d3 | endgame | ultra | 0.944x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.950x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.954x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.965x |
| windows-2025-native | fen_parse | tactical | shakmaty | 0.967x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.967x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.968x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.970x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.974x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.979x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.980x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.983x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.985x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.990x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.998x |
| macos-15-native | materialize | tactical | ultra | 0.999x |
| ubuntu-24.04-pext | snapshot | start | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | endgame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7375 | shakmaty | 13375 | 1.814x |
| ubuntu-24.04-arm-native | 6055 | shakmaty | 9047 | 1.494x |
| ubuntu-24.04-native | 5765 | shakmaty | 7199 | 1.249x |
| ubuntu-24.04-pext | 5887 | cozy | 8246 | 1.401x |
| ubuntu-24.04-portable | 7935 | shakmaty | 11401 | 1.437x |
| windows-2025-native | 13000 | shakmaty | 18400 | 1.415x |

Cold wins: 6/6 (separate from native cases).
