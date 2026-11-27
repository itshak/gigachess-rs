| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 70 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 78 | 12 | 87 |
| ubuntu-24.04-native | 82 | 79 | 9 | 87 |
| ubuntu-24.04-pext | 79 | 79 | 13 | 87 |
| ubuntu-24.04-portable | 79 | 77 | 15 | 87 |
| windows-2025-native | 81 | 78 | 13 | 87 |
| ALL | 474 | 461 | 76 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.989x | 1.244x | 34/36 | 0.825x |
| chess960_materialize | 0.974x | 1.437x | 12/12 | 1.166x |
| chess960_perft_d3 | 0.980x | 1.482x | 12/12 | 1.279x |
| corpus_count | 1.002x | 1.259x | 11/12 | 0.801x |
| corpus_materialize | 0.978x | 1.162x | 12/12 | 1.071x |
| corpus_uci_sorted | 0.982x | 1.509x | 12/12 | 1.078x |
| count | 0.992x | 1.466x | 35/36 | 0.925x |
| fen_parse | 0.963x | 1.168x | 35/36 | 0.947x |
| fen_write_native_ep | 1.016x | 1.723x | 36/36 | 1.394x |
| make_unmake_full | 1.016x | 1.317x | 35/36 | 0.972x |
| materialize | 0.973x | 1.283x | 34/36 | 0.820x |
| perft_bulk_d4 | 0.994x | 1.285x | 33/36 | 0.903x |
| perft_full_d3 | 0.993x | 1.292x | 36/36 | 1.037x |
| polyglot_key | 0.993x | 30.543x | 12/12 | 16.017x |
| polyglot_recompute | 0.970x | 0.989x | 7/12 | 0.841x |
| repetition_key | 1.011x | 21.218x | 12/12 | 13.737x |
| replay_checked_native | 1.017x | 1.155x | 12/12 | 1.021x |
| replay_trusted_native | 0.990x | 0.755x | 4/12 | 0.512x |
| san_parse_all | 1.002x | 1.851x | 36/36 | 1.364x |
| san_render_native | 0.998x | 2.568x | 36/36 | 1.825x |
| snapshot | 0.944x | 1.045x | 12/36 | 0.745x |
| startpos | 0.993x | 1.304x | 6/6 | 1.228x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.512x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.538x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.576x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.664x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.667x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.684x |
| macos-15-native | snapshot | start | ferrum | 0.745x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.747x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.801x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.811x |
| macos-15-native | materialize | middlegame | ultra | 0.820x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.825x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.841x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.858x |
| macos-15-native | snapshot | endgame | ferrum | 0.888x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.897x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.900x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.903x |
| macos-15-native | apply_children_full | promotion | cozy | 0.907x |
| macos-15-native | materialize | kiwipete | ultra | 0.914x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.925x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.942x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.944x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.954x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.961x |
| macos-15-native | snapshot | tactical | ferrum | 0.964x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.965x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.969x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.972x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.973x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.973x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.973x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.974x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.981x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.981x |
| macos-15-native | perft_bulk_d4 | endgame | ultra | 0.983x |
| windows-2025-native | snapshot | kiwipete | chess | 0.987x |
| windows-2025-native | snapshot | promotion | ferrum | 0.988x |
| windows-2025-native | snapshot | tactical | cozy | 0.992x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.994x |
| windows-2025-native | snapshot | endgame | cozy | 0.997x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6834 | shakmaty | 12750 | 1.866x |
| ubuntu-24.04-arm-native | 5671 | ferrum | 9920 | 1.749x |
| ubuntu-24.04-native | 6259 | chess | 9785 | 1.563x |
| ubuntu-24.04-pext | 7634 | shakmaty | 12063 | 1.580x |
| ubuntu-24.04-portable | 7059 | chess | 8446 | 1.196x |
| windows-2025-native | 17900 | shakmaty | 26000 | 1.453x |

Cold wins: 6/6 (separate from native cases).
