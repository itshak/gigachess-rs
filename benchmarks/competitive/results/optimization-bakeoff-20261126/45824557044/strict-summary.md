| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 68 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 81 | 78 | 9 | 87 |
| ubuntu-24.04-pext | 80 | 80 | 13 | 87 |
| ubuntu-24.04-portable | 85 | 76 | 12 | 87 |
| windows-2025-native | 76 | 75 | 11 | 87 |
| ALL | 475 | 454 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.913x | 1.159x | 30/36 | 0.819x |
| chess960_materialize | 0.979x | 1.414x | 12/12 | 1.103x |
| chess960_perft_d3 | 0.997x | 1.383x | 12/12 | 1.148x |
| corpus_count | 0.998x | 1.087x | 7/12 | 0.807x |
| corpus_materialize | 1.005x | 1.114x | 10/12 | 0.969x |
| corpus_uci_sorted | 1.010x | 1.526x | 12/12 | 1.298x |
| count | 1.015x | 1.329x | 35/36 | 0.950x |
| fen_parse | 1.037x | 1.215x | 36/36 | 1.088x |
| fen_write_native_ep | 0.988x | 1.702x | 36/36 | 1.389x |
| make_unmake_full | 1.019x | 1.291x | 36/36 | 1.077x |
| materialize | 0.997x | 1.206x | 34/36 | 0.863x |
| perft_bulk_d4 | 0.994x | 1.139x | 30/36 | 0.800x |
| perft_full_d3 | 1.000x | 1.238x | 31/36 | 0.857x |
| polyglot_key | 1.021x | 30.033x | 12/12 | 16.246x |
| polyglot_recompute | 1.007x | 1.022x | 8/12 | 0.869x |
| repetition_key | 1.095x | 20.164x | 12/12 | 14.150x |
| replay_checked_native | 1.006x | 1.081x | 11/12 | 0.970x |
| replay_trusted_native | 1.026x | 0.783x | 4/12 | 0.496x |
| san_parse_all | 1.006x | 1.941x | 36/36 | 1.553x |
| san_render_native | 0.985x | 2.459x | 36/36 | 1.597x |
| snapshot | 0.901x | 1.058x | 29/36 | 0.864x |
| startpos | 0.974x | 1.209x | 6/6 | 1.156x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.496x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.530x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.552x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.651x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.711x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.800x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.807x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.819x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.819x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.830x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.832x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.850x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.857x |
| macos-15-native | materialize | middlegame | ultra | 0.863x |
| macos-15-native | snapshot | start | ferrum | 0.864x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.869x |
| windows-2025-native | apply_children_full | middlegame | cozy | 0.880x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.894x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.900x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.902x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.908x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.910x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.914x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.916x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.916x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.918x |
| windows-2025-native | apply_children_full | endgame | cozy | 0.919x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.930x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.933x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.936x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.937x |
| macos-15-native | materialize | kiwipete | ultra | 0.938x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.939x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.941x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.942x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.942x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.945x |
| windows-2025-native | apply_children_full | start | cozy | 0.947x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.947x |
| macos-15-native | count | middlegame | cozy | 0.950x |
| windows-2025-native | corpus_materialize | seeded | ultra | 0.969x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.970x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.972x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.984x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.997x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7458 | ferrum | 7583 | 1.017x |
| ubuntu-24.04-arm-native | 7056 | shakmaty | 10433 | 1.479x |
| ubuntu-24.04-native | 5484 | shakmaty | 6879 | 1.254x |
| ubuntu-24.04-pext | 6840 | shakmaty | 9745 | 1.425x |
| ubuntu-24.04-portable | 7725 | shakmaty | 11171 | 1.446x |
| windows-2025-native | 14200 | shakmaty | 17300 | 1.218x |

Cold wins: 6/6 (separate from native cases).
