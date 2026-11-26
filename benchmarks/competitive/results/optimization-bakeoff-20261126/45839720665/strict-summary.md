| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 67 | 12 | 87 |
| ubuntu-24.04-arm-native | 73 | 73 | 12 | 87 |
| ubuntu-24.04-native | 82 | 80 | 9 | 87 |
| ubuntu-24.04-pext | 82 | 80 | 10 | 87 |
| ubuntu-24.04-portable | 76 | 71 | 10 | 87 |
| windows-2025-native | 78 | 75 | 14 | 87 |
| ALL | 463 | 446 | 67 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.010x | 1.243x | 33/36 | 0.913x |
| chess960_materialize | 1.004x | 1.464x | 12/12 | 1.238x |
| chess960_perft_d3 | 1.036x | 1.519x | 12/12 | 1.413x |
| corpus_count | 0.991x | 1.238x | 9/12 | 0.816x |
| corpus_materialize | 1.024x | 1.151x | 12/12 | 1.041x |
| corpus_uci_sorted | 0.996x | 1.510x | 12/12 | 1.291x |
| count | 0.954x | 1.421x | 35/36 | 0.904x |
| fen_parse | 1.024x | 1.202x | 35/36 | 0.997x |
| fen_write_native_ep | 0.994x | 1.637x | 36/36 | 1.400x |
| make_unmake_full | 0.980x | 1.309x | 35/36 | 0.999x |
| materialize | 0.992x | 1.270x | 32/36 | 0.892x |
| perft_bulk_d4 | 0.995x | 1.258x | 34/36 | 0.894x |
| perft_full_d3 | 1.014x | 1.070x | 25/36 | 0.767x |
| polyglot_key | 0.982x | 32.598x | 12/12 | 16.798x |
| polyglot_recompute | 1.007x | 1.013x | 7/12 | 0.903x |
| repetition_key | 0.999x | 22.437x | 12/12 | 14.812x |
| replay_checked_native | 1.102x | 1.190x | 12/12 | 1.088x |
| replay_trusted_native | 1.080x | 0.760x | 4/12 | 0.532x |
| san_parse_all | 0.981x | 1.853x | 36/36 | 1.384x |
| san_render_native | 0.997x | 2.528x | 36/36 | 1.770x |
| snapshot | 0.963x | 1.061x | 16/36 | 0.710x |
| startpos | 1.024x | 1.234x | 6/6 | 1.119x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.532x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.559x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.583x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.587x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.658x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.683x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.689x |
| macos-15-native | snapshot | start | cozy | 0.710x |
| macos-15-native | snapshot | endgame | ferrum | 0.745x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.767x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.790x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.795x |
| windows-2025-native | snapshot | promotion | ferrum | 0.806x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.816x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | shakmaty | 0.843x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.850x |
| ubuntu-24.04-arm-native | perft_full_d3 | endgame | ultra | 0.852x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.869x |
| windows-2025-native | perft_full_d3 | tactical | shakmaty | 0.872x |
| macos-15-native | materialize | endgame | ultra | 0.892x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.894x |
| windows-2025-native | perft_full_d3 | kiwipete | shakmaty | 0.897x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.903x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.903x |
| macos-15-native | count | endgame | cozy | 0.904x |
| macos-15-native | materialize | kiwipete | ultra | 0.912x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.913x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.919x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.921x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.928x |
| ubuntu-24.04-portable | apply_children_full | tactical | ferrum | 0.939x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.944x |
| macos-15-native | materialize | tactical | ultra | 0.945x |
| windows-2025-native | snapshot | endgame | chess | 0.946x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.952x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.953x |
| windows-2025-native | snapshot | start | ferrum | 0.956x |
| macos-15-native | materialize | middlegame | ultra | 0.956x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.966x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.971x |
| macos-15-native | perft_full_d3 | kiwipete | shakmaty | 0.979x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.981x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.986x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.987x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.991x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.992x |
| ubuntu-24.04-arm-native | perft_full_d3 | start | ultra | 0.993x |
| macos-15-native | snapshot | promotion | ferrum | 0.995x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.996x |
| macos-15-native | snapshot | tactical | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.997x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.997x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.998x |
| ubuntu-24.04-pext | make_unmake_full | kiwipete | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 12291 | chess | 15792 | 1.285x |
| ubuntu-24.04-arm-native | 6457 | chess | 8264 | 1.280x |
| ubuntu-24.04-native | 5655 | shakmaty | 7108 | 1.257x |
| ubuntu-24.04-pext | 6660 | cozy | 9885 | 1.484x |
| ubuntu-24.04-portable | 5528 | shakmaty | 8893 | 1.609x |
| windows-2025-native | 23000 | shakmaty | 26200 | 1.139x |

Cold wins: 6/6 (separate from native cases).
