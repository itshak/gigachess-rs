| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 66 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 71 | 11 | 87 |
| ubuntu-24.04-native | 81 | 75 | 9 | 87 |
| ubuntu-24.04-pext | 80 | 79 | 9 | 87 |
| ubuntu-24.04-portable | 81 | 74 | 12 | 87 |
| windows-2025-native | 68 | 66 | 11 | 87 |
| ALL | 458 | 431 | 65 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.993x | 1.057x | 29/36 | 0.844x |
| chess960_materialize | 0.972x | 1.342x | 12/12 | 1.008x |
| chess960_perft_d3 | 0.974x | 1.372x | 12/12 | 1.103x |
| corpus_count | 0.740x | 1.045x | 7/12 | 0.895x |
| corpus_materialize | 0.958x | 1.083x | 10/12 | 0.990x |
| corpus_uci_sorted | 0.976x | 1.499x | 12/12 | 1.285x |
| count | 1.011x | 1.348x | 35/36 | 0.957x |
| fen_parse | 0.987x | 1.179x | 35/36 | 0.995x |
| fen_write_native_ep | 0.989x | 1.740x | 36/36 | 1.393x |
| make_unmake_full | 0.994x | 1.226x | 36/36 | 1.005x |
| materialize | 0.991x | 1.287x | 31/36 | 0.849x |
| perft_bulk_d4 | 0.887x | 1.116x | 28/36 | 0.798x |
| perft_full_d3 | 0.974x | 1.304x | 32/36 | 0.914x |
| polyglot_key | 1.014x | 33.319x | 12/12 | 16.057x |
| polyglot_recompute | 0.984x | 1.048x | 10/12 | 0.899x |
| repetition_key | 0.989x | 19.418x | 12/12 | 13.904x |
| replay_checked_native | 0.976x | 1.027x | 8/12 | 0.780x |
| replay_trusted_native | 1.014x | 0.665x | 0/12 | 0.488x |
| san_parse_all | 1.000x | 1.904x | 36/36 | 1.522x |
| san_render_native | 1.001x | 2.466x | 36/36 | 1.893x |
| snapshot | 0.936x | 0.995x | 23/36 | 0.694x |
| startpos | 1.000x | 1.315x | 6/6 | 1.171x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.488x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.495x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.509x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.578x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.600x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.607x |
| macos-15-native | snapshot | promotion | ferrum | 0.694x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.697x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.759x |
| macos-15-native | snapshot | start | ferrum | 0.777x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.780x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.789x |
| windows-2025-native | snapshot | promotion | ferrum | 0.794x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.798x |
| windows-2025-native | snapshot | start | ferrum | 0.802x |
| windows-2025-native | snapshot | endgame | ferrum | 0.805x |
| windows-2025-native | snapshot | tactical | ferrum | 0.808x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.816x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.829x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.835x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.837x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.844x |
| macos-15-native | materialize | start | ultra | 0.849x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.855x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.866x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.874x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.875x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.886x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.895x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.899x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.902x |
| macos-15-native | snapshot | middlegame | ferrum | 0.913x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.914x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.914x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.914x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.915x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.916x |
| macos-15-native | materialize | middlegame | ultra | 0.925x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.926x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.931x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.932x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.936x |
| windows-2025-native | materialize | kiwipete | ultra | 0.950x |
| macos-15-native | materialize | kiwipete | ultra | 0.951x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.951x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.951x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.954x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.956x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.957x |
| windows-2025-native | materialize | middlegame | ultra | 0.962x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.965x |
| macos-15-native | snapshot | tactical | ferrum | 0.966x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.970x |
| macos-15-native | apply_children_full | tactical | chess | 0.980x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.981x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.982x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.982x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.985x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.987x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.990x |
| macos-15-native | corpus_materialize | lichess300 | ferrum | 0.991x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.993x |
| ubuntu-24.04-pext | fen_parse | promotion | ferrum | 0.995x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4292 | shakmaty | 7459 | 1.738x |
| ubuntu-24.04-arm-native | 5512 | shakmaty | 8295 | 1.505x |
| ubuntu-24.04-native | 4939 | shakmaty | 8238 | 1.668x |
| ubuntu-24.04-pext | 5384 | cozy | 7408 | 1.376x |
| ubuntu-24.04-portable | 5676 | shakmaty | 7451 | 1.313x |
| windows-2025-native | 13100 | shakmaty | 16200 | 1.237x |

Cold wins: 6/6 (separate from native cases).
