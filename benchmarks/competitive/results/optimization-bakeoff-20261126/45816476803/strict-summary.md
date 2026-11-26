| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 70 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 77 | 75 | 14 | 87 |
| ubuntu-24.04-pext | 77 | 75 | 12 | 87 |
| ubuntu-24.04-portable | 81 | 73 | 9 | 87 |
| windows-2025-native | 82 | 81 | 11 | 87 |
| ALL | 470 | 451 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.030x | 1.200x | 35/36 | 0.981x |
| chess960_materialize | 0.994x | 1.387x | 12/12 | 1.215x |
| chess960_perft_d3 | 0.985x | 1.374x | 12/12 | 1.157x |
| corpus_count | 0.999x | 1.133x | 9/12 | 0.800x |
| corpus_materialize | 1.005x | 1.124x | 12/12 | 1.008x |
| corpus_uci_sorted | 0.991x | 1.530x | 12/12 | 1.371x |
| count | 0.993x | 1.304x | 35/36 | 0.955x |
| fen_parse | 0.993x | 1.214x | 36/36 | 1.082x |
| fen_write_native_ep | 0.996x | 1.703x | 36/36 | 1.414x |
| make_unmake_full | 0.995x | 1.253x | 35/36 | 0.921x |
| materialize | 1.024x | 1.218x | 31/36 | 0.887x |
| perft_bulk_d4 | 0.997x | 1.184x | 34/36 | 0.914x |
| perft_full_d3 | 1.007x | 1.216x | 31/36 | 0.880x |
| polyglot_key | 0.984x | 29.938x | 12/12 | 16.625x |
| polyglot_recompute | 0.998x | 1.004x | 6/12 | 0.902x |
| repetition_key | 0.991x | 18.367x | 12/12 | 14.282x |
| replay_checked_native | 0.995x | 1.033x | 10/12 | 0.922x |
| replay_trusted_native | 1.019x | 0.709x | 2/12 | 0.485x |
| san_parse_all | 1.011x | 1.931x | 36/36 | 1.403x |
| san_render_native | 0.999x | 2.494x | 36/36 | 1.560x |
| snapshot | 0.990x | 1.088x | 20/36 | 0.946x |
| startpos | 0.983x | 1.286x | 6/6 | 1.200x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.485x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.502x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.533x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.574x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.645x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.675x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.675x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.702x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.800x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.880x |
| macos-15-native | materialize | kiwipete | ultra | 0.887x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.887x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.889x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.902x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.903x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.914x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.917x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.921x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.922x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.922x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.930x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.933x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.933x |
| macos-15-native | materialize | tactical | ultra | 0.936x |
| macos-15-native | materialize | middlegame | ultra | 0.945x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.952x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.953x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.954x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.955x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.960x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.965x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.968x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.968x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.971x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.972x |
| ubuntu-24.04-pext | snapshot | promotion | ferrum | 0.972x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.975x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.977x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.979x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.981x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.982x |
| macos-15-native | snapshot | endgame | ferrum | 0.992x |
| macos-15-native | snapshot | tactical | ferrum | 0.996x |
| macos-15-native | snapshot | promotion | ferrum | 0.997x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.998x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.999x |
| ubuntu-24.04-native | materialize | middlegame | ultra | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3584 | shakmaty | 5875 | 1.639x |
| ubuntu-24.04-arm-native | 6560 | shakmaty | 8768 | 1.337x |
| ubuntu-24.04-native | 6040 | shakmaty | 7992 | 1.323x |
| ubuntu-24.04-pext | 8165 | chess | 12704 | 1.556x |
| ubuntu-24.04-portable | 5279 | shakmaty | 6897 | 1.306x |
| windows-2025-native | 13900 | shakmaty | 14300 | 1.029x |

Cold wins: 6/6 (separate from native cases).
