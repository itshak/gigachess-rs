| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 66 | 12 | 87 |
| ubuntu-24.04-arm-native | 71 | 69 | 11 | 87 |
| ubuntu-24.04-native | 78 | 74 | 12 | 87 |
| ubuntu-24.04-pext | 81 | 76 | 10 | 87 |
| ubuntu-24.04-portable | 83 | 76 | 12 | 87 |
| windows-2025-native | 75 | 72 | 11 | 87 |
| ALL | 459 | 433 | 68 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.998x | 1.036x | 24/36 | 0.846x |
| chess960_materialize | 1.003x | 1.336x | 12/12 | 1.221x |
| chess960_perft_d3 | 1.022x | 1.400x | 12/12 | 1.269x |
| corpus_count | 1.013x | 1.358x | 12/12 | 1.141x |
| corpus_materialize | 1.025x | 1.157x | 11/12 | 0.891x |
| corpus_uci_sorted | 1.020x | 1.531x | 12/12 | 1.018x |
| count | 0.992x | 1.335x | 36/36 | 1.051x |
| fen_parse | 0.998x | 1.190x | 35/36 | 0.991x |
| fen_write_native_ep | 0.994x | 1.775x | 36/36 | 1.162x |
| make_unmake_full | 1.033x | 1.235x | 33/36 | 0.871x |
| materialize | 0.987x | 1.297x | 34/36 | 0.964x |
| perft_bulk_d4 | 1.024x | 1.285x | 36/36 | 1.026x |
| perft_full_d3 | 0.995x | 1.336x | 31/36 | 0.865x |
| polyglot_key | 0.974x | 33.821x | 12/12 | 16.413x |
| polyglot_recompute | 1.016x | 1.047x | 9/12 | 0.932x |
| repetition_key | 0.991x | 22.759x | 12/12 | 13.992x |
| replay_checked_native | 1.021x | 1.044x | 11/12 | 0.975x |
| replay_trusted_native | 1.008x | 0.631x | 0/12 | 0.462x |
| san_parse_all | 0.986x | 1.811x | 36/36 | 1.306x |
| san_render_native | 1.018x | 2.609x | 36/36 | 1.751x |
| snapshot | 1.127x | 0.973x | 15/36 | 0.804x |
| startpos | 1.046x | 1.143x | 4/6 | 0.831x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.462x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.486x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.541x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.563x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.611x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.615x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.633x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.720x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.735x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.738x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.783x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.799x |
| macos-15-native | snapshot | tactical | cozy | 0.804x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.831x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.846x |
| macos-15-native | apply_children_full | tactical | chess | 0.850x |
| macos-15-native | snapshot | middlegame | ferrum | 0.850x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.859x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.859x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.860x |
| ubuntu-24.04-native | snapshot | promotion | ferrum | 0.861x |
| ubuntu-24.04-native | snapshot | tactical | ferrum | 0.861x |
| windows-2025-native | snapshot | start | ferrum | 0.862x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.865x |
| windows-2025-native | make_unmake_full | promotion | ultra | 0.871x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.874x |
| windows-2025-native | snapshot | promotion | chess | 0.878x |
| macos-15-native | corpus_materialize | lichess300 | ultra | 0.891x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.893x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.893x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.893x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.893x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.895x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.895x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.900x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.902x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.915x |
| macos-15-native | snapshot | start | chess | 0.920x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.932x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.933x |
| macos-15-native | apply_children_full | start | shakmaty | 0.936x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.936x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.937x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.937x |
| windows-2025-native | apply_children_full | start | cozy | 0.941x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.955x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.958x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.961x |
| macos-15-native | materialize | middlegame | ultra | 0.964x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.967x |
| windows-2025-native | snapshot | tactical | ferrum | 0.970x |
| windows-2025-native | snapshot | kiwipete | ferrum | 0.973x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.975x |
| windows-2025-native | snapshot | middlegame | cozy | 0.982x |
| macos-15-native | materialize | endgame | ultra | 0.984x |
| windows-2025-native | startpos | start | shakmaty | 0.989x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.991x |
| macos-15-native | make_unmake_full | kiwipete | ultra | 0.995x |
| windows-2025-native | make_unmake_full | tactical | ultra | 0.996x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.998x |
| ubuntu-24.04-pext | apply_children_full | kiwipete | chess | 0.998x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.999x |
| ubuntu-24.04-portable | snapshot | tactical | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 20500 | shakmaty | 7750 | 0.378x |
| ubuntu-24.04-arm-native | 9632 | shakmaty | 7431 | 0.771x |
| ubuntu-24.04-native | 15599 | shakmaty | 10250 | 0.657x |
| ubuntu-24.04-pext | 10319 | cozy | 9160 | 0.888x |
| ubuntu-24.04-portable | 12239 | shakmaty | 8878 | 0.725x |
| windows-2025-native | 29600 | shakmaty | 19600 | 0.662x |

Cold wins: 0/6 (separate from native cases).
