| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 68 | 12 | 87 |
| ubuntu-24.04-arm-native | 77 | 75 | 11 | 87 |
| ubuntu-24.04-native | 76 | 74 | 15 | 87 |
| ubuntu-24.04-pext | 84 | 81 | 12 | 87 |
| ubuntu-24.04-portable | 79 | 74 | 10 | 87 |
| windows-2025-native | 79 | 74 | 12 | 87 |
| ALL | 466 | 446 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.140x | 1.177x | 34/36 | 0.921x |
| chess960_materialize | 1.101x | 1.377x | 12/12 | 1.200x |
| chess960_perft_d3 | 1.057x | 1.381x | 12/12 | 1.261x |
| corpus_count | 0.866x | 1.166x | 9/12 | 0.848x |
| corpus_materialize | 0.952x | 1.144x | 12/12 | 1.065x |
| corpus_uci_sorted | 0.980x | 1.525x | 12/12 | 1.186x |
| count | 0.998x | 1.298x | 35/36 | 0.967x |
| fen_parse | 1.005x | 1.171x | 32/36 | 0.954x |
| fen_write_native_ep | 1.010x | 1.683x | 36/36 | 1.380x |
| make_unmake_full | 1.017x | 1.293x | 36/36 | 1.082x |
| materialize | 1.021x | 1.162x | 31/36 | 0.814x |
| perft_bulk_d4 | 0.919x | 1.177x | 34/36 | 0.788x |
| perft_full_d3 | 0.976x | 1.169x | 31/36 | 0.919x |
| polyglot_key | 1.017x | 28.662x | 12/12 | 16.547x |
| polyglot_recompute | 1.055x | 1.077x | 12/12 | 1.005x |
| repetition_key | 0.901x | 18.596x | 12/12 | 14.353x |
| replay_checked_native | 1.032x | 1.006x | 6/12 | 0.916x |
| replay_trusted_native | 1.045x | 0.679x | 1/12 | 0.464x |
| san_parse_all | 0.988x | 1.928x | 36/36 | 1.303x |
| san_render_native | 1.002x | 2.591x | 36/36 | 1.658x |
| snapshot | 1.119x | 1.049x | 19/36 | 0.622x |
| startpos | 1.140x | 1.242x | 6/6 | 1.179x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.464x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.485x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.514x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.578x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.601x |
| macos-15-native | snapshot | start | ferrum | 0.622x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.637x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.716x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.788x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.814x |
| macos-15-native | materialize | kiwipete | ultra | 0.814x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.816x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.848x |
| windows-2025-native | snapshot | endgame | cozy | 0.866x |
| macos-15-native | materialize | middlegame | ultra | 0.898x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.911x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.916x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.919x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.921x |
| macos-15-native | materialize | tactical | ultra | 0.923x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.923x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.925x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.925x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.929x |
| macos-15-native | materialize | endgame | ultra | 0.937x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.938x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.941x |
| macos-15-native | materialize | start | ultra | 0.942x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.954x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.954x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.967x |
| macos-15-native | snapshot | promotion | ferrum | 0.975x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.980x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.983x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.984x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.984x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.987x |
| windows-2025-native | snapshot | tactical | ferrum | 0.987x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.989x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.990x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.995x |
| windows-2025-native | snapshot | promotion | cozy | 0.996x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.996x |
| macos-15-native | snapshot | endgame | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| ubuntu-24.04-native | fen_parse | middlegame | ultra | 0.998x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.999x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.999x |
| windows-2025-native | snapshot | kiwipete | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5916 | shakmaty | 11500 | 1.944x |
| ubuntu-24.04-arm-native | 6984 | chess | 10344 | 1.481x |
| ubuntu-24.04-native | 6319 | shakmaty | 8262 | 1.307x |
| ubuntu-24.04-pext | 9448 | shakmaty | 13495 | 1.428x |
| ubuntu-24.04-portable | 5989 | shakmaty | 7772 | 1.298x |
| windows-2025-native | 22800 | ferrum | 29300 | 1.285x |

Cold wins: 6/6 (separate from native cases).
