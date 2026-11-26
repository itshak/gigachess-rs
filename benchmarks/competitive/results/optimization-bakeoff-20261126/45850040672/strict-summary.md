| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 73 | 15 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 9 | 87 |
| ubuntu-24.04-pext | 85 | 85 | 17 | 87 |
| ubuntu-24.04-portable | 80 | 76 | 11 | 87 |
| windows-2025-native | 83 | 81 | 11 | 87 |
| ALL | 484 | 477 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.035x | 1.250x | 34/36 | 0.749x |
| chess960_materialize | 1.006x | 1.488x | 12/12 | 1.242x |
| chess960_perft_d3 | 1.011x | 1.479x | 12/12 | 1.226x |
| corpus_count | 1.027x | 1.198x | 9/12 | 0.989x |
| corpus_materialize | 1.000x | 1.149x | 12/12 | 1.042x |
| corpus_uci_sorted | 0.984x | 1.460x | 11/12 | 0.835x |
| count | 1.009x | 1.492x | 36/36 | 1.020x |
| fen_parse | 1.013x | 1.193x | 36/36 | 1.053x |
| fen_write_native_ep | 1.016x | 1.769x | 36/36 | 1.213x |
| make_unmake_full | 1.019x | 1.269x | 36/36 | 1.024x |
| materialize | 1.006x | 1.356x | 34/36 | 0.919x |
| perft_bulk_d4 | 1.006x | 1.250x | 32/36 | 0.872x |
| perft_full_d3 | 1.008x | 1.327x | 35/36 | 0.924x |
| polyglot_key | 1.009x | 32.957x | 12/12 | 16.041x |
| polyglot_recompute | 1.024x | 1.034x | 8/12 | 0.896x |
| repetition_key | 1.036x | 21.890x | 12/12 | 14.259x |
| replay_checked_native | 1.114x | 1.168x | 12/12 | 1.058x |
| replay_trusted_native | 1.053x | 0.845x | 5/12 | 0.570x |
| san_parse_all | 1.033x | 1.930x | 36/36 | 1.477x |
| san_render_native | 1.053x | 2.641x | 36/36 | 1.680x |
| snapshot | 1.019x | 1.076x | 22/36 | 0.639x |
| startpos | 1.099x | 1.230x | 6/6 | 1.008x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.570x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.581x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.639x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.648x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.682x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.686x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.697x |
| macos-15-native | snapshot | endgame | chess | 0.746x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.749x |
| macos-15-native | snapshot | tactical | ferrum | 0.759x |
| macos-15-native | corpus_uci_sorted | seeded | ferrum | 0.835x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.872x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.883x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.893x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| macos-15-native | materialize | tactical | ultra | 0.919x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.924x |
| macos-15-native | materialize | middlegame | ultra | 0.927x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.929x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.931x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.931x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.940x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.942x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.946x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.954x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.986x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.988x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.989x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.992x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.995x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.995x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6917 | shakmaty | 9875 | 1.428x |
| ubuntu-24.04-arm-native | 5896 | shakmaty | 9280 | 1.574x |
| ubuntu-24.04-native | 5658 | shakmaty | 7019 | 1.241x |
| ubuntu-24.04-pext | 6509 | shakmaty | 8612 | 1.323x |
| ubuntu-24.04-portable | 5982 | shakmaty | 6983 | 1.167x |
| windows-2025-native | 17700 | shakmaty | 25100 | 1.418x |

Cold wins: 6/6 (separate from native cases).
