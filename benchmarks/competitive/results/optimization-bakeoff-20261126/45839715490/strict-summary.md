| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 78 | 76 | 11 | 87 |
| ubuntu-24.04-arm-native | 73 | 73 | 12 | 87 |
| ubuntu-24.04-native | 79 | 75 | 14 | 87 |
| ubuntu-24.04-pext | 81 | 79 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 73 | 10 | 87 |
| windows-2025-native | 74 | 72 | 15 | 87 |
| ALL | 467 | 448 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.021x | 1.301x | 35/36 | 0.988x |
| chess960_materialize | 1.040x | 1.509x | 12/12 | 1.292x |
| chess960_perft_d3 | 1.018x | 1.519x | 12/12 | 1.388x |
| corpus_count | 1.033x | 1.223x | 11/12 | 0.836x |
| corpus_materialize | 1.017x | 1.155x | 12/12 | 1.042x |
| corpus_uci_sorted | 1.005x | 1.490x | 12/12 | 1.292x |
| count | 0.997x | 1.462x | 36/36 | 1.005x |
| fen_parse | 1.003x | 1.156x | 35/36 | 0.988x |
| fen_write_native_ep | 1.009x | 1.719x | 36/36 | 1.417x |
| make_unmake_full | 1.021x | 1.300x | 36/36 | 1.049x |
| materialize | 1.009x | 1.345x | 33/36 | 0.735x |
| perft_bulk_d4 | 0.978x | 1.244x | 33/36 | 0.897x |
| perft_full_d3 | 1.002x | 1.092x | 25/36 | 0.756x |
| polyglot_key | 1.021x | 34.253x | 12/12 | 15.620x |
| polyglot_recompute | 1.008x | 1.003x | 6/12 | 0.891x |
| repetition_key | 0.993x | 21.706x | 12/12 | 14.091x |
| replay_checked_native | 1.035x | 1.075x | 10/12 | 0.987x |
| replay_trusted_native | 1.037x | 0.765x | 3/12 | 0.531x |
| san_parse_all | 1.004x | 1.855x | 36/36 | 1.375x |
| san_render_native | 1.007x | 2.480x | 36/36 | 1.782x |
| snapshot | 0.997x | 1.052x | 18/36 | 0.706x |
| startpos | 0.975x | 1.257x | 6/6 | 1.182x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.531x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.538x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.573x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.659x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.685x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.699x |
| macos-15-native | snapshot | tactical | chess | 0.706x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.707x |
| macos-15-native | materialize | kiwipete | ultra | 0.735x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.756x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.758x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.788x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.789x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.819x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.836x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.838x |
| windows-2025-native | snapshot | kiwipete | chess | 0.841x |
| ubuntu-24.04-arm-native | perft_full_d3 | endgame | ultra | 0.846x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.847x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.891x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.897x |
| windows-2025-native | perft_full_d3 | tactical | shakmaty | 0.917x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.918x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.922x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.932x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.939x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.940x |
| windows-2025-native | snapshot | tactical | chess | 0.951x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.955x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.958x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.965x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.972x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.978x |
| windows-2025-native | snapshot | endgame | ferrum | 0.978x |
| windows-2025-native | snapshot | start | cozy | 0.980x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| macos-15-native | materialize | middlegame | ultra | 0.985x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.987x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.987x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.988x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.988x |
| macos-15-native | materialize | tactical | ultra | 0.990x |
| ubuntu-24.04-arm-native | perft_full_d3 | start | ultra | 0.992x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.994x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.995x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.997x |
| ubuntu-24.04-native | snapshot | middlegame | chess | 0.999x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.999x |
| ubuntu-24.04-native | snapshot | start | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7459 | shakmaty | 9750 | 1.307x |
| ubuntu-24.04-arm-native | 6640 | shakmaty | 9823 | 1.479x |
| ubuntu-24.04-native | 5329 | shakmaty | 5906 | 1.108x |
| ubuntu-24.04-pext | 5779 | shakmaty | 9374 | 1.622x |
| ubuntu-24.04-portable | 5430 | shakmaty | 6582 | 1.212x |
| windows-2025-native | 18900 | shakmaty | 24300 | 1.286x |

Cold wins: 6/6 (separate from native cases).
