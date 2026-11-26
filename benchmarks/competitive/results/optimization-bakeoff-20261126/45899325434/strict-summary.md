| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 69 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-pext | 85 | 84 | 13 | 87 |
| ubuntu-24.04-portable | 82 | 78 | 11 | 87 |
| windows-2025-native | 84 | 83 | 12 | 87 |
| ALL | 482 | 472 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.985x | 1.260x | 34/36 | 0.865x |
| chess960_materialize | 0.991x | 1.495x | 12/12 | 1.287x |
| chess960_perft_d3 | 0.970x | 1.525x | 12/12 | 1.371x |
| corpus_count | 1.046x | 1.395x | 11/12 | 0.917x |
| corpus_materialize | 0.974x | 1.171x | 11/12 | 0.981x |
| corpus_uci_sorted | 0.982x | 1.579x | 12/12 | 1.366x |
| count | 1.003x | 1.500x | 36/36 | 1.001x |
| fen_parse | 0.999x | 1.177x | 35/36 | 0.908x |
| fen_write_native_ep | 0.997x | 1.721x | 36/36 | 1.422x |
| make_unmake_full | 1.005x | 1.333x | 35/36 | 0.886x |
| materialize | 0.983x | 1.273x | 33/36 | 0.945x |
| perft_bulk_d4 | 1.025x | 1.349x | 35/36 | 0.982x |
| perft_full_d3 | 1.208x | 1.562x | 36/36 | 1.258x |
| polyglot_key | 0.979x | 27.987x | 12/12 | 16.485x |
| polyglot_recompute | 0.989x | 1.052x | 9/12 | 0.882x |
| repetition_key | 1.012x | 20.149x | 12/12 | 15.192x |
| replay_checked_native | 0.988x | 1.135x | 12/12 | 1.061x |
| replay_trusted_native | 1.008x | 0.846x | 6/12 | 0.521x |
| san_parse_all | 0.988x | 1.892x | 36/36 | 1.266x |
| san_render_native | 0.987x | 2.542x | 36/36 | 1.589x |
| snapshot | 0.960x | 1.036x | 17/36 | 0.797x |
| startpos | 0.899x | 1.128x | 4/6 | 0.822x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.521x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.585x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.643x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.674x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.696x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.700x |
| macos-15-native | snapshot | start | chess | 0.797x |
| macos-15-native | snapshot | promotion | ferrum | 0.803x |
| macos-15-native | startpos | start | shakmaty | 0.822x |
| macos-15-native | apply_children_full | promotion | cozy | 0.865x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.882x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.886x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.897x |
| macos-15-native | fen_parse | tactical | shakmaty | 0.908x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.917x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.931x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.944x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.945x |
| macos-15-native | materialize | kiwipete | ultra | 0.945x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.971x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.973x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.973x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.974x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.975x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.975x |
| macos-15-native | materialize | promotion | ultra | 0.979x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.981x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.982x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.990x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| windows-2025-native | startpos | start | shakmaty | 0.999x |
| windows-2025-native | snapshot | promotion | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6833 | ferrum | 12625 | 1.848x |
| ubuntu-24.04-arm-native | 6440 | chess | 8832 | 1.371x |
| ubuntu-24.04-native | 7962 | shakmaty | 11747 | 1.475x |
| ubuntu-24.04-pext | 7243 | shakmaty | 9738 | 1.344x |
| ubuntu-24.04-portable | 5265 | shakmaty | 7389 | 1.403x |
| windows-2025-native | 13500 | shakmaty | 18800 | 1.393x |

Cold wins: 6/6 (separate from native cases).
