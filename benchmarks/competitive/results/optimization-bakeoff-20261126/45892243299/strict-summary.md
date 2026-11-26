| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 77 | 76 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-pext | 84 | 81 | 10 | 87 |
| ubuntu-24.04-portable | 80 | 80 | 13 | 87 |
| windows-2025-native | 84 | 83 | 12 | 87 |
| ALL | 483 | 478 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.000x | 1.278x | 35/36 | 0.968x |
| chess960_materialize | 0.980x | 1.476x | 12/12 | 1.290x |
| chess960_perft_d3 | 0.982x | 1.499x | 12/12 | 1.344x |
| corpus_count | 1.013x | 1.239x | 10/12 | 0.844x |
| corpus_materialize | 1.018x | 1.175x | 12/12 | 1.064x |
| corpus_uci_sorted | 1.015x | 1.506x | 12/12 | 1.275x |
| count | 0.998x | 1.472x | 36/36 | 1.092x |
| fen_parse | 0.987x | 1.164x | 36/36 | 1.019x |
| fen_write_native_ep | 1.015x | 1.745x | 36/36 | 1.442x |
| make_unmake_full | 0.992x | 1.344x | 36/36 | 1.062x |
| materialize | 0.989x | 1.289x | 34/36 | 0.858x |
| perft_bulk_d4 | 0.989x | 1.286x | 35/36 | 0.918x |
| perft_full_d3 | 1.012x | 1.323x | 36/36 | 1.166x |
| polyglot_key | 1.020x | 31.232x | 12/12 | 16.669x |
| polyglot_recompute | 1.085x | 1.094x | 9/12 | 0.937x |
| repetition_key | 0.984x | 21.357x | 12/12 | 14.578x |
| replay_checked_native | 1.006x | 1.154x | 12/12 | 1.066x |
| replay_trusted_native | 0.996x | 0.796x | 4/12 | 0.522x |
| san_parse_all | 0.985x | 1.894x | 36/36 | 1.411x |
| san_render_native | 1.007x | 2.615x | 36/36 | 1.720x |
| snapshot | 0.960x | 1.103x | 14/36 | 0.945x |
| startpos | 1.036x | 1.217x | 6/6 | 1.006x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.522x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.531x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.592x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.672x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.700x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.702x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.826x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.844x |
| macos-15-native | materialize | middlegame | ultra | 0.858x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.918x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.937x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.943x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.947x |
| macos-15-native | materialize | kiwipete | ultra | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.952x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.952x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.955x |
| macos-15-native | snapshot | tactical | ferrum | 0.959x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.960x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.960x |
| macos-15-native | snapshot | promotion | ferrum | 0.966x |
| macos-15-native | apply_children_full | tactical | chess | 0.968x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.970x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.972x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.972x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.973x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.973x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.974x |
| ubuntu-24.04-native | snapshot | promotion | cozy | 0.974x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.974x |
| ubuntu-24.04-native | snapshot | endgame | cozy | 0.975x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.975x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.978x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.980x |
| macos-15-native | snapshot | middlegame | ferrum | 0.985x |
| macos-15-native | snapshot | start | ferrum | 0.991x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3584 | shakmaty | 6000 | 1.674x |
| ubuntu-24.04-arm-native | 5960 | shakmaty | 8704 | 1.460x |
| ubuntu-24.04-native | 7183 | shakmaty | 10691 | 1.488x |
| ubuntu-24.04-pext | 5962 | shakmaty | 8906 | 1.494x |
| ubuntu-24.04-portable | 6771 | shakmaty | 9774 | 1.444x |
| windows-2025-native | 12500 | shakmaty | 16900 | 1.352x |

Cold wins: 6/6 (separate from native cases).
