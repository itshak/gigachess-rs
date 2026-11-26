| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 65 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 73 | 11 | 87 |
| ubuntu-24.04-native | 82 | 81 | 10 | 87 |
| ubuntu-24.04-pext | 73 | 71 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 74 | 14 | 87 |
| windows-2025-native | 81 | 81 | 11 | 87 |
| ALL | 465 | 445 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.131x | 1.128x | 29/36 | 0.696x |
| chess960_materialize | 0.959x | 1.324x | 12/12 | 1.073x |
| chess960_perft_d3 | 1.003x | 1.380x | 12/12 | 1.263x |
| corpus_count | 1.047x | 1.360x | 12/12 | 1.073x |
| corpus_materialize | 1.024x | 1.173x | 12/12 | 1.041x |
| corpus_uci_sorted | 0.995x | 1.537x | 12/12 | 1.456x |
| count | 0.998x | 1.320x | 33/36 | 0.845x |
| fen_parse | 0.980x | 1.120x | 30/36 | 0.886x |
| fen_write_native_ep | 0.995x | 1.734x | 36/36 | 1.139x |
| make_unmake_full | 0.999x | 1.229x | 34/36 | 0.960x |
| materialize | 0.990x | 1.273x | 34/36 | 0.794x |
| perft_bulk_d4 | 1.008x | 1.239x | 36/36 | 1.001x |
| perft_full_d3 | 0.989x | 1.278x | 30/36 | 0.748x |
| polyglot_key | 1.011x | 35.780x | 12/12 | 16.239x |
| polyglot_recompute | 1.019x | 1.012x | 7/12 | 0.909x |
| repetition_key | 1.080x | 22.954x | 12/12 | 13.753x |
| replay_checked_native | 0.988x | 0.987x | 7/12 | 0.758x |
| replay_trusted_native | 0.999x | 0.630x | 0/12 | 0.471x |
| san_parse_all | 1.027x | 1.851x | 36/36 | 1.521x |
| san_render_native | 1.013x | 2.593x | 36/36 | 1.891x |
| snapshot | 1.024x | 1.019x | 28/36 | 0.441x |
| startpos | 1.013x | 1.178x | 5/6 | 0.985x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | start | ferrum | 0.441x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.471x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.487x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.503x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.508x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.573x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.623x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.633x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.679x |
| macos-15-native | snapshot | endgame | ferrum | 0.693x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.696x |
| macos-15-native | snapshot | tactical | chess | 0.738x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.748x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.755x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.758x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.762x |
| macos-15-native | snapshot | promotion | ferrum | 0.783x |
| macos-15-native | materialize | endgame | ultra | 0.794x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.805x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.806x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.819x |
| macos-15-native | apply_children_full | start | chess | 0.838x |
| macos-15-native | count | kiwipete | ultra | 0.845x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.845x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.845x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.860x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.875x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.886x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.909x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.919x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.922x |
| macos-15-native | apply_children_full | promotion | chess | 0.929x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.931x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.942x |
| macos-15-native | materialize | kiwipete | ultra | 0.944x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.944x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.953x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.959x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.960x |
| ubuntu-24.04-pext | fen_parse | tactical | ferrum | 0.961x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.966x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.973x |
| macos-15-native | count | middlegame | cozy | 0.974x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.976x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.983x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.983x |
| ubuntu-24.04-pext | perft_full_d3 | endgame | ultra | 0.984x |
| windows-2025-native | startpos | start | shakmaty | 0.985x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.992x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.992x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.995x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.995x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.997x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.997x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.997x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.997x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 9125 | shakmaty | 6959 | 0.763x |
| ubuntu-24.04-arm-native | 10977 | shakmaty | 8992 | 0.819x |
| ubuntu-24.04-native | 9231 | shakmaty | 7019 | 0.760x |
| ubuntu-24.04-pext | 14632 | cozy | 10095 | 0.690x |
| ubuntu-24.04-portable | 9295 | shakmaty | 7645 | 0.822x |
| windows-2025-native | 32500 | shakmaty | 31100 | 0.957x |

Cold wins: 0/6 (separate from native cases).
