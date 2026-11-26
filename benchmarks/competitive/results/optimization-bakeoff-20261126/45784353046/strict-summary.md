| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 65 | 14 | 87 |
| ubuntu-24.04-arm-native | 74 | 70 | 10 | 87 |
| ubuntu-24.04-native | 79 | 73 | 9 | 87 |
| ubuntu-24.04-pext | 79 | 75 | 10 | 87 |
| ubuntu-24.04-portable | 83 | 76 | 11 | 87 |
| windows-2025-native | 71 | 69 | 11 | 87 |
| ALL | 453 | 428 | 65 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.940x | 1.002x | 22/36 | 0.793x |
| chess960_materialize | 0.978x | 1.393x | 12/12 | 1.293x |
| chess960_perft_d3 | 0.960x | 1.351x | 12/12 | 1.095x |
| corpus_count | 0.949x | 1.355x | 12/12 | 1.131x |
| corpus_materialize | 0.993x | 1.131x | 11/12 | 0.976x |
| corpus_uci_sorted | 1.035x | 1.597x | 12/12 | 1.447x |
| count | 1.002x | 1.343x | 35/36 | 0.861x |
| fen_parse | 1.028x | 1.175x | 36/36 | 1.006x |
| fen_write_native_ep | 1.001x | 1.743x | 36/36 | 1.026x |
| make_unmake_full | 1.018x | 1.230x | 34/36 | 0.987x |
| materialize | 1.008x | 1.304x | 32/36 | 0.849x |
| perft_bulk_d4 | 1.008x | 1.270x | 36/36 | 1.088x |
| perft_full_d3 | 1.032x | 1.362x | 35/36 | 0.913x |
| polyglot_key | 1.016x | 34.763x | 12/12 | 16.091x |
| polyglot_recompute | 0.921x | 0.983x | 4/12 | 0.904x |
| repetition_key | 1.012x | 20.204x | 12/12 | 14.120x |
| replay_checked_native | 1.002x | 1.057x | 10/12 | 0.929x |
| replay_trusted_native | 0.955x | 0.631x | 0/12 | 0.462x |
| san_parse_all | 1.023x | 1.887x | 36/36 | 1.506x |
| san_render_native | 1.011x | 2.566x | 36/36 | 1.871x |
| snapshot | 0.956x | 0.961x | 12/36 | 0.593x |
| startpos | 1.013x | 1.198x | 6/6 | 1.056x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.462x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.489x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.534x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.537x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.585x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.592x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.593x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.628x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.640x |
| macos-15-native | snapshot | promotion | chess | 0.732x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.793x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.794x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.802x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.803x |
| windows-2025-native | apply_children_full | middlegame | cozy | 0.804x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.806x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.814x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.815x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.816x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.846x |
| macos-15-native | materialize | tactical | ultra | 0.849x |
| macos-15-native | materialize | kiwipete | ultra | 0.854x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.858x |
| windows-2025-native | apply_children_full | endgame | chess | 0.859x |
| macos-15-native | count | middlegame | cozy | 0.861x |
| macos-15-native | apply_children_full | tactical | chess | 0.865x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.894x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.894x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.894x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.896x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.896x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.896x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.904x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.913x |
| macos-15-native | snapshot | tactical | ferrum | 0.916x |
| macos-15-native | materialize | endgame | ultra | 0.919x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.929x |
| windows-2025-native | apply_children_full | start | chess | 0.931x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.933x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.942x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.949x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.952x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.952x |
| windows-2025-native | snapshot | promotion | ferrum | 0.953x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.964x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.967x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.971x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.972x |
| macos-15-native | snapshot | middlegame | ferrum | 0.974x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.976x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.977x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.977x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.978x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.981x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.982x |
| windows-2025-native | snapshot | endgame | ferrum | 0.984x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.987x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.988x |
| macos-15-native | materialize | promotion | ultra | 0.993x |
| windows-2025-native | snapshot | tactical | ferrum | 0.996x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.997x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.997x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.998x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.999x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.999x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.999x |
| ubuntu-24.04-pext | snapshot | promotion | ferrum | 0.999x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.999x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 11875 | shakmaty | 8834 | 0.744x |
| ubuntu-24.04-arm-native | 11151 | shakmaty | 8208 | 0.736x |
| ubuntu-24.04-native | 8534 | shakmaty | 7055 | 0.827x |
| ubuntu-24.04-pext | 8215 | cozy | 7834 | 0.954x |
| ubuntu-24.04-portable | 14056 | shakmaty | 10530 | 0.749x |
| windows-2025-native | 22500 | shakmaty | 17200 | 0.764x |

Cold wins: 0/6 (separate from native cases).
