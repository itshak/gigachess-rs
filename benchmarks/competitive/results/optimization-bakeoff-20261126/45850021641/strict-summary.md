| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 70 | 13 | 87 |
| ubuntu-24.04-arm-native | 76 | 75 | 12 | 87 |
| ubuntu-24.04-native | 81 | 78 | 9 | 87 |
| ubuntu-24.04-pext | 86 | 84 | 12 | 87 |
| ubuntu-24.04-portable | 83 | 78 | 11 | 87 |
| windows-2025-native | 83 | 78 | 13 | 87 |
| ALL | 482 | 463 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.012x | 1.238x | 32/36 | 0.801x |
| chess960_materialize | 1.004x | 1.476x | 12/12 | 1.291x |
| chess960_perft_d3 | 0.974x | 1.547x | 12/12 | 1.428x |
| corpus_count | 0.965x | 1.259x | 10/12 | 0.974x |
| corpus_materialize | 0.952x | 1.173x | 12/12 | 1.031x |
| corpus_uci_sorted | 0.974x | 1.537x | 12/12 | 1.114x |
| count | 0.979x | 1.488x | 36/36 | 1.109x |
| fen_parse | 0.993x | 1.191x | 34/36 | 0.972x |
| fen_write_native_ep | 0.977x | 1.665x | 36/36 | 1.244x |
| make_unmake_full | 0.990x | 1.347x | 36/36 | 1.019x |
| materialize | 0.969x | 1.326x | 35/36 | 0.808x |
| perft_bulk_d4 | 0.988x | 1.334x | 34/36 | 0.909x |
| perft_full_d3 | 0.986x | 1.299x | 36/36 | 1.165x |
| polyglot_key | 0.963x | 32.122x | 12/12 | 16.868x |
| polyglot_recompute | 0.951x | 1.022x | 8/12 | 0.824x |
| repetition_key | 0.973x | 21.940x | 12/12 | 14.873x |
| replay_checked_native | 0.989x | 1.065x | 11/12 | 0.981x |
| replay_trusted_native | 1.010x | 0.822x | 4/12 | 0.551x |
| san_parse_all | 0.998x | 1.912x | 36/36 | 1.206x |
| san_render_native | 1.015x | 2.444x | 36/36 | 1.503x |
| snapshot | 1.011x | 1.033x | 21/36 | 0.502x |
| startpos | 0.964x | 1.102x | 5/6 | 0.574x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | endgame | ferrum | 0.502x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.551x |
| macos-15-native | startpos | start | shakmaty | 0.574x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.678x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.707x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.727x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.755x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.767x |
| macos-15-native | apply_children_full | promotion | cozy | 0.801x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.805x |
| macos-15-native | materialize | kiwipete | ultra | 0.808x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.822x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.824x |
| macos-15-native | snapshot | promotion | chess | 0.825x |
| macos-15-native | apply_children_full | tactical | chess | 0.874x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.909x |
| macos-15-native | snapshot | middlegame | cozy | 0.934x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | chess | 0.937x |
| ubuntu-24.04-arm-native | snapshot | endgame | cozy | 0.945x |
| ubuntu-24.04-portable | snapshot | promotion | chess | 0.949x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.955x |
| macos-15-native | snapshot | start | ferrum | 0.957x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.971x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.972x |
| macos-15-native | corpus_count | seeded | ultra | 0.974x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.974x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.978x |
| ubuntu-24.04-arm-native | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.981x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.982x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | chess | 0.982x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.985x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.987x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | kiwipete | chess | 0.997x |
| macos-15-native | snapshot | tactical | ferrum | 0.997x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.997x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.998x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.999x |
| windows-2025-native | snapshot | tactical | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5792 | shakmaty | 9667 | 1.669x |
| ubuntu-24.04-arm-native | 3624 | shakmaty | 4016 | 1.108x |
| ubuntu-24.04-native | 5423 | shakmaty | 7188 | 1.325x |
| ubuntu-24.04-pext | 9908 | shakmaty | 11321 | 1.143x |
| ubuntu-24.04-portable | 9107 | shakmaty | 12764 | 1.402x |
| windows-2025-native | 18300 | shakmaty | 22800 | 1.246x |

Cold wins: 6/6 (separate from native cases).
