| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 69 | 12 | 87 |
| ubuntu-24.04-arm-native | 73 | 72 | 12 | 87 |
| ubuntu-24.04-native | 86 | 84 | 12 | 87 |
| ubuntu-24.04-pext | 83 | 83 | 9 | 87 |
| ubuntu-24.04-portable | 81 | 73 | 13 | 87 |
| windows-2025-native | 78 | 75 | 12 | 87 |
| ALL | 474 | 456 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.009x | 1.245x | 36/36 | 1.021x |
| chess960_materialize | 1.067x | 1.451x | 12/12 | 1.225x |
| chess960_perft_d3 | 1.041x | 1.542x | 12/12 | 1.370x |
| corpus_count | 1.033x | 1.265x | 12/12 | 1.014x |
| corpus_materialize | 1.045x | 1.156x | 12/12 | 1.058x |
| corpus_uci_sorted | 1.019x | 1.580x | 12/12 | 1.403x |
| count | 0.993x | 1.487x | 36/36 | 1.021x |
| fen_parse | 0.999x | 1.195x | 35/36 | 0.993x |
| fen_write_native_ep | 0.987x | 1.778x | 36/36 | 1.422x |
| make_unmake_full | 0.987x | 1.286x | 34/36 | 0.873x |
| materialize | 1.018x | 1.324x | 33/36 | 0.843x |
| perft_bulk_d4 | 1.006x | 1.310x | 34/36 | 0.914x |
| perft_full_d3 | 0.983x | 1.080x | 24/36 | 0.761x |
| polyglot_key | 1.000x | 31.154x | 12/12 | 16.989x |
| polyglot_recompute | 1.079x | 1.064x | 11/12 | 0.955x |
| repetition_key | 1.111x | 21.051x | 12/12 | 15.112x |
| replay_checked_native | 1.006x | 1.043x | 9/12 | 0.870x |
| replay_trusted_native | 0.988x | 0.794x | 4/12 | 0.508x |
| san_parse_all | 0.982x | 1.901x | 36/36 | 1.472x |
| san_render_native | 1.002x | 2.497x | 36/36 | 1.855x |
| snapshot | 0.963x | 1.053x | 22/36 | 0.486x |
| startpos | 1.000x | 1.175x | 4/6 | 0.939x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | start | ferrum | 0.486x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.508x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.567x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.633x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.664x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.676x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.728x |
| macos-15-native | snapshot | kiwipete | cozy | 0.737x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.761x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.761x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.773x |
| macos-15-native | snapshot | endgame | ferrum | 0.793x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.799x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.825x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.836x |
| macos-15-native | materialize | kiwipete | ultra | 0.843x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.853x |
| ubuntu-24.04-arm-native | perft_full_d3 | endgame | ultra | 0.859x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.870x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.873x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.879x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.910x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.914x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.916x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.917x |
| macos-15-native | snapshot | promotion | chess | 0.920x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.926x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.928x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.928x |
| macos-15-native | materialize | middlegame | ultra | 0.930x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.936x |
| macos-15-native | startpos | start | shakmaty | 0.939x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.941x |
| macos-15-native | perft_full_d3 | tactical | ultra | 0.949x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.955x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.956x |
| macos-15-native | materialize | tactical | ultra | 0.965x |
| ubuntu-24.04-arm-native | perft_full_d3 | start | ultra | 0.972x |
| macos-15-native | snapshot | tactical | ferrum | 0.978x |
| windows-2025-native | startpos | start | shakmaty | 0.982x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.986x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.987x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.992x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.993x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.996x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 2667 | shakmaty | 5000 | 1.875x |
| ubuntu-24.04-arm-native | 6320 | shakmaty | 9743 | 1.542x |
| ubuntu-24.04-native | 7714 | shakmaty | 10630 | 1.378x |
| ubuntu-24.04-pext | 5354 | shakmaty | 8862 | 1.655x |
| ubuntu-24.04-portable | 5692 | shakmaty | 6652 | 1.169x |
| windows-2025-native | 13000 | shakmaty | 17900 | 1.377x |

Cold wins: 6/6 (separate from native cases).
