| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 71 | 16 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 83 | 83 | 10 | 87 |
| ubuntu-24.04-pext | 86 | 84 | 15 | 87 |
| ubuntu-24.04-portable | 82 | 79 | 14 | 87 |
| windows-2025-native | 85 | 82 | 13 | 87 |
| ALL | 488 | 478 | 80 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.021x | 1.297x | 35/36 | 0.960x |
| chess960_materialize | 0.984x | 1.454x | 12/12 | 1.270x |
| chess960_perft_d3 | 1.018x | 1.522x | 12/12 | 1.365x |
| corpus_count | 1.064x | 1.374x | 11/12 | 0.902x |
| corpus_materialize | 0.997x | 1.184x | 12/12 | 1.098x |
| corpus_uci_sorted | 0.985x | 1.580x | 12/12 | 1.456x |
| count | 1.019x | 1.481x | 35/36 | 0.623x |
| fen_parse | 0.978x | 1.203x | 35/36 | 0.957x |
| fen_write_native_ep | 0.994x | 1.690x | 36/36 | 1.156x |
| make_unmake_full | 1.014x | 1.346x | 36/36 | 1.057x |
| materialize | 0.991x | 1.272x | 32/36 | 0.804x |
| perft_bulk_d4 | 1.052x | 1.382x | 35/36 | 0.995x |
| perft_full_d3 | 1.254x | 1.556x | 36/36 | 1.161x |
| polyglot_key | 1.007x | 31.492x | 12/12 | 16.597x |
| polyglot_recompute | 0.974x | 1.001x | 8/12 | 0.816x |
| repetition_key | 0.959x | 20.829x | 12/12 | 14.390x |
| replay_checked_native | 0.976x | 1.180x | 12/12 | 1.014x |
| replay_trusted_native | 0.958x | 0.813x | 4/12 | 0.528x |
| san_parse_all | 0.982x | 1.868x | 36/36 | 1.392x |
| san_render_native | 0.993x | 2.525x | 36/36 | 1.817x |
| snapshot | 1.020x | 1.099x | 23/36 | 0.658x |
| startpos | 1.012x | 1.215x | 6/6 | 1.002x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.528x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.567x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.589x |
| macos-15-native | count | promotion | cozy | 0.623x |
| macos-15-native | snapshot | endgame | ferrum | 0.658x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.704x |
| macos-15-native | snapshot | start | ferrum | 0.707x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.724x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.793x |
| macos-15-native | materialize | middlegame | ultra | 0.804x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.816x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.818x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.872x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.873x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.875x |
| macos-15-native | materialize | tactical | ultra | 0.894x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.902x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.902x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | cozy | 0.933x |
| ubuntu-24.04-arm-native | snapshot | start | chess | 0.939x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.941x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.947x |
| macos-15-native | snapshot | tactical | ferrum | 0.951x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.957x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.960x |
| macos-15-native | materialize | kiwipete | ultra | 0.962x |
| macos-15-native | snapshot | promotion | ferrum | 0.963x |
| macos-15-native | materialize | endgame | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.989x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.994x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.995x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6042 | shakmaty | 9875 | 1.634x |
| ubuntu-24.04-arm-native | 6712 | shakmaty | 8360 | 1.246x |
| ubuntu-24.04-native | 6720 | shakmaty | 8553 | 1.273x |
| ubuntu-24.04-pext | 7293 | shakmaty | 13115 | 1.798x |
| ubuntu-24.04-portable | 4973 | shakmaty | 6654 | 1.338x |
| windows-2025-native | 18900 | shakmaty | 24300 | 1.286x |

Cold wins: 6/6 (separate from native cases).
