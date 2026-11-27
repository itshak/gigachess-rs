| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 69 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 77 | 12 | 87 |
| ubuntu-24.04-native | 84 | 81 | 12 | 87 |
| ubuntu-24.04-pext | 86 | 84 | 16 | 87 |
| ubuntu-24.04-portable | 81 | 76 | 11 | 87 |
| windows-2025-native | 81 | 79 | 13 | 87 |
| ALL | 485 | 466 | 78 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.977x | 1.241x | 33/36 | 0.836x |
| chess960_materialize | 1.039x | 1.487x | 12/12 | 1.275x |
| chess960_perft_d3 | 1.008x | 1.550x | 12/12 | 1.375x |
| corpus_count | 1.044x | 1.348x | 12/12 | 1.001x |
| corpus_materialize | 1.003x | 1.163x | 12/12 | 1.026x |
| corpus_uci_sorted | 0.989x | 1.528x | 12/12 | 1.189x |
| count | 1.003x | 1.474x | 36/36 | 1.006x |
| fen_parse | 1.011x | 1.175x | 35/36 | 0.984x |
| fen_write_native_ep | 1.014x | 1.764x | 36/36 | 1.421x |
| make_unmake_full | 1.017x | 1.342x | 36/36 | 1.041x |
| materialize | 0.990x | 1.295x | 35/36 | 0.806x |
| perft_bulk_d4 | 1.015x | 1.302x | 34/36 | 0.808x |
| perft_full_d3 | 1.015x | 1.322x | 36/36 | 1.083x |
| polyglot_key | 0.987x | 28.372x | 12/12 | 16.520x |
| polyglot_recompute | 0.986x | 1.020x | 7/12 | 0.905x |
| repetition_key | 1.013x | 19.542x | 12/12 | 14.533x |
| replay_checked_native | 0.967x | 1.112x | 12/12 | 1.007x |
| replay_trusted_native | 0.944x | 0.803x | 5/12 | 0.547x |
| san_parse_all | 1.004x | 1.951x | 36/36 | 1.414x |
| san_render_native | 1.017x | 2.652x | 36/36 | 1.720x |
| snapshot | 0.988x | 1.039x | 19/36 | 0.451x |
| startpos | 0.978x | 1.160x | 5/6 | 0.731x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | promotion | cozy | 0.451x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.547x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.557x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.613x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.661x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.688x |
| macos-15-native | snapshot | endgame | ferrum | 0.728x |
| macos-15-native | startpos | start | shakmaty | 0.731x |
| macos-15-native | materialize | middlegame | ultra | 0.806x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.808x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.836x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.845x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.884x |
| macos-15-native | apply_children_full | tactical | chess | 0.895x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.905x |
| macos-15-native | apply_children_full | kiwipete | cozy | 0.933x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.937x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.948x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.950x |
| macos-15-native | snapshot | middlegame | ferrum | 0.965x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.984x |
| windows-2025-native | snapshot | endgame | cozy | 0.985x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.987x |
| windows-2025-native | snapshot | promotion | chess | 0.987x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.988x |
| windows-2025-native | snapshot | tactical | chess | 0.988x |
| windows-2025-native | snapshot | kiwipete | chess | 0.988x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.993x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.995x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.995x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6292 | shakmaty | 11916 | 1.894x |
| ubuntu-24.04-arm-native | 5497 | shakmaty | 8960 | 1.630x |
| ubuntu-24.04-native | 7725 | shakmaty | 10990 | 1.423x |
| ubuntu-24.04-pext | 6389 | shakmaty | 9604 | 1.503x |
| ubuntu-24.04-portable | 6084 | shakmaty | 8034 | 1.321x |
| windows-2025-native | 18400 | shakmaty | 26400 | 1.435x |

Cold wins: 6/6 (separate from native cases).
