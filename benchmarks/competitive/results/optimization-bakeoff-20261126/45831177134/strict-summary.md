| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 68 | 66 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 76 | 11 | 87 |
| ubuntu-24.04-native | 78 | 77 | 9 | 87 |
| ubuntu-24.04-pext | 83 | 79 | 12 | 87 |
| ubuntu-24.04-portable | 85 | 81 | 11 | 87 |
| windows-2025-native | 79 | 76 | 11 | 87 |
| ALL | 471 | 455 | 65 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.030x | 1.262x | 35/36 | 0.942x |
| chess960_materialize | 0.989x | 1.308x | 12/12 | 1.040x |
| chess960_perft_d3 | 1.034x | 1.384x | 12/12 | 1.161x |
| corpus_count | 0.992x | 1.088x | 7/12 | 0.745x |
| corpus_materialize | 1.038x | 1.126x | 11/12 | 0.985x |
| corpus_uci_sorted | 1.013x | 1.554x | 12/12 | 1.252x |
| count | 0.993x | 1.299x | 35/36 | 0.941x |
| fen_parse | 1.036x | 1.203x | 33/36 | 0.937x |
| fen_write_native_ep | 0.983x | 1.672x | 36/36 | 1.425x |
| make_unmake_full | 1.026x | 1.343x | 34/36 | 0.885x |
| materialize | 1.012x | 1.225x | 32/36 | 0.971x |
| perft_bulk_d4 | 0.982x | 1.154x | 29/36 | 0.817x |
| perft_full_d3 | 0.998x | 1.243x | 32/36 | 0.909x |
| polyglot_key | 1.002x | 28.523x | 12/12 | 16.231x |
| polyglot_recompute | 1.030x | 1.003x | 7/12 | 0.864x |
| repetition_key | 1.030x | 20.764x | 12/12 | 14.452x |
| replay_checked_native | 1.014x | 1.059x | 10/12 | 0.982x |
| replay_trusted_native | 1.037x | 0.835x | 5/12 | 0.519x |
| san_parse_all | 1.006x | 1.894x | 36/36 | 1.328x |
| san_render_native | 1.009x | 2.475x | 36/36 | 1.733x |
| snapshot | 1.022x | 1.068x | 27/36 | 0.583x |
| startpos | 0.958x | 1.269x | 6/6 | 1.000x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.519x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| macos-15-native | snapshot | start | ferrum | 0.583x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.653x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.678x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.692x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.745x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.749x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.797x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.817x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.864x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.865x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.880x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.885x |
| macos-15-native | snapshot | kiwipete | cozy | 0.886x |
| macos-15-native | corpus_count | seeded | ultra | 0.894x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.896x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.909x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.909x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.914x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.916x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.925x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.925x |
| macos-15-native | snapshot | tactical | ferrum | 0.927x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.937x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.937x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.940x |
| macos-15-native | count | kiwipete | ultra | 0.941x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.941x |
| macos-15-native | apply_children_full | kiwipete | cozy | 0.942x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.958x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.962x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.964x |
| macos-15-native | fen_parse | endgame | shakmaty | 0.966x |
| ubuntu-24.04-native | perft_bulk_d4 | tactical | ultra | 0.969x |
| windows-2025-native | materialize | kiwipete | ultra | 0.971x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.972x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.982x |
| macos-15-native | materialize | tactical | ultra | 0.983x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.985x |
| macos-15-native | materialize | middlegame | ultra | 0.986x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.987x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.991x |
| windows-2025-native | materialize | middlegame | ultra | 0.991x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.994x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7333 | shakmaty | 11625 | 1.585x |
| ubuntu-24.04-arm-native | 6648 | shakmaty | 9712 | 1.461x |
| ubuntu-24.04-native | 6404 | shakmaty | 8379 | 1.308x |
| ubuntu-24.04-pext | 7294 | shakmaty | 9578 | 1.313x |
| ubuntu-24.04-portable | 8035 | shakmaty | 10439 | 1.299x |
| windows-2025-native | 13700 | shakmaty | 20800 | 1.518x |

Cold wins: 6/6 (separate from native cases).
