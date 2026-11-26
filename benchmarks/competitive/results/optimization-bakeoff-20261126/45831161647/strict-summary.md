| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 68 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 75 | 11 | 87 |
| ubuntu-24.04-native | 81 | 78 | 15 | 87 |
| ubuntu-24.04-pext | 81 | 79 | 9 | 87 |
| ubuntu-24.04-portable | 84 | 75 | 11 | 87 |
| windows-2025-native | 79 | 77 | 11 | 87 |
| ALL | 473 | 452 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.961x | 1.192x | 32/36 | 0.833x |
| chess960_materialize | 1.011x | 1.398x | 12/12 | 1.211x |
| chess960_perft_d3 | 0.977x | 1.359x | 12/12 | 1.091x |
| corpus_count | 1.002x | 1.121x | 8/12 | 0.841x |
| corpus_materialize | 0.974x | 1.079x | 10/12 | 0.863x |
| corpus_uci_sorted | 1.015x | 1.557x | 12/12 | 1.395x |
| count | 1.010x | 1.313x | 35/36 | 0.740x |
| fen_parse | 1.031x | 1.189x | 35/36 | 1.000x |
| fen_write_native_ep | 0.999x | 1.702x | 36/36 | 1.343x |
| make_unmake_full | 0.979x | 1.295x | 35/36 | 0.964x |
| materialize | 1.017x | 1.214x | 32/36 | 0.888x |
| perft_bulk_d4 | 1.005x | 1.161x | 32/36 | 0.873x |
| perft_full_d3 | 1.010x | 1.220x | 33/36 | 0.912x |
| polyglot_key | 1.009x | 30.480x | 12/12 | 16.209x |
| polyglot_recompute | 0.973x | 1.018x | 7/12 | 0.902x |
| repetition_key | 1.023x | 20.418x | 12/12 | 14.357x |
| replay_checked_native | 0.969x | 1.071x | 11/12 | 0.975x |
| replay_trusted_native | 0.951x | 0.804x | 4/12 | 0.506x |
| san_parse_all | 1.010x | 1.924x | 36/36 | 1.308x |
| san_render_native | 1.016x | 2.507x | 36/36 | 1.633x |
| snapshot | 0.985x | 1.116x | 25/36 | 0.814x |
| startpos | 0.963x | 1.198x | 6/6 | 1.004x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.506x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.564x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.638x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.717x |
| macos-15-native | count | middlegame | cozy | 0.740x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.788x |
| macos-15-native | snapshot | tactical | ferrum | 0.814x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.833x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.841x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.863x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.873x |
| macos-15-native | materialize | middlegame | ultra | 0.888x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.890x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.894x |
| macos-15-native | apply_children_full | tactical | chess | 0.899x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.902x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.903x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.912x |
| macos-15-native | snapshot | middlegame | ferrum | 0.917x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.918x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.920x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.921x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.922x |
| macos-15-native | materialize | tactical | ultra | 0.924x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.928x |
| macos-15-native | snapshot | start | ferrum | 0.939x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.945x |
| macos-15-native | materialize | kiwipete | ultra | 0.947x |
| windows-2025-native | corpus_materialize | seeded | ultra | 0.949x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.956x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.956x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.957x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.959x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.961x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.962x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.962x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.964x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.964x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.965x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.975x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.977x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.978x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.982x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.985x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.988x |
| macos-15-native | snapshot | promotion | ferrum | 0.989x |
| ubuntu-24.04-portable | fen_parse | kiwipete | ultra | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6375 | shakmaty | 10875 | 1.706x |
| ubuntu-24.04-arm-native | 6520 | chess | 9497 | 1.457x |
| ubuntu-24.04-native | 5999 | shakmaty | 9034 | 1.506x |
| ubuntu-24.04-pext | 5555 | chess | 9103 | 1.639x |
| ubuntu-24.04-portable | 7955 | shakmaty | 10029 | 1.261x |
| windows-2025-native | 11700 | shakmaty | 16900 | 1.444x |

Cold wins: 6/6 (separate from native cases).
