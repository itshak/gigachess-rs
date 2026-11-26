| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 67 | 12 | 87 |
| ubuntu-24.04-arm-native | 82 | 78 | 11 | 87 |
| ubuntu-24.04-native | 79 | 79 | 11 | 87 |
| ubuntu-24.04-pext | 82 | 80 | 10 | 87 |
| ubuntu-24.04-portable | 83 | 82 | 12 | 87 |
| windows-2025-native | 77 | 75 | 10 | 87 |
| ALL | 478 | 461 | 66 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.908x | 1.151x | 30/36 | 0.805x |
| chess960_materialize | 0.950x | 1.384x | 12/12 | 1.083x |
| chess960_perft_d3 | 0.937x | 1.404x | 12/12 | 1.315x |
| corpus_count | 0.972x | 1.163x | 8/12 | 0.815x |
| corpus_materialize | 1.015x | 1.138x | 12/12 | 1.037x |
| corpus_uci_sorted | 1.041x | 1.578x | 12/12 | 1.385x |
| count | 0.969x | 1.361x | 36/36 | 1.064x |
| fen_parse | 1.027x | 1.226x | 36/36 | 1.077x |
| fen_write_native_ep | 0.942x | 1.608x | 36/36 | 1.213x |
| make_unmake_full | 0.944x | 1.243x | 34/36 | 0.845x |
| materialize | 0.977x | 1.225x | 33/36 | 0.907x |
| perft_bulk_d4 | 0.945x | 1.193x | 35/36 | 0.874x |
| perft_full_d3 | 0.865x | 1.112x | 30/36 | 0.751x |
| polyglot_key | 1.055x | 31.484x | 12/12 | 18.811x |
| polyglot_recompute | 1.007x | 1.007x | 7/12 | 0.896x |
| repetition_key | 1.064x | 22.248x | 12/12 | 17.474x |
| replay_checked_native | 0.927x | 1.044x | 8/12 | 0.905x |
| replay_trusted_native | 0.996x | 0.782x | 4/12 | 0.495x |
| san_parse_all | 0.980x | 1.875x | 36/36 | 1.470x |
| san_render_native | 0.956x | 2.446x | 36/36 | 1.510x |
| snapshot | 1.006x | 1.219x | 32/36 | 0.739x |
| startpos | 1.310x | 1.631x | 5/6 | 0.913x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.495x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.542x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.561x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.636x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.655x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.670x |
| macos-15-native | snapshot | promotion | ferrum | 0.739x |
| macos-15-native | perft_full_d3 | tactical | ultra | 0.751x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.757x |
| windows-2025-native | apply_children_full | middlegame | chess | 0.805x |
| windows-2025-native | apply_children_full | promotion | chess | 0.812x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.815x |
| windows-2025-native | apply_children_full | endgame | chess | 0.828x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.833x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.845x |
| windows-2025-native | apply_children_full | kiwipete | chess | 0.871x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.874x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.898x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.903x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.905x |
| macos-15-native | materialize | kiwipete | ultra | 0.907x |
| windows-2025-native | startpos | start | shakmaty | 0.913x |
| windows-2025-native | apply_children_full | tactical | chess | 0.917x |
| macos-15-native | snapshot | endgame | chess | 0.924x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.933x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.937x |
| macos-15-native | perft_full_d3 | endgame | ultra | 0.939x |
| windows-2025-native | apply_children_full | start | chess | 0.939x |
| macos-15-native | materialize | tactical | ultra | 0.954x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.964x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.964x |
| ubuntu-24.04-portable | replay_checked_native | lichess300 | cozy | 0.969x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.975x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.976x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.976x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.983x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.983x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.984x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.990x |
| macos-15-native | perft_full_d3 | middlegame | ultra | 0.990x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.991x |
| ubuntu-24.04-arm-native | snapshot | endgame | cozy | 0.996x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5708 | shakmaty | 8459 | 1.482x |
| ubuntu-24.04-arm-native | 5360 | shakmaty | 9104 | 1.699x |
| ubuntu-24.04-native | 7071 | shakmaty | 7562 | 1.069x |
| ubuntu-24.04-pext | 5699 | cozy | 8154 | 1.431x |
| ubuntu-24.04-portable | 8837 | shakmaty | 12043 | 1.363x |
| windows-2025-native | 11600 | shakmaty | 13100 | 1.129x |

Cold wins: 6/6 (separate from native cases).
