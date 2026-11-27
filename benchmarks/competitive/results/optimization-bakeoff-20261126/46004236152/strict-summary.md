| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 78 | 77 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 80 | 14 | 87 |
| ubuntu-24.04-pext | 84 | 84 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 77 | 16 | 87 |
| windows-2025-native | 85 | 84 | 12 | 87 |
| ALL | 492 | 481 | 78 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.010x | 1.287x | 35/36 | 0.980x |
| chess960_materialize | 0.999x | 1.475x | 12/12 | 1.327x |
| chess960_perft_d3 | 1.002x | 1.545x | 12/12 | 1.354x |
| corpus_count | 1.169x | 1.364x | 12/12 | 1.043x |
| corpus_materialize | 1.019x | 1.153x | 12/12 | 1.049x |
| corpus_uci_sorted | 0.986x | 1.492x | 12/12 | 1.274x |
| count | 1.029x | 1.549x | 36/36 | 1.072x |
| fen_parse | 0.983x | 1.202x | 36/36 | 1.007x |
| fen_write_native_ep | 0.992x | 1.820x | 36/36 | 1.428x |
| make_unmake_full | 0.999x | 1.312x | 36/36 | 1.033x |
| materialize | 1.013x | 1.400x | 35/36 | 0.971x |
| perft_bulk_d4 | 1.064x | 1.339x | 33/36 | 0.912x |
| perft_full_d3 | 1.202x | 1.630x | 36/36 | 1.084x |
| polyglot_key | 1.006x | 33.066x | 12/12 | 16.143x |
| polyglot_recompute | 0.999x | 1.007x | 7/12 | 0.883x |
| repetition_key | 1.031x | 21.717x | 12/12 | 14.178x |
| replay_checked_native | 0.997x | 1.150x | 12/12 | 1.076x |
| replay_trusted_native | 1.050x | 0.860x | 5/12 | 0.566x |
| san_parse_all | 1.006x | 1.852x | 36/36 | 1.543x |
| san_render_native | 0.987x | 2.495x | 36/36 | 1.856x |
| snapshot | 0.990x | 1.056x | 24/36 | 0.852x |
| startpos | 0.934x | 1.154x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.566x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.586x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.675x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.704x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.713x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.722x |
| macos-15-native | snapshot | endgame | ferrum | 0.852x |
| macos-15-native | snapshot | tactical | ferrum | 0.881x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.883x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.912x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.926x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.939x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.949x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.950x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.965x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.966x |
| macos-15-native | materialize | kiwipete | ultra | 0.971x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.979x |
| macos-15-native | apply_children_full | tactical | chess | 0.980x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.986x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.991x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.998x |
| macos-15-native | snapshot | promotion | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.999x |
| windows-2025-native | startpos | start | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4833 | shakmaty | 9000 | 1.862x |
| ubuntu-24.04-arm-native | 9728 | chess | 10151 | 1.043x |
| ubuntu-24.04-native | 6120 | shakmaty | 8004 | 1.308x |
| ubuntu-24.04-pext | 5057 | shakmaty | 8524 | 1.686x |
| ubuntu-24.04-portable | 5676 | shakmaty | 7222 | 1.272x |
| windows-2025-native | 13100 | shakmaty | 18500 | 1.412x |

Cold wins: 6/6 (separate from native cases).
