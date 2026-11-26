| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 69 | 11 | 87 |
| ubuntu-24.04-arm-native | 77 | 77 | 10 | 87 |
| ubuntu-24.04-native | 75 | 69 | 12 | 87 |
| ubuntu-24.04-pext | 82 | 77 | 12 | 87 |
| ubuntu-24.04-portable | 78 | 77 | 12 | 87 |
| windows-2025-native | 81 | 75 | 14 | 87 |
| ALL | 464 | 444 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.001x | 1.180x | 34/36 | 0.885x |
| chess960_materialize | 1.009x | 1.390x | 12/12 | 1.240x |
| chess960_perft_d3 | 0.993x | 1.369x | 12/12 | 1.107x |
| corpus_count | 0.989x | 1.049x | 6/12 | 0.785x |
| corpus_materialize | 1.005x | 1.090x | 11/12 | 0.995x |
| corpus_uci_sorted | 1.001x | 1.525x | 12/12 | 1.372x |
| count | 1.015x | 1.335x | 36/36 | 1.001x |
| fen_parse | 0.971x | 1.191x | 35/36 | 0.992x |
| fen_write_native_ep | 0.986x | 1.762x | 36/36 | 1.442x |
| make_unmake_full | 1.002x | 1.252x | 36/36 | 1.025x |
| materialize | 1.011x | 1.281x | 33/36 | 0.819x |
| perft_bulk_d4 | 1.000x | 1.139x | 33/36 | 0.839x |
| perft_full_d3 | 0.991x | 1.310x | 35/36 | 0.936x |
| polyglot_key | 0.989x | 31.626x | 12/12 | 16.450x |
| polyglot_recompute | 0.980x | 1.034x | 8/12 | 0.962x |
| repetition_key | 0.955x | 17.541x | 12/12 | 14.169x |
| replay_checked_native | 0.988x | 1.068x | 10/12 | 0.977x |
| replay_trusted_native | 0.996x | 0.709x | 2/12 | 0.495x |
| san_parse_all | 1.033x | 1.930x | 36/36 | 1.414x |
| san_render_native | 1.016x | 2.549x | 36/36 | 1.891x |
| snapshot | 0.997x | 1.002x | 11/36 | 0.838x |
| startpos | 1.065x | 1.254x | 6/6 | 1.120x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.495x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.529x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.561x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.609x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.645x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.662x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.676x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.684x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.785x |
| macos-15-native | materialize | kiwipete | ultra | 0.819x |
| macos-15-native | snapshot | start | ferrum | 0.838x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.839x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.884x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.885x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.889x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.903x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.906x |
| macos-15-native | materialize | middlegame | ultra | 0.929x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.934x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.934x |
| macos-15-native | materialize | tactical | ultra | 0.935x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.936x |
| macos-15-native | corpus_count | seeded | ultra | 0.936x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.936x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.943x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.946x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.958x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.962x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.963x |
| macos-15-native | snapshot | middlegame | ferrum | 0.965x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.966x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.969x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.969x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.970x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.972x |
| windows-2025-native | snapshot | endgame | cozy | 0.974x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.977x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.978x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.981x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.985x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 0.990x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.992x |
| windows-2025-native | snapshot | promotion | cozy | 0.992x |
| windows-2025-native | snapshot | start | chess | 0.994x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.995x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.995x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.996x |
| macos-15-native | snapshot | promotion | ferrum | 0.997x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.998x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.998x |
| macos-15-native | snapshot | endgame | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.999x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.999x |
| macos-15-native | apply_children_full | tactical | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3250 | shakmaty | 5583 | 1.718x |
| ubuntu-24.04-arm-native | 6408 | shakmaty | 8463 | 1.321x |
| ubuntu-24.04-native | 5517 | shakmaty | 7187 | 1.303x |
| ubuntu-24.04-pext | 4833 | shakmaty | 7667 | 1.586x |
| ubuntu-24.04-portable | 7464 | shakmaty | 10149 | 1.360x |
| windows-2025-native | 14800 | shakmaty | 22200 | 1.500x |

Cold wins: 6/6 (separate from native cases).
