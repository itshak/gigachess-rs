| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 65 | 13 | 87 |
| ubuntu-24.04-arm-native | 77 | 77 | 10 | 87 |
| ubuntu-24.04-native | 80 | 78 | 10 | 87 |
| ubuntu-24.04-pext | 68 | 67 | 10 | 87 |
| ubuntu-24.04-portable | 82 | 78 | 9 | 87 |
| windows-2025-native | 81 | 77 | 11 | 87 |
| ALL | 457 | 442 | 63 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.907x | 1.063x | 27/36 | 0.841x |
| chess960_materialize | 0.959x | 1.358x | 12/12 | 1.193x |
| chess960_perft_d3 | 0.931x | 1.317x | 12/12 | 1.159x |
| corpus_count | 0.981x | 1.392x | 12/12 | 1.210x |
| corpus_materialize | 0.978x | 1.169x | 12/12 | 1.085x |
| corpus_uci_sorted | 0.953x | 1.511x | 12/12 | 1.301x |
| count | 0.983x | 1.372x | 35/36 | 0.970x |
| fen_parse | 0.875x | 1.062x | 29/36 | 0.885x |
| fen_write_native_ep | 1.001x | 1.758x | 36/36 | 1.338x |
| make_unmake_full | 0.888x | 1.094x | 28/36 | 0.873x |
| materialize | 0.964x | 1.299x | 33/36 | 0.789x |
| perft_bulk_d4 | 0.947x | 1.224x | 35/36 | 0.965x |
| perft_full_d3 | 0.937x | 1.220x | 28/36 | 0.797x |
| polyglot_key | 1.047x | 33.714x | 12/12 | 18.196x |
| polyglot_recompute | 0.984x | 1.005x | 7/12 | 0.882x |
| repetition_key | 1.015x | 20.936x | 12/12 | 16.570x |
| replay_checked_native | 0.900x | 0.958x | 4/12 | 0.756x |
| replay_trusted_native | 0.858x | 0.635x | 0/12 | 0.439x |
| san_parse_all | 0.924x | 1.772x | 36/36 | 1.387x |
| san_render_native | 0.948x | 2.427x | 36/36 | 1.630x |
| snapshot | 1.004x | 1.186x | 33/36 | 0.824x |
| startpos | 1.253x | 1.586x | 6/6 | 1.214x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.439x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.469x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.471x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.516x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.552x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.620x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.624x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.649x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.756x |
| macos-15-native | materialize | middlegame | ultra | 0.789x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.797x |
| macos-15-native | snapshot | tactical | cozy | 0.824x |
| macos-15-native | snapshot | promotion | ferrum | 0.827x |
| macos-15-native | apply_children_full | endgame | cozy | 0.841x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.849x |
| macos-15-native | materialize | tactical | ultra | 0.872x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.873x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.874x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.874x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.875x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.881x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.882x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.884x |
| macos-15-native | fen_parse | kiwipete | cozy | 0.885x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.889x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.902x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.903x |
| ubuntu-24.04-pext | make_unmake_full | middlegame | ultra | 0.903x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.907x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.909x |
| macos-15-native | apply_children_full | promotion | chess | 0.909x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.910x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.912x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.912x |
| ubuntu-24.04-pext | fen_parse | middlegame | ultra | 0.917x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.919x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.922x |
| macos-15-native | apply_children_full | tactical | chess | 0.924x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.925x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.925x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.925x |
| macos-15-native | fen_parse | start | ferrum | 0.930x |
| macos-15-native | snapshot | start | ferrum | 0.931x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.934x |
| ubuntu-24.04-arm-native | apply_children_full | tactical | chess | 0.943x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.944x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.946x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.952x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.956x |
| macos-15-native | materialize | kiwipete | ultra | 0.957x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.959x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.965x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.970x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.970x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.972x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.973x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.977x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.984x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.986x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.989x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.990x |
| ubuntu-24.04-portable | make_unmake_full | tactical | ultra | 0.994x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.996x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.996x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 16000 | shakmaty | 13542 | 0.846x |
| ubuntu-24.04-arm-native | 10664 | shakmaty | 8488 | 0.796x |
| ubuntu-24.04-native | 11537 | shakmaty | 9356 | 0.811x |
| ubuntu-24.04-pext | 10556 | shakmaty | 10266 | 0.973x |
| ubuntu-24.04-portable | 11381 | shakmaty | 8361 | 0.735x |
| windows-2025-native | 31800 | shakmaty | 24200 | 0.761x |

Cold wins: 0/6 (separate from native cases).
