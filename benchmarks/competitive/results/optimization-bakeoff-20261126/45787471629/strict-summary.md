| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 68 | 12 | 87 |
| ubuntu-24.04-arm-native | 75 | 73 | 11 | 87 |
| ubuntu-24.04-native | 81 | 79 | 10 | 87 |
| ubuntu-24.04-pext | 75 | 73 | 14 | 87 |
| ubuntu-24.04-portable | 79 | 76 | 10 | 87 |
| windows-2025-native | 79 | 75 | 11 | 87 |
| ALL | 462 | 444 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.012x | 1.053x | 28/36 | 0.796x |
| chess960_materialize | 1.004x | 1.330x | 12/12 | 1.117x |
| chess960_perft_d3 | 0.981x | 1.360x | 12/12 | 1.232x |
| corpus_count | 0.977x | 1.364x | 12/12 | 1.075x |
| corpus_materialize | 1.004x | 1.199x | 12/12 | 1.134x |
| corpus_uci_sorted | 0.975x | 1.535x | 12/12 | 1.247x |
| count | 1.001x | 1.333x | 35/36 | 0.990x |
| fen_parse | 0.999x | 1.184x | 35/36 | 0.996x |
| fen_write_native_ep | 0.998x | 1.733x | 36/36 | 1.437x |
| make_unmake_full | 0.997x | 1.230x | 35/36 | 0.925x |
| materialize | 0.982x | 1.248x | 33/36 | 0.858x |
| perft_bulk_d4 | 1.022x | 1.304x | 36/36 | 1.042x |
| perft_full_d3 | 0.991x | 1.209x | 30/36 | 0.640x |
| polyglot_key | 0.975x | 30.229x | 12/12 | 16.650x |
| polyglot_recompute | 0.990x | 1.036x | 9/12 | 0.884x |
| repetition_key | 1.008x | 19.169x | 12/12 | 8.360x |
| replay_checked_native | 0.889x | 0.910x | 1/12 | 0.808x |
| replay_trusted_native | 0.959x | 0.630x | 0/12 | 0.360x |
| san_parse_all | 1.009x | 1.964x | 36/36 | 1.432x |
| san_render_native | 0.961x | 2.461x | 36/36 | 1.594x |
| snapshot | 0.895x | 1.004x | 22/36 | 0.697x |
| startpos | 0.969x | 1.259x | 6/6 | 1.183x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.360x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.483x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.509x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.592x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.608x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.614x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.640x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.641x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.651x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.675x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.686x |
| macos-15-native | snapshot | middlegame | cozy | 0.697x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.719x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.720x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.752x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.796x |
| ubuntu-24.04-pext | perft_full_d3 | endgame | ultra | 0.805x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.808x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.821x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.852x |
| macos-15-native | materialize | kiwipete | ultra | 0.858x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.865x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.871x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.878x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.880x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.884x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.892x |
| macos-15-native | materialize | tactical | ultra | 0.899x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.900x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.903x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.906x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.913x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.914x |
| ubuntu-24.04-portable | replay_checked_native | lichess300 | cozy | 0.915x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.916x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.919x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.925x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.926x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.927x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.930x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.934x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.938x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.938x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.941x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.948x |
| macos-15-native | materialize | middlegame | ultra | 0.956x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.957x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.959x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.969x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.971x |
| macos-15-native | snapshot | endgame | ferrum | 0.971x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.971x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.979x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.990x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.990x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.996x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.997x |
| windows-2025-native | snapshot | start | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 16417 | chess | 12875 | 0.784x |
| ubuntu-24.04-arm-native | 10399 | shakmaty | 8312 | 0.799x |
| ubuntu-24.04-native | 12894 | chess | 11782 | 0.914x |
| ubuntu-24.04-pext | 11247 | shakmaty | 9224 | 0.820x |
| ubuntu-24.04-portable | 9881 | shakmaty | 7485 | 0.758x |
| windows-2025-native | 30800 | shakmaty | 23200 | 0.753x |

Cold wins: 0/6 (separate from native cases).
