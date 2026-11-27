| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 72 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 83 | 11 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 14 | 87 |
| ubuntu-24.04-portable | 81 | 76 | 11 | 87 |
| windows-2025-native | 84 | 78 | 12 | 87 |
| ALL | 486 | 469 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.995x | 1.292x | 35/36 | 0.949x |
| chess960_materialize | 0.960x | 1.454x | 12/12 | 1.193x |
| chess960_perft_d3 | 1.038x | 1.546x | 12/12 | 1.350x |
| corpus_count | 1.149x | 1.364x | 12/12 | 1.108x |
| corpus_materialize | 0.994x | 1.134x | 12/12 | 1.019x |
| corpus_uci_sorted | 1.026x | 1.524x | 12/12 | 1.429x |
| count | 1.043x | 1.542x | 36/36 | 1.072x |
| fen_parse | 0.994x | 1.170x | 35/36 | 0.989x |
| fen_write_native_ep | 0.993x | 1.791x | 36/36 | 1.281x |
| make_unmake_full | 0.992x | 1.289x | 36/36 | 1.049x |
| materialize | 1.010x | 1.429x | 34/36 | 0.991x |
| perft_bulk_d4 | 1.060x | 1.338x | 34/36 | 0.947x |
| perft_full_d3 | 1.143x | 1.528x | 36/36 | 1.064x |
| polyglot_key | 0.992x | 35.332x | 12/12 | 16.321x |
| polyglot_recompute | 0.961x | 0.984x | 6/12 | 0.846x |
| repetition_key | 1.026x | 21.348x | 12/12 | 14.205x |
| replay_checked_native | 1.004x | 1.152x | 12/12 | 1.073x |
| replay_trusted_native | 0.995x | 0.784x | 4/12 | 0.510x |
| san_parse_all | 1.008x | 1.920x | 36/36 | 1.572x |
| san_render_native | 0.984x | 2.580x | 36/36 | 1.841x |
| snapshot | 0.953x | 0.980x | 20/36 | 0.380x |
| startpos | 1.016x | 1.331x | 6/6 | 1.112x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | kiwipete | ferrum | 0.380x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.510x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| macos-15-native | snapshot | endgame | ferrum | 0.590x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.678x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.689x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.695x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.710x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.713x |
| macos-15-native | snapshot | promotion | ferrum | 0.728x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.732x |
| macos-15-native | snapshot | tactical | ferrum | 0.752x |
| macos-15-native | snapshot | start | chess | 0.804x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.846x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.905x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.927x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.946x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.947x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.949x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.949x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.951x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.962x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.970x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.984x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.988x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.989x |
| macos-15-native | materialize | kiwipete | ultra | 0.991x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.992x |
| macos-15-native | materialize | middlegame | cozy | 0.993x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6875 | shakmaty | 12917 | 1.879x |
| ubuntu-24.04-arm-native | 6159 | shakmaty | 9392 | 1.525x |
| ubuntu-24.04-native | 5044 | shakmaty | 6657 | 1.320x |
| ubuntu-24.04-pext | 4760 | shakmaty | 7622 | 1.601x |
| ubuntu-24.04-portable | 5949 | shakmaty | 6056 | 1.018x |
| windows-2025-native | 14300 | shakmaty | 19800 | 1.385x |

Cold wins: 6/6 (separate from native cases).
