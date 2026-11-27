| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 76 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-pext | 83 | 81 | 9 | 87 |
| ubuntu-24.04-portable | 83 | 77 | 12 | 87 |
| windows-2025-native | 85 | 82 | 12 | 87 |
| ALL | 486 | 473 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.006x | 1.274x | 35/36 | 0.975x |
| chess960_materialize | 1.038x | 1.502x | 12/12 | 1.272x |
| chess960_perft_d3 | 1.002x | 1.521x | 12/12 | 1.370x |
| corpus_count | 1.008x | 1.240x | 11/12 | 0.993x |
| corpus_materialize | 0.982x | 1.135x | 12/12 | 1.025x |
| corpus_uci_sorted | 0.999x | 1.502x | 12/12 | 1.171x |
| count | 1.008x | 1.474x | 36/36 | 1.023x |
| fen_parse | 1.000x | 1.193x | 30/36 | 0.891x |
| fen_write_native_ep | 0.992x | 1.767x | 36/36 | 1.229x |
| make_unmake_full | 0.989x | 1.281x | 36/36 | 1.010x |
| materialize | 0.991x | 1.425x | 35/36 | 0.774x |
| perft_bulk_d4 | 1.014x | 1.267x | 33/36 | 0.891x |
| perft_full_d3 | 1.002x | 1.354x | 36/36 | 1.141x |
| polyglot_key | 0.990x | 32.140x | 12/12 | 16.093x |
| polyglot_recompute | 0.975x | 1.041x | 8/12 | 0.868x |
| repetition_key | 0.976x | 20.164x | 12/12 | 14.508x |
| replay_checked_native | 0.982x | 1.123x | 11/12 | 0.868x |
| replay_trusted_native | 0.953x | 0.828x | 5/12 | 0.585x |
| san_parse_all | 0.982x | 1.840x | 36/36 | 1.475x |
| san_render_native | 0.987x | 2.476x | 36/36 | 1.900x |
| snapshot | 1.031x | 1.052x | 25/36 | 0.633x |
| startpos | 1.036x | 1.286x | 5/6 | 0.999x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.585x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.622x |
| macos-15-native | snapshot | tactical | ferrum | 0.633x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.652x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.657x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.690x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.732x |
| macos-15-native | snapshot | promotion | cozy | 0.749x |
| macos-15-native | materialize | kiwipete | ultra | 0.774x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.868x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.868x |
| macos-15-native | fen_parse | middlegame | ultra | 0.891x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.891x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.909x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.919x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.926x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.940x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.952x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.954x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.955x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.961x |
| macos-15-native | snapshot | middlegame | ferrum | 0.963x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.973x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.975x |
| macos-15-native | apply_children_full | promotion | chess | 0.975x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.983x |
| ubuntu-24.04-native | fen_parse | endgame | ultra | 0.992x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.993x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.994x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.997x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.999x |
| windows-2025-native | startpos | start | shakmaty | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6541 | shakmaty | 11458 | 1.752x |
| ubuntu-24.04-arm-native | 5960 | shakmaty | 6976 | 1.170x |
| ubuntu-24.04-native | 5455 | shakmaty | 7780 | 1.426x |
| ubuntu-24.04-pext | 5572 | shakmaty | 6319 | 1.134x |
| ubuntu-24.04-portable | 5264 | shakmaty | 6847 | 1.301x |
| windows-2025-native | 17700 | ferrum | 22000 | 1.243x |

Cold wins: 6/6 (separate from native cases).
