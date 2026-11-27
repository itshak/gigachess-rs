| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 81 | 78 | 13 | 87 |
| ubuntu-24.04-arm-native | 77 | 75 | 13 | 87 |
| ubuntu-24.04-native | 86 | 85 | 13 | 87 |
| ubuntu-24.04-pext | 84 | 81 | 15 | 87 |
| ubuntu-24.04-portable | 85 | 81 | 12 | 87 |
| windows-2025-native | 80 | 78 | 13 | 87 |
| ALL | 493 | 478 | 79 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.019x | 1.269x | 34/36 | 0.941x |
| chess960_materialize | 1.011x | 1.447x | 12/12 | 1.252x |
| chess960_perft_d3 | 1.043x | 1.585x | 12/12 | 1.363x |
| corpus_count | 1.109x | 1.401x | 12/12 | 1.118x |
| corpus_materialize | 1.008x | 1.180x | 12/12 | 1.041x |
| corpus_uci_sorted | 1.029x | 1.584x | 12/12 | 1.309x |
| count | 1.059x | 1.519x | 36/36 | 1.055x |
| fen_parse | 1.027x | 1.193x | 36/36 | 1.008x |
| fen_write_native_ep | 1.001x | 1.703x | 36/36 | 1.330x |
| make_unmake_full | 1.001x | 1.351x | 36/36 | 1.041x |
| materialize | 1.006x | 1.337x | 36/36 | 1.002x |
| perft_bulk_d4 | 1.056x | 1.384x | 34/36 | 0.893x |
| perft_full_d3 | 1.199x | 1.523x | 36/36 | 1.253x |
| polyglot_key | 1.008x | 31.557x | 12/12 | 16.965x |
| polyglot_recompute | 1.002x | 1.026x | 8/12 | 0.939x |
| repetition_key | 0.962x | 20.389x | 12/12 | 14.756x |
| replay_checked_native | 1.005x | 1.169x | 12/12 | 1.032x |
| replay_trusted_native | 1.006x | 0.805x | 4/12 | 0.541x |
| san_parse_all | 1.022x | 1.984x | 36/36 | 1.368x |
| san_render_native | 1.016x | 2.497x | 36/36 | 1.507x |
| snapshot | 1.053x | 1.066x | 23/36 | 0.835x |
| startpos | 1.062x | 1.214x | 6/6 | 1.053x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.541x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.550x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.598x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.707x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.715x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.729x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.824x |
| windows-2025-native | snapshot | kiwipete | chess | 0.835x |
| macos-15-native | snapshot | endgame | ferrum | 0.885x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.893x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.939x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | chess | 0.941x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.943x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.955x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.971x |
| windows-2025-native | snapshot | tactical | ferrum | 0.972x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.977x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.982x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.984x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.984x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | ferrum | 0.986x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.988x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.991x |
| macos-15-native | snapshot | middlegame | ferrum | 0.991x |
| windows-2025-native | snapshot | start | cozy | 0.991x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.992x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.993x |
| windows-2025-native | snapshot | endgame | chess | 0.995x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4166 | shakmaty | 11459 | 2.751x |
| ubuntu-24.04-arm-native | 5504 | ferrum | 5780 | 1.050x |
| ubuntu-24.04-native | 7384 | shakmaty | 11822 | 1.601x |
| ubuntu-24.04-pext | 5392 | cozy | 9037 | 1.676x |
| ubuntu-24.04-portable | 7293 | shakmaty | 8466 | 1.161x |
| windows-2025-native | 19100 | shakmaty | 19100 | 1.000x |

Cold wins: 5/6 (separate from native cases).
