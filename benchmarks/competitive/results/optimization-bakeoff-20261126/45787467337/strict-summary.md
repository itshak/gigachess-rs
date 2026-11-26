| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 71 | 13 | 87 |
| ubuntu-24.04-arm-native | 80 | 75 | 11 | 87 |
| ubuntu-24.04-native | 77 | 73 | 10 | 87 |
| ubuntu-24.04-pext | 73 | 68 | 12 | 87 |
| ubuntu-24.04-portable | 84 | 78 | 13 | 87 |
| windows-2025-native | 84 | 78 | 12 | 87 |
| ALL | 469 | 443 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.976x | 1.084x | 27/36 | 0.904x |
| chess960_materialize | 1.053x | 1.361x | 12/12 | 1.218x |
| chess960_perft_d3 | 1.028x | 1.374x | 12/12 | 1.264x |
| corpus_count | 1.005x | 1.407x | 12/12 | 1.209x |
| corpus_materialize | 1.034x | 1.155x | 12/12 | 1.060x |
| corpus_uci_sorted | 0.993x | 1.516x | 12/12 | 1.160x |
| count | 0.996x | 1.352x | 35/36 | 0.860x |
| fen_parse | 1.000x | 1.165x | 31/36 | 0.930x |
| fen_write_native_ep | 0.997x | 1.770x | 36/36 | 1.284x |
| make_unmake_full | 0.991x | 1.262x | 36/36 | 1.011x |
| materialize | 0.994x | 1.276x | 32/36 | 0.764x |
| perft_bulk_d4 | 0.993x | 1.272x | 36/36 | 1.098x |
| perft_full_d3 | 0.980x | 1.316x | 35/36 | 0.917x |
| polyglot_key | 1.027x | 33.351x | 12/12 | 16.060x |
| polyglot_recompute | 0.995x | 1.038x | 10/12 | 0.835x |
| repetition_key | 1.013x | 19.287x | 12/12 | 13.900x |
| replay_checked_native | 1.042x | 1.064x | 9/12 | 0.914x |
| replay_trusted_native | 1.058x | 0.659x | 0/12 | 0.498x |
| san_parse_all | 1.032x | 1.952x | 36/36 | 1.528x |
| san_render_native | 0.974x | 2.506x | 36/36 | 1.873x |
| snapshot | 1.013x | 0.992x | 20/36 | 0.651x |
| startpos | 0.921x | 1.236x | 6/6 | 1.006x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.498x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.507x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.546x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.573x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.640x |
| macos-15-native | snapshot | tactical | ferrum | 0.651x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.651x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.653x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.656x |
| macos-15-native | snapshot | promotion | cozy | 0.711x |
| macos-15-native | snapshot | start | ferrum | 0.724x |
| macos-15-native | materialize | middlegame | ultra | 0.764x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.802x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.834x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.835x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.844x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.847x |
| macos-15-native | count | middlegame | cozy | 0.860x |
| macos-15-native | materialize | kiwipete | ultra | 0.903x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.904x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.914x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.917x |
| macos-15-native | materialize | tactical | ultra | 0.923x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.930x |
| macos-15-native | apply_children_full | tactical | chess | 0.932x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.939x |
| macos-15-native | materialize | start | ultra | 0.953x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.954x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.957x |
| ubuntu-24.04-pext | fen_parse | endgame | ultra | 0.960x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.976x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.982x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.984x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.985x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.988x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.991x |
| ubuntu-24.04-native | apply_children_full | kiwipete | chess | 0.994x |
| ubuntu-24.04-portable | snapshot | promotion | chess | 0.994x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.996x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.997x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.997x |
| ubuntu-24.04-native | snapshot | middlegame | chess | 0.997x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.997x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.997x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.997x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.997x |
| ubuntu-24.04-native | snapshot | start | chess | 0.998x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.998x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.999x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.999x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.999x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.999x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 16750 | shakmaty | 13542 | 0.808x |
| ubuntu-24.04-arm-native | 11672 | shakmaty | 9752 | 0.836x |
| ubuntu-24.04-native | 9469 | shakmaty | 7471 | 0.789x |
| ubuntu-24.04-pext | 8815 | shakmaty | 7198 | 0.817x |
| ubuntu-24.04-portable | 14207 | shakmaty | 10069 | 0.709x |
| windows-2025-native | 27100 | shakmaty | 19600 | 0.723x |

Cold wins: 0/6 (separate from native cases).
