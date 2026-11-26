| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 69 | 15 | 87 |
| ubuntu-24.04-arm-native | 77 | 77 | 11 | 87 |
| ubuntu-24.04-native | 74 | 70 | 12 | 87 |
| ubuntu-24.04-pext | 80 | 77 | 9 | 87 |
| ubuntu-24.04-portable | 75 | 70 | 10 | 87 |
| windows-2025-native | 82 | 81 | 11 | 87 |
| ALL | 459 | 444 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.977x | 1.239x | 34/36 | 0.847x |
| chess960_materialize | 0.961x | 1.441x | 12/12 | 1.119x |
| chess960_perft_d3 | 0.940x | 1.425x | 12/12 | 1.257x |
| corpus_count | 0.956x | 1.149x | 7/12 | 0.810x |
| corpus_materialize | 1.015x | 1.160x | 12/12 | 1.036x |
| corpus_uci_sorted | 0.999x | 1.540x | 12/12 | 1.285x |
| count | 0.955x | 1.461x | 36/36 | 1.028x |
| fen_parse | 0.893x | 1.109x | 25/36 | 0.885x |
| fen_write_native_ep | 0.974x | 1.665x | 36/36 | 1.338x |
| make_unmake_full | 0.900x | 1.209x | 33/36 | 0.934x |
| materialize | 0.963x | 1.279x | 35/36 | 0.860x |
| perft_bulk_d4 | 0.950x | 1.258x | 34/36 | 0.872x |
| perft_full_d3 | 0.892x | 1.187x | 36/36 | 1.019x |
| polyglot_key | 0.992x | 32.134x | 12/12 | 16.431x |
| polyglot_recompute | 1.000x | 1.009x | 7/12 | 0.888x |
| repetition_key | 0.968x | 20.985x | 12/12 | 14.353x |
| replay_checked_native | 0.948x | 1.026x | 6/12 | 0.970x |
| replay_trusted_native | 0.951x | 0.683x | 1/12 | 0.421x |
| san_parse_all | 0.956x | 1.778x | 36/36 | 1.342x |
| san_render_native | 0.918x | 2.385x | 36/36 | 1.697x |
| snapshot | 0.962x | 1.115x | 21/36 | 0.601x |
| startpos | 0.939x | 1.116x | 4/6 | 0.842x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.421x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.527x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.542x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.553x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.601x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.613x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.640x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.668x |
| macos-15-native | snapshot | start | ferrum | 0.670x |
| macos-15-native | snapshot | tactical | ferrum | 0.723x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.758x |
| macos-15-native | snapshot | promotion | cozy | 0.772x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.810x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.824x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.842x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.847x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.858x |
| macos-15-native | materialize | kiwipete | ultra | 0.860x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.872x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.885x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.888x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.899x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.912x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.912x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.914x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.920x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.928x |
| ubuntu-24.04-portable | fen_parse | promotion | ultra | 0.931x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.934x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.936x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.940x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.951x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.952x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.952x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.952x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 0.952x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.953x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.954x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.958x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.960x |
| ubuntu-24.04-portable | fen_parse | tactical | ultra | 0.969x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.970x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.971x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.976x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.977x |
| ubuntu-24.04-native | fen_parse | promotion | ferrum | 0.978x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.983x |
| macos-15-native | fen_parse | middlegame | cozy | 0.984x |
| ubuntu-24.04-portable | fen_parse | middlegame | ultra | 0.986x |
| ubuntu-24.04-native | fen_parse | kiwipete | ferrum | 0.992x |
| macos-15-native | snapshot | middlegame | ferrum | 0.993x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.993x |
| ubuntu-24.04-native | fen_parse | middlegame | ferrum | 0.993x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.994x |
| macos-15-native | snapshot | endgame | ferrum | 0.995x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.997x |
| ubuntu-24.04-portable | replay_checked_native | lichess300 | cozy | 0.999x |
| ubuntu-24.04-portable | snapshot | endgame | chess | 0.999x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.999x |
| ubuntu-24.04-portable | snapshot | promotion | chess | 0.999x |
| ubuntu-24.04-portable | snapshot | middlegame | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5084 | chess | 7167 | 1.410x |
| ubuntu-24.04-arm-native | 6496 | shakmaty | 8721 | 1.343x |
| ubuntu-24.04-native | 5154 | shakmaty | 5946 | 1.154x |
| ubuntu-24.04-pext | 7862 | cozy | 10526 | 1.339x |
| ubuntu-24.04-portable | 7464 | shakmaty | 10740 | 1.439x |
| windows-2025-native | 14700 | shakmaty | 21100 | 1.435x |

Cold wins: 6/6 (separate from native cases).
