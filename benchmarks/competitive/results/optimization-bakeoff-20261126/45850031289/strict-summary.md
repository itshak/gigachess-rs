| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 69 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 10 | 87 |
| ubuntu-24.04-pext | 84 | 82 | 10 | 87 |
| ubuntu-24.04-portable | 78 | 75 | 13 | 87 |
| windows-2025-native | 84 | 83 | 12 | 87 |
| ALL | 479 | 471 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.029x | 1.258x | 34/36 | 0.950x |
| chess960_materialize | 0.993x | 1.511x | 12/12 | 1.328x |
| chess960_perft_d3 | 0.959x | 1.455x | 12/12 | 1.109x |
| corpus_count | 1.005x | 1.179x | 9/12 | 0.849x |
| corpus_materialize | 0.973x | 1.181x | 12/12 | 1.030x |
| corpus_uci_sorted | 1.006x | 1.538x | 12/12 | 1.452x |
| count | 0.993x | 1.490x | 35/36 | 0.954x |
| fen_parse | 0.982x | 1.196x | 34/36 | 0.933x |
| fen_write_native_ep | 0.975x | 1.726x | 36/36 | 1.442x |
| make_unmake_full | 1.003x | 1.301x | 36/36 | 1.014x |
| materialize | 1.020x | 1.358x | 35/36 | 0.934x |
| perft_bulk_d4 | 0.986x | 1.269x | 34/36 | 0.894x |
| perft_full_d3 | 1.001x | 1.319x | 36/36 | 1.104x |
| polyglot_key | 0.985x | 29.414x | 12/12 | 16.038x |
| polyglot_recompute | 1.006x | 1.004x | 6/12 | 0.885x |
| repetition_key | 0.932x | 19.213x | 12/12 | 14.299x |
| replay_checked_native | 1.065x | 1.147x | 12/12 | 1.067x |
| replay_trusted_native | 1.027x | 0.863x | 6/12 | 0.576x |
| san_parse_all | 1.004x | 1.873x | 36/36 | 1.434x |
| san_render_native | 1.004x | 2.480x | 36/36 | 1.790x |
| snapshot | 0.989x | 1.055x | 18/36 | 0.656x |
| startpos | 0.940x | 1.107x | 4/6 | 0.743x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.576x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.584x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.648x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.656x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.663x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.700x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.701x |
| macos-15-native | snapshot | tactical | ferrum | 0.731x |
| macos-15-native | startpos | start | shakmaty | 0.743x |
| macos-15-native | snapshot | endgame | cozy | 0.780x |
| macos-15-native | snapshot | promotion | cozy | 0.832x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.849x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.885x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.885x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.894x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.912x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.929x |
| macos-15-native | snapshot | middlegame | ferrum | 0.931x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 0.933x |
| macos-15-native | materialize | middlegame | ultra | 0.934x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.949x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.950x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.952x |
| macos-15-native | count | middlegame | cozy | 0.954x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.955x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.964x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.967x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.972x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.974x |
| macos-15-native | apply_children_full | tactical | chess | 0.974x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.977x |
| macos-15-native | snapshot | start | ferrum | 0.980x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.983x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.983x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.984x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.986x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.991x |
| windows-2025-native | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7791 | chess | 8917 | 1.145x |
| ubuntu-24.04-arm-native | 5344 | shakmaty | 7272 | 1.361x |
| ubuntu-24.04-native | 5353 | shakmaty | 7999 | 1.494x |
| ubuntu-24.04-pext | 5683 | shakmaty | 8372 | 1.473x |
| ubuntu-24.04-portable | 6129 | shakmaty | 10266 | 1.675x |
| windows-2025-native | 13300 | shakmaty | 21600 | 1.624x |

Cold wins: 6/6 (separate from native cases).
