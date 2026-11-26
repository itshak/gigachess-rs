| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 67 | 13 | 87 |
| ubuntu-24.04-arm-native | 74 | 71 | 11 | 87 |
| ubuntu-24.04-native | 82 | 81 | 14 | 87 |
| ubuntu-24.04-pext | 78 | 77 | 13 | 87 |
| ubuntu-24.04-portable | 78 | 75 | 14 | 87 |
| windows-2025-native | 78 | 74 | 11 | 87 |
| ALL | 457 | 445 | 76 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.967x | 1.098x | 27/36 | 0.794x |
| chess960_materialize | 0.986x | 1.407x | 12/12 | 1.172x |
| chess960_perft_d3 | 0.965x | 1.411x | 12/12 | 1.267x |
| corpus_count | 1.005x | 1.366x | 12/12 | 1.156x |
| corpus_materialize | 1.008x | 1.182x | 12/12 | 1.064x |
| corpus_uci_sorted | 0.998x | 1.554x | 12/12 | 1.420x |
| count | 1.012x | 1.343x | 34/36 | 0.977x |
| fen_parse | 1.030x | 1.228x | 36/36 | 1.051x |
| fen_write_native_ep | 1.017x | 1.832x | 36/36 | 1.420x |
| make_unmake_full | 0.952x | 1.162x | 32/36 | 0.923x |
| materialize | 1.029x | 1.271x | 33/36 | 0.878x |
| perft_bulk_d4 | 1.011x | 1.275x | 36/36 | 1.052x |
| perft_full_d3 | 1.001x | 1.225x | 29/36 | 0.867x |
| polyglot_key | 1.013x | 29.754x | 12/12 | 16.287x |
| polyglot_recompute | 0.977x | 0.989x | 6/12 | 0.842x |
| repetition_key | 0.760x | 14.525x | 12/12 | 12.559x |
| replay_checked_native | 0.977x | 0.984x | 4/12 | 0.876x |
| replay_trusted_native | 0.939x | 0.668x | 2/12 | 0.468x |
| san_parse_all | 1.030x | 1.892x | 36/36 | 1.448x |
| san_render_native | 1.029x | 2.498x | 36/36 | 1.658x |
| snapshot | 0.937x | 1.058x | 20/36 | 0.806x |
| startpos | 0.993x | 1.222x | 6/6 | 1.080x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.468x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.473x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.482x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.582x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.595x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.598x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.611x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.615x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.794x |
| macos-15-native | snapshot | start | ferrum | 0.806x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.842x |
| macos-15-native | apply_children_full | tactical | chess | 0.843x |
| macos-15-native | snapshot | middlegame | cozy | 0.863x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.867x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.873x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.876x |
| macos-15-native | materialize | kiwipete | ultra | 0.878x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.891x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.893x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.904x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.904x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.907x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.913x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.914x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.917x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.919x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.923x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.939x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.943x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.943x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.944x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.949x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.949x |
| macos-15-native | make_unmake_full | middlegame | ultra | 0.959x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.961x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.961x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.962x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.962x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.962x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.964x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.966x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.968x |
| macos-15-native | materialize | middlegame | ultra | 0.970x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.972x |
| macos-15-native | snapshot | endgame | ferrum | 0.973x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.974x |
| windows-2025-native | materialize | kiwipete | ultra | 0.975x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.977x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.977x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.979x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.982x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.982x |
| ubuntu-24.04-portable | count | start | ferrum | 0.983x |
| macos-15-native | snapshot | promotion | ferrum | 0.983x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.984x |
| macos-15-native | snapshot | tactical | ferrum | 0.988x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | kiwipete | chess | 0.994x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.995x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.996x |
| ubuntu-24.04-portable | snapshot | promotion | chess | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | endgame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 13875 | shakmaty | 11209 | 0.808x |
| ubuntu-24.04-arm-native | 10680 | shakmaty | 8256 | 0.773x |
| ubuntu-24.04-native | 12006 | shakmaty | 8917 | 0.743x |
| ubuntu-24.04-pext | 10666 | cozy | 9544 | 0.895x |
| ubuntu-24.04-portable | 11129 | shakmaty | 7942 | 0.714x |
| windows-2025-native | 26300 | shakmaty | 27900 | 1.061x |

Cold wins: 1/6 (separate from native cases).
