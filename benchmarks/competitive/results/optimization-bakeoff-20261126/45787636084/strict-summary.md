| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 68 | 65 | 14 | 87 |
| ubuntu-24.04-arm-native | 77 | 77 | 11 | 87 |
| ubuntu-24.04-native | 78 | 75 | 9 | 87 |
| ubuntu-24.04-pext | 77 | 69 | 8 | 87 |
| ubuntu-24.04-portable | 79 | 73 | 14 | 87 |
| windows-2025-native | 82 | 81 | 11 | 87 |
| ALL | 461 | 440 | 67 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.042x | 1.125x | 32/36 | 0.821x |
| chess960_materialize | 1.011x | 1.317x | 12/12 | 1.087x |
| chess960_perft_d3 | 0.963x | 1.361x | 12/12 | 1.200x |
| corpus_count | 0.765x | 1.013x | 6/12 | 0.808x |
| corpus_materialize | 0.963x | 1.121x | 12/12 | 1.015x |
| corpus_uci_sorted | 0.958x | 1.470x | 12/12 | 1.271x |
| count | 0.982x | 1.330x | 35/36 | 0.956x |
| fen_parse | 0.980x | 1.170x | 35/36 | 0.992x |
| fen_write_native_ep | 0.988x | 1.780x | 36/36 | 1.078x |
| make_unmake_full | 0.996x | 1.237x | 36/36 | 1.019x |
| materialize | 0.992x | 1.336x | 34/36 | 0.719x |
| perft_bulk_d4 | 0.850x | 1.101x | 29/36 | 0.838x |
| perft_full_d3 | 0.993x | 1.310x | 34/36 | 0.933x |
| polyglot_key | 0.978x | 36.356x | 12/12 | 16.511x |
| polyglot_recompute | 0.995x | 1.029x | 9/12 | 0.896x |
| repetition_key | 0.999x | 19.499x | 12/12 | 14.347x |
| replay_checked_native | 0.996x | 1.023x | 8/12 | 0.800x |
| replay_trusted_native | 0.999x | 0.640x | 0/12 | 0.442x |
| san_parse_all | 0.987x | 1.890x | 36/36 | 1.478x |
| san_render_native | 0.984x | 2.499x | 36/36 | 1.866x |
| snapshot | 1.249x | 1.068x | 18/36 | 0.710x |
| startpos | 1.051x | 1.221x | 5/6 | 0.998x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.442x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.491x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.509x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.534x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.616x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.634x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.662x |
| macos-15-native | snapshot | endgame | ferrum | 0.710x |
| macos-15-native | materialize | kiwipete | ultra | 0.719x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.733x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.762x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.800x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.805x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.808x |
| macos-15-native | corpus_count | seeded | ultra | 0.809x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.814x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.821x |
| macos-15-native | perft_bulk_d4 | middlegame | ultra | 0.838x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.843x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.844x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.848x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.857x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.874x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.875x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.910x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.917x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.929x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.932x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.932x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.933x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.933x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.933x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.937x |
| macos-15-native | apply_children_full | promotion | cozy | 0.939x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.940x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.941x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.946x |
| macos-15-native | snapshot | start | ferrum | 0.953x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.956x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.970x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.972x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.979x |
| macos-15-native | materialize | middlegame | ultra | 0.981x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.982x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.982x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 0.987x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.991x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.992x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.993x |
| macos-15-native | snapshot | promotion | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.996x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.997x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 10792 | chess | 13917 | 1.290x |
| ubuntu-24.04-arm-native | 4648 | shakmaty | 9009 | 1.938x |
| ubuntu-24.04-native | 5546 | shakmaty | 7662 | 1.382x |
| ubuntu-24.04-pext | 5660 | shakmaty | 7108 | 1.256x |
| ubuntu-24.04-portable | 5523 | shakmaty | 6275 | 1.136x |
| windows-2025-native | 8600 | shakmaty | 12500 | 1.453x |

Cold wins: 6/6 (separate from native cases).
