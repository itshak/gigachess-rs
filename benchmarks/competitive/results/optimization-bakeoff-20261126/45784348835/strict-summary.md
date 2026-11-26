| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 68 | 65 | 14 | 87 |
| ubuntu-24.04-arm-native | 80 | 74 | 11 | 87 |
| ubuntu-24.04-native | 85 | 83 | 13 | 87 |
| ubuntu-24.04-pext | 79 | 75 | 15 | 87 |
| ubuntu-24.04-portable | 82 | 76 | 12 | 87 |
| windows-2025-native | 83 | 81 | 11 | 87 |
| ALL | 477 | 454 | 76 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.050x | 1.144x | 30/36 | 0.755x |
| chess960_materialize | 0.983x | 1.361x | 12/12 | 1.216x |
| chess960_perft_d3 | 0.961x | 1.338x | 12/12 | 1.019x |
| corpus_count | 1.015x | 1.418x | 12/12 | 1.209x |
| corpus_materialize | 1.042x | 1.170x | 12/12 | 1.009x |
| corpus_uci_sorted | 1.022x | 1.566x | 12/12 | 1.368x |
| count | 0.988x | 1.328x | 35/36 | 0.747x |
| fen_parse | 1.011x | 1.178x | 36/36 | 1.017x |
| fen_write_native_ep | 0.972x | 1.735x | 36/36 | 1.264x |
| make_unmake_full | 0.986x | 1.258x | 35/36 | 0.896x |
| materialize | 1.028x | 1.243x | 33/36 | 0.798x |
| perft_bulk_d4 | 0.966x | 1.265x | 35/36 | 0.899x |
| perft_full_d3 | 0.986x | 1.249x | 31/36 | 0.887x |
| polyglot_key | 1.045x | 29.273x | 12/12 | 15.853x |
| polyglot_recompute | 0.999x | 1.068x | 8/12 | 0.918x |
| repetition_key | 1.059x | 19.879x | 12/12 | 14.856x |
| replay_checked_native | 1.022x | 1.019x | 8/12 | 0.891x |
| replay_trusted_native | 1.063x | 0.690x | 2/12 | 0.494x |
| san_parse_all | 0.985x | 1.984x | 36/36 | 1.467x |
| san_render_native | 0.981x | 2.591x | 36/36 | 1.672x |
| snapshot | 0.939x | 1.020x | 27/36 | 0.824x |
| startpos | 0.756x | 0.988x | 5/6 | 0.483x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.483x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.494x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.506x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.549x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.552x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.619x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.636x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.637x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.640x |
| macos-15-native | count | promotion | cozy | 0.747x |
| macos-15-native | apply_children_full | tactical | chess | 0.755x |
| macos-15-native | materialize | middlegame | ultra | 0.798x |
| macos-15-native | snapshot | tactical | ferrum | 0.824x |
| macos-15-native | snapshot | middlegame | ferrum | 0.854x |
| macos-15-native | snapshot | promotion | ferrum | 0.856x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.867x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.867x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.869x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.887x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.891x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.894x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.896x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.899x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.903x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.907x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.910x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.918x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.921x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.924x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.960x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.963x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.964x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.969x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.978x |
| macos-15-native | snapshot | endgame | ferrum | 0.979x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.980x |
| macos-15-native | materialize | kiwipete | ultra | 0.983x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.985x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.986x |
| macos-15-native | snapshot | start | ferrum | 0.989x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.989x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.993x |
| macos-15-native | materialize | tactical | ultra | 0.998x |
| ubuntu-24.04-portable | snapshot | start | chess | 0.998x |
| ubuntu-24.04-portable | snapshot | middlegame | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 15041 | ferrum | 9708 | 0.645x |
| ubuntu-24.04-arm-native | 10544 | shakmaty | 8567 | 0.812x |
| ubuntu-24.04-native | 9105 | shakmaty | 5871 | 0.645x |
| ubuntu-24.04-pext | 11277 | shakmaty | 9344 | 0.829x |
| ubuntu-24.04-portable | 14898 | shakmaty | 10439 | 0.701x |
| windows-2025-native | 25800 | shakmaty | 22700 | 0.880x |

Cold wins: 0/6 (separate from native cases).
