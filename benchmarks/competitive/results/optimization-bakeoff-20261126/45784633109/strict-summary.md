| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 64 | 12 | 87 |
| ubuntu-24.04-arm-native | 80 | 75 | 11 | 87 |
| ubuntu-24.04-native | 77 | 74 | 11 | 87 |
| ubuntu-24.04-pext | 76 | 74 | 14 | 87 |
| ubuntu-24.04-portable | 77 | 75 | 13 | 87 |
| windows-2025-native | 70 | 69 | 11 | 87 |
| ALL | 449 | 431 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.996x | 1.093x | 24/36 | 0.864x |
| chess960_materialize | 1.019x | 1.358x | 12/12 | 1.087x |
| chess960_perft_d3 | 0.963x | 1.373x | 12/12 | 1.066x |
| corpus_count | 0.851x | 1.147x | 7/12 | 0.847x |
| corpus_materialize | 0.940x | 1.139x | 12/12 | 1.081x |
| corpus_uci_sorted | 0.993x | 1.580x | 12/12 | 1.363x |
| count | 1.000x | 1.326x | 36/36 | 1.029x |
| fen_parse | 1.054x | 1.242x | 36/36 | 1.021x |
| fen_write_native_ep | 1.026x | 1.697x | 36/36 | 1.244x |
| make_unmake_full | 1.009x | 1.261x | 36/36 | 1.053x |
| materialize | 1.020x | 1.176x | 32/36 | 0.849x |
| perft_bulk_d4 | 0.943x | 1.215x | 34/36 | 0.961x |
| perft_full_d3 | 0.995x | 1.143x | 29/36 | 0.841x |
| polyglot_key | 1.035x | 25.971x | 12/12 | 16.023x |
| polyglot_recompute | 0.981x | 1.015x | 8/12 | 0.870x |
| repetition_key | 0.979x | 18.401x | 12/12 | 13.458x |
| replay_checked_native | 1.000x | 0.985x | 6/12 | 0.897x |
| replay_trusted_native | 1.017x | 0.649x | 0/12 | 0.457x |
| san_parse_all | 0.993x | 1.959x | 36/36 | 1.521x |
| san_render_native | 1.016x | 2.535x | 36/36 | 1.648x |
| snapshot | 1.144x | 0.986x | 15/36 | 0.858x |
| startpos | 1.070x | 1.233x | 6/6 | 1.147x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.457x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.484x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.501x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.573x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.608x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.618x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.653x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.675x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.767x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.841x |
| macos-15-native | corpus_count | seeded | ultra | 0.847x |
| macos-15-native | materialize | kiwipete | ultra | 0.849x |
| macos-15-native | materialize | middlegame | ultra | 0.849x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.853x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.858x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.858x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.858x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.858x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.859x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.864x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.865x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.868x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.870x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.875x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.880x |
| macos-15-native | materialize | tactical | ultra | 0.888x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.888x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.892x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.897x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.898x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.901x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.902x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.904x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.907x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.909x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.912x |
| macos-15-native | snapshot | endgame | ferrum | 0.921x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.923x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.924x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.934x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.937x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.937x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.938x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.948x |
| ubuntu-24.04-arm-native | snapshot | middlegame | cozy | 0.950x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.951x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.957x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.960x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.961x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.965x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.968x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.968x |
| macos-15-native | snapshot | start | ferrum | 0.970x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.974x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.975x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.976x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| macos-15-native | snapshot | tactical | ferrum | 0.976x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.977x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.979x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.981x |
| windows-2025-native | snapshot | promotion | ferrum | 0.982x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.982x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.985x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.986x |
| windows-2025-native | materialize | kiwipete | ultra | 0.986x |
| windows-2025-native | snapshot | start | ferrum | 0.987x |
| windows-2025-native | snapshot | tactical | ferrum | 0.987x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.989x |
| ubuntu-24.04-portable | apply_children_full | promotion | ferrum | 0.989x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.994x |
| windows-2025-native | snapshot | endgame | ferrum | 0.994x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7916 | ferrum | 11333 | 1.432x |
| ubuntu-24.04-arm-native | 5095 | shakmaty | 8263 | 1.622x |
| ubuntu-24.04-native | 7665 | shakmaty | 8316 | 1.085x |
| ubuntu-24.04-pext | 5899 | cozy | 9414 | 1.596x |
| ubuntu-24.04-portable | 7695 | shakmaty | 8436 | 1.096x |
| windows-2025-native | 13600 | shakmaty | 22200 | 1.632x |

Cold wins: 6/6 (separate from native cases).
