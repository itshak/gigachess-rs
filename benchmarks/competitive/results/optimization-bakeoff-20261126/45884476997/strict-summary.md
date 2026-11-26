| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 74 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 82 | 11 | 87 |
| ubuntu-24.04-pext | 78 | 77 | 15 | 87 |
| ubuntu-24.04-portable | 80 | 79 | 15 | 87 |
| windows-2025-native | 82 | 81 | 12 | 87 |
| ALL | 479 | 472 | 79 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.033x | 1.276x | 35/36 | 0.968x |
| chess960_materialize | 1.064x | 1.526x | 12/12 | 1.317x |
| chess960_perft_d3 | 0.999x | 1.524x | 12/12 | 1.378x |
| corpus_count | 1.011x | 1.205x | 11/12 | 0.979x |
| corpus_materialize | 0.983x | 1.173x | 12/12 | 1.042x |
| corpus_uci_sorted | 1.007x | 1.515x | 12/12 | 1.347x |
| count | 0.997x | 1.491x | 36/36 | 1.042x |
| fen_parse | 0.996x | 1.181x | 31/36 | 0.940x |
| fen_write_native_ep | 1.000x | 1.798x | 36/36 | 1.295x |
| make_unmake_full | 1.026x | 1.331x | 36/36 | 1.047x |
| materialize | 0.976x | 1.326x | 32/36 | 0.821x |
| perft_bulk_d4 | 0.986x | 1.284x | 33/36 | 0.906x |
| perft_full_d3 | 1.160x | 1.597x | 36/36 | 1.324x |
| polyglot_key | 1.032x | 29.525x | 12/12 | 16.244x |
| polyglot_recompute | 1.078x | 1.069x | 12/12 | 1.007x |
| repetition_key | 0.965x | 19.545x | 12/12 | 14.575x |
| replay_checked_native | 0.924x | 1.070x | 8/12 | 0.919x |
| replay_trusted_native | 0.878x | 0.742x | 3/12 | 0.463x |
| san_parse_all | 1.011x | 1.931x | 36/36 | 1.341x |
| san_render_native | 1.028x | 2.582x | 36/36 | 1.691x |
| snapshot | 0.993x | 1.074x | 21/36 | 0.716x |
| startpos | 1.009x | 1.192x | 5/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.463x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.541x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.572x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.605x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.626x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.630x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| macos-15-native | snapshot | middlegame | ferrum | 0.716x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.809x |
| macos-15-native | materialize | kiwipete | ultra | 0.821x |
| macos-15-native | materialize | middlegame | ultra | 0.867x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.906x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.916x |
| macos-15-native | snapshot | tactical | ferrum | 0.918x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.919x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.940x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.943x |
| macos-15-native | materialize | tactical | ultra | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| windows-2025-native | snapshot | start | ferrum | 0.954x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.956x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.957x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.961x |
| macos-15-native | apply_children_full | tactical | chess | 0.968x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.973x |
| ubuntu-24.04-pext | fen_parse | endgame | ultra | 0.973x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.974x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.979x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.979x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.984x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.985x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.987x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.989x |
| windows-2025-native | materialize | kiwipete | ultra | 0.994x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.996x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.996x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.997x |
| windows-2025-native | startpos | start | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5583 | ferrum | 8917 | 1.597x |
| ubuntu-24.04-arm-native | 6303 | shakmaty | 8448 | 1.340x |
| ubuntu-24.04-native | 5891 | shakmaty | 7631 | 1.295x |
| ubuntu-24.04-pext | 5374 | shakmaty | 7854 | 1.461x |
| ubuntu-24.04-portable | 7221 | chess | 10345 | 1.433x |
| windows-2025-native | 12400 | shakmaty | 16600 | 1.339x |

Cold wins: 6/6 (separate from native cases).
