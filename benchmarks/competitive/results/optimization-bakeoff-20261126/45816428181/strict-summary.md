| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 66 | 14 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 79 | 76 | 15 | 87 |
| ubuntu-24.04-pext | 80 | 76 | 12 | 87 |
| ubuntu-24.04-portable | 76 | 71 | 11 | 87 |
| windows-2025-native | 83 | 76 | 12 | 87 |
| ALL | 467 | 442 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.179x | 1.257x | 35/36 | 0.935x |
| chess960_materialize | 1.023x | 1.359x | 12/12 | 1.131x |
| chess960_perft_d3 | 0.983x | 1.360x | 12/12 | 1.067x |
| corpus_count | 0.794x | 1.083x | 6/12 | 0.815x |
| corpus_materialize | 0.940x | 1.093x | 12/12 | 1.013x |
| corpus_uci_sorted | 0.969x | 1.458x | 12/12 | 1.258x |
| count | 0.999x | 1.347x | 35/36 | 0.944x |
| fen_parse | 0.985x | 1.184x | 33/36 | 0.894x |
| fen_write_native_ep | 1.007x | 1.833x | 36/36 | 1.452x |
| make_unmake_full | 1.029x | 1.287x | 35/36 | 0.993x |
| materialize | 0.979x | 1.256x | 32/36 | 0.717x |
| perft_bulk_d4 | 0.903x | 1.151x | 32/36 | 0.884x |
| perft_full_d3 | 1.002x | 1.266x | 31/36 | 0.862x |
| polyglot_key | 0.953x | 30.283x | 12/12 | 16.198x |
| polyglot_recompute | 0.982x | 1.033x | 8/12 | 0.868x |
| repetition_key | 0.885x | 17.942x | 12/12 | 14.350x |
| replay_checked_native | 1.035x | 1.052x | 11/12 | 0.928x |
| replay_trusted_native | 1.146x | 0.804x | 5/12 | 0.524x |
| san_parse_all | 0.980x | 1.899x | 36/36 | 1.371x |
| san_render_native | 1.042x | 2.495x | 36/36 | 1.649x |
| snapshot | 1.111x | 1.007x | 18/36 | 0.644x |
| startpos | 1.112x | 1.233x | 6/6 | 1.118x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.524x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.560x |
| macos-15-native | snapshot | endgame | ferrum | 0.644x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.669x |
| macos-15-native | snapshot | promotion | ferrum | 0.686x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.704x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.706x |
| macos-15-native | materialize | kiwipete | ultra | 0.717x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.731x |
| macos-15-native | snapshot | middlegame | chess | 0.761x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.763x |
| macos-15-native | snapshot | start | ferrum | 0.804x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.815x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.862x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.868x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.882x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.884x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.893x |
| macos-15-native | fen_parse | start | shakmaty | 0.894x |
| windows-2025-native | snapshot | endgame | cozy | 0.901x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.904x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.906x |
| macos-15-native | materialize | middlegame | ultra | 0.917x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.923x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.928x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.932x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.934x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.935x |
| windows-2025-native | snapshot | start | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.937x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.938x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.939x |
| macos-15-native | count | promotion | cozy | 0.944x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.950x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.955x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.957x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.958x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.963x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.969x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.971x |
| ubuntu-24.04-native | materialize | middlegame | ultra | 0.972x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.973x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.975x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.977x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.977x |
| windows-2025-native | snapshot | tactical | chess | 0.977x |
| ubuntu-24.04-portable | perft_bulk_d4 | tactical | ultra | 0.989x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.990x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.993x |
| ubuntu-24.04-portable | make_unmake_full | promotion | ultra | 0.993x |
| ubuntu-24.04-pext | fen_parse | endgame | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7375 | shakmaty | 11709 | 1.588x |
| ubuntu-24.04-arm-native | 6944 | chess | 9295 | 1.339x |
| ubuntu-24.04-native | 7871 | shakmaty | 8382 | 1.065x |
| ubuntu-24.04-pext | 6355 | cozy | 7444 | 1.171x |
| ubuntu-24.04-portable | 6251 | shakmaty | 8996 | 1.439x |
| windows-2025-native | 16800 | shakmaty | 26100 | 1.554x |

Cold wins: 6/6 (separate from native cases).
