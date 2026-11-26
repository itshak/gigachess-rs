| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 71 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 75 | 11 | 87 |
| ubuntu-24.04-native | 71 | 67 | 10 | 87 |
| ubuntu-24.04-pext | 68 | 65 | 15 | 87 |
| ubuntu-24.04-portable | 77 | 74 | 9 | 87 |
| windows-2025-native | 80 | 78 | 11 | 87 |
| ALL | 448 | 430 | 67 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.059x | 1.173x | 34/36 | 0.923x |
| chess960_materialize | 0.989x | 1.376x | 12/12 | 1.222x |
| chess960_perft_d3 | 0.981x | 1.373x | 12/12 | 1.199x |
| corpus_count | 0.779x | 1.037x | 6/12 | 0.758x |
| corpus_materialize | 0.963x | 1.130x | 12/12 | 1.005x |
| corpus_uci_sorted | 0.992x | 1.515x | 12/12 | 1.209x |
| count | 0.992x | 1.331x | 35/36 | 0.960x |
| fen_parse | 0.987x | 1.179x | 36/36 | 1.076x |
| fen_write_native_ep | 0.980x | 1.742x | 36/36 | 1.148x |
| make_unmake_full | 0.965x | 1.148x | 26/36 | 0.807x |
| materialize | 1.010x | 1.302x | 34/36 | 0.880x |
| perft_bulk_d4 | 0.888x | 1.136x | 29/36 | 0.831x |
| perft_full_d3 | 0.984x | 1.304x | 31/36 | 0.900x |
| polyglot_key | 1.087x | 34.073x | 12/12 | 16.489x |
| polyglot_recompute | 1.041x | 1.087x | 11/12 | 0.966x |
| repetition_key | 1.009x | 19.212x | 12/12 | 14.326x |
| replay_checked_native | 1.008x | 1.043x | 10/12 | 0.948x |
| replay_trusted_native | 1.014x | 0.699x | 1/12 | 0.477x |
| san_parse_all | 0.954x | 1.826x | 36/36 | 1.339x |
| san_render_native | 1.017x | 2.591x | 36/36 | 1.497x |
| snapshot | 1.033x | 0.934x | 10/36 | 0.623x |
| startpos | 1.043x | 1.209x | 5/6 | 0.981x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.477x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.531x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.568x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.584x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.608x |
| ubuntu-24.04-pext | snapshot | endgame | ferrum | 0.623x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.654x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.654x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.658x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.661x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.663x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.663x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.700x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.718x |
| windows-2025-native | corpus_count | seeded | ultra | 0.758x |
| ubuntu-24.04-native | make_unmake_full | middlegame | ultra | 0.807x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.820x |
| ubuntu-24.04-pext | make_unmake_full | middlegame | ultra | 0.830x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.831x |
| ubuntu-24.04-pext | make_unmake_full | kiwipete | ultra | 0.854x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.864x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.867x |
| macos-15-native | materialize | kiwipete | ultra | 0.880x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.896x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.896x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.900x |
| ubuntu-24.04-native | make_unmake_full | kiwipete | ultra | 0.903x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.909x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.912x |
| macos-15-native | snapshot | tactical | ferrum | 0.917x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.920x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.923x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.923x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.927x |
| macos-15-native | snapshot | start | cozy | 0.933x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.935x |
| macos-15-native | materialize | start | ultra | 0.936x |
| windows-2025-native | perft_bulk_d4 | start | ultra | 0.940x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.943x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.946x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.948x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.951x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.956x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.958x |
| macos-15-native | snapshot | middlegame | ferrum | 0.959x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.960x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.961x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.963x |
| macos-15-native | apply_children_full | tactical | chess | 0.966x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.966x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.967x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.970x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.971x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.975x |
| macos-15-native | startpos | start | shakmaty | 0.981x |
| windows-2025-native | perft_bulk_d4 | tactical | ultra | 0.982x |
| macos-15-native | snapshot | promotion | ferrum | 0.983x |
| ubuntu-24.04-native | make_unmake_full | endgame | ultra | 0.984x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.987x |
| windows-2025-native | perft_bulk_d4 | kiwipete | ultra | 0.989x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.992x |
| ubuntu-24.04-native | snapshot | start | chess | 0.992x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.996x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.996x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.996x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| ubuntu-24.04-native | snapshot | middlegame | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6250 | shakmaty | 7916 | 1.267x |
| ubuntu-24.04-arm-native | 6841 | chess | 9256 | 1.353x |
| ubuntu-24.04-native | 6551 | shakmaty | 7567 | 1.155x |
| ubuntu-24.04-pext | 6951 | shakmaty | 10186 | 1.465x |
| ubuntu-24.04-portable | 5097 | shakmaty | 7130 | 1.399x |
| windows-2025-native | 17800 | shakmaty | 27200 | 1.528x |

Cold wins: 6/6 (separate from native cases).
