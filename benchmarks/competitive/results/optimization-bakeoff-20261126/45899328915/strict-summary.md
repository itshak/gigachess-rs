| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 70 | 12 | 87 |
| ubuntu-24.04-arm-native | 81 | 78 | 12 | 87 |
| ubuntu-24.04-native | 79 | 75 | 11 | 87 |
| ubuntu-24.04-pext | 75 | 73 | 11 | 87 |
| ubuntu-24.04-portable | 79 | 73 | 11 | 87 |
| windows-2025-native | 78 | 76 | 11 | 87 |
| ALL | 463 | 445 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.862x | 1.213x | 30/36 | 0.476x |
| chess960_materialize | 0.911x | 1.412x | 12/12 | 1.231x |
| chess960_perft_d3 | 0.918x | 1.440x | 12/12 | 1.132x |
| corpus_count | 1.033x | 1.295x | 12/12 | 1.036x |
| corpus_materialize | 0.971x | 1.159x | 12/12 | 1.055x |
| corpus_uci_sorted | 0.987x | 1.517x | 12/12 | 1.348x |
| count | 0.924x | 1.381x | 35/36 | 0.950x |
| fen_parse | 0.983x | 1.190x | 35/36 | 0.979x |
| fen_write_native_ep | 0.920x | 1.644x | 36/36 | 1.133x |
| make_unmake_full | 0.944x | 1.231x | 35/36 | 0.936x |
| materialize | 0.979x | 1.282x | 33/36 | 0.928x |
| perft_bulk_d4 | 0.996x | 1.276x | 33/36 | 0.939x |
| perft_full_d3 | 0.995x | 1.304x | 35/36 | 0.990x |
| polyglot_key | 1.029x | 29.947x | 12/12 | 17.752x |
| polyglot_recompute | 0.980x | 1.011x | 7/12 | 0.896x |
| repetition_key | 1.005x | 20.051x | 12/12 | 15.898x |
| replay_checked_native | 0.920x | 1.056x | 9/12 | 0.947x |
| replay_trusted_native | 0.918x | 0.780x | 3/12 | 0.472x |
| san_parse_all | 0.973x | 1.860x | 36/36 | 1.402x |
| san_render_native | 0.990x | 2.506x | 36/36 | 1.565x |
| snapshot | 0.839x | 0.976x | 14/36 | 0.857x |
| startpos | 0.829x | 1.004x | 2/6 | 0.768x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.472x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.476x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.478x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.503x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.505x |
| macos-15-native | apply_children_full | tactical | chess | 0.551x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.561x |
| macos-15-native | apply_children_full | start | chess | 0.599x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.615x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.635x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.651x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.728x |
| macos-15-native | startpos | start | shakmaty | 0.768x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.795x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.805x |
| windows-2025-native | startpos | start | shakmaty | 0.834x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.857x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.857x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.859x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.864x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.866x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.867x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| windows-2025-native | snapshot | endgame | ferrum | 0.909x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.914x |
| windows-2025-native | snapshot | promotion | ferrum | 0.915x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.916x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.921x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.922x |
| macos-15-native | snapshot | promotion | ferrum | 0.923x |
| macos-15-native | materialize | middlegame | ultra | 0.928x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.936x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.939x |
| macos-15-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.946x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.947x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.950x |
| macos-15-native | materialize | kiwipete | ultra | 0.952x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.955x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.959x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.959x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.974x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.979x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.980x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.984x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.986x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.990x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.990x |
| windows-2025-native | snapshot | tactical | ferrum | 0.993x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.993x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.997x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-arm-native | startpos | start | shakmaty | 0.998x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4917 | shakmaty | 6583 | 1.339x |
| ubuntu-24.04-arm-native | 4936 | shakmaty | 8728 | 1.768x |
| ubuntu-24.04-native | 6870 | shakmaty | 9304 | 1.354x |
| ubuntu-24.04-pext | 5843 | shakmaty | 7486 | 1.281x |
| ubuntu-24.04-portable | 6126 | shakmaty | 7527 | 1.229x |
| windows-2025-native | 17800 | shakmaty | 22600 | 1.270x |

Cold wins: 6/6 (separate from native cases).
