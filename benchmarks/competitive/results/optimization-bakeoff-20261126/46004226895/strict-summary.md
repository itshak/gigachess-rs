| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 73 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 80 | 10 | 87 |
| ubuntu-24.04-pext | 79 | 74 | 14 | 87 |
| ubuntu-24.04-portable | 82 | 77 | 12 | 87 |
| windows-2025-native | 85 | 83 | 12 | 87 |
| ALL | 483 | 466 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.012x | 1.262x | 34/36 | 0.933x |
| chess960_materialize | 1.015x | 1.491x | 12/12 | 1.309x |
| chess960_perft_d3 | 1.046x | 1.518x | 12/12 | 1.303x |
| corpus_count | 1.178x | 1.355x | 12/12 | 1.033x |
| corpus_materialize | 1.032x | 1.155x | 12/12 | 1.049x |
| corpus_uci_sorted | 1.008x | 1.523x | 12/12 | 1.323x |
| count | 1.047x | 1.543x | 36/36 | 1.050x |
| fen_parse | 0.980x | 1.168x | 35/36 | 0.937x |
| fen_write_native_ep | 0.992x | 1.776x | 36/36 | 1.415x |
| make_unmake_full | 1.001x | 1.272x | 36/36 | 1.049x |
| materialize | 1.025x | 1.406x | 35/36 | 0.849x |
| perft_bulk_d4 | 1.084x | 1.338x | 34/36 | 0.953x |
| perft_full_d3 | 1.248x | 1.618x | 36/36 | 1.297x |
| polyglot_key | 1.057x | 32.623x | 12/12 | 16.185x |
| polyglot_recompute | 1.024x | 1.001x | 5/12 | 0.907x |
| repetition_key | 1.028x | 19.977x | 12/12 | 14.498x |
| replay_checked_native | 1.008x | 1.154x | 12/12 | 1.076x |
| replay_trusted_native | 1.073x | 0.823x | 4/12 | 0.553x |
| san_parse_all | 1.002x | 1.833x | 36/36 | 1.176x |
| san_render_native | 0.997x | 2.496x | 36/36 | 1.728x |
| snapshot | 1.009x | 1.033x | 19/36 | 0.728x |
| startpos | 0.997x | 1.123x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.553x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.584x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.689x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.709x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.717x |
| macos-15-native | snapshot | endgame | ferrum | 0.728x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.735x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.751x |
| macos-15-native | snapshot | promotion | ferrum | 0.773x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.841x |
| macos-15-native | materialize | kiwipete | ultra | 0.849x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.858x |
| macos-15-native | snapshot | middlegame | ferrum | 0.890x |
| macos-15-native | snapshot | start | ferrum | 0.900x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.907x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.916x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.933x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.936x |
| macos-15-native | fen_parse | kiwipete | ultra | 0.937x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.939x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.947x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.953x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.954x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.971x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.993x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.994x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.998x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.999x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 1.000x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5833 | shakmaty | 9167 | 1.572x |
| ubuntu-24.04-arm-native | 6440 | chess | 9263 | 1.438x |
| ubuntu-24.04-native | 5346 | chess | 8340 | 1.560x |
| ubuntu-24.04-pext | 5676 | cozy | 8650 | 1.524x |
| ubuntu-24.04-portable | 5511 | shakmaty | 7232 | 1.312x |
| windows-2025-native | 12800 | shakmaty | 16500 | 1.289x |

Cold wins: 6/6 (separate from native cases).
