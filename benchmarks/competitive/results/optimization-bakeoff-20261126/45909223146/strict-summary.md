| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 78 | 73 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 10 | 87 |
| ubuntu-24.04-pext | 78 | 76 | 9 | 87 |
| ubuntu-24.04-portable | 86 | 79 | 13 | 87 |
| windows-2025-native | 82 | 78 | 13 | 87 |
| ALL | 486 | 468 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.018x | 1.286x | 35/36 | 0.865x |
| chess960_materialize | 0.993x | 1.549x | 12/12 | 1.213x |
| chess960_perft_d3 | 0.957x | 1.527x | 12/12 | 1.326x |
| corpus_count | 1.117x | 1.430x | 12/12 | 1.121x |
| corpus_materialize | 0.959x | 1.107x | 11/12 | 0.899x |
| corpus_uci_sorted | 0.951x | 1.482x | 12/12 | 1.180x |
| count | 1.043x | 1.517x | 36/36 | 1.061x |
| fen_parse | 0.993x | 1.169x | 34/36 | 0.988x |
| fen_write_native_ep | 0.982x | 1.720x | 36/36 | 1.200x |
| make_unmake_full | 1.013x | 1.307x | 36/36 | 1.040x |
| materialize | 1.004x | 1.403x | 35/36 | 0.993x |
| perft_bulk_d4 | 1.058x | 1.320x | 34/36 | 0.953x |
| perft_full_d3 | 1.013x | 1.351x | 36/36 | 1.158x |
| polyglot_key | 0.935x | 35.018x | 12/12 | 16.013x |
| polyglot_recompute | 1.009x | 1.059x | 8/12 | 0.974x |
| repetition_key | 1.009x | 21.466x | 12/12 | 14.049x |
| replay_checked_native | 0.968x | 1.142x | 12/12 | 1.003x |
| replay_trusted_native | 0.930x | 0.756x | 4/12 | 0.358x |
| san_parse_all | 0.979x | 1.850x | 36/36 | 1.434x |
| san_render_native | 1.013x | 2.576x | 36/36 | 1.894x |
| snapshot | 1.019x | 1.000x | 20/36 | 0.666x |
| startpos | 0.987x | 1.293x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.358x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.645x |
| macos-15-native | snapshot | middlegame | chess | 0.666x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.683x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.685x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.694x |
| macos-15-native | snapshot | promotion | chess | 0.787x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.827x |
| macos-15-native | apply_children_full | promotion | chess | 0.865x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.899x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.952x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.953x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.955x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.974x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.982x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.986x |
| windows-2025-native | snapshot | tactical | ferrum | 0.987x |
| macos-15-native | fen_parse | kiwipete | cozy | 0.988x |
| macos-15-native | materialize | tactical | ultra | 0.993x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.994x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.995x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.996x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.997x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.997x |
| windows-2025-native | snapshot | middlegame | cozy | 0.999x |
| windows-2025-native | snapshot | endgame | chess | 0.999x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | tactical | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4584 | ferrum | 8000 | 1.745x |
| ubuntu-24.04-arm-native | 5840 | shakmaty | 7992 | 1.368x |
| ubuntu-24.04-native | 5524 | shakmaty | 6506 | 1.178x |
| ubuntu-24.04-pext | 5054 | cozy | 8298 | 1.642x |
| ubuntu-24.04-portable | 5400 | shakmaty | 6550 | 1.213x |
| windows-2025-native | 15800 | shakmaty | 20500 | 1.297x |

Cold wins: 6/6 (separate from native cases).
