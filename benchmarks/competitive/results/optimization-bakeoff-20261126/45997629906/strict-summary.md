| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 70 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 82 | 81 | 9 | 87 |
| ubuntu-24.04-pext | 83 | 78 | 13 | 87 |
| ubuntu-24.04-portable | 83 | 75 | 12 | 87 |
| windows-2025-native | 83 | 83 | 12 | 87 |
| ALL | 485 | 466 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.010x | 1.289x | 35/36 | 0.925x |
| chess960_materialize | 1.006x | 1.503x | 12/12 | 1.288x |
| chess960_perft_d3 | 1.001x | 1.498x | 12/12 | 1.354x |
| corpus_count | 1.005x | 1.174x | 9/12 | 0.821x |
| corpus_materialize | 1.027x | 1.152x | 12/12 | 1.047x |
| corpus_uci_sorted | 1.037x | 1.543x | 12/12 | 1.354x |
| count | 0.981x | 1.464x | 36/36 | 1.025x |
| fen_parse | 1.008x | 1.188x | 34/36 | 0.928x |
| fen_write_native_ep | 1.017x | 1.706x | 36/36 | 1.360x |
| make_unmake_full | 0.973x | 1.265x | 35/36 | 0.926x |
| materialize | 1.024x | 1.336x | 33/36 | 0.885x |
| perft_bulk_d4 | 1.006x | 1.275x | 33/36 | 0.921x |
| perft_full_d3 | 1.002x | 1.280x | 35/36 | 0.940x |
| polyglot_key | 1.015x | 33.968x | 12/12 | 16.754x |
| polyglot_recompute | 1.047x | 1.013x | 8/12 | 0.873x |
| repetition_key | 0.984x | 22.104x | 12/12 | 14.794x |
| replay_checked_native | 1.010x | 1.141x | 12/12 | 1.015x |
| replay_trusted_native | 1.010x | 0.733x | 4/12 | 0.471x |
| san_parse_all | 1.004x | 1.851x | 36/36 | 1.342x |
| san_render_native | 0.996x | 2.618x | 36/36 | 1.852x |
| snapshot | 1.016x | 1.164x | 26/36 | 0.921x |
| startpos | 0.983x | 1.192x | 5/6 | 0.998x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.471x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.517x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.546x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.560x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.656x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.657x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.815x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.821x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.823x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.873x |
| macos-15-native | materialize | tactical | ultra | 0.885x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.921x |
| macos-15-native | snapshot | promotion | ferrum | 0.921x |
| macos-15-native | apply_children_full | kiwipete | cozy | 0.925x |
| ubuntu-24.04-native | make_unmake_full | kiwipete | ultra | 0.926x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.928x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.928x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.935x |
| ubuntu-24.04-arm-native | snapshot | endgame | cozy | 0.938x |
| macos-15-native | perft_full_d3 | endgame | ultra | 0.940x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.949x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.961x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.965x |
| macos-15-native | materialize | kiwipete | ultra | 0.975x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| macos-15-native | materialize | middlegame | ultra | 0.991x |
| macos-15-native | snapshot | tactical | ferrum | 0.991x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.993x |
| windows-2025-native | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.999x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6500 | shakmaty | 10375 | 1.596x |
| ubuntu-24.04-arm-native | 5272 | shakmaty | 7960 | 1.510x |
| ubuntu-24.04-native | 7641 | shakmaty | 9274 | 1.214x |
| ubuntu-24.04-pext | 5698 | shakmaty | 7064 | 1.240x |
| ubuntu-24.04-portable | 6552 | shakmaty | 5780 | 0.882x |
| windows-2025-native | 16900 | shakmaty | 21300 | 1.260x |

Cold wins: 5/6 (separate from native cases).
