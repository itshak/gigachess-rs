| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 67 | 16 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 13 | 87 |
| ubuntu-24.04-native | 79 | 75 | 15 | 87 |
| ubuntu-24.04-pext | 84 | 80 | 9 | 87 |
| ubuntu-24.04-portable | 79 | 78 | 12 | 87 |
| windows-2025-native | 79 | 78 | 13 | 87 |
| ALL | 467 | 457 | 78 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.949x | 1.245x | 35/36 | 0.940x |
| chess960_materialize | 0.992x | 1.538x | 12/12 | 1.338x |
| chess960_perft_d3 | 0.974x | 1.509x | 12/12 | 1.293x |
| corpus_count | 0.973x | 1.169x | 8/12 | 0.911x |
| corpus_materialize | 0.974x | 1.110x | 11/12 | 0.841x |
| corpus_uci_sorted | 0.999x | 1.544x | 12/12 | 1.262x |
| count | 0.977x | 1.478x | 36/36 | 1.041x |
| fen_parse | 0.993x | 1.177x | 34/36 | 0.985x |
| fen_write_native_ep | 0.995x | 1.770x | 36/36 | 1.119x |
| make_unmake_full | 0.937x | 1.274x | 34/36 | 0.874x |
| materialize | 0.971x | 1.358x | 33/36 | 0.902x |
| perft_bulk_d4 | 0.967x | 1.225x | 31/36 | 0.842x |
| perft_full_d3 | 0.974x | 1.350x | 35/36 | 0.945x |
| polyglot_key | 0.997x | 35.166x | 12/12 | 16.031x |
| polyglot_recompute | 1.004x | 1.035x | 9/12 | 0.911x |
| repetition_key | 1.006x | 21.938x | 12/12 | 14.157x |
| replay_checked_native | 1.000x | 1.150x | 11/12 | 0.940x |
| replay_trusted_native | 1.019x | 0.805x | 3/12 | 0.537x |
| san_parse_all | 0.985x | 1.925x | 36/36 | 1.495x |
| san_render_native | 0.957x | 2.596x | 36/36 | 1.577x |
| snapshot | 0.969x | 0.998x | 13/36 | 0.785x |
| startpos | 1.014x | 1.157x | 6/6 | 1.002x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.537x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.598x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.618x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.637x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.678x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.755x |
| macos-15-native | snapshot | start | ferrum | 0.785x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.799x |
| macos-15-native | snapshot | tactical | chess | 0.804x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.841x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.842x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.860x |
| windows-2025-native | make_unmake_full | kiwipete | ultra | 0.874x |
| macos-15-native | materialize | kiwipete | ultra | 0.902x |
| windows-2025-native | snapshot | tactical | cozy | 0.903x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.911x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.911x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.922x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.929x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.940x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.940x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.945x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.945x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.946x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.947x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.948x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.951x |
| windows-2025-native | corpus_count | seeded | ultra | 0.954x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.957x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.962x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.962x |
| macos-15-native | snapshot | endgame | ferrum | 0.964x |
| macos-15-native | materialize | endgame | ultra | 0.965x |
| windows-2025-native | perft_bulk_d4 | endgame | ultra | 0.965x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.967x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.969x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.969x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.976x |
| macos-15-native | snapshot | middlegame | ferrum | 0.979x |
| macos-15-native | fen_parse | tactical | cozy | 0.985x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.985x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.986x |
| ubuntu-24.04-native | snapshot | start | chess | 0.986x |
| macos-15-native | materialize | tactical | ultra | 0.986x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.988x |
| macos-15-native | fen_parse | promotion | shakmaty | 0.990x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.993x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.997x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3667 | shakmaty | 5959 | 1.625x |
| ubuntu-24.04-arm-native | 5280 | shakmaty | 8455 | 1.601x |
| ubuntu-24.04-native | 6822 | shakmaty | 7562 | 1.108x |
| ubuntu-24.04-pext | 6086 | shakmaty | 7103 | 1.167x |
| ubuntu-24.04-portable | 8667 | shakmaty | 9157 | 1.057x |
| windows-2025-native | 15900 | shakmaty | 19900 | 1.252x |

Cold wins: 6/6 (separate from native cases).
