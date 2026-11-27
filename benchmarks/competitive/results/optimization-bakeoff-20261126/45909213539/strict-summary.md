| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 70 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 77 | 75 | 9 | 87 |
| ubuntu-24.04-pext | 78 | 73 | 11 | 87 |
| ubuntu-24.04-portable | 81 | 81 | 13 | 87 |
| windows-2025-native | 78 | 75 | 12 | 87 |
| ALL | 465 | 453 | 69 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.008x | 1.293x | 36/36 | 1.028x |
| chess960_materialize | 0.992x | 1.511x | 12/12 | 1.280x |
| chess960_perft_d3 | 0.987x | 1.500x | 12/12 | 1.227x |
| corpus_count | 1.014x | 1.181x | 10/12 | 0.931x |
| corpus_materialize | 1.015x | 1.164x | 12/12 | 1.007x |
| corpus_uci_sorted | 1.006x | 1.513x | 12/12 | 1.302x |
| count | 1.026x | 1.500x | 36/36 | 1.087x |
| fen_parse | 1.017x | 1.201x | 35/36 | 0.907x |
| fen_write_native_ep | 1.011x | 1.774x | 36/36 | 1.425x |
| make_unmake_full | 1.013x | 1.289x | 35/36 | 0.996x |
| materialize | 1.007x | 1.361x | 33/36 | 0.787x |
| perft_bulk_d4 | 0.995x | 1.233x | 31/36 | 0.871x |
| perft_full_d3 | 0.987x | 1.332x | 36/36 | 1.100x |
| polyglot_key | 1.021x | 33.384x | 12/12 | 16.346x |
| polyglot_recompute | 0.950x | 0.988x | 6/12 | 0.851x |
| repetition_key | 0.982x | 21.223x | 12/12 | 14.590x |
| replay_checked_native | 0.987x | 1.170x | 12/12 | 1.100x |
| replay_trusted_native | 0.983x | 0.802x | 5/12 | 0.490x |
| san_parse_all | 1.001x | 1.901x | 36/36 | 1.476x |
| san_render_native | 0.981x | 2.577x | 36/36 | 1.512x |
| snapshot | 0.972x | 0.981x | 5/36 | 0.711x |
| startpos | 0.982x | 1.206x | 5/6 | 0.998x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.490x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.579x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.642x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.669x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.672x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.679x |
| macos-15-native | snapshot | endgame | cozy | 0.711x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.725x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.765x |
| macos-15-native | materialize | middlegame | ultra | 0.787x |
| macos-15-native | snapshot | tactical | ferrum | 0.794x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.851x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.871x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.900x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.901x |
| macos-15-native | materialize | tactical | ultra | 0.904x |
| macos-15-native | fen_parse | kiwipete | ultra | 0.907x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.909x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.920x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.928x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.931x |
| macos-15-native | snapshot | promotion | cozy | 0.936x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.949x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.953x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.953x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.954x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.955x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.962x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.962x |
| macos-15-native | materialize | promotion | ultra | 0.962x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.963x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.968x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.968x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.973x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.973x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.974x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.975x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.980x |
| macos-15-native | perft_bulk_d4 | middlegame | cozy | 0.981x |
| windows-2025-native | snapshot | start | cozy | 0.986x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.987x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.987x |
| windows-2025-native | snapshot | tactical | ferrum | 0.990x |
| windows-2025-native | snapshot | promotion | ferrum | 0.992x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.992x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.995x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.996x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.996x |
| ubuntu-24.04-pext | snapshot | kiwipete | chess | 0.996x |
| ubuntu-24.04-pext | snapshot | start | chess | 0.996x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.997x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.997x |
| ubuntu-24.04-pext | snapshot | middlegame | chess | 0.997x |
| ubuntu-24.04-native | snapshot | kiwipete | cozy | 0.998x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-native | snapshot | middlegame | chess | 1.000x |
| ubuntu-24.04-pext | snapshot | endgame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5833 | shakmaty | 8833 | 1.514x |
| ubuntu-24.04-arm-native | 5488 | shakmaty | 7583 | 1.382x |
| ubuntu-24.04-native | 6123 | shakmaty | 7944 | 1.297x |
| ubuntu-24.04-pext | 5254 | shakmaty | 7486 | 1.425x |
| ubuntu-24.04-portable | 5939 | shakmaty | 8212 | 1.383x |
| windows-2025-native | 15500 | shakmaty | 20100 | 1.297x |

Cold wins: 6/6 (separate from native cases).
