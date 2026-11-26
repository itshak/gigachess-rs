| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 66 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 76 | 11 | 87 |
| ubuntu-24.04-native | 73 | 69 | 13 | 87 |
| ubuntu-24.04-pext | 81 | 75 | 9 | 87 |
| ubuntu-24.04-portable | 80 | 74 | 6 | 87 |
| windows-2025-native | 78 | 73 | 11 | 87 |
| ALL | 461 | 433 | 61 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.010x | 1.165x | 33/36 | 0.917x |
| chess960_materialize | 0.986x | 1.391x | 12/12 | 1.269x |
| chess960_perft_d3 | 0.984x | 1.352x | 12/12 | 1.226x |
| corpus_count | 1.000x | 1.033x | 6/12 | 0.780x |
| corpus_materialize | 1.040x | 1.069x | 11/12 | 0.988x |
| corpus_uci_sorted | 1.007x | 1.482x | 12/12 | 1.373x |
| count | 1.014x | 1.339x | 35/36 | 0.956x |
| fen_parse | 1.000x | 1.181x | 35/36 | 0.979x |
| fen_write_native_ep | 1.023x | 1.790x | 36/36 | 1.453x |
| make_unmake_full | 1.000x | 1.204x | 35/36 | 0.957x |
| materialize | 1.018x | 1.339x | 35/36 | 0.908x |
| perft_bulk_d4 | 1.003x | 1.093x | 27/36 | 0.853x |
| perft_full_d3 | 1.013x | 1.310x | 35/36 | 0.928x |
| polyglot_key | 1.014x | 35.163x | 12/12 | 16.938x |
| polyglot_recompute | 0.975x | 1.029x | 9/12 | 0.863x |
| repetition_key | 1.000x | 18.666x | 12/12 | 14.578x |
| replay_checked_native | 1.013x | 1.058x | 10/12 | 0.946x |
| replay_trusted_native | 0.999x | 0.717x | 2/12 | 0.482x |
| san_parse_all | 1.039x | 1.853x | 36/36 | 1.437x |
| san_render_native | 0.998x | 2.430x | 36/36 | 1.616x |
| snapshot | 1.025x | 1.011x | 14/36 | 0.815x |
| startpos | 1.035x | 1.252x | 6/6 | 1.112x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.482x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.528x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.568x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.592x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.670x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.696x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.701x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.741x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.780x |
| windows-2025-native | snapshot | middlegame | chess | 0.815x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.853x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.863x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.863x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.879x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.896x |
| macos-15-native | materialize | kiwipete | ultra | 0.908x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.917x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.918x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.926x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.928x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.934x |
| macos-15-native | corpus_count | seeded | ultra | 0.935x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.940x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.944x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.945x |
| macos-15-native | apply_children_full | tactical | chess | 0.946x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.946x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.952x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.956x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.957x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.957x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.957x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.959x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.960x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.962x |
| macos-15-native | snapshot | endgame | ferrum | 0.968x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.969x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.974x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.974x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.979x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.985x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.987x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.988x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.988x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.988x |
| windows-2025-native | snapshot | endgame | chess | 0.990x |
| windows-2025-native | snapshot | tactical | chess | 0.991x |
| ubuntu-24.04-native | snapshot | start | chess | 0.992x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.992x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.993x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.994x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.994x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.994x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.995x |
| macos-15-native | snapshot | promotion | ferrum | 0.995x |
| windows-2025-native | snapshot | start | chess | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.998x |
| macos-15-native | snapshot | tactical | ferrum | 0.998x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 8458 | ferrum | 10667 | 1.261x |
| ubuntu-24.04-arm-native | 7008 | shakmaty | 9983 | 1.425x |
| ubuntu-24.04-native | 5620 | shakmaty | 8010 | 1.425x |
| ubuntu-24.04-pext | 5921 | shakmaty | 7514 | 1.269x |
| ubuntu-24.04-portable | 5322 | shakmaty | 6745 | 1.267x |
| windows-2025-native | 21800 | shakmaty | 29400 | 1.349x |

Cold wins: 6/6 (separate from native cases).
