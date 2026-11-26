| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 67 | 62 | 12 | 87 |
| ubuntu-24.04-arm-native | 74 | 69 | 10 | 87 |
| ubuntu-24.04-native | 83 | 82 | 8 | 87 |
| ubuntu-24.04-pext | 85 | 82 | 8 | 87 |
| ubuntu-24.04-portable | 82 | 75 | 9 | 87 |
| windows-2025-native | 74 | 71 | 11 | 87 |
| ALL | 465 | 441 | 58 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.940x | 1.032x | 27/36 | 0.749x |
| chess960_materialize | 1.028x | 1.405x | 12/12 | 1.201x |
| chess960_perft_d3 | 1.029x | 1.343x | 12/12 | 1.238x |
| corpus_count | 1.005x | 1.341x | 12/12 | 1.202x |
| corpus_materialize | 0.984x | 1.120x | 11/12 | 0.951x |
| corpus_uci_sorted | 0.990x | 1.446x | 12/12 | 1.177x |
| count | 0.981x | 1.308x | 34/36 | 0.813x |
| fen_parse | 1.027x | 1.197x | 35/36 | 0.982x |
| fen_write_native_ep | 0.980x | 1.165x | 28/36 | 0.911x |
| make_unmake_full | 0.992x | 1.237x | 35/36 | 0.952x |
| materialize | 1.021x | 1.343x | 35/36 | 0.863x |
| perft_bulk_d4 | 1.004x | 1.264x | 36/36 | 1.027x |
| perft_full_d3 | 0.993x | 1.323x | 34/36 | 0.923x |
| polyglot_key | 1.041x | 35.058x | 12/12 | 16.383x |
| polyglot_recompute | 0.989x | 1.023x | 10/12 | 0.888x |
| repetition_key | 0.985x | 20.371x | 12/12 | 14.328x |
| replay_checked_native | 1.017x | 1.063x | 8/12 | 0.922x |
| replay_trusted_native | 0.991x | 0.638x | 0/12 | 0.462x |
| san_parse_all | 0.985x | 1.870x | 36/36 | 1.265x |
| san_render_native | 0.988x | 2.524x | 36/36 | 1.880x |
| snapshot | 1.146x | 1.002x | 22/36 | 0.574x |
| startpos | 1.296x | 1.195x | 6/6 | 1.122x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.462x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.489x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.522x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.550x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.555x |
| macos-15-native | snapshot | promotion | ferrum | 0.574x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.601x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.605x |
| macos-15-native | snapshot | start | chess | 0.647x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.688x |
| macos-15-native | snapshot | middlegame | chess | 0.728x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.738x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.749x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.760x |
| macos-15-native | apply_children_full | tactical | chess | 0.787x |
| macos-15-native | apply_children_full | promotion | ferrum | 0.792x |
| macos-15-native | count | middlegame | cozy | 0.813x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.828x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.831x |
| macos-15-native | snapshot | endgame | chess | 0.840x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.842x |
| ubuntu-24.04-arm-native | snapshot | middlegame | chess | 0.849x |
| macos-15-native | materialize | tactical | ultra | 0.863x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.863x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.871x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.875x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.876x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.879x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.888x |
| windows-2025-native | fen_write_native_ep | kiwipete | ultra | 0.911x |
| windows-2025-native | fen_write_native_ep | middlegame | ultra | 0.913x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.922x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.923x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.923x |
| windows-2025-native | fen_write_native_ep | promotion | ultra | 0.937x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.940x |
| macos-15-native | corpus_materialize | seeded | ferrum | 0.951x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.952x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.952x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.963x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.965x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.968x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.970x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.970x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.976x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.978x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.978x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.982x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.983x |
| macos-15-native | fen_write_native_ep | kiwipete | ultra | 0.987x |
| macos-15-native | fen_write_native_ep | start | ultra | 0.991x |
| macos-15-native | snapshot | tactical | ferrum | 0.993x |
| windows-2025-native | fen_write_native_ep | start | ultra | 0.994x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.995x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.997x |
| windows-2025-native | fen_write_native_ep | endgame | ultra | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 13833 | shakmaty | 10875 | 0.786x |
| ubuntu-24.04-arm-native | 12273 | shakmaty | 11105 | 0.905x |
| ubuntu-24.04-native | 8701 | shakmaty | 6836 | 0.786x |
| ubuntu-24.04-pext | 7995 | shakmaty | 7425 | 0.929x |
| ubuntu-24.04-portable | 8603 | shakmaty | 6649 | 0.773x |
| windows-2025-native | 30500 | shakmaty | 24700 | 0.810x |

Cold wins: 0/6 (separate from native cases).
