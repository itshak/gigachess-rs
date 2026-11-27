| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 72 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 13 | 87 |
| ubuntu-24.04-native | 81 | 77 | 12 | 87 |
| ubuntu-24.04-pext | 85 | 85 | 12 | 87 |
| ubuntu-24.04-portable | 84 | 78 | 12 | 87 |
| windows-2025-native | 81 | 79 | 11 | 87 |
| ALL | 483 | 470 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.030x | 1.274x | 34/36 | 0.974x |
| chess960_materialize | 1.030x | 1.478x | 12/12 | 1.273x |
| chess960_perft_d3 | 1.022x | 1.543x | 12/12 | 1.429x |
| corpus_count | 0.984x | 1.174x | 8/12 | 0.931x |
| corpus_materialize | 1.001x | 1.154x | 12/12 | 1.043x |
| corpus_uci_sorted | 1.007x | 1.580x | 12/12 | 1.291x |
| count | 1.015x | 1.497x | 36/36 | 1.148x |
| fen_parse | 0.969x | 1.130x | 34/36 | 0.932x |
| fen_write_native_ep | 1.000x | 1.743x | 36/36 | 1.229x |
| make_unmake_full | 0.998x | 1.309x | 36/36 | 1.045x |
| materialize | 1.033x | 1.313x | 33/36 | 0.816x |
| perft_bulk_d4 | 1.028x | 1.284x | 34/36 | 0.858x |
| perft_full_d3 | 1.017x | 1.331x | 36/36 | 1.154x |
| polyglot_key | 1.009x | 31.335x | 12/12 | 15.049x |
| polyglot_recompute | 1.005x | 1.023x | 7/12 | 0.945x |
| repetition_key | 1.059x | 21.170x | 12/12 | 13.961x |
| replay_checked_native | 1.008x | 1.161x | 12/12 | 1.069x |
| replay_trusted_native | 1.004x | 0.783x | 4/12 | 0.514x |
| san_parse_all | 0.999x | 1.840x | 36/36 | 1.412x |
| san_render_native | 1.014x | 2.615x | 36/36 | 1.553x |
| snapshot | 1.006x | 1.067x | 24/36 | 0.708x |
| startpos | 1.024x | 1.270x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.514x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.618x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.675x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.677x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| macos-15-native | snapshot | start | cozy | 0.708x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.759x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.763x |
| macos-15-native | materialize | kiwipete | ultra | 0.816x |
| macos-15-native | snapshot | middlegame | cozy | 0.822x |
| macos-15-native | snapshot | endgame | ferrum | 0.842x |
| macos-15-native | materialize | tactical | ultra | 0.854x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.858x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.874x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.895x |
| macos-15-native | snapshot | promotion | chess | 0.905x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.931x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.932x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.945x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.950x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.951x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.953x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.956x |
| windows-2025-native | corpus_count | seeded | ultra | 0.956x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.958x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.963x |
| macos-15-native | materialize | middlegame | ultra | 0.965x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.972x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.974x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.976x |
| ubuntu-24.04-native | fen_parse | endgame | ultra | 0.978x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.986x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.992x |
| ubuntu-24.04-portable | snapshot | tactical | chess | 0.997x |
| windows-2025-native | startpos | start | shakmaty | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5334 | shakmaty | 9292 | 1.742x |
| ubuntu-24.04-arm-native | 6168 | shakmaty | 9015 | 1.462x |
| ubuntu-24.04-native | 5670 | shakmaty | 6255 | 1.103x |
| ubuntu-24.04-pext | 8907 | shakmaty | 10680 | 1.199x |
| ubuntu-24.04-portable | 8125 | shakmaty | 11021 | 1.356x |
| windows-2025-native | 15400 | shakmaty | 25200 | 1.636x |

Cold wins: 6/6 (separate from native cases).
