| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 70 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 79 | 77 | 11 | 87 |
| ubuntu-24.04-pext | 83 | 82 | 12 | 87 |
| ubuntu-24.04-portable | 86 | 82 | 12 | 87 |
| windows-2025-native | 86 | 85 | 12 | 87 |
| ALL | 486 | 475 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.986x | 1.267x | 34/36 | 0.878x |
| chess960_materialize | 0.941x | 1.403x | 12/12 | 1.171x |
| chess960_perft_d3 | 1.023x | 1.546x | 12/12 | 1.358x |
| corpus_count | 1.144x | 1.373x | 12/12 | 1.074x |
| corpus_materialize | 1.003x | 1.157x | 12/12 | 1.040x |
| corpus_uci_sorted | 1.020x | 1.534x | 12/12 | 1.233x |
| count | 1.035x | 1.574x | 36/36 | 1.248x |
| fen_parse | 0.944x | 1.169x | 35/36 | 0.994x |
| fen_write_native_ep | 0.998x | 1.730x | 36/36 | 1.428x |
| make_unmake_full | 0.982x | 1.313x | 36/36 | 1.021x |
| materialize | 0.968x | 1.306x | 33/36 | 0.856x |
| perft_bulk_d4 | 1.045x | 1.345x | 34/36 | 0.955x |
| perft_full_d3 | 1.176x | 1.557x | 35/36 | 0.947x |
| polyglot_key | 1.008x | 29.990x | 12/12 | 16.783x |
| polyglot_recompute | 1.005x | 1.022x | 8/12 | 0.893x |
| repetition_key | 0.993x | 19.950x | 12/12 | 14.542x |
| replay_checked_native | 1.020x | 1.176x | 12/12 | 1.050x |
| replay_trusted_native | 1.039x | 0.867x | 6/12 | 0.540x |
| san_parse_all | 0.997x | 1.845x | 36/36 | 1.440x |
| san_render_native | 0.988x | 2.532x | 36/36 | 1.683x |
| snapshot | 0.973x | 1.061x | 20/36 | 0.758x |
| startpos | 1.027x | 1.150x | 5/6 | 0.994x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.540x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.588x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.656x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.689x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.709x |
| macos-15-native | snapshot | endgame | ferrum | 0.758x |
| macos-15-native | snapshot | promotion | ferrum | 0.775x |
| macos-15-native | snapshot | start | chess | 0.787x |
| macos-15-native | snapshot | tactical | ferrum | 0.841x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.851x |
| macos-15-native | materialize | endgame | ultra | 0.856x |
| macos-15-native | apply_children_full | kiwipete | cozy | 0.878x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.893x |
| macos-15-native | materialize | middlegame | ultra | 0.900x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.912x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.938x |
| ubuntu-24.04-arm-native | snapshot | tactical | chess | 0.945x |
| macos-15-native | apply_children_full | promotion | cozy | 0.946x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.946x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.947x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.949x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.951x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.955x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.955x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.959x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.970x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.972x |
| ubuntu-24.04-native | snapshot | start | cozy | 0.979x |
| ubuntu-24.04-native | snapshot | endgame | chess | 0.981x |
| ubuntu-24.04-native | snapshot | tactical | chess | 0.993x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.994x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.994x |
| ubuntu-24.04-native | snapshot | promotion | cozy | 0.995x |
| macos-15-native | materialize | tactical | ultra | 0.997x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6958 | shakmaty | 9333 | 1.341x |
| ubuntu-24.04-arm-native | 5560 | shakmaty | 8376 | 1.506x |
| ubuntu-24.04-native | 8220 | shakmaty | 7958 | 0.968x |
| ubuntu-24.04-pext | 5052 | shakmaty | 8471 | 1.677x |
| ubuntu-24.04-portable | 7912 | shakmaty | 12108 | 1.530x |
| windows-2025-native | 15600 | shakmaty | 22200 | 1.423x |

Cold wins: 5/6 (separate from native cases).
