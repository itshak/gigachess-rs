| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 78 | 74 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 13 | 87 |
| ubuntu-24.04-native | 79 | 79 | 10 | 87 |
| ubuntu-24.04-pext | 82 | 78 | 11 | 87 |
| ubuntu-24.04-portable | 82 | 75 | 12 | 87 |
| windows-2025-native | 80 | 76 | 12 | 87 |
| ALL | 480 | 461 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.040x | 1.262x | 36/36 | 1.073x |
| chess960_materialize | 1.026x | 1.493x | 12/12 | 1.248x |
| chess960_perft_d3 | 1.018x | 1.519x | 12/12 | 1.373x |
| corpus_count | 0.991x | 1.169x | 10/12 | 0.902x |
| corpus_materialize | 1.023x | 1.124x | 12/12 | 1.001x |
| corpus_uci_sorted | 1.019x | 1.493x | 12/12 | 1.369x |
| count | 1.026x | 1.485x | 36/36 | 1.026x |
| fen_parse | 0.983x | 1.161x | 35/36 | 0.983x |
| fen_write_native_ep | 1.010x | 1.775x | 36/36 | 1.428x |
| make_unmake_full | 0.994x | 1.257x | 32/36 | 0.963x |
| materialize | 1.028x | 1.404x | 33/36 | 0.907x |
| perft_bulk_d4 | 1.026x | 1.250x | 32/36 | 0.858x |
| perft_full_d3 | 1.006x | 1.327x | 35/36 | 0.959x |
| polyglot_key | 1.019x | 35.262x | 12/12 | 16.371x |
| polyglot_recompute | 1.019x | 1.025x | 7/12 | 0.932x |
| repetition_key | 0.928x | 19.958x | 12/12 | 14.474x |
| replay_checked_native | 1.002x | 1.069x | 11/12 | 0.919x |
| replay_trusted_native | 1.051x | 0.821x | 5/12 | 0.562x |
| san_parse_all | 1.050x | 1.874x | 36/36 | 1.281x |
| san_render_native | 1.024x | 2.502x | 36/36 | 1.879x |
| snapshot | 0.992x | 1.049x | 22/36 | 0.672x |
| startpos | 1.015x | 1.256x | 6/6 | 1.132x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.562x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.599x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.606x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.648x |
| windows-2025-native | snapshot | endgame | chess | 0.672x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.688x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.786x |
| windows-2025-native | snapshot | kiwipete | chess | 0.829x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.858x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.890x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.902x |
| macos-15-native | materialize | kiwipete | ultra | 0.907x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.919x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.921x |
| macos-15-native | materialize | tactical | ultra | 0.924x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.932x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.939x |
| macos-15-native | snapshot | start | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.954x |
| macos-15-native | snapshot | promotion | ferrum | 0.954x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.959x |
| ubuntu-24.04-pext | make_unmake_full | promotion | ultra | 0.963x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.967x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.968x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.973x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.975x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.983x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.985x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.992x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.996x |
| macos-15-native | materialize | middlegame | ultra | 0.996x |
| windows-2025-native | snapshot | tactical | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 2458 | shakmaty | 6500 | 2.644x |
| ubuntu-24.04-arm-native | 5416 | ferrum | 9672 | 1.786x |
| ubuntu-24.04-native | 5256 | shakmaty | 7037 | 1.339x |
| ubuntu-24.04-pext | 5253 | shakmaty | 7194 | 1.370x |
| ubuntu-24.04-portable | 5776 | shakmaty | 6671 | 1.155x |
| windows-2025-native | 19600 | shakmaty | 27100 | 1.383x |

Cold wins: 6/6 (separate from native cases).
