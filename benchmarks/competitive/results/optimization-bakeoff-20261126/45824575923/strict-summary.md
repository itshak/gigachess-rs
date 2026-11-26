| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 70 | 13 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 12 | 87 |
| ubuntu-24.04-native | 79 | 74 | 12 | 87 |
| ubuntu-24.04-pext | 80 | 79 | 10 | 87 |
| ubuntu-24.04-portable | 84 | 78 | 12 | 87 |
| windows-2025-native | 83 | 83 | 11 | 87 |
| ALL | 477 | 461 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.052x | 1.270x | 34/36 | 0.968x |
| chess960_materialize | 1.098x | 1.428x | 12/12 | 1.244x |
| chess960_perft_d3 | 1.107x | 1.528x | 12/12 | 1.335x |
| corpus_count | 1.138x | 1.193x | 9/12 | 0.774x |
| corpus_materialize | 1.041x | 1.141x | 12/12 | 1.043x |
| corpus_uci_sorted | 1.024x | 1.533x | 12/12 | 1.239x |
| count | 1.095x | 1.443x | 35/36 | 0.733x |
| fen_parse | 1.037x | 1.176x | 34/36 | 0.937x |
| fen_write_native_ep | 1.003x | 1.663x | 36/36 | 1.340x |
| make_unmake_full | 1.010x | 1.284x | 35/36 | 0.974x |
| materialize | 1.037x | 1.275x | 34/36 | 0.885x |
| perft_bulk_d4 | 1.129x | 1.295x | 35/36 | 0.894x |
| perft_full_d3 | 1.024x | 1.301x | 34/36 | 0.959x |
| polyglot_key | 1.018x | 33.061x | 12/12 | 16.063x |
| polyglot_recompute | 0.995x | 1.019x | 9/12 | 0.830x |
| repetition_key | 0.973x | 19.614x | 12/12 | 13.997x |
| replay_checked_native | 1.002x | 1.103x | 12/12 | 1.020x |
| replay_trusted_native | 1.006x | 0.719x | 2/12 | 0.526x |
| san_parse_all | 0.979x | 1.827x | 36/36 | 1.414x |
| san_render_native | 0.987x | 2.544x | 36/36 | 1.873x |
| snapshot | 1.015x | 1.112x | 20/36 | 0.598x |
| startpos | 0.918x | 1.154x | 4/6 | 0.898x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.526x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.537x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.559x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.567x |
| macos-15-native | snapshot | promotion | ferrum | 0.598x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.618x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.656x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.708x |
| macos-15-native | count | endgame | cozy | 0.733x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.747x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.774x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.830x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.835x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.843x |
| macos-15-native | snapshot | middlegame | chess | 0.854x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.862x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.866x |
| macos-15-native | materialize | kiwipete | ultra | 0.885x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.894x |
| macos-15-native | startpos | start | shakmaty | 0.898x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.906x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.930x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.936x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.937x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.940x |
| macos-15-native | materialize | tactical | ultra | 0.951x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.954x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.954x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.959x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.968x |
| macos-15-native | apply_children_full | start | chess | 0.971x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.974x |
| macos-15-native | snapshot | start | ferrum | 0.978x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.982x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.987x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.995x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.997x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | middlegame | chess | 0.998x |
| ubuntu-24.04-portable | snapshot | kiwipete | chess | 0.998x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.999x |
| ubuntu-24.04-native | snapshot | start | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4583 | shakmaty | 9209 | 2.009x |
| ubuntu-24.04-arm-native | 6696 | chess | 8576 | 1.281x |
| ubuntu-24.04-native | 5344 | chess | 7488 | 1.401x |
| ubuntu-24.04-pext | 6500 | cozy | 10596 | 1.630x |
| ubuntu-24.04-portable | 7685 | shakmaty | 9267 | 1.206x |
| windows-2025-native | 10900 | shakmaty | 14000 | 1.284x |

Cold wins: 6/6 (separate from native cases).
