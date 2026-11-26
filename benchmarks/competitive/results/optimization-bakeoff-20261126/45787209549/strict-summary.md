| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 69 | 12 | 87 |
| ubuntu-24.04-arm-native | 75 | 73 | 11 | 87 |
| ubuntu-24.04-native | 78 | 76 | 12 | 87 |
| ubuntu-24.04-pext | 84 | 79 | 10 | 87 |
| ubuntu-24.04-portable | 75 | 74 | 13 | 87 |
| windows-2025-native | 77 | 75 | 11 | 87 |
| ALL | 458 | 446 | 69 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.001x | 1.070x | 27/36 | 0.793x |
| chess960_materialize | 1.014x | 1.363x | 12/12 | 1.190x |
| chess960_perft_d3 | 0.995x | 1.392x | 12/12 | 1.221x |
| corpus_count | 0.977x | 1.345x | 12/12 | 1.151x |
| corpus_materialize | 0.975x | 1.146x | 10/12 | 0.916x |
| corpus_uci_sorted | 0.997x | 1.556x | 12/12 | 1.305x |
| count | 0.988x | 1.336x | 35/36 | 0.966x |
| fen_parse | 0.962x | 1.154x | 34/36 | 0.939x |
| fen_write_native_ep | 0.977x | 1.687x | 36/36 | 1.424x |
| make_unmake_full | 0.972x | 1.251x | 35/36 | 0.996x |
| materialize | 0.963x | 1.242x | 33/36 | 0.768x |
| perft_bulk_d4 | 0.978x | 1.293x | 36/36 | 1.090x |
| perft_full_d3 | 0.908x | 1.182x | 30/36 | 0.853x |
| polyglot_key | 0.993x | 30.319x | 12/12 | 16.549x |
| polyglot_recompute | 1.058x | 1.079x | 12/12 | 1.029x |
| repetition_key | 0.983x | 19.516x | 12/12 | 14.438x |
| replay_checked_native | 0.980x | 1.001x | 5/12 | 0.952x |
| replay_trusted_native | 0.992x | 0.614x | 0/12 | 0.431x |
| san_parse_all | 1.001x | 1.963x | 36/36 | 1.513x |
| san_render_native | 0.997x | 2.617x | 36/36 | 1.891x |
| snapshot | 1.047x | 0.992x | 15/36 | 0.891x |
| startpos | 1.039x | 1.218x | 6/6 | 1.107x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.431x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.474x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.495x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.536x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.544x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.561x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.598x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.609x |
| macos-15-native | materialize | kiwipete | ultra | 0.768x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.793x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.811x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.821x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.821x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.853x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.876x |
| macos-15-native | materialize | middlegame | ultra | 0.881x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.891x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.897x |
| ubuntu-24.04-native | perft_full_d3 | tactical | shakmaty | 0.897x |
| macos-15-native | snapshot | start | ferrum | 0.898x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.900x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.907x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.910x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | shakmaty | 0.910x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.913x |
| windows-2025-native | snapshot | start | ferrum | 0.915x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.916x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.916x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.918x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.922x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.925x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.927x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.927x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.929x |
| windows-2025-native | snapshot | tactical | ferrum | 0.930x |
| windows-2025-native | snapshot | promotion | ferrum | 0.930x |
| macos-15-native | snapshot | endgame | ferrum | 0.932x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.935x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.937x |
| macos-15-native | apply_children_full | tactical | chess | 0.938x |
| macos-15-native | fen_parse | endgame | shakmaty | 0.939x |
| windows-2025-native | snapshot | endgame | ferrum | 0.940x |
| macos-15-native | fen_parse | promotion | shakmaty | 0.942x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.948x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.949x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.952x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.952x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.956x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.961x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.963x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.964x |
| ubuntu-24.04-portable | apply_children_full | promotion | ferrum | 0.966x |
| ubuntu-24.04-portable | count | start | ferrum | 0.966x |
| macos-15-native | snapshot | middlegame | ferrum | 0.967x |
| macos-15-native | apply_children_full | promotion | chess | 0.970x |
| macos-15-native | materialize | tactical | ultra | 0.972x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.973x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.976x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.984x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.986x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.989x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.996x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 22875 | shakmaty | 13042 | 0.570x |
| ubuntu-24.04-arm-native | 12648 | chess | 8975 | 0.710x |
| ubuntu-24.04-native | 14527 | shakmaty | 10490 | 0.722x |
| ubuntu-24.04-pext | 7863 | shakmaty | 7223 | 0.919x |
| ubuntu-24.04-portable | 11708 | shakmaty | 7752 | 0.662x |
| windows-2025-native | 40000 | ferrum | 27300 | 0.682x |

Cold wins: 0/6 (separate from native cases).
