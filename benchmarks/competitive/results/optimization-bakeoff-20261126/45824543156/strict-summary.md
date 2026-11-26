| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 68 | 10 | 87 |
| ubuntu-24.04-arm-native | 78 | 76 | 11 | 87 |
| ubuntu-24.04-native | 86 | 82 | 12 | 87 |
| ubuntu-24.04-pext | 77 | 75 | 15 | 87 |
| ubuntu-24.04-portable | 85 | 76 | 12 | 87 |
| windows-2025-native | 81 | 80 | 11 | 87 |
| ALL | 479 | 457 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.021x | 1.246x | 34/36 | 0.907x |
| chess960_materialize | 1.020x | 1.350x | 12/12 | 1.208x |
| chess960_perft_d3 | 1.019x | 1.402x | 12/12 | 1.094x |
| corpus_count | 1.008x | 1.148x | 9/12 | 0.803x |
| corpus_materialize | 1.018x | 1.125x | 11/12 | 0.941x |
| corpus_uci_sorted | 0.992x | 1.553x | 12/12 | 1.227x |
| count | 1.009x | 1.337x | 36/36 | 1.008x |
| fen_parse | 0.995x | 1.165x | 32/36 | 0.944x |
| fen_write_native_ep | 0.976x | 1.661x | 36/36 | 1.343x |
| make_unmake_full | 1.017x | 1.330x | 36/36 | 1.070x |
| materialize | 0.993x | 1.151x | 31/36 | 0.751x |
| perft_bulk_d4 | 1.007x | 1.220x | 36/36 | 1.005x |
| perft_full_d3 | 1.002x | 1.188x | 31/36 | 0.896x |
| polyglot_key | 1.044x | 28.682x | 12/12 | 16.893x |
| polyglot_recompute | 1.029x | 1.038x | 8/12 | 0.913x |
| repetition_key | 1.131x | 21.642x | 12/12 | 14.502x |
| replay_checked_native | 0.980x | 1.047x | 11/12 | 0.803x |
| replay_trusted_native | 1.022x | 0.799x | 4/12 | 0.537x |
| san_parse_all | 1.000x | 1.983x | 36/36 | 1.364x |
| san_render_native | 1.019x | 2.520x | 36/36 | 1.721x |
| snapshot | 0.996x | 1.156x | 28/36 | 0.617x |
| startpos | 0.768x | 1.003x | 4/6 | 0.444x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | startpos | start | shakmaty | 0.444x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.537x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.556x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| macos-15-native | snapshot | start | ferrum | 0.617x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.674x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.689x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.717x |
| macos-15-native | materialize | kiwipete | ultra | 0.751x |
| macos-15-native | materialize | promotion | ultra | 0.791x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.803x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.803x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.822x |
| macos-15-native | snapshot | endgame | chess | 0.844x |
| macos-15-native | startpos | start | shakmaty | 0.865x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.896x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.907x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.913x |
| macos-15-native | corpus_count | seeded | ultra | 0.919x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.920x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.929x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.930x |
| windows-2025-native | corpus_materialize | seeded | ultra | 0.941x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.944x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.951x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.954x |
| macos-15-native | materialize | middlegame | ultra | 0.954x |
| macos-15-native | materialize | endgame | ultra | 0.956x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.960x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.960x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.964x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.965x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.966x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.972x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.973x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.975x |
| macos-15-native | materialize | tactical | ultra | 0.983x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.985x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.990x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.995x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.995x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6083 | shakmaty | 13250 | 2.178x |
| ubuntu-24.04-arm-native | 6720 | chess | 8360 | 1.244x |
| ubuntu-24.04-native | 9567 | shakmaty | 11091 | 1.159x |
| ubuntu-24.04-pext | 6179 | shakmaty | 10125 | 1.639x |
| ubuntu-24.04-portable | 8677 | shakmaty | 10810 | 1.246x |
| windows-2025-native | 14800 | shakmaty | 21500 | 1.453x |

Cold wins: 6/6 (separate from native cases).
