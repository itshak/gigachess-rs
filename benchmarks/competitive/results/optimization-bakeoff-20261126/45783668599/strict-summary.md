| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 65 | 10 | 87 |
| ubuntu-24.04-arm-native | 74 | 67 | 10 | 87 |
| ubuntu-24.04-native | 82 | 81 | 8 | 87 |
| ubuntu-24.04-pext | 70 | 67 | 11 | 87 |
| ubuntu-24.04-portable | 77 | 75 | 8 | 87 |
| windows-2025-native | 80 | 77 | 11 | 87 |
| ALL | 450 | 432 | 58 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.998x | 1.078x | 27/36 | 0.793x |
| chess960_materialize | 1.023x | 1.354x | 12/12 | 1.225x |
| chess960_perft_d3 | 1.039x | 1.365x | 12/12 | 1.251x |
| corpus_count | 1.033x | 1.384x | 12/12 | 1.212x |
| corpus_materialize | 1.029x | 1.176x | 12/12 | 1.059x |
| corpus_uci_sorted | 1.035x | 1.526x | 12/12 | 1.400x |
| count | 1.003x | 1.319x | 33/36 | 0.960x |
| fen_parse | 1.018x | 1.197x | 33/36 | 0.963x |
| fen_write_native_ep | 0.987x | 1.126x | 26/36 | 0.869x |
| make_unmake_full | 0.991x | 1.227x | 35/36 | 0.861x |
| materialize | 0.975x | 1.281x | 32/36 | 0.803x |
| perft_bulk_d4 | 0.972x | 1.267x | 36/36 | 1.034x |
| perft_full_d3 | 0.942x | 1.203x | 30/36 | 0.823x |
| polyglot_key | 1.026x | 31.681x | 12/12 | 16.493x |
| polyglot_recompute | 1.038x | 1.030x | 8/12 | 0.959x |
| repetition_key | 1.007x | 20.077x | 12/12 | 14.682x |
| replay_checked_native | 0.992x | 1.015x | 7/12 | 0.930x |
| replay_trusted_native | 0.976x | 0.626x | 0/12 | 0.411x |
| san_parse_all | 0.982x | 1.892x | 36/36 | 1.093x |
| san_render_native | 0.994x | 2.467x | 36/36 | 1.590x |
| snapshot | 1.009x | 1.001x | 21/36 | 0.633x |
| startpos | 0.989x | 1.243x | 6/6 | 1.086x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.411x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.486x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.516x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.531x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.572x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.614x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.623x |
| macos-15-native | snapshot | tactical | ferrum | 0.633x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.663x |
| macos-15-native | snapshot | start | ferrum | 0.682x |
| macos-15-native | snapshot | middlegame | ferrum | 0.711x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.734x |
| macos-15-native | snapshot | promotion | ferrum | 0.787x |
| macos-15-native | apply_children_full | promotion | chess | 0.793x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.797x |
| macos-15-native | materialize | tactical | ultra | 0.803x |
| macos-15-native | materialize | promotion | ferrum | 0.819x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | shakmaty | 0.823x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.825x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.826x |
| macos-15-native | apply_children_full | start | chess | 0.827x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.861x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.861x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | shakmaty | 0.864x |
| ubuntu-24.04-pext | fen_write_native_ep | kiwipete | ultra | 0.869x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.878x |
| ubuntu-24.04-pext | fen_write_native_ep | middlegame | ultra | 0.879x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.882x |
| macos-15-native | materialize | kiwipete | ultra | 0.887x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.911x |
| macos-15-native | apply_children_full | tactical | cozy | 0.924x |
| ubuntu-24.04-pext | fen_write_native_ep | start | ultra | 0.924x |
| ubuntu-24.04-pext | fen_write_native_ep | promotion | ultra | 0.929x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.930x |
| macos-15-native | materialize | start | ultra | 0.947x |
| ubuntu-24.04-pext | fen_write_native_ep | endgame | ultra | 0.948x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.951x |
| ubuntu-24.04-pext | fen_write_native_ep | tactical | ultra | 0.952x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.959x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.959x |
| macos-15-native | count | middlegame | cozy | 0.960x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.962x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.963x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.963x |
| windows-2025-native | fen_write_native_ep | kiwipete | ultra | 0.963x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.963x |
| macos-15-native | snapshot | endgame | ferrum | 0.964x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.964x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.971x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.972x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.974x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.978x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.978x |
| ubuntu-24.04-pext | perft_full_d3 | endgame | ultra | 0.978x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.978x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.979x |
| ubuntu-24.04-portable | count | start | ferrum | 0.983x |
| macos-15-native | apply_children_full | middlegame | ferrum | 0.985x |
| windows-2025-native | fen_write_native_ep | middlegame | ultra | 0.987x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.988x |
| ubuntu-24.04-arm-native | fen_write_native_ep | middlegame | ultra | 0.989x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.990x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.993x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.993x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.994x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.996x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 15250 | shakmaty | 9500 | 0.623x |
| ubuntu-24.04-arm-native | 10385 | shakmaty | 9296 | 0.895x |
| ubuntu-24.04-native | 8728 | shakmaty | 6844 | 0.784x |
| ubuntu-24.04-pext | 14778 | cozy | 11471 | 0.776x |
| ubuntu-24.04-portable | 9749 | shakmaty | 7020 | 0.720x |
| windows-2025-native | 46100 | shakmaty | 30400 | 0.659x |

Cold wins: 0/6 (separate from native cases).
