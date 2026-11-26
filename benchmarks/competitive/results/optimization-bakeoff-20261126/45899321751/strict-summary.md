| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 79 | 75 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 81 | 76 | 9 | 87 |
| ubuntu-24.04-pext | 83 | 83 | 11 | 87 |
| ubuntu-24.04-portable | 80 | 78 | 11 | 87 |
| windows-2025-native | 84 | 84 | 12 | 87 |
| ALL | 486 | 475 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.996x | 1.257x | 34/36 | 0.829x |
| chess960_materialize | 0.967x | 1.497x | 12/12 | 1.319x |
| chess960_perft_d3 | 0.947x | 1.491x | 12/12 | 1.297x |
| corpus_count | 1.154x | 1.350x | 12/12 | 1.035x |
| corpus_materialize | 1.013x | 1.150x | 12/12 | 1.053x |
| corpus_uci_sorted | 0.996x | 1.507x | 12/12 | 1.295x |
| count | 1.011x | 1.474x | 35/36 | 0.969x |
| fen_parse | 0.986x | 1.164x | 35/36 | 0.988x |
| fen_write_native_ep | 1.025x | 1.807x | 36/36 | 1.397x |
| make_unmake_full | 1.001x | 1.281x | 36/36 | 1.004x |
| materialize | 1.032x | 1.404x | 35/36 | 0.888x |
| perft_bulk_d4 | 1.048x | 1.306x | 34/36 | 0.901x |
| perft_full_d3 | 1.006x | 1.303x | 36/36 | 1.078x |
| polyglot_key | 1.017x | 33.310x | 12/12 | 16.030x |
| polyglot_recompute | 1.004x | 1.019x | 8/12 | 0.871x |
| repetition_key | 0.986x | 20.759x | 12/12 | 14.064x |
| replay_checked_native | 1.004x | 1.175x | 12/12 | 1.074x |
| replay_trusted_native | 1.006x | 0.808x | 4/12 | 0.503x |
| san_parse_all | 0.996x | 1.837x | 36/36 | 1.312x |
| san_render_native | 1.001x | 2.502x | 36/36 | 1.802x |
| snapshot | 1.011x | 1.053x | 21/36 | 0.853x |
| startpos | 0.969x | 1.197x | 4/6 | 0.999x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.503x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.582x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.655x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.679x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.684x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.781x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.829x |
| macos-15-native | snapshot | endgame | ferrum | 0.853x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.871x |
| macos-15-native | materialize | middlegame | ultra | 0.888x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.901x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.915x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.943x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.943x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.954x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.954x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.955x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.956x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.959x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.961x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.964x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.964x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.969x |
| ubuntu-24.04-native | snapshot | start | chess | 0.975x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.978x |
| macos-15-native | snapshot | start | ferrum | 0.986x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.990x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.999x |
| ubuntu-24.04-native | snapshot | promotion | cozy | 0.999x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |
| windows-2025-native | startpos | start | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4541 | shakmaty | 8750 | 1.927x |
| ubuntu-24.04-arm-native | 5855 | shakmaty | 8032 | 1.372x |
| ubuntu-24.04-native | 7769 | shakmaty | 11141 | 1.434x |
| ubuntu-24.04-pext | 6467 | shakmaty | 7916 | 1.224x |
| ubuntu-24.04-portable | 5219 | shakmaty | 7285 | 1.396x |
| windows-2025-native | 17700 | shakmaty | 21500 | 1.215x |

Cold wins: 6/6 (separate from native cases).
