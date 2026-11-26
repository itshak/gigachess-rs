| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 73 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 80 | 11 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 78 | 12 | 87 |
| windows-2025-native | 84 | 82 | 12 | 87 |
| ALL | 486 | 473 | 69 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.987x | 1.280x | 34/36 | 0.861x |
| chess960_materialize | 0.984x | 1.468x | 12/12 | 1.315x |
| chess960_perft_d3 | 0.994x | 1.531x | 12/12 | 1.311x |
| corpus_count | 1.019x | 1.194x | 9/12 | 0.829x |
| corpus_materialize | 1.015x | 1.168x | 12/12 | 1.016x |
| corpus_uci_sorted | 1.046x | 1.590x | 12/12 | 1.474x |
| count | 1.026x | 1.509x | 36/36 | 1.029x |
| fen_parse | 1.003x | 1.206x | 35/36 | 0.928x |
| fen_write_native_ep | 0.985x | 1.646x | 36/36 | 1.251x |
| make_unmake_full | 1.000x | 1.305x | 36/36 | 1.028x |
| materialize | 0.983x | 1.328x | 33/36 | 0.851x |
| perft_bulk_d4 | 0.980x | 1.288x | 33/36 | 0.786x |
| perft_full_d3 | 1.213x | 1.595x | 36/36 | 1.152x |
| polyglot_key | 1.034x | 32.700x | 12/12 | 16.644x |
| polyglot_recompute | 1.030x | 1.019x | 8/12 | 0.896x |
| repetition_key | 1.011x | 21.098x | 12/12 | 14.556x |
| replay_checked_native | 1.050x | 1.170x | 12/12 | 1.095x |
| replay_trusted_native | 1.007x | 0.787x | 5/12 | 0.500x |
| san_parse_all | 0.995x | 1.851x | 36/36 | 1.368x |
| san_render_native | 0.982x | 2.519x | 36/36 | 1.891x |
| snapshot | 0.992x | 1.075x | 23/36 | 0.611x |
| startpos | 0.968x | 1.189x | 6/6 | 1.001x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.500x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.544x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| macos-15-native | snapshot | endgame | ferrum | 0.611x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.652x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.676x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.703x |
| macos-15-native | snapshot | promotion | ferrum | 0.726x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.786x |
| macos-15-native | snapshot | start | ferrum | 0.814x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.829x |
| macos-15-native | materialize | middlegame | ultra | 0.851x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.861x |
| macos-15-native | materialize | tactical | ultra | 0.861x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.912x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.928x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.935x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.937x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.940x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.942x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.950x |
| windows-2025-native | materialize | kiwipete | ultra | 0.961x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.963x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.975x |
| macos-15-native | apply_children_full | kiwipete | cozy | 0.985x |
| macos-15-native | snapshot | middlegame | ferrum | 0.988x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.990x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.992x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.996x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6500 | shakmaty | 10875 | 1.673x |
| ubuntu-24.04-arm-native | 5480 | shakmaty | 7704 | 1.406x |
| ubuntu-24.04-native | 5744 | shakmaty | 7362 | 1.282x |
| ubuntu-24.04-pext | 5929 | shakmaty | 9795 | 1.652x |
| ubuntu-24.04-portable | 5486 | shakmaty | 6817 | 1.243x |
| windows-2025-native | 16100 | shakmaty | 23100 | 1.435x |

Cold wins: 6/6 (separate from native cases).
