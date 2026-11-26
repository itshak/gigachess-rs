| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 67 | 11 | 87 |
| ubuntu-24.04-arm-native | 76 | 76 | 11 | 87 |
| ubuntu-24.04-native | 78 | 73 | 10 | 87 |
| ubuntu-24.04-pext | 82 | 80 | 9 | 87 |
| ubuntu-24.04-portable | 81 | 74 | 9 | 87 |
| windows-2025-native | 78 | 73 | 12 | 87 |
| ALL | 468 | 443 | 62 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.114x | 1.220x | 32/36 | 0.867x |
| chess960_materialize | 1.068x | 1.386x | 12/12 | 1.196x |
| chess960_perft_d3 | 0.927x | 1.345x | 12/12 | 1.047x |
| corpus_count | 0.829x | 1.124x | 8/12 | 0.805x |
| corpus_materialize | 0.898x | 1.084x | 11/12 | 0.930x |
| corpus_uci_sorted | 0.959x | 1.475x | 12/12 | 1.134x |
| count | 1.002x | 1.299x | 34/36 | 0.854x |
| fen_parse | 1.045x | 1.218x | 35/36 | 0.972x |
| fen_write_native_ep | 0.994x | 1.625x | 36/36 | 1.322x |
| make_unmake_full | 1.006x | 1.294x | 36/36 | 1.102x |
| materialize | 1.029x | 1.220x | 33/36 | 0.833x |
| perft_bulk_d4 | 0.901x | 1.152x | 33/36 | 0.908x |
| perft_full_d3 | 1.030x | 1.215x | 33/36 | 0.883x |
| polyglot_key | 0.982x | 33.888x | 12/12 | 16.265x |
| polyglot_recompute | 0.999x | 1.039x | 9/12 | 0.895x |
| repetition_key | 0.827x | 18.716x | 12/12 | 13.460x |
| replay_checked_native | 1.014x | 0.993x | 5/12 | 0.879x |
| replay_trusted_native | 0.992x | 0.664x | 1/12 | 0.477x |
| san_parse_all | 0.977x | 1.874x | 36/36 | 1.383x |
| san_render_native | 1.023x | 2.585x | 36/36 | 1.880x |
| snapshot | 1.308x | 1.112x | 24/36 | 0.607x |
| startpos | 1.020x | 1.274x | 6/6 | 1.191x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.477x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.486x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.499x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.557x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.558x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.577x |
| macos-15-native | snapshot | start | ferrum | 0.607x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.686x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.740x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.805x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.805x |
| macos-15-native | materialize | tactical | ultra | 0.833x |
| windows-2025-native | snapshot | start | chess | 0.834x |
| macos-15-native | count | middlegame | cozy | 0.854x |
| macos-15-native | apply_children_full | tactical | chess | 0.867x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.870x |
| macos-15-native | apply_children_full | promotion | cozy | 0.870x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.879x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.883x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.886x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.895x |
| macos-15-native | materialize | kiwipete | ultra | 0.896x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.899x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.903x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.908x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.910x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.916x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.919x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.930x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.932x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.934x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.942x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.952x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.954x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.960x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.961x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.963x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.968x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.972x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.975x |
| macos-15-native | materialize | endgame | ultra | 0.977x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.983x |
| windows-2025-native | snapshot | middlegame | cozy | 0.983x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.992x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.995x |
| windows-2025-native | snapshot | tactical | cozy | 0.996x |
| windows-2025-native | snapshot | promotion | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7084 | shakmaty | 9875 | 1.394x |
| ubuntu-24.04-arm-native | 5623 | chess | 8591 | 1.528x |
| ubuntu-24.04-native | 5459 | shakmaty | 7721 | 1.414x |
| ubuntu-24.04-pext | 5839 | cozy | 8372 | 1.434x |
| ubuntu-24.04-portable | 5310 | shakmaty | 7362 | 1.386x |
| windows-2025-native | 20200 | shakmaty | 30000 | 1.485x |

Cold wins: 6/6 (separate from native cases).
