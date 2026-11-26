| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 68 | 11 | 87 |
| ubuntu-24.04-arm-native | 74 | 72 | 11 | 87 |
| ubuntu-24.04-native | 72 | 68 | 13 | 87 |
| ubuntu-24.04-pext | 81 | 73 | 11 | 87 |
| ubuntu-24.04-portable | 80 | 77 | 13 | 87 |
| windows-2025-native | 77 | 77 | 11 | 87 |
| ALL | 455 | 435 | 70 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.063x | 1.046x | 25/36 | 0.764x |
| chess960_materialize | 1.013x | 1.366x | 12/12 | 1.202x |
| chess960_perft_d3 | 0.993x | 1.371x | 12/12 | 1.259x |
| corpus_count | 1.011x | 1.339x | 12/12 | 1.114x |
| corpus_materialize | 0.995x | 1.157x | 12/12 | 1.052x |
| corpus_uci_sorted | 0.978x | 1.512x | 12/12 | 1.097x |
| count | 1.008x | 1.316x | 35/36 | 0.986x |
| fen_parse | 1.169x | 1.331x | 35/36 | 0.845x |
| fen_write_native_ep | 0.999x | 1.765x | 36/36 | 1.328x |
| make_unmake_full | 1.030x | 1.240x | 36/36 | 1.030x |
| materialize | 1.017x | 1.271x | 34/36 | 0.805x |
| perft_bulk_d4 | 1.029x | 1.258x | 36/36 | 1.041x |
| perft_full_d3 | 1.029x | 1.195x | 27/36 | 0.863x |
| polyglot_key | 1.034x | 32.719x | 12/12 | 16.287x |
| polyglot_recompute | 1.021x | 1.019x | 9/12 | 0.886x |
| repetition_key | 1.051x | 21.105x | 12/12 | 14.642x |
| replay_checked_native | 1.021x | 0.985x | 7/12 | 0.845x |
| replay_trusted_native | 0.968x | 0.638x | 0/12 | 0.479x |
| san_parse_all | 1.010x | 1.893x | 36/36 | 1.350x |
| san_render_native | 1.010x | 2.495x | 36/36 | 1.581x |
| snapshot | 1.112x | 1.008x | 14/36 | 0.844x |
| startpos | 1.016x | 1.245x | 5/6 | 0.997x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.479x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.484x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.490x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.523x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.591x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.614x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.643x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.657x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.753x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.764x |
| macos-15-native | materialize | start | ultra | 0.805x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.820x |
| macos-15-native | snapshot | start | ferrum | 0.844x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.845x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 0.845x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.863x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.866x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.874x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.875x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.884x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.886x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.887x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.892x |
| macos-15-native | apply_children_full | tactical | chess | 0.898x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.905x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.905x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.905x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.907x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.908x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.913x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.915x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.917x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.919x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.931x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.931x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.932x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.938x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.938x |
| macos-15-native | snapshot | middlegame | ferrum | 0.939x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.944x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.946x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.955x |
| windows-2025-native | apply_children_full | kiwipete | chess | 0.957x |
| macos-15-native | snapshot | tactical | ferrum | 0.959x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.962x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.966x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.969x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.970x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.974x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.978x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.981x |
| macos-15-native | snapshot | promotion | ferrum | 0.985x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.986x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.986x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.986x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.989x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.992x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.992x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.993x |
| macos-15-native | apply_children_full | endgame | cozy | 0.994x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.998x |
| macos-15-native | materialize | promotion | ultra | 0.999x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 16042 | ferrum | 7792 | 0.486x |
| ubuntu-24.04-arm-native | 10161 | shakmaty | 7592 | 0.747x |
| ubuntu-24.04-native | 11527 | shakmaty | 8573 | 0.744x |
| ubuntu-24.04-pext | 7960 | cozy | 7940 | 0.997x |
| ubuntu-24.04-portable | 8812 | shakmaty | 6137 | 0.696x |
| windows-2025-native | 23600 | shakmaty | 19700 | 0.835x |

Cold wins: 0/6 (separate from native cases).
