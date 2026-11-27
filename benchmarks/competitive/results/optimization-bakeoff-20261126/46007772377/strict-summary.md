| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 79 | 76 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 79 | 79 | 13 | 87 |
| ubuntu-24.04-pext | 85 | 83 | 16 | 87 |
| ubuntu-24.04-portable | 84 | 78 | 11 | 87 |
| windows-2025-native | 81 | 78 | 14 | 87 |
| ALL | 487 | 473 | 78 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.999x | 1.289x | 36/36 | 1.024x |
| chess960_materialize | 1.007x | 1.490x | 12/12 | 1.229x |
| chess960_perft_d3 | 0.979x | 1.524x | 12/12 | 1.226x |
| corpus_count | 1.069x | 1.427x | 12/12 | 1.038x |
| corpus_materialize | 1.000x | 1.204x | 12/12 | 1.054x |
| corpus_uci_sorted | 0.997x | 1.510x | 12/12 | 1.312x |
| count | 1.020x | 1.521x | 36/36 | 1.084x |
| fen_parse | 0.964x | 1.196x | 35/36 | 0.992x |
| fen_write_native_ep | 0.983x | 1.720x | 36/36 | 1.365x |
| make_unmake_full | 0.999x | 1.323x | 35/36 | 0.894x |
| materialize | 0.997x | 1.296x | 34/36 | 0.917x |
| perft_bulk_d4 | 1.036x | 1.379x | 35/36 | 0.998x |
| perft_full_d3 | 1.195x | 1.564x | 36/36 | 1.346x |
| polyglot_key | 0.996x | 29.597x | 12/12 | 16.413x |
| polyglot_recompute | 0.980x | 1.024x | 9/12 | 0.883x |
| repetition_key | 0.974x | 20.710x | 12/12 | 14.743x |
| replay_checked_native | 0.987x | 1.131x | 11/12 | 0.980x |
| replay_trusted_native | 0.996x | 0.838x | 5/12 | 0.536x |
| san_parse_all | 0.978x | 1.936x | 36/36 | 1.285x |
| san_render_native | 0.992x | 2.609x | 36/36 | 1.691x |
| snapshot | 0.955x | 1.012x | 17/36 | 0.901x |
| startpos | 1.003x | 1.270x | 6/6 | 1.202x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.536x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.577x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.660x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.685x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.748x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.762x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.844x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.883x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.894x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.901x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.901x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.903x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.903x |
| ubuntu-24.04-native | snapshot | middlegame | chess | 0.903x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.903x |
| macos-15-native | materialize | kiwipete | ultra | 0.917x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.940x |
| macos-15-native | snapshot | tactical | ferrum | 0.942x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.954x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.980x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| macos-15-native | snapshot | promotion | ferrum | 0.981x |
| macos-15-native | snapshot | start | ferrum | 0.987x |
| windows-2025-native | snapshot | middlegame | cozy | 0.988x |
| windows-2025-native | snapshot | tactical | cozy | 0.990x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.992x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.993x |
| macos-15-native | materialize | tactical | ultra | 0.994x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.998x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3083 | shakmaty | 6250 | 2.027x |
| ubuntu-24.04-arm-native | 6313 | shakmaty | 8280 | 1.312x |
| ubuntu-24.04-native | 8576 | shakmaty | 10690 | 1.247x |
| ubuntu-24.04-pext | 6960 | shakmaty | 9414 | 1.353x |
| ubuntu-24.04-portable | 6423 | shakmaty | 7274 | 1.132x |
| windows-2025-native | 13700 | shakmaty | 20700 | 1.511x |

Cold wins: 6/6 (separate from native cases).
