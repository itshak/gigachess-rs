| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 73 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 82 | 82 | 11 | 87 |
| ubuntu-24.04-pext | 82 | 76 | 8 | 87 |
| ubuntu-24.04-portable | 78 | 78 | 13 | 87 |
| windows-2025-native | 83 | 82 | 12 | 87 |
| ALL | 480 | 470 | 69 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.987x | 1.222x | 34/36 | 0.751x |
| chess960_materialize | 0.977x | 1.484x | 12/12 | 1.258x |
| chess960_perft_d3 | 0.955x | 1.443x | 11/12 | 0.921x |
| corpus_count | 0.996x | 1.175x | 9/12 | 0.822x |
| corpus_materialize | 1.000x | 1.156x | 12/12 | 1.035x |
| corpus_uci_sorted | 0.955x | 1.442x | 12/12 | 1.087x |
| count | 0.994x | 1.478x | 36/36 | 1.093x |
| fen_parse | 1.034x | 1.219x | 36/36 | 1.091x |
| fen_write_native_ep | 0.993x | 1.720x | 36/36 | 1.423x |
| make_unmake_full | 1.001x | 1.311x | 36/36 | 1.064x |
| materialize | 1.010x | 1.342x | 35/36 | 0.983x |
| perft_bulk_d4 | 0.983x | 1.257x | 34/36 | 0.859x |
| perft_full_d3 | 1.220x | 1.306x | 36/36 | 1.018x |
| polyglot_key | 0.959x | 32.627x | 12/12 | 16.684x |
| polyglot_recompute | 1.005x | 1.020x | 7/12 | 0.906x |
| repetition_key | 0.921x | 21.635x | 12/12 | 14.736x |
| replay_checked_native | 0.969x | 1.126x | 12/12 | 1.038x |
| replay_trusted_native | 0.995x | 0.775x | 2/12 | 0.511x |
| san_parse_all | 0.972x | 1.837x | 36/36 | 1.230x |
| san_render_native | 1.001x | 2.544x | 36/36 | 1.534x |
| snapshot | 1.023x | 1.096x | 19/36 | 0.739x |
| startpos | 0.933x | 1.088x | 5/6 | 0.758x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.511x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.568x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.573x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.662x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.666x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.688x |
| macos-15-native | snapshot | tactical | ferrum | 0.739x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.751x |
| macos-15-native | startpos | start | shakmaty | 0.758x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.822x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.830x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.859x |
| macos-15-native | snapshot | promotion | ferrum | 0.891x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.904x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.906x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.906x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.908x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.917x |
| macos-15-native | chess960_perft_d3 | 960_castle | cozy | 0.921x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.924x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.939x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.940x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.946x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.949x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.955x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.956x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.957x |
| macos-15-native | snapshot | start | ferrum | 0.962x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.968x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.971x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.975x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.980x |
| macos-15-native | materialize | middlegame | ultra | 0.983x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.985x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.987x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.994x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.995x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3708 | shakmaty | 5333 | 1.438x |
| ubuntu-24.04-arm-native | 5359 | shakmaty | 6736 | 1.257x |
| ubuntu-24.04-native | 6652 | shakmaty | 7871 | 1.183x |
| ubuntu-24.04-pext | 5028 | shakmaty | 8084 | 1.608x |
| ubuntu-24.04-portable | 6079 | shakmaty | 8063 | 1.326x |
| windows-2025-native | 10200 | shakmaty | 12600 | 1.235x |

Cold wins: 6/6 (separate from native cases).
