| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 79 | 78 | 14 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 84 | 9 | 87 |
| ubuntu-24.04-pext | 86 | 86 | 11 | 87 |
| ubuntu-24.04-portable | 82 | 79 | 15 | 87 |
| windows-2025-native | 83 | 78 | 13 | 87 |
| ALL | 493 | 484 | 74 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.005x | 1.292x | 35/36 | 0.985x |
| chess960_materialize | 0.967x | 1.451x | 12/12 | 1.203x |
| chess960_perft_d3 | 1.000x | 1.530x | 12/12 | 1.369x |
| corpus_count | 1.138x | 1.399x | 10/12 | 0.903x |
| corpus_materialize | 1.031x | 1.194x | 12/12 | 1.098x |
| corpus_uci_sorted | 0.994x | 1.541x | 12/12 | 1.273x |
| count | 1.026x | 1.532x | 36/36 | 1.076x |
| fen_parse | 1.008x | 1.232x | 36/36 | 1.034x |
| fen_write_native_ep | 1.013x | 1.686x | 36/36 | 1.383x |
| make_unmake_full | 1.003x | 1.336x | 35/36 | 0.996x |
| materialize | 0.986x | 1.324x | 35/36 | 0.995x |
| perft_bulk_d4 | 1.031x | 1.345x | 35/36 | 0.994x |
| perft_full_d3 | 1.190x | 1.523x | 36/36 | 1.153x |
| polyglot_key | 0.985x | 32.994x | 12/12 | 16.179x |
| polyglot_recompute | 1.388x | 1.438x | 12/12 | 1.242x |
| repetition_key | 0.992x | 21.814x | 12/12 | 14.199x |
| replay_checked_native | 0.945x | 1.119x | 10/12 | 0.859x |
| replay_trusted_native | 0.984x | 0.788x | 4/12 | 0.564x |
| san_parse_all | 0.992x | 1.901x | 36/36 | 1.334x |
| san_render_native | 1.003x | 2.663x | 36/36 | 1.863x |
| snapshot | 1.012x | 1.098x | 24/36 | 0.736x |
| startpos | 0.999x | 1.188x | 5/6 | 0.866x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.564x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.575x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.602x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.670x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.706x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.717x |
| macos-15-native | snapshot | promotion | ferrum | 0.736x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.742x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.823x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.859x |
| macos-15-native | startpos | start | shakmaty | 0.866x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.903x |
| windows-2025-native | snapshot | start | chess | 0.914x |
| macos-15-native | snapshot | start | ferrum | 0.934x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.940x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.952x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.970x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.985x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.993x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.994x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.994x |
| macos-15-native | materialize | tactical | ultra | 0.995x |
| ubuntu-24.04-native | make_unmake_full | promotion | ultra | 0.996x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6125 | shakmaty | 9459 | 1.544x |
| ubuntu-24.04-arm-native | 6512 | chess | 8831 | 1.356x |
| ubuntu-24.04-native | 6991 | chess | 9404 | 1.345x |
| ubuntu-24.04-pext | 8226 | cozy | 10440 | 1.269x |
| ubuntu-24.04-portable | 6204 | shakmaty | 8756 | 1.411x |
| windows-2025-native | 20400 | shakmaty | 27600 | 1.353x |

Cold wins: 6/6 (separate from native cases).
