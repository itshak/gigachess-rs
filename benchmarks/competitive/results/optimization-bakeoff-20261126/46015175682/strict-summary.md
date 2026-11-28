| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 78 | 74 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 85 | 82 | 13 | 87 |
| ubuntu-24.04-pext | 82 | 78 | 12 | 87 |
| ubuntu-24.04-portable | 82 | 81 | 14 | 87 |
| windows-2025-native | 86 | 85 | 12 | 87 |
| ALL | 492 | 479 | 76 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.033x | 1.285x | 35/36 | 0.996x |
| chess960_materialize | 1.011x | 1.498x | 12/12 | 1.285x |
| chess960_perft_d3 | 1.009x | 1.556x | 12/12 | 1.361x |
| corpus_count | 1.123x | 1.340x | 12/12 | 1.081x |
| corpus_materialize | 1.009x | 1.178x | 12/12 | 1.043x |
| corpus_uci_sorted | 1.022x | 1.519x | 12/12 | 1.220x |
| count | 1.052x | 1.538x | 36/36 | 1.178x |
| fen_parse | 0.997x | 1.183x | 36/36 | 1.010x |
| fen_write_native_ep | 1.005x | 1.755x | 36/36 | 1.413x |
| make_unmake_full | 1.024x | 1.316x | 36/36 | 1.088x |
| materialize | 1.024x | 1.355x | 35/36 | 0.969x |
| perft_bulk_d4 | 1.058x | 1.348x | 33/36 | 0.935x |
| perft_full_d3 | 1.223x | 1.598x | 36/36 | 1.289x |
| polyglot_key | 0.982x | 28.850x | 12/12 | 16.190x |
| polyglot_recompute | 1.441x | 1.426x | 12/12 | 1.135x |
| repetition_key | 0.996x | 19.255x | 12/12 | 14.907x |
| replay_checked_native | 1.007x | 1.152x | 12/12 | 1.063x |
| replay_trusted_native | 1.020x | 0.849x | 6/12 | 0.518x |
| san_parse_all | 0.998x | 1.878x | 36/36 | 1.456x |
| san_render_native | 1.020x | 2.552x | 36/36 | 1.691x |
| snapshot | 0.993x | 1.036x | 18/36 | 0.931x |
| startpos | 1.023x | 1.224x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.518x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.586x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.673x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.683x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.697x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.774x |
| macos-15-native | snapshot | tactical | ferrum | 0.931x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.935x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.955x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.956x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.963x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.968x |
| macos-15-native | materialize | tactical | ultra | 0.969x |
| macos-15-native | snapshot | promotion | ferrum | 0.974x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.988x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.991x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.994x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.995x |
| macos-15-native | apply_children_full | middlegame | chess | 0.996x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.996x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |
| macos-15-native | snapshot | endgame | ferrum | 0.999x |
| windows-2025-native | startpos | start | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 2791 | shakmaty | 5083 | 1.821x |
| ubuntu-24.04-arm-native | 6656 | chess | 9464 | 1.422x |
| ubuntu-24.04-native | 6252 | shakmaty | 5923 | 0.947x |
| ubuntu-24.04-pext | 6198 | shakmaty | 6455 | 1.041x |
| ubuntu-24.04-portable | 7681 | shakmaty | 9884 | 1.287x |
| windows-2025-native | 12300 | shakmaty | 16900 | 1.374x |

Cold wins: 5/6 (separate from native cases).
