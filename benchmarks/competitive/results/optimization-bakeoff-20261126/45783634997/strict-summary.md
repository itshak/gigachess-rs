| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| ubuntu-24.04-arm-native | 78 | 74 | 11 | 87 |
| ubuntu-24.04-native | 75 | 72 | 9 | 87 |
| ubuntu-24.04-pext | 73 | 71 | 9 | 87 |
| ubuntu-24.04-portable | 77 | 77 | 10 | 87 |
| ALL | 303 | 294 | 39 | 348 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.982x | 1.032x | 16/24 | 0.842x |
| chess960_materialize | 1.016x | 1.381x | 8/8 | 1.299x |
| chess960_perft_d3 | 0.997x | 1.391x | 8/8 | 1.274x |
| corpus_count | 0.995x | 1.327x | 8/8 | 1.002x |
| corpus_materialize | 0.988x | 1.142x | 8/8 | 1.042x |
| corpus_uci_sorted | 1.004x | 1.558x | 8/8 | 1.449x |
| count | 0.999x | 1.322x | 22/24 | 0.972x |
| fen_parse | 0.995x | 1.165x | 24/24 | 1.060x |
| fen_write_native_ep | 1.489x | 1.674x | 24/24 | 1.328x |
| make_unmake_full | 1.009x | 1.269x | 24/24 | 1.048x |
| materialize | 0.988x | 1.370x | 24/24 | 1.018x |
| perft_bulk_d4 | 1.004x | 1.246x | 24/24 | 1.044x |
| perft_full_d3 | 1.037x | 1.268x | 22/24 | 0.932x |
| polyglot_key | 1.028x | 36.001x | 8/8 | 16.134x |
| polyglot_recompute | 0.983x | 1.027x | 5/8 | 0.934x |
| repetition_key | 0.964x | 22.759x | 8/8 | 14.107x |
| replay_checked_native | 0.997x | 1.006x | 5/8 | 0.807x |
| replay_trusted_native | 1.006x | 0.636x | 0/8 | 0.487x |
| san_parse_all | 1.010x | 1.841x | 24/24 | 1.410x |
| san_render_native | 1.013x | 2.276x | 24/24 | 1.891x |
| snapshot | 0.929x | 0.984x | 5/24 | 0.936x |
| startpos | 0.907x | 1.193x | 4/4 | 1.081x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.487x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.489x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.545x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.633x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.643x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.747x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.807x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.812x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.838x |
| ubuntu-24.04-pext | apply_children_full | kiwipete | cozy | 0.842x |
| ubuntu-24.04-pext | apply_children_full | tactical | cozy | 0.851x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.854x |
| ubuntu-24.04-pext | apply_children_full | start | cozy | 0.868x |
| ubuntu-24.04-pext | apply_children_full | middlegame | cozy | 0.881x |
| ubuntu-24.04-pext | apply_children_full | endgame | chess | 0.903x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.932x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.934x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.936x |
| ubuntu-24.04-arm-native | snapshot | endgame | chess | 0.939x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.943x |
| ubuntu-24.04-native | snapshot | start | ferrum | 0.954x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.954x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.956x |
| ubuntu-24.04-native | snapshot | kiwipete | ferrum | 0.956x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-native | snapshot | middlegame | ferrum | 0.963x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.967x |
| ubuntu-24.04-native | snapshot | tactical | cozy | 0.971x |
| ubuntu-24.04-portable | count | start | ferrum | 0.972x |
| ubuntu-24.04-native | snapshot | promotion | cozy | 0.975x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.976x |
| ubuntu-24.04-portable | snapshot | start | cozy | 0.986x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.990x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 0.991x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.991x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.993x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.995x |
| ubuntu-24.04-portable | snapshot | kiwipete | cozy | 0.995x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | middlegame | chess | 0.997x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | cozy | 0.997x |
| ubuntu-24.04-pext | snapshot | kiwipete | cozy | 0.997x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.998x |
| ubuntu-24.04-pext | snapshot | promotion | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| ubuntu-24.04-arm-native | 10625 | shakmaty | 7048 | 0.663x |
| ubuntu-24.04-native | 13650 | shakmaty | 8633 | 0.632x |
| ubuntu-24.04-pext | 9849 | cozy | 8403 | 0.853x |
| ubuntu-24.04-portable | 9386 | shakmaty | 7217 | 0.769x |

Cold wins: 0/4 (separate from native cases).
