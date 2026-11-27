| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 70 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 82 | 13 | 87 |
| ubuntu-24.04-pext | 86 | 85 | 13 | 87 |
| ubuntu-24.04-portable | 81 | 76 | 10 | 87 |
| windows-2025-native | 81 | 78 | 14 | 87 |
| ALL | 487 | 470 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.032x | 1.280x | 35/36 | 0.902x |
| chess960_materialize | 1.014x | 1.469x | 12/12 | 1.155x |
| chess960_perft_d3 | 1.022x | 1.506x | 12/12 | 1.331x |
| corpus_count | 1.118x | 1.351x | 12/12 | 1.099x |
| corpus_materialize | 1.020x | 1.167x | 12/12 | 1.043x |
| corpus_uci_sorted | 1.018x | 1.526x | 12/12 | 1.245x |
| count | 1.026x | 1.458x | 35/36 | 0.962x |
| fen_parse | 0.974x | 1.167x | 36/36 | 1.017x |
| fen_write_native_ep | 1.009x | 1.823x | 36/36 | 1.326x |
| make_unmake_full | 1.009x | 1.299x | 35/36 | 0.982x |
| materialize | 1.003x | 1.306x | 33/36 | 0.871x |
| perft_bulk_d4 | 1.046x | 1.318x | 33/36 | 0.847x |
| perft_full_d3 | 1.009x | 1.307x | 36/36 | 1.063x |
| polyglot_key | 1.039x | 30.996x | 12/12 | 16.182x |
| polyglot_recompute | 1.011x | 1.003x | 6/12 | 0.906x |
| repetition_key | 1.064x | 20.586x | 12/12 | 14.865x |
| replay_checked_native | 1.027x | 1.160x | 12/12 | 1.076x |
| replay_trusted_native | 1.009x | 0.825x | 5/12 | 0.514x |
| san_parse_all | 0.984x | 1.905x | 36/36 | 1.323x |
| san_render_native | 1.021x | 2.572x | 36/36 | 1.734x |
| snapshot | 1.049x | 1.063x | 23/36 | 0.871x |
| startpos | 1.001x | 1.220x | 6/6 | 1.117x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.514x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.669x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.763x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.847x |
| macos-15-native | materialize | kiwipete | ultra | 0.871x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.871x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.878x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.902x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.906x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.926x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.943x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.947x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.956x |
| macos-15-native | count | endgame | cozy | 0.962x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.970x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.978x |
| macos-15-native | materialize | endgame | ultra | 0.980x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.982x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.985x |
| windows-2025-native | materialize | endgame | ultra | 0.987x |
| windows-2025-native | snapshot | endgame | chess | 0.991x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.992x |
| windows-2025-native | snapshot | promotion | chess | 0.992x |
| windows-2025-native | snapshot | start | cozy | 0.996x |
| macos-15-native | snapshot | middlegame | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4500 | shakmaty | 8375 | 1.861x |
| ubuntu-24.04-arm-native | 5536 | shakmaty | 6743 | 1.218x |
| ubuntu-24.04-native | 6529 | shakmaty | 6476 | 0.992x |
| ubuntu-24.04-pext | 7831 | cozy | 12118 | 1.547x |
| ubuntu-24.04-portable | 4978 | chess | 8275 | 1.662x |
| windows-2025-native | 13900 | shakmaty | 21200 | 1.525x |

Cold wins: 5/6 (separate from native cases).
