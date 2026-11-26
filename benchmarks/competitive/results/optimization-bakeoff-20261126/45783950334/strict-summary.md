| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 66 | 12 | 87 |
| ubuntu-24.04-arm-native | 76 | 73 | 11 | 87 |
| ubuntu-24.04-native | 69 | 68 | 15 | 87 |
| ubuntu-24.04-pext | 84 | 81 | 9 | 87 |
| ubuntu-24.04-portable | 72 | 68 | 14 | 87 |
| windows-2025-native | 81 | 80 | 11 | 87 |
| ALL | 452 | 436 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.988x | 1.053x | 27/36 | 0.791x |
| chess960_materialize | 0.965x | 1.368x | 12/12 | 1.194x |
| chess960_perft_d3 | 0.986x | 1.424x | 12/12 | 1.244x |
| corpus_count | 0.967x | 1.321x | 12/12 | 1.134x |
| corpus_materialize | 0.975x | 1.195x | 12/12 | 1.062x |
| corpus_uci_sorted | 0.967x | 1.522x | 12/12 | 1.331x |
| count | 0.985x | 1.349x | 35/36 | 0.968x |
| fen_parse | 0.982x | 1.170x | 35/36 | 0.936x |
| fen_write_native_ep | 0.974x | 1.737x | 36/36 | 1.234x |
| make_unmake_full | 0.979x | 1.228x | 35/36 | 0.985x |
| materialize | 0.973x | 1.231x | 33/36 | 0.946x |
| perft_bulk_d4 | 0.998x | 1.281x | 36/36 | 1.049x |
| perft_full_d3 | 0.971x | 1.174x | 30/36 | 0.843x |
| polyglot_key | 0.981x | 27.972x | 12/12 | 16.634x |
| polyglot_recompute | 1.009x | 1.039x | 9/12 | 0.896x |
| repetition_key | 0.972x | 19.484x | 12/12 | 14.263x |
| replay_checked_native | 0.964x | 1.000x | 3/12 | 0.880x |
| replay_trusted_native | 0.920x | 0.615x | 1/12 | 0.452x |
| san_parse_all | 1.003x | 2.014x | 36/36 | 1.543x |
| san_render_native | 0.988x | 2.559x | 36/36 | 1.670x |
| snapshot | 0.966x | 0.983x | 12/36 | 0.724x |
| startpos | 0.856x | 1.023x | 4/6 | 0.546x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.452x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.460x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.461x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.515x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.515x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.539x |
| windows-2025-native | startpos | start | shakmaty | 0.546x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.601x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.705x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.721x |
| macos-15-native | snapshot | endgame | ferrum | 0.724x |
| macos-15-native | snapshot | promotion | ferrum | 0.776x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.785x |
| macos-15-native | apply_children_full | promotion | cozy | 0.791x |
| macos-15-native | apply_children_full | middlegame | chess | 0.817x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.822x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.843x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.867x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.880x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.893x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.901x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.905x |
| macos-15-native | startpos | start | shakmaty | 0.906x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.906x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.911x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.917x |
| macos-15-native | snapshot | start | ferrum | 0.917x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.917x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.917x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.919x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.919x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.920x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.921x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.922x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.924x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.924x |
| ubuntu-24.04-native | apply_children_full | kiwipete | cozy | 0.929x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.931x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.934x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.936x |
| macos-15-native | fen_parse | promotion | shakmaty | 0.936x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.936x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.938x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.940x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.941x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.942x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.943x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.943x |
| macos-15-native | materialize | middlegame | ultra | 0.946x |
| ubuntu-24.04-portable | replay_checked_native | lichess300 | cozy | 0.947x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.948x |
| macos-15-native | materialize | start | ferrum | 0.949x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.949x |
| ubuntu-24.04-portable | apply_children_full | promotion | ferrum | 0.955x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.960x |
| windows-2025-native | snapshot | tactical | ferrum | 0.964x |
| ubuntu-24.04-portable | count | start | ferrum | 0.968x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.974x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.985x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.988x |
| macos-15-native | apply_children_full | tactical | chess | 0.988x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.989x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.990x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.990x |
| macos-15-native | materialize | tactical | ultra | 0.997x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.998x |
| macos-15-native | snapshot | middlegame | ferrum | 0.998x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 21875 | chess | 13459 | 0.615x |
| ubuntu-24.04-arm-native | 11408 | shakmaty | 8728 | 0.765x |
| ubuntu-24.04-native | 12038 | shakmaty | 9465 | 0.786x |
| ubuntu-24.04-pext | 7814 | cozy | 6606 | 0.845x |
| ubuntu-24.04-portable | 10235 | shakmaty | 7160 | 0.700x |
| windows-2025-native | 29900 | shakmaty | 23200 | 0.776x |

Cold wins: 0/6 (separate from native cases).
