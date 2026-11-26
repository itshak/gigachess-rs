| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 63 | 12 | 87 |
| ubuntu-24.04-arm-native | 75 | 73 | 11 | 87 |
| ubuntu-24.04-native | 83 | 79 | 11 | 87 |
| ubuntu-24.04-pext | 72 | 71 | 11 | 87 |
| ubuntu-24.04-portable | 77 | 70 | 10 | 87 |
| windows-2025-native | 76 | 72 | 11 | 87 |
| ALL | 450 | 428 | 66 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.946x | 0.989x | 18/36 | 0.681x |
| chess960_materialize | 0.982x | 1.325x | 12/12 | 1.182x |
| chess960_perft_d3 | 0.984x | 1.446x | 12/12 | 1.255x |
| corpus_count | 0.989x | 1.307x | 11/12 | 0.992x |
| corpus_materialize | 0.994x | 1.196x | 12/12 | 1.026x |
| corpus_uci_sorted | 0.996x | 1.539x | 12/12 | 1.279x |
| count | 0.990x | 1.319x | 35/36 | 0.973x |
| fen_parse | 0.994x | 1.213x | 36/36 | 1.008x |
| fen_write_native_ep | 0.992x | 1.660x | 36/36 | 1.326x |
| make_unmake_full | 0.978x | 1.218x | 34/36 | 0.754x |
| materialize | 0.981x | 1.148x | 30/36 | 0.767x |
| perft_bulk_d4 | 1.014x | 1.296x | 36/36 | 1.033x |
| perft_full_d3 | 0.964x | 1.132x | 28/36 | 0.860x |
| polyglot_key | 0.966x | 25.672x | 12/12 | 15.505x |
| polyglot_recompute | 0.978x | 0.999x | 7/12 | 0.820x |
| repetition_key | 1.028x | 19.879x | 12/12 | 13.703x |
| replay_checked_native | 0.982x | 0.966x | 5/12 | 0.827x |
| replay_trusted_native | 0.974x | 0.640x | 0/12 | 0.481x |
| san_parse_all | 0.997x | 1.944x | 36/36 | 1.533x |
| san_render_native | 0.963x | 2.460x | 36/36 | 1.551x |
| snapshot | 1.012x | 1.037x | 25/36 | 0.775x |
| startpos | 0.990x | 1.174x | 5/6 | 0.876x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.481x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.495x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.496x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.509x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.575x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.587x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.618x |
| macos-15-native | apply_children_full | promotion | chess | 0.681x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.736x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.754x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.766x |
| macos-15-native | materialize | middlegame | ferrum | 0.767x |
| macos-15-native | snapshot | tactical | ferrum | 0.775x |
| macos-15-native | apply_children_full | tactical | cozy | 0.805x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.820x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.826x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.827x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.836x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.837x |
| macos-15-native | snapshot | middlegame | ferrum | 0.846x |
| macos-15-native | materialize | tactical | ultra | 0.854x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.859x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.860x |
| ubuntu-24.04-pext | apply_children_full | kiwipete | cozy | 0.863x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.864x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.866x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.872x |
| windows-2025-native | startpos | start | shakmaty | 0.876x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.880x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.885x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.885x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.887x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.889x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.890x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.897x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.901x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.909x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.911x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.913x |
| macos-15-native | materialize | kiwipete | ultra | 0.914x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.916x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.918x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.923x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.924x |
| ubuntu-24.04-pext | apply_children_full | start | cozy | 0.927x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.929x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.935x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.935x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.944x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.950x |
| ubuntu-24.04-pext | apply_children_full | endgame | cozy | 0.950x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.951x |
| ubuntu-24.04-portable | apply_children_full | tactical | ferrum | 0.954x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.954x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.955x |
| ubuntu-24.04-pext | apply_children_full | middlegame | cozy | 0.956x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.959x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.960x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.966x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.968x |
| ubuntu-24.04-portable | count | start | ferrum | 0.973x |
| windows-2025-native | materialize | kiwipete | ultra | 0.973x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.975x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.979x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.990x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.992x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.992x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.993x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.995x |
| macos-15-native | materialize | promotion | ultra | 0.996x |
| windows-2025-native | materialize | middlegame | ultra | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 10417 | shakmaty | 5375 | 0.516x |
| ubuntu-24.04-arm-native | 13609 | shakmaty | 10144 | 0.745x |
| ubuntu-24.04-native | 14457 | shakmaty | 9589 | 0.663x |
| ubuntu-24.04-pext | 11957 | shakmaty | 9384 | 0.785x |
| ubuntu-24.04-portable | 12208 | shakmaty | 11176 | 0.915x |
| windows-2025-native | 21600 | shakmaty | 16800 | 0.778x |

Cold wins: 0/6 (separate from native cases).
