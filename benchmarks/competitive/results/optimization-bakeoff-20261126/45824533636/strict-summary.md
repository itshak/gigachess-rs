| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 69 | 14 | 87 |
| ubuntu-24.04-arm-native | 78 | 78 | 11 | 87 |
| ubuntu-24.04-native | 80 | 77 | 14 | 87 |
| ubuntu-24.04-pext | 85 | 83 | 13 | 87 |
| ubuntu-24.04-portable | 75 | 72 | 9 | 87 |
| windows-2025-native | 82 | 75 | 12 | 87 |
| ALL | 473 | 454 | 73 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.971x | 1.219x | 33/36 | 0.862x |
| chess960_materialize | 0.972x | 1.345x | 12/12 | 1.043x |
| chess960_perft_d3 | 0.960x | 1.342x | 12/12 | 1.063x |
| corpus_count | 0.950x | 1.138x | 9/12 | 0.817x |
| corpus_materialize | 0.980x | 1.109x | 11/12 | 0.885x |
| corpus_uci_sorted | 0.973x | 1.558x | 12/12 | 1.413x |
| count | 0.980x | 1.276x | 34/36 | 0.956x |
| fen_parse | 1.035x | 1.236x | 36/36 | 1.056x |
| fen_write_native_ep | 0.989x | 1.683x | 36/36 | 1.376x |
| make_unmake_full | 0.989x | 1.308x | 36/36 | 1.027x |
| materialize | 1.002x | 1.178x | 33/36 | 0.890x |
| perft_bulk_d4 | 0.959x | 1.171x | 32/36 | 0.834x |
| perft_full_d3 | 0.995x | 1.193x | 31/36 | 0.838x |
| polyglot_key | 1.030x | 29.300x | 12/12 | 16.695x |
| polyglot_recompute | 0.973x | 1.022x | 8/12 | 0.891x |
| repetition_key | 0.895x | 17.035x | 12/12 | 11.628x |
| replay_checked_native | 1.011x | 1.074x | 10/12 | 0.970x |
| replay_trusted_native | 0.990x | 0.771x | 4/12 | 0.529x |
| san_parse_all | 0.993x | 1.913x | 36/36 | 1.168x |
| san_render_native | 0.968x | 2.472x | 36/36 | 1.583x |
| snapshot | 1.007x | 1.079x | 22/36 | 0.940x |
| startpos | 0.995x | 1.263x | 6/6 | 1.199x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.529x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.529x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.569x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.670x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.681x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.699x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.714x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.817x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.830x |
| macos-15-native | perft_bulk_d4 | kiwipete | ultra | 0.834x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.838x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.842x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.846x |
| macos-15-native | apply_children_full | tactical | chess | 0.862x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.885x |
| macos-15-native | materialize | tactical | ultra | 0.890x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.891x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.893x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.907x |
| macos-15-native | materialize | kiwipete | ultra | 0.916x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.929x |
| macos-15-native | materialize | middlegame | ultra | 0.931x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.932x |
| macos-15-native | snapshot | tactical | cozy | 0.940x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.940x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.942x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.944x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.946x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.950x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.956x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.961x |
| windows-2025-native | snapshot | tactical | chess | 0.962x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.963x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.968x |
| macos-15-native | count | promotion | cozy | 0.968x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.970x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.974x |
| ubuntu-24.04-portable | apply_children_full | tactical | ferrum | 0.983x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.984x |
| windows-2025-native | snapshot | start | cozy | 0.985x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.990x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.994x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.995x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.996x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.999x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6167 | shakmaty | 9792 | 1.588x |
| ubuntu-24.04-arm-native | 5304 | shakmaty | 7768 | 1.465x |
| ubuntu-24.04-native | 7742 | shakmaty | 11277 | 1.457x |
| ubuntu-24.04-pext | 7824 | shakmaty | 12032 | 1.538x |
| ubuntu-24.04-portable | 6399 | shakmaty | 9073 | 1.418x |
| windows-2025-native | 14800 | shakmaty | 21400 | 1.446x |

Cold wins: 6/6 (separate from native cases).
