| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 72 | 11 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 84 | 13 | 87 |
| ubuntu-24.04-pext | 83 | 82 | 8 | 87 |
| ubuntu-24.04-portable | 80 | 79 | 11 | 87 |
| windows-2025-native | 83 | 81 | 12 | 87 |
| ALL | 484 | 477 | 67 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.987x | 1.251x | 36/36 | 1.006x |
| chess960_materialize | 1.019x | 1.507x | 12/12 | 1.308x |
| chess960_perft_d3 | 1.006x | 1.532x | 12/12 | 1.390x |
| corpus_count | 0.993x | 1.283x | 9/12 | 0.966x |
| corpus_materialize | 1.002x | 1.196x | 12/12 | 1.040x |
| corpus_uci_sorted | 1.033x | 1.582x | 12/12 | 1.444x |
| count | 0.996x | 1.498x | 36/36 | 1.103x |
| fen_parse | 0.996x | 1.216x | 36/36 | 1.063x |
| fen_write_native_ep | 0.986x | 1.712x | 36/36 | 1.390x |
| make_unmake_full | 0.993x | 1.321x | 35/36 | 0.994x |
| materialize | 0.967x | 1.240x | 33/36 | 0.747x |
| perft_bulk_d4 | 0.983x | 1.310x | 34/36 | 0.908x |
| perft_full_d3 | 0.994x | 1.311x | 36/36 | 1.087x |
| polyglot_key | 1.006x | 27.752x | 12/12 | 15.899x |
| polyglot_recompute | 0.991x | 1.025x | 7/12 | 0.898x |
| repetition_key | 0.973x | 18.946x | 12/12 | 14.207x |
| replay_checked_native | 0.983x | 1.099x | 12/12 | 1.030x |
| replay_trusted_native | 0.911x | 0.763x | 4/12 | 0.424x |
| san_parse_all | 0.984x | 1.902x | 36/36 | 1.422x |
| san_render_native | 1.004x | 2.542x | 36/36 | 1.580x |
| snapshot | 0.983x | 1.074x | 20/36 | 0.706x |
| startpos | 1.096x | 1.254x | 6/6 | 1.198x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.424x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.551x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.595x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.656x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.661x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.669x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.706x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.724x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.740x |
| macos-15-native | materialize | kiwipete | ultra | 0.747x |
| macos-15-native | materialize | start | ultra | 0.751x |
| macos-15-native | snapshot | start | chess | 0.789x |
| macos-15-native | snapshot | tactical | chess | 0.791x |
| macos-15-native | materialize | middlegame | ultra | 0.860x |
| macos-15-native | snapshot | middlegame | cozy | 0.884x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.898x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.908x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.913x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.922x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.924x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.951x |
| windows-2025-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.966x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.974x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.975x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.977x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.979x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.982x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.986x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.987x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.992x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.993x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.994x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5875 | shakmaty | 10333 | 1.759x |
| ubuntu-24.04-arm-native | 5688 | shakmaty | 8032 | 1.412x |
| ubuntu-24.04-native | 6499 | shakmaty | 8753 | 1.347x |
| ubuntu-24.04-pext | 5016 | cozy | 7946 | 1.584x |
| ubuntu-24.04-portable | 8005 | shakmaty | 10550 | 1.318x |
| windows-2025-native | 17800 | shakmaty | 23500 | 1.320x |

Cold wins: 6/6 (separate from native cases).
