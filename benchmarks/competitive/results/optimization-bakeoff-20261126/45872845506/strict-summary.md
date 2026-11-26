| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 72 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 82 | 15 | 87 |
| ubuntu-24.04-pext | 86 | 83 | 13 | 87 |
| ubuntu-24.04-portable | 82 | 76 | 12 | 87 |
| windows-2025-native | 82 | 78 | 13 | 87 |
| ALL | 489 | 470 | 77 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.013x | 1.286x | 36/36 | 1.014x |
| chess960_materialize | 0.981x | 1.446x | 12/12 | 1.213x |
| chess960_perft_d3 | 0.997x | 1.507x | 12/12 | 1.384x |
| corpus_count | 1.007x | 1.266x | 11/12 | 0.896x |
| corpus_materialize | 0.993x | 1.153x | 12/12 | 1.024x |
| corpus_uci_sorted | 0.990x | 1.536x | 12/12 | 1.319x |
| count | 0.995x | 1.460x | 36/36 | 1.017x |
| fen_parse | 0.998x | 1.193x | 36/36 | 1.030x |
| fen_write_native_ep | 0.986x | 1.742x | 36/36 | 1.420x |
| make_unmake_full | 0.995x | 1.310x | 36/36 | 1.044x |
| materialize | 0.982x | 1.357x | 33/36 | 0.954x |
| perft_bulk_d4 | 0.995x | 1.284x | 34/36 | 0.891x |
| perft_full_d3 | 0.992x | 1.324x | 36/36 | 1.157x |
| polyglot_key | 1.021x | 33.283x | 12/12 | 17.202x |
| polyglot_recompute | 0.999x | 1.029x | 9/12 | 0.969x |
| repetition_key | 1.021x | 20.380x | 12/12 | 14.984x |
| replay_checked_native | 0.975x | 1.134x | 12/12 | 1.012x |
| replay_trusted_native | 0.932x | 0.738x | 4/12 | 0.505x |
| san_parse_all | 1.004x | 1.924x | 36/36 | 1.390x |
| san_render_native | 1.000x | 2.586x | 36/36 | 1.879x |
| snapshot | 0.989x | 1.075x | 20/36 | 0.838x |
| startpos | 1.016x | 1.270x | 6/6 | 1.208x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.505x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.567x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.599x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.602x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.623x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.635x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.644x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.751x |
| windows-2025-native | snapshot | endgame | ferrum | 0.838x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.891x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.896x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.939x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.952x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.954x |
| macos-15-native | materialize | middlegame | ultra | 0.954x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.955x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.958x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.963x |
| macos-15-native | snapshot | start | ferrum | 0.966x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.969x |
| macos-15-native | materialize | kiwipete | ultra | 0.970x |
| macos-15-native | snapshot | endgame | ferrum | 0.972x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.978x |
| macos-15-native | snapshot | promotion | ferrum | 0.983x |
| windows-2025-native | snapshot | middlegame | cozy | 0.986x |
| windows-2025-native | snapshot | kiwipete | chess | 0.987x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.989x |
| macos-15-native | materialize | tactical | ultra | 0.992x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.993x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.995x |
| macos-15-native | snapshot | tactical | ferrum | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3291 | shakmaty | 6083 | 1.848x |
| ubuntu-24.04-arm-native | 5487 | shakmaty | 7416 | 1.352x |
| ubuntu-24.04-native | 5524 | shakmaty | 6380 | 1.155x |
| ubuntu-24.04-pext | 7555 | cozy | 10019 | 1.326x |
| ubuntu-24.04-portable | 5763 | shakmaty | 8433 | 1.463x |
| windows-2025-native | 16400 | shakmaty | 26600 | 1.622x |

Cold wins: 6/6 (separate from native cases).
