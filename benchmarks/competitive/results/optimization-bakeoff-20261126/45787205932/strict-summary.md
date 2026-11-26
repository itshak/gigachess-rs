| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 69 | 14 | 87 |
| ubuntu-24.04-arm-native | 77 | 74 | 11 | 87 |
| ubuntu-24.04-native | 79 | 74 | 14 | 87 |
| ubuntu-24.04-pext | 73 | 71 | 16 | 87 |
| ubuntu-24.04-portable | 81 | 75 | 10 | 87 |
| windows-2025-native | 77 | 75 | 11 | 87 |
| ALL | 458 | 438 | 76 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.036x | 1.069x | 28/36 | 0.767x |
| chess960_materialize | 1.005x | 1.319x | 12/12 | 1.124x |
| chess960_perft_d3 | 0.975x | 1.381x | 12/12 | 1.244x |
| corpus_count | 1.010x | 1.343x | 12/12 | 1.028x |
| corpus_materialize | 1.035x | 1.232x | 12/12 | 1.066x |
| corpus_uci_sorted | 1.001x | 1.501x | 12/12 | 1.219x |
| count | 1.004x | 1.323x | 35/36 | 0.978x |
| fen_parse | 1.019x | 1.200x | 34/36 | 0.941x |
| fen_write_native_ep | 1.004x | 1.717x | 36/36 | 1.374x |
| make_unmake_full | 1.001x | 1.268x | 35/36 | 0.795x |
| materialize | 0.985x | 1.208x | 33/36 | 0.627x |
| perft_bulk_d4 | 1.006x | 1.278x | 36/36 | 1.021x |
| perft_full_d3 | 0.930x | 1.142x | 26/36 | 0.716x |
| polyglot_key | 0.933x | 28.945x | 12/12 | 15.838x |
| polyglot_recompute | 1.016x | 1.013x | 7/12 | 0.866x |
| repetition_key | 0.959x | 18.485x | 12/12 | 13.496x |
| replay_checked_native | 0.992x | 0.993x | 7/12 | 0.856x |
| replay_trusted_native | 1.002x | 0.639x | 0/12 | 0.469x |
| san_parse_all | 0.981x | 1.971x | 36/36 | 1.504x |
| san_render_native | 0.997x | 2.535x | 36/36 | 1.507x |
| snapshot | 1.023x | 0.993x | 19/36 | 0.623x |
| startpos | 1.031x | 1.196x | 6/6 | 1.127x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.469x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.472x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.485x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.545x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.582x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.613x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.613x |
| macos-15-native | snapshot | promotion | ferrum | 0.623x |
| macos-15-native | materialize | kiwipete | ultra | 0.627x |
| macos-15-native | snapshot | middlegame | ferrum | 0.636x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.699x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.716x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.722x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.765x |
| macos-15-native | apply_children_full | promotion | ferrum | 0.767x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.795x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.810x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.817x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.826x |
| macos-15-native | snapshot | start | ferrum | 0.830x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.837x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.839x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.841x |
| macos-15-native | apply_children_full | tactical | chess | 0.845x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.855x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.856x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.858x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.858x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.859x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.859x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.866x |
| macos-15-native | snapshot | tactical | cozy | 0.866x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.882x |
| macos-15-native | materialize | middlegame | ultra | 0.883x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.886x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.894x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.901x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.911x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.916x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.917x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.917x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.917x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.926x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.926x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.926x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.927x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.931x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.934x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.934x |
| macos-15-native | materialize | tactical | ultra | 0.938x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.939x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.941x |
| windows-2025-native | perft_full_d3 | endgame | ultra | 0.946x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.964x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.965x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.978x |
| windows-2025-native | apply_children_full | promotion | chess | 0.980x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.981x |
| ubuntu-24.04-pext | apply_children_full | kiwipete | cozy | 0.987x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.993x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.994x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.995x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.997x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7375 | shakmaty | 5958 | 0.808x |
| ubuntu-24.04-arm-native | 12112 | shakmaty | 9792 | 0.808x |
| ubuntu-24.04-native | 14236 | shakmaty | 8205 | 0.576x |
| ubuntu-24.04-pext | 10926 | shakmaty | 9604 | 0.879x |
| ubuntu-24.04-portable | 12326 | shakmaty | 8472 | 0.687x |
| windows-2025-native | 19400 | shakmaty | 13900 | 0.716x |

Cold wins: 0/6 (separate from native cases).
