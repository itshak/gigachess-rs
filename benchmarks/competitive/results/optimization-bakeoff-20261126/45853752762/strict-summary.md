| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 70 | 12 | 87 |
| ubuntu-24.04-arm-native | 74 | 74 | 12 | 87 |
| ubuntu-24.04-native | 79 | 71 | 12 | 87 |
| ubuntu-24.04-pext | 78 | 74 | 15 | 87 |
| ubuntu-24.04-portable | 83 | 77 | 11 | 87 |
| windows-2025-native | 76 | 75 | 13 | 87 |
| ALL | 464 | 441 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.901x | 1.136x | 28/36 | 0.852x |
| chess960_materialize | 1.005x | 1.474x | 12/12 | 1.276x |
| chess960_perft_d3 | 0.998x | 1.518x | 12/12 | 1.344x |
| corpus_count | 0.993x | 1.228x | 9/12 | 0.941x |
| corpus_materialize | 0.991x | 1.167x | 12/12 | 1.019x |
| corpus_uci_sorted | 0.982x | 1.487x | 12/12 | 1.293x |
| count | 0.999x | 1.484x | 36/36 | 1.024x |
| fen_parse | 0.976x | 1.194x | 34/36 | 0.964x |
| fen_write_native_ep | 0.984x | 1.797x | 36/36 | 1.447x |
| make_unmake_full | 0.929x | 1.212x | 33/36 | 0.989x |
| materialize | 0.999x | 1.325x | 32/36 | 0.878x |
| perft_bulk_d4 | 0.991x | 1.281x | 34/36 | 0.844x |
| perft_full_d3 | 0.987x | 1.307x | 36/36 | 1.101x |
| polyglot_key | 0.997x | 31.666x | 12/12 | 15.896x |
| polyglot_recompute | 0.988x | 1.031x | 8/12 | 0.895x |
| repetition_key | 0.948x | 20.944x | 12/12 | 14.184x |
| replay_checked_native | 0.982x | 1.051x | 8/12 | 0.950x |
| replay_trusted_native | 0.952x | 0.773x | 2/12 | 0.527x |
| san_parse_all | 0.996x | 1.900x | 36/36 | 1.376x |
| san_render_native | 0.985x | 2.476x | 36/36 | 1.732x |
| snapshot | 0.984x | 1.016x | 19/36 | 0.843x |
| startpos | 0.976x | 1.229x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.527x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.631x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.657x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.698x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.755x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.772x |
| windows-2025-native | snapshot | promotion | cozy | 0.843x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.844x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.852x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.867x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.868x |
| macos-15-native | materialize | kiwipete | ultra | 0.878x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.895x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | chess | 0.908x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.918x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.926x |
| macos-15-native | snapshot | endgame | ferrum | 0.931x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.941x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.950x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.954x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.954x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.958x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.958x |
| macos-15-native | snapshot | promotion | ferrum | 0.959x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.960x |
| windows-2025-native | materialize | middlegame | cozy | 0.960x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.964x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.964x |
| macos-15-native | snapshot | start | ferrum | 0.966x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.967x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.969x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| macos-15-native | materialize | tactical | ultra | 0.970x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.971x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.976x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.980x |
| macos-15-native | snapshot | middlegame | ferrum | 0.981x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.982x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.983x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.985x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.986x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.986x |
| ubuntu-24.04-arm-native | apply_children_full | middlegame | ferrum | 0.988x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.989x |
| windows-2025-native | snapshot | tactical | cozy | 0.990x |
| macos-15-native | materialize | middlegame | ultra | 0.990x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.992x |
| ubuntu-24.04-portable | make_unmake_full | promotion | ultra | 0.995x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.997x |
| windows-2025-native | snapshot | middlegame | chess | 0.998x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.999x |
| windows-2025-native | snapshot | endgame | chess | 0.999x |
| ubuntu-24.04-native | startpos | start | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 2750 | shakmaty | 5833 | 2.121x |
| ubuntu-24.04-arm-native | 6232 | cozy | 8831 | 1.417x |
| ubuntu-24.04-native | 6565 | chess | 9436 | 1.437x |
| ubuntu-24.04-pext | 6269 | shakmaty | 9504 | 1.516x |
| ubuntu-24.04-portable | 5409 | shakmaty | 7049 | 1.303x |
| windows-2025-native | 20600 | shakmaty | 29000 | 1.408x |

Cold wins: 6/6 (separate from native cases).
