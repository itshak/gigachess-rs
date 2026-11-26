| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 69 | 11 | 87 |
| ubuntu-24.04-arm-native | 75 | 70 | 10 | 87 |
| ubuntu-24.04-native | 85 | 83 | 8 | 87 |
| ubuntu-24.04-pext | 78 | 72 | 9 | 87 |
| ubuntu-24.04-portable | 73 | 71 | 14 | 87 |
| windows-2025-native | 72 | 70 | 11 | 87 |
| ALL | 456 | 435 | 63 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.987x | 1.132x | 28/36 | 0.816x |
| chess960_materialize | 0.990x | 1.370x | 12/12 | 1.195x |
| chess960_perft_d3 | 0.987x | 1.389x | 12/12 | 1.147x |
| corpus_count | 0.972x | 1.300x | 11/12 | 0.945x |
| corpus_materialize | 1.018x | 1.169x | 12/12 | 1.073x |
| corpus_uci_sorted | 0.994x | 1.489x | 12/12 | 1.282x |
| count | 0.991x | 1.326x | 35/36 | 0.949x |
| fen_parse | 1.030x | 1.199x | 34/36 | 0.971x |
| fen_write_native_ep | 0.993x | 1.183x | 33/36 | 0.875x |
| make_unmake_full | 0.990x | 1.254x | 36/36 | 1.034x |
| materialize | 0.988x | 1.283x | 34/36 | 0.845x |
| perft_bulk_d4 | 0.991x | 1.280x | 36/36 | 1.086x |
| perft_full_d3 | 0.990x | 1.295x | 34/36 | 0.951x |
| polyglot_key | 1.026x | 31.614x | 12/12 | 16.530x |
| polyglot_recompute | 1.005x | 1.024x | 9/12 | 0.924x |
| repetition_key | 1.026x | 20.755x | 12/12 | 14.627x |
| replay_checked_native | 0.987x | 0.998x | 6/12 | 0.917x |
| replay_trusted_native | 1.003x | 0.633x | 0/12 | 0.475x |
| san_parse_all | 0.972x | 1.917x | 36/36 | 1.524x |
| san_render_native | 0.988x | 2.539x | 36/36 | 1.862x |
| snapshot | 0.981x | 0.967x | 10/36 | 0.814x |
| startpos | 0.951x | 1.233x | 6/6 | 1.138x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.475x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.482x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.482x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.498x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.573x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.632x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.635x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.693x |
| windows-2025-native | snapshot | promotion | ferrum | 0.814x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.816x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.822x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.826x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.830x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.835x |
| macos-15-native | materialize | tactical | ultra | 0.845x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.847x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.857x |
| windows-2025-native | snapshot | start | ferrum | 0.858x |
| windows-2025-native | snapshot | tactical | ferrum | 0.859x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.860x |
| windows-2025-native | snapshot | endgame | ferrum | 0.861x |
| windows-2025-native | fen_write_native_ep | middlegame | ultra | 0.875x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.894x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.903x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.906x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.909x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.915x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.916x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.917x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.917x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.918x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.923x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.924x |
| windows-2025-native | fen_write_native_ep | kiwipete | ultra | 0.924x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.928x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.929x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.929x |
| macos-15-native | materialize | middlegame | ultra | 0.930x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.932x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.936x |
| macos-15-native | snapshot | promotion | ferrum | 0.937x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.938x |
| macos-15-native | snapshot | start | ferrum | 0.939x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.943x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.945x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.946x |
| ubuntu-24.04-portable | count | start | ferrum | 0.949x |
| macos-15-native | apply_children_full | tactical | chess | 0.950x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.951x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.955x |
| macos-15-native | snapshot | middlegame | ferrum | 0.957x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.967x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.971x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.979x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.979x |
| ubuntu-24.04-portable | apply_children_full | promotion | ferrum | 0.981x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.981x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.985x |
| macos-15-native | snapshot | tactical | ferrum | 0.987x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.990x |
| ubuntu-24.04-pext | snapshot | endgame | ferrum | 0.990x |
| ubuntu-24.04-pext | snapshot | promotion | ferrum | 0.990x |
| ubuntu-24.04-pext | fen_parse | endgame | ultra | 0.993x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.995x |
| ubuntu-24.04-pext | snapshot | middlegame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7792 | shakmaty | 6875 | 0.882x |
| ubuntu-24.04-arm-native | 10952 | shakmaty | 8712 | 0.795x |
| ubuntu-24.04-native | 9757 | shakmaty | 6959 | 0.713x |
| ubuntu-24.04-pext | 9630 | shakmaty | 7044 | 0.731x |
| ubuntu-24.04-portable | 11317 | shakmaty | 8332 | 0.736x |
| windows-2025-native | 25700 | shakmaty | 22500 | 0.875x |

Cold wins: 0/6 (separate from native cases).
