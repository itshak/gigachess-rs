| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 74 | 12 | 87 |
| ubuntu-24.04-arm-native | 75 | 74 | 10 | 87 |
| ubuntu-24.04-native | 84 | 83 | 8 | 87 |
| ubuntu-24.04-pext | 76 | 75 | 9 | 87 |
| ubuntu-24.04-portable | 75 | 74 | 9 | 87 |
| windows-2025-native | 76 | 73 | 12 | 87 |
| ALL | 463 | 453 | 60 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.094x | 1.181x | 34/36 | 0.865x |
| chess960_materialize | 1.028x | 1.366x | 12/12 | 1.126x |
| chess960_perft_d3 | 1.002x | 1.356x | 12/12 | 1.257x |
| corpus_count | 0.999x | 1.332x | 11/12 | 0.896x |
| corpus_materialize | 0.972x | 1.153x | 12/12 | 1.047x |
| corpus_uci_sorted | 1.006x | 1.490x | 12/12 | 1.221x |
| count | 1.005x | 1.296x | 33/36 | 0.946x |
| fen_parse | 1.028x | 1.199x | 34/36 | 0.897x |
| fen_write_native_ep | 1.016x | 1.212x | 35/36 | 0.990x |
| make_unmake_full | 1.041x | 1.281x | 35/36 | 0.992x |
| materialize | 1.003x | 1.259x | 33/36 | 0.862x |
| perft_bulk_d4 | 0.996x | 1.255x | 36/36 | 1.033x |
| perft_full_d3 | 1.016x | 1.222x | 27/36 | 0.827x |
| polyglot_key | 0.970x | 35.592x | 12/12 | 17.035x |
| polyglot_recompute | 0.978x | 1.006x | 7/12 | 0.890x |
| repetition_key | 0.981x | 21.545x | 12/12 | 14.987x |
| replay_checked_native | 1.067x | 1.081x | 9/12 | 0.946x |
| replay_trusted_native | 1.114x | 0.697x | 1/12 | 0.497x |
| san_parse_all | 1.000x | 1.871x | 36/36 | 1.468x |
| san_render_native | 1.005x | 2.513x | 36/36 | 1.708x |
| snapshot | 1.017x | 1.029x | 18/36 | 0.812x |
| startpos | 1.038x | 1.217x | 6/6 | 1.120x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.497x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.503x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.509x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.560x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.601x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.617x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.686x |
| macos-15-native | snapshot | promotion | ferrum | 0.812x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.827x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.838x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.845x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.856x |
| macos-15-native | materialize | middlegame | ultra | 0.862x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.865x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.870x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.873x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.878x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.883x |
| macos-15-native | materialize | kiwipete | ultra | 0.886x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.890x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.893x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.896x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.897x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.897x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.901x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.911x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.917x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.919x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.925x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.931x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.941x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.942x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.946x |
| macos-15-native | count | middlegame | ferrum | 0.946x |
| macos-15-native | materialize | tactical | ultra | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.953x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.967x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.974x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.975x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.975x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.978x |
| macos-15-native | snapshot | start | ferrum | 0.982x |
| ubuntu-24.04-pext | count | endgame | cozy | 0.982x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.983x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.983x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.983x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.986x |
| ubuntu-24.04-portable | snapshot | endgame | chess | 0.989x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.990x |
| ubuntu-24.04-portable | snapshot | start | cozy | 0.990x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.992x |
| windows-2025-native | snapshot | promotion | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | tactical | chess | 0.996x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.997x |
| windows-2025-native | snapshot | start | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | promotion | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6833 | shakmaty | 9041 | 1.323x |
| ubuntu-24.04-arm-native | 11864 | shakmaty | 8224 | 0.693x |
| ubuntu-24.04-native | 9593 | shakmaty | 6840 | 0.713x |
| ubuntu-24.04-pext | 13972 | cozy | 9474 | 0.678x |
| ubuntu-24.04-portable | 8914 | shakmaty | 7261 | 0.815x |
| windows-2025-native | 16200 | shakmaty | 14300 | 0.883x |

Cold wins: 1/6 (separate from native cases).
