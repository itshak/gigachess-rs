| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 66 | 12 | 87 |
| ubuntu-24.04-arm-native | 75 | 74 | 12 | 87 |
| ubuntu-24.04-native | 69 | 65 | 11 | 87 |
| ubuntu-24.04-pext | 76 | 68 | 13 | 87 |
| ubuntu-24.04-portable | 78 | 73 | 13 | 87 |
| windows-2025-native | 81 | 76 | 14 | 87 |
| ALL | 449 | 422 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.952x | 1.313x | 33/36 | 0.984x |
| chess960_materialize | 0.951x | 1.407x | 12/12 | 1.116x |
| chess960_perft_d3 | 0.957x | 1.445x | 12/12 | 1.222x |
| corpus_count | 0.974x | 1.245x | 10/12 | 0.954x |
| corpus_materialize | 0.977x | 1.146x | 12/12 | 1.050x |
| corpus_uci_sorted | 0.995x | 1.522x | 12/12 | 1.278x |
| count | 0.968x | 1.435x | 35/36 | 0.990x |
| fen_parse | 0.911x | 1.094x | 25/36 | 0.853x |
| fen_write_native_ep | 0.999x | 1.801x | 36/36 | 1.222x |
| make_unmake_full | 0.937x | 1.185x | 34/36 | 0.971x |
| materialize | 0.985x | 1.315x | 35/36 | 0.960x |
| perft_bulk_d4 | 0.982x | 1.262x | 33/36 | 0.898x |
| perft_full_d3 | 0.880x | 1.149x | 31/36 | 0.872x |
| polyglot_key | 1.026x | 30.248x | 12/12 | 17.218x |
| polyglot_recompute | 1.013x | 1.011x | 8/12 | 0.853x |
| repetition_key | 0.968x | 19.801x | 12/12 | 15.765x |
| replay_checked_native | 0.911x | 0.970x | 5/12 | 0.858x |
| replay_trusted_native | 0.887x | 0.770x | 2/12 | 0.524x |
| san_parse_all | 0.952x | 1.790x | 36/36 | 1.414x |
| san_render_native | 0.942x | 2.320x | 36/36 | 1.475x |
| snapshot | 0.834x | 0.997x | 16/36 | 0.900x |
| startpos | 0.814x | 1.028x | 2/6 | 0.910x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.524x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.538x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.610x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.628x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.670x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.730x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.853x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.853x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.858x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.872x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.888x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.889x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.891x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.898x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.900x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.900x |
| ubuntu-24.04-pext | fen_parse | middlegame | ferrum | 0.907x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.908x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.910x |
| ubuntu-24.04-arm-native | snapshot | middlegame | cozy | 0.912x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.913x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.916x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.916x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.924x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.926x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.929x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.931x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.933x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.937x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.938x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.939x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.939x |
| macos-15-native | perft_full_d3 | tactical | ultra | 0.944x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.954x |
| macos-15-native | materialize | kiwipete | ultra | 0.960x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.960x |
| ubuntu-24.04-arm-native | startpos | start | shakmaty | 0.961x |
| macos-15-native | snapshot | endgame | ferrum | 0.967x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.967x |
| ubuntu-24.04-native | fen_parse | promotion | ultra | 0.968x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.970x |
| macos-15-native | snapshot | start | ferrum | 0.970x |
| macos-15-native | snapshot | middlegame | ferrum | 0.971x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.971x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.977x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.978x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.978x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.979x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.980x |
| ubuntu-24.04-native | fen_parse | tactical | ultra | 0.980x |
| macos-15-native | snapshot | tactical | ferrum | 0.982x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.982x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.984x |
| macos-15-native | startpos | start | shakmaty | 0.987x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.990x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.990x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.991x |
| macos-15-native | apply_children_full | tactical | chess | 0.991x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.991x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.992x |
| ubuntu-24.04-native | fen_parse | middlegame | ultra | 0.996x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.998x |
| macos-15-native | fen_parse | promotion | cozy | 0.998x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.998x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.998x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.999x |
| ubuntu-24.04-portable | snapshot | promotion | chess | 0.999x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |
| ubuntu-24.04-native | snapshot | start | ferrum | 1.000x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4666 | shakmaty | 7167 | 1.536x |
| ubuntu-24.04-arm-native | 6111 | shakmaty | 8552 | 1.399x |
| ubuntu-24.04-native | 6470 | shakmaty | 8933 | 1.381x |
| ubuntu-24.04-pext | 5718 | shakmaty | 6867 | 1.201x |
| ubuntu-24.04-portable | 5736 | shakmaty | 6869 | 1.198x |
| windows-2025-native | 12900 | shakmaty | 19000 | 1.473x |

Cold wins: 6/6 (separate from native cases).
