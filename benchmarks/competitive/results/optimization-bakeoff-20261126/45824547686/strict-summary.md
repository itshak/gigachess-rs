| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 67 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 78 | 11 | 87 |
| ubuntu-24.04-native | 74 | 68 | 13 | 87 |
| ubuntu-24.04-pext | 80 | 77 | 9 | 87 |
| ubuntu-24.04-portable | 78 | 71 | 9 | 87 |
| windows-2025-native | 79 | 77 | 11 | 87 |
| ALL | 459 | 438 | 64 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.991x | 1.232x | 33/36 | 0.968x |
| chess960_materialize | 0.994x | 1.408x | 12/12 | 1.166x |
| chess960_perft_d3 | 1.002x | 1.342x | 12/12 | 1.110x |
| corpus_count | 0.999x | 0.989x | 5/12 | 0.826x |
| corpus_materialize | 1.020x | 1.093x | 12/12 | 1.004x |
| corpus_uci_sorted | 1.068x | 1.489x | 12/12 | 1.359x |
| count | 0.978x | 1.357x | 35/36 | 0.956x |
| fen_parse | 0.975x | 1.136x | 33/36 | 0.976x |
| fen_write_native_ep | 0.973x | 1.777x | 36/36 | 1.142x |
| make_unmake_full | 0.995x | 1.266x | 36/36 | 1.016x |
| materialize | 1.009x | 1.357x | 32/36 | 0.769x |
| perft_bulk_d4 | 0.995x | 1.094x | 28/36 | 0.832x |
| perft_full_d3 | 1.009x | 1.383x | 35/36 | 0.942x |
| polyglot_key | 0.987x | 35.253x | 12/12 | 16.483x |
| polyglot_recompute | 0.954x | 0.997x | 6/12 | 0.886x |
| repetition_key | 0.961x | 17.724x | 12/12 | 8.406x |
| replay_checked_native | 1.026x | 1.049x | 11/12 | 0.993x |
| replay_trusted_native | 0.996x | 0.768x | 3/12 | 0.528x |
| san_parse_all | 0.995x | 1.866x | 36/36 | 1.523x |
| san_render_native | 1.005x | 2.510x | 36/36 | 1.760x |
| snapshot | 0.979x | 1.043x | 17/36 | 0.685x |
| startpos | 1.017x | 1.237x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.528x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.559x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.630x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.662x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| macos-15-native | snapshot | start | cozy | 0.685x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.709x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.718x |
| macos-15-native | materialize | kiwipete | ultra | 0.769x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.805x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.826x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.832x |
| macos-15-native | corpus_count | seeded | ultra | 0.868x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.873x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.875x |
| macos-15-native | materialize | tactical | ultra | 0.877x |
| macos-15-native | snapshot | promotion | ferrum | 0.881x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.886x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.900x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.904x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.904x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.911x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.916x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.916x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.928x |
| windows-2025-native | materialize | middlegame | cozy | 0.933x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.938x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.938x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.942x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.943x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.946x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.950x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.956x |
| macos-15-native | apply_children_full | tactical | cozy | 0.968x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.969x |
| macos-15-native | snapshot | middlegame | cozy | 0.972x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.976x |
| macos-15-native | materialize | promotion | ultra | 0.977x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.978x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.981x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.983x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.984x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.985x |
| ubuntu-24.04-native | perft_bulk_d4 | tactical | ultra | 0.985x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.991x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.993x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.995x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.996x |
| windows-2025-native | startpos | start | shakmaty | 0.997x |
| ubuntu-24.04-pext | fen_parse | middlegame | ultra | 0.997x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.998x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |
| windows-2025-native | corpus_count | seeded | ultra | 0.999x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6458 | shakmaty | 10417 | 1.613x |
| ubuntu-24.04-arm-native | 6888 | chess | 9920 | 1.440x |
| ubuntu-24.04-native | 5253 | shakmaty | 6118 | 1.165x |
| ubuntu-24.04-pext | 6577 | shakmaty | 8512 | 1.294x |
| ubuntu-24.04-portable | 5489 | shakmaty | 6879 | 1.253x |
| windows-2025-native | 21100 | shakmaty | 26500 | 1.256x |

Cold wins: 6/6 (separate from native cases).
