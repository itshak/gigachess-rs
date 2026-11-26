| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 68 | 10 | 87 |
| ubuntu-24.04-arm-native | 72 | 67 | 10 | 87 |
| ubuntu-24.04-native | 82 | 82 | 8 | 87 |
| ubuntu-24.04-pext | 75 | 72 | 11 | 87 |
| ubuntu-24.04-portable | 74 | 71 | 13 | 87 |
| windows-2025-native | 78 | 75 | 13 | 87 |
| ALL | 452 | 435 | 65 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.977x | 1.090x | 26/36 | 0.907x |
| chess960_materialize | 1.030x | 1.341x | 12/12 | 1.188x |
| chess960_perft_d3 | 1.003x | 1.397x | 12/12 | 1.182x |
| corpus_count | 1.031x | 1.359x | 12/12 | 1.170x |
| corpus_materialize | 0.996x | 1.180x | 11/12 | 0.823x |
| corpus_uci_sorted | 0.998x | 1.542x | 12/12 | 1.228x |
| count | 0.999x | 1.333x | 35/36 | 0.949x |
| fen_parse | 0.964x | 1.154x | 34/36 | 0.966x |
| fen_write_native_ep | 0.960x | 1.093x | 24/36 | 0.869x |
| make_unmake_full | 0.977x | 1.229x | 33/36 | 0.904x |
| materialize | 0.981x | 1.212x | 33/36 | 0.756x |
| perft_bulk_d4 | 0.986x | 1.284x | 36/36 | 1.002x |
| perft_full_d3 | 0.992x | 1.237x | 34/36 | 0.913x |
| polyglot_key | 0.974x | 28.499x | 12/12 | 15.361x |
| polyglot_recompute | 1.007x | 1.024x | 8/12 | 0.897x |
| repetition_key | 1.015x | 20.320x | 12/12 | 13.815x |
| replay_checked_native | 1.022x | 1.012x | 8/12 | 0.763x |
| replay_trusted_native | 1.009x | 0.657x | 0/12 | 0.463x |
| san_parse_all | 1.013x | 2.004x | 36/36 | 1.275x |
| san_render_native | 0.981x | 2.522x | 36/36 | 1.786x |
| snapshot | 0.878x | 1.028x | 20/36 | 0.751x |
| startpos | 0.967x | 1.365x | 6/6 | 1.073x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.463x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.506x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.538x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.549x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.603x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.613x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.637x |
| macos-15-native | snapshot | endgame | ferrum | 0.751x |
| macos-15-native | materialize | tactical | ultra | 0.756x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.763x |
| macos-15-native | snapshot | tactical | ferrum | 0.766x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.792x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.813x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.814x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.818x |
| macos-15-native | corpus_materialize | lichess300 | cozy | 0.823x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.867x |
| ubuntu-24.04-pext | fen_write_native_ep | middlegame | ultra | 0.869x |
| ubuntu-24.04-pext | fen_write_native_ep | kiwipete | ultra | 0.872x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.873x |
| windows-2025-native | fen_write_native_ep | kiwipete | ultra | 0.874x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.894x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.894x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.895x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.896x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.896x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.896x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.897x |
| macos-15-native | fen_write_native_ep | start | ultra | 0.903x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.904x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.907x |
| ubuntu-24.04-pext | fen_write_native_ep | start | ultra | 0.910x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.913x |
| macos-15-native | snapshot | middlegame | chess | 0.915x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.916x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.919x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.921x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.925x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.927x |
| macos-15-native | materialize | middlegame | ultra | 0.928x |
| windows-2025-native | fen_write_native_ep | promotion | ultra | 0.934x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.939x |
| ubuntu-24.04-portable | apply_children_full | promotion | ferrum | 0.939x |
| ubuntu-24.04-pext | fen_write_native_ep | promotion | ultra | 0.947x |
| ubuntu-24.04-pext | fen_write_native_ep | tactical | ultra | 0.947x |
| ubuntu-24.04-portable | count | start | ferrum | 0.949x |
| windows-2025-native | fen_write_native_ep | middlegame | ultra | 0.952x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.955x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.957x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.960x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.962x |
| macos-15-native | apply_children_full | start | chess | 0.963x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.963x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.965x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.966x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.968x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| ubuntu-24.04-pext | fen_write_native_ep | endgame | ultra | 0.970x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.971x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.971x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.973x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.975x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.981x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.982x |
| windows-2025-native | materialize | kiwipete | ultra | 0.982x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.986x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | chess | 0.989x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-arm-native | fen_write_native_ep | middlegame | ultra | 0.993x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.995x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 14167 | shakmaty | 12375 | 0.874x |
| ubuntu-24.04-arm-native | 10528 | shakmaty | 9392 | 0.892x |
| ubuntu-24.04-native | 10329 | shakmaty | 6575 | 0.637x |
| ubuntu-24.04-pext | 14247 | shakmaty | 11762 | 0.826x |
| ubuntu-24.04-portable | 11077 | shakmaty | 9554 | 0.863x |
| windows-2025-native | 27300 | shakmaty | 26300 | 0.963x |

Cold wins: 0/6 (separate from native cases).
