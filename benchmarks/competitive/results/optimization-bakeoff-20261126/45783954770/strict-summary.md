| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 67 | 11 | 87 |
| ubuntu-24.04-arm-native | 77 | 76 | 11 | 87 |
| ubuntu-24.04-native | 75 | 74 | 9 | 87 |
| ubuntu-24.04-pext | 83 | 75 | 12 | 87 |
| ubuntu-24.04-portable | 80 | 75 | 10 | 87 |
| windows-2025-native | 72 | 70 | 13 | 87 |
| ALL | 458 | 437 | 66 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.934x | 0.999x | 21/36 | 0.797x |
| chess960_materialize | 1.019x | 1.375x | 12/12 | 1.182x |
| chess960_perft_d3 | 0.988x | 1.375x | 12/12 | 1.257x |
| corpus_count | 1.006x | 1.350x | 12/12 | 1.122x |
| corpus_materialize | 1.001x | 1.153x | 12/12 | 1.063x |
| corpus_uci_sorted | 1.011x | 1.525x | 12/12 | 1.255x |
| count | 0.993x | 1.342x | 35/36 | 0.987x |
| fen_parse | 1.024x | 1.178x | 36/36 | 1.046x |
| fen_write_native_ep | 1.002x | 1.755x | 36/36 | 1.425x |
| make_unmake_full | 1.001x | 1.220x | 35/36 | 0.966x |
| materialize | 0.982x | 1.299x | 32/36 | 0.798x |
| perft_bulk_d4 | 1.013x | 1.275x | 36/36 | 1.044x |
| perft_full_d3 | 1.009x | 1.320x | 33/36 | 0.932x |
| polyglot_key | 1.016x | 32.775x | 12/12 | 16.515x |
| polyglot_recompute | 1.014x | 1.003x | 7/12 | 0.862x |
| repetition_key | 1.058x | 20.179x | 12/12 | 14.553x |
| replay_checked_native | 1.014x | 1.037x | 9/12 | 0.874x |
| replay_trusted_native | 0.978x | 0.635x | 0/12 | 0.474x |
| san_parse_all | 1.000x | 1.875x | 36/36 | 1.485x |
| san_render_native | 0.994x | 2.468x | 36/36 | 1.908x |
| snapshot | 0.970x | 0.979x | 17/36 | 0.578x |
| startpos | 0.930x | 1.180x | 5/6 | 0.998x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.474x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.486x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.529x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.533x |
| macos-15-native | snapshot | tactical | ferrum | 0.578x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.596x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.600x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.621x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.633x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.756x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.797x |
| macos-15-native | materialize | kiwipete | ultra | 0.798x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.811x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.813x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.817x |
| macos-15-native | apply_children_full | tactical | chess | 0.821x |
| windows-2025-native | apply_children_full | endgame | cozy | 0.824x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.830x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.845x |
| windows-2025-native | apply_children_full | start | cozy | 0.851x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.854x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.862x |
| windows-2025-native | apply_children_full | middlegame | cozy | 0.873x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.874x |
| macos-15-native | snapshot | promotion | ferrum | 0.883x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.885x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.909x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.910x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.914x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.915x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.917x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.918x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.920x |
| macos-15-native | materialize | tactical | ultra | 0.925x |
| windows-2025-native | materialize | kiwipete | ultra | 0.930x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.932x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.933x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.937x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.941x |
| windows-2025-native | snapshot | start | ferrum | 0.942x |
| macos-15-native | snapshot | endgame | ferrum | 0.951x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.952x |
| macos-15-native | apply_children_full | start | chess | 0.961x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.964x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.966x |
| macos-15-native | make_unmake_full | middlegame | ultra | 0.966x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.967x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.973x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.974x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.975x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.985x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.987x |
| macos-15-native | materialize | start | ferrum | 0.989x |
| ubuntu-24.04-pext | snapshot | tactical | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.998x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.999x |
| ubuntu-24.04-native | snapshot | start | cozy | 1.000x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 1.000x |
| ubuntu-24.04-native | snapshot | endgame | chess | 1.000x |
| ubuntu-24.04-native | snapshot | promotion | chess | 1.000x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 1.000x |
| ubuntu-24.04-native | snapshot | middlegame | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 9041 | chess | 7667 | 0.848x |
| ubuntu-24.04-arm-native | 10232 | shakmaty | 8176 | 0.799x |
| ubuntu-24.04-native | 8572 | shakmaty | 7295 | 0.851x |
| ubuntu-24.04-pext | 7771 | shakmaty | 7678 | 0.988x |
| ubuntu-24.04-portable | 9740 | shakmaty | 6667 | 0.684x |
| windows-2025-native | 29400 | shakmaty | 24300 | 0.827x |

Cold wins: 0/6 (separate from native cases).
