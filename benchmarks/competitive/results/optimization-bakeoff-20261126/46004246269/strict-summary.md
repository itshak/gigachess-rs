| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 74 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 80 | 78 | 16 | 87 |
| ubuntu-24.04-pext | 80 | 79 | 15 | 87 |
| ubuntu-24.04-portable | 80 | 77 | 16 | 87 |
| windows-2025-native | 82 | 78 | 12 | 87 |
| ALL | 478 | 465 | 85 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.000x | 1.272x | 35/36 | 0.864x |
| chess960_materialize | 0.988x | 1.455x | 12/12 | 1.205x |
| chess960_perft_d3 | 0.980x | 1.521x | 12/12 | 1.369x |
| corpus_count | 1.073x | 1.415x | 11/12 | 0.862x |
| corpus_materialize | 0.980x | 1.189x | 12/12 | 1.092x |
| corpus_uci_sorted | 0.984x | 1.551x | 12/12 | 1.246x |
| count | 1.013x | 1.490x | 36/36 | 1.072x |
| fen_parse | 1.009x | 1.217x | 35/36 | 0.985x |
| fen_write_native_ep | 1.000x | 1.783x | 36/36 | 1.428x |
| make_unmake_full | 1.029x | 1.345x | 36/36 | 1.031x |
| materialize | 1.019x | 1.328x | 35/36 | 0.851x |
| perft_bulk_d4 | 1.035x | 1.370x | 35/36 | 0.958x |
| perft_full_d3 | 1.191x | 1.550x | 36/36 | 1.193x |
| polyglot_key | 0.967x | 29.406x | 12/12 | 16.635x |
| polyglot_recompute | 0.969x | 1.039x | 9/12 | 0.895x |
| repetition_key | 0.983x | 20.145x | 12/12 | 14.406x |
| replay_checked_native | 1.005x | 1.164x | 12/12 | 1.063x |
| replay_trusted_native | 1.020x | 0.839x | 5/12 | 0.547x |
| san_parse_all | 0.981x | 1.941x | 36/36 | 1.477x |
| san_render_native | 1.001x | 2.623x | 36/36 | 1.668x |
| snapshot | 0.966x | 0.910x | 7/36 | 0.660x |
| startpos | 1.031x | 1.241x | 6/6 | 1.107x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.547x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.582x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.660x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.661x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.661x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.661x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.662x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.662x |
| macos-15-native | snapshot | tactical | ferrum | 0.666x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.688x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.696x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.700x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.746x |
| macos-15-native | snapshot | middlegame | ferrum | 0.755x |
| macos-15-native | materialize | kiwipete | ultra | 0.851x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.862x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.864x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.895x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.895x |
| macos-15-native | snapshot | start | ferrum | 0.938x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.954x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.958x |
| ubuntu-24.04-pext | snapshot | start | ferrum | 0.971x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.973x |
| ubuntu-24.04-pext | snapshot | tactical | cozy | 0.973x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.974x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.976x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.976x |
| windows-2025-native | fen_parse | middlegame | shakmaty | 0.985x |
| windows-2025-native | snapshot | middlegame | chess | 0.992x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.992x |
| windows-2025-native | snapshot | tactical | cozy | 0.993x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.994x |
| windows-2025-native | snapshot | promotion | cozy | 0.995x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6542 | shakmaty | 8666 | 1.325x |
| ubuntu-24.04-arm-native | 5408 | shakmaty | 8759 | 1.620x |
| ubuntu-24.04-native | 6359 | shakmaty | 7962 | 1.252x |
| ubuntu-24.04-pext | 7254 | shakmaty | 10540 | 1.453x |
| ubuntu-24.04-portable | 5201 | shakmaty | 7263 | 1.396x |
| windows-2025-native | 16600 | shakmaty | 21500 | 1.295x |

Cold wins: 6/6 (separate from native cases).
