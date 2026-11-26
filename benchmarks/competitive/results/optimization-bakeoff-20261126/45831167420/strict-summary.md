| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 73 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 12 | 87 |
| ubuntu-24.04-native | 80 | 79 | 16 | 87 |
| ubuntu-24.04-pext | 84 | 82 | 13 | 87 |
| ubuntu-24.04-portable | 85 | 79 | 12 | 87 |
| windows-2025-native | 80 | 78 | 12 | 87 |
| ALL | 482 | 468 | 77 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.001x | 1.262x | 36/36 | 1.031x |
| chess960_materialize | 1.074x | 1.474x | 12/12 | 1.266x |
| chess960_perft_d3 | 1.091x | 1.513x | 12/12 | 1.257x |
| corpus_count | 1.121x | 1.248x | 10/12 | 0.978x |
| corpus_materialize | 1.062x | 1.179x | 12/12 | 1.106x |
| corpus_uci_sorted | 0.999x | 1.529x | 12/12 | 1.358x |
| count | 1.121x | 1.474x | 36/36 | 1.116x |
| fen_parse | 1.019x | 1.226x | 36/36 | 1.060x |
| fen_write_native_ep | 1.005x | 1.736x | 36/36 | 1.434x |
| make_unmake_full | 1.003x | 1.347x | 36/36 | 1.059x |
| materialize | 1.065x | 1.259x | 33/36 | 0.914x |
| perft_bulk_d4 | 1.116x | 1.301x | 34/36 | 0.959x |
| perft_full_d3 | 0.988x | 1.200x | 30/36 | 0.735x |
| polyglot_key | 0.997x | 27.967x | 12/12 | 16.354x |
| polyglot_recompute | 1.000x | 1.039x | 9/12 | 0.888x |
| repetition_key | 1.019x | 20.132x | 12/12 | 14.803x |
| replay_checked_native | 0.982x | 1.047x | 10/12 | 0.880x |
| replay_trusted_native | 0.996x | 0.810x | 4/12 | 0.531x |
| san_parse_all | 1.006x | 1.966x | 36/36 | 1.406x |
| san_render_native | 0.997x | 2.580x | 36/36 | 1.687x |
| snapshot | 1.007x | 1.049x | 22/36 | 0.929x |
| startpos | 1.040x | 1.204x | 6/6 | 1.001x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.531x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.573x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.657x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.688x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.720x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.721x |
| macos-15-native | perft_full_d3 | kiwipete | ultra | 0.735x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.799x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.880x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.888x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.891x |
| macos-15-native | materialize | kiwipete | ultra | 0.914x |
| windows-2025-native | snapshot | start | chess | 0.929x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.930x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.931x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.940x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.945x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.950x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.959x |
| macos-15-native | materialize | tactical | ultra | 0.963x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.970x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.971x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.973x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.978x |
| windows-2025-native | snapshot | promotion | cozy | 0.978x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.978x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.979x |
| macos-15-native | materialize | middlegame | ultra | 0.981x |
| windows-2025-native | snapshot | middlegame | cozy | 0.987x |
| windows-2025-native | snapshot | endgame | cozy | 0.988x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.989x |
| macos-15-native | snapshot | middlegame | ferrum | 0.991x |
| macos-15-native | snapshot | endgame | ferrum | 0.994x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | start | chess | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4542 | shakmaty | 7375 | 1.624x |
| ubuntu-24.04-arm-native | 6807 | shakmaty | 8272 | 1.215x |
| ubuntu-24.04-native | 6620 | shakmaty | 7982 | 1.206x |
| ubuntu-24.04-pext | 7128 | shakmaty | 10555 | 1.481x |
| ubuntu-24.04-portable | 7784 | shakmaty | 10430 | 1.340x |
| windows-2025-native | 13300 | shakmaty | 20300 | 1.526x |

Cold wins: 6/6 (separate from native cases).
