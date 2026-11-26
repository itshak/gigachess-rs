| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 68 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 78 | 11 | 87 |
| ubuntu-24.04-native | 79 | 77 | 9 | 87 |
| ubuntu-24.04-pext | 80 | 79 | 15 | 87 |
| ubuntu-24.04-portable | 78 | 73 | 14 | 87 |
| windows-2025-native | 81 | 79 | 12 | 87 |
| ALL | 468 | 454 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.013x | 1.275x | 34/36 | 0.925x |
| chess960_materialize | 0.992x | 1.364x | 12/12 | 1.162x |
| chess960_perft_d3 | 1.069x | 1.370x | 12/12 | 1.143x |
| corpus_count | 0.984x | 1.116x | 8/12 | 0.798x |
| corpus_materialize | 0.981x | 1.107x | 10/12 | 0.993x |
| corpus_uci_sorted | 1.009x | 1.503x | 12/12 | 1.244x |
| count | 0.998x | 1.317x | 33/36 | 0.957x |
| fen_parse | 1.062x | 1.214x | 36/36 | 1.028x |
| fen_write_native_ep | 1.005x | 1.793x | 36/36 | 1.347x |
| make_unmake_full | 1.021x | 1.316x | 36/36 | 1.082x |
| materialize | 0.995x | 1.269x | 33/36 | 0.803x |
| perft_bulk_d4 | 0.999x | 1.145x | 32/36 | 0.838x |
| perft_full_d3 | 1.003x | 1.256x | 31/36 | 0.829x |
| polyglot_key | 1.005x | 31.027x | 12/12 | 16.303x |
| polyglot_recompute | 0.990x | 1.012x | 7/12 | 0.871x |
| repetition_key | 0.959x | 18.005x | 12/12 | 14.188x |
| replay_checked_native | 1.047x | 1.137x | 11/12 | 0.895x |
| replay_trusted_native | 1.018x | 0.828x | 5/12 | 0.507x |
| san_parse_all | 0.972x | 1.930x | 36/36 | 1.384x |
| san_render_native | 1.010x | 2.556x | 36/36 | 1.692x |
| snapshot | 0.997x | 1.070x | 20/36 | 0.657x |
| startpos | 0.867x | 1.067x | 4/6 | 0.580x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.507x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.574x |
| macos-15-native | startpos | start | shakmaty | 0.580x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.651x |
| macos-15-native | snapshot | tactical | chess | 0.657x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.664x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.701x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.798x |
| macos-15-native | materialize | tactical | ultra | 0.803x |
| macos-15-native | snapshot | middlegame | ferrum | 0.815x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.829x |
| macos-15-native | materialize | kiwipete | ultra | 0.837x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.838x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.847x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.871x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.875x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.892x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.895x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.910x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.925x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.932x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.946x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.946x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.954x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.954x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.955x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.957x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.959x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.961x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.963x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.969x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.969x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.971x |
| windows-2025-native | snapshot | start | cozy | 0.979x |
| windows-2025-native | snapshot | promotion | ferrum | 0.982x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.984x |
| macos-15-native | materialize | middlegame | ultra | 0.985x |
| windows-2025-native | snapshot | middlegame | chess | 0.987x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.990x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.992x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.992x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.993x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.994x |
| ubuntu-24.04-pext | count | middlegame | cozy | 0.994x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.996x |
| macos-15-native | count | middlegame | cozy | 0.998x |
| windows-2025-native | snapshot | tactical | cozy | 0.998x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5583 | shakmaty | 9292 | 1.664x |
| ubuntu-24.04-arm-native | 6752 | chess | 8640 | 1.280x |
| ubuntu-24.04-native | 5150 | shakmaty | 6867 | 1.333x |
| ubuntu-24.04-pext | 6690 | shakmaty | 9355 | 1.398x |
| ubuntu-24.04-portable | 5269 | shakmaty | 6409 | 1.216x |
| windows-2025-native | 16600 | shakmaty | 24300 | 1.464x |

Cold wins: 6/6 (separate from native cases).
