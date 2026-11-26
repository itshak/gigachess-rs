| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 68 | 11 | 87 |
| ubuntu-24.04-arm-native | 75 | 73 | 10 | 87 |
| ubuntu-24.04-native | 77 | 73 | 10 | 87 |
| ubuntu-24.04-pext | 79 | 75 | 9 | 87 |
| ubuntu-24.04-portable | 72 | 68 | 10 | 87 |
| windows-2025-native | 83 | 79 | 11 | 87 |
| ALL | 460 | 436 | 61 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.021x | 1.097x | 31/36 | 0.729x |
| chess960_materialize | 0.997x | 1.311x | 12/12 | 1.243x |
| chess960_perft_d3 | 0.983x | 1.315x | 12/12 | 1.056x |
| corpus_count | 1.002x | 1.352x | 12/12 | 1.164x |
| corpus_materialize | 1.012x | 1.155x | 12/12 | 1.058x |
| corpus_uci_sorted | 1.006x | 1.535x | 12/12 | 1.117x |
| count | 1.031x | 1.354x | 36/36 | 1.007x |
| fen_parse | 1.004x | 1.152x | 34/36 | 0.984x |
| fen_write_native_ep | 0.985x | 1.071x | 24/36 | 0.879x |
| make_unmake_full | 1.004x | 1.213x | 35/36 | 0.969x |
| materialize | 1.006x | 1.243x | 34/36 | 0.689x |
| perft_bulk_d4 | 1.011x | 1.272x | 36/36 | 1.001x |
| perft_full_d3 | 1.011x | 1.329x | 35/36 | 0.950x |
| polyglot_key | 1.015x | 33.123x | 12/12 | 16.256x |
| polyglot_recompute | 0.982x | 1.028x | 9/12 | 0.920x |
| repetition_key | 0.943x | 20.127x | 12/12 | 14.214x |
| replay_checked_native | 1.045x | 1.032x | 10/12 | 0.934x |
| replay_trusted_native | 1.010x | 0.647x | 0/12 | 0.471x |
| san_parse_all | 0.985x | 1.893x | 36/36 | 1.499x |
| san_render_native | 0.993x | 2.649x | 36/36 | 1.919x |
| snapshot | 1.322x | 1.019x | 14/36 | 0.661x |
| startpos | 1.276x | 1.295x | 6/6 | 1.014x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.471x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.488x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.545x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.566x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.600x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.631x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.647x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.660x |
| macos-15-native | snapshot | tactical | ferrum | 0.661x |
| macos-15-native | materialize | middlegame | ultra | 0.689x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.729x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.810x |
| macos-15-native | snapshot | promotion | ferrum | 0.813x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.825x |
| macos-15-native | materialize | kiwipete | ultra | 0.826x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.840x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.844x |
| ubuntu-24.04-portable | fen_write_native_ep | kiwipete | ultra | 0.879x |
| ubuntu-24.04-native | fen_write_native_ep | middlegame | ultra | 0.881x |
| ubuntu-24.04-portable | fen_write_native_ep | middlegame | ultra | 0.886x |
| ubuntu-24.04-native | fen_write_native_ep | kiwipete | ultra | 0.903x |
| macos-15-native | snapshot | middlegame | ferrum | 0.919x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.920x |
| ubuntu-24.04-portable | fen_write_native_ep | promotion | ultra | 0.923x |
| ubuntu-24.04-portable | fen_write_native_ep | start | ultra | 0.929x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.934x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.934x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.937x |
| macos-15-native | apply_children_full | tactical | chess | 0.941x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.942x |
| ubuntu-24.04-portable | fen_write_native_ep | tactical | ultra | 0.943x |
| ubuntu-24.04-native | fen_write_native_ep | promotion | ultra | 0.944x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.948x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.950x |
| ubuntu-24.04-native | fen_write_native_ep | start | ultra | 0.954x |
| ubuntu-24.04-portable | fen_write_native_ep | endgame | ultra | 0.963x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.965x |
| macos-15-native | snapshot | endgame | cozy | 0.969x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.969x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.969x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.971x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.971x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.975x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.976x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.978x |
| ubuntu-24.04-native | fen_write_native_ep | tactical | ultra | 0.979x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.979x |
| macos-15-native | apply_children_full | promotion | cozy | 0.980x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.982x |
| macos-15-native | fen_parse | start | ultra | 0.984x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.985x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.985x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.988x |
| ubuntu-24.04-pext | snapshot | kiwipete | chess | 0.988x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.997x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.999x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.999x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 22958 | ferrum | 12916 | 0.563x |
| ubuntu-24.04-arm-native | 10831 | shakmaty | 8664 | 0.800x |
| ubuntu-24.04-native | 13345 | shakmaty | 10339 | 0.775x |
| ubuntu-24.04-pext | 9766 | cozy | 7432 | 0.761x |
| ubuntu-24.04-portable | 12874 | shakmaty | 9948 | 0.773x |
| windows-2025-native | 44300 | shakmaty | 25000 | 0.564x |

Cold wins: 0/6 (separate from native cases).
