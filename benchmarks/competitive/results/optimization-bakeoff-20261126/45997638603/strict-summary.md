| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 73 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 80 | 76 | 16 | 87 |
| ubuntu-24.04-pext | 84 | 84 | 16 | 87 |
| ubuntu-24.04-portable | 82 | 79 | 12 | 87 |
| windows-2025-native | 85 | 85 | 12 | 87 |
| ALL | 484 | 476 | 83 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.995x | 1.293x | 35/36 | 0.823x |
| chess960_materialize | 0.988x | 1.466x | 12/12 | 1.306x |
| chess960_perft_d3 | 0.996x | 1.502x | 12/12 | 1.040x |
| corpus_count | 1.132x | 1.315x | 12/12 | 1.020x |
| corpus_materialize | 0.991x | 1.163x | 12/12 | 1.055x |
| corpus_uci_sorted | 0.966x | 1.503x | 12/12 | 1.376x |
| count | 1.058x | 1.540x | 36/36 | 1.077x |
| fen_parse | 1.040x | 1.257x | 35/36 | 0.979x |
| fen_write_native_ep | 1.012x | 1.816x | 36/36 | 1.385x |
| make_unmake_full | 1.011x | 1.295x | 36/36 | 1.055x |
| materialize | 1.020x | 1.365x | 34/36 | 0.762x |
| perft_bulk_d4 | 1.058x | 1.320x | 33/36 | 0.903x |
| perft_full_d3 | 1.206x | 1.601x | 36/36 | 1.177x |
| polyglot_key | 0.990x | 33.584x | 12/12 | 16.496x |
| polyglot_recompute | 0.986x | 1.004x | 7/12 | 0.803x |
| repetition_key | 0.991x | 22.088x | 12/12 | 14.648x |
| replay_checked_native | 0.984x | 1.158x | 11/12 | 0.877x |
| replay_trusted_native | 0.956x | 0.813x | 4/12 | 0.583x |
| san_parse_all | 0.998x | 1.770x | 36/36 | 1.391x |
| san_render_native | 1.003x | 2.529x | 36/36 | 1.855x |
| snapshot | 1.001x | 1.031x | 20/36 | 0.490x |
| startpos | 0.964x | 1.197x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | tactical | cozy | 0.490x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.656x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.658x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.667x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.701x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.710x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.759x |
| macos-15-native | materialize | kiwipete | ultra | 0.762x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.803x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.823x |
| macos-15-native | materialize | middlegame | ferrum | 0.851x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.877x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.877x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.903x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.917x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.918x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.943x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.943x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.952x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| macos-15-native | snapshot | middlegame | ferrum | 0.979x |
| macos-15-native | fen_parse | start | shakmaty | 0.979x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.986x |
| macos-15-native | snapshot | start | ferrum | 0.987x |
| macos-15-native | snapshot | endgame | ferrum | 0.993x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.994x |
| ubuntu-24.04-native | snapshot | start | chess | 0.995x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.999x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |
| ubuntu-24.04-native | startpos | start | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6833 | shakmaty | 9625 | 1.409x |
| ubuntu-24.04-arm-native | 5840 | shakmaty | 8655 | 1.482x |
| ubuntu-24.04-native | 6140 | shakmaty | 8241 | 1.342x |
| ubuntu-24.04-pext | 6935 | cozy | 9962 | 1.436x |
| ubuntu-24.04-portable | 5342 | shakmaty | 7118 | 1.332x |
| windows-2025-native | 18200 | shakmaty | 22800 | 1.253x |

Cold wins: 6/6 (separate from native cases).
