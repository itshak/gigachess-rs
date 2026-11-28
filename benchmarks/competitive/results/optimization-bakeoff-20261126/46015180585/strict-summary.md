| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 74 | 12 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 11 | 87 |
| ubuntu-24.04-native | 85 | 85 | 11 | 87 |
| ubuntu-24.04-pext | 85 | 85 | 18 | 87 |
| ubuntu-24.04-portable | 83 | 79 | 12 | 87 |
| windows-2025-native | 84 | 84 | 11 | 87 |
| ALL | 490 | 486 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.002x | 1.260x | 34/36 | 0.868x |
| chess960_materialize | 1.006x | 1.489x | 12/12 | 1.264x |
| chess960_perft_d3 | 0.991x | 1.493x | 12/12 | 1.339x |
| corpus_count | 1.115x | 1.287x | 10/12 | 0.919x |
| corpus_materialize | 1.004x | 1.138x | 12/12 | 1.042x |
| corpus_uci_sorted | 0.990x | 1.498x | 12/12 | 1.274x |
| count | 1.050x | 1.541x | 36/36 | 1.076x |
| fen_parse | 1.006x | 1.213x | 36/36 | 1.053x |
| fen_write_native_ep | 0.993x | 1.777x | 36/36 | 1.351x |
| make_unmake_full | 0.982x | 1.294x | 35/36 | 0.973x |
| materialize | 1.031x | 1.391x | 34/36 | 0.829x |
| perft_bulk_d4 | 1.063x | 1.334x | 34/36 | 0.912x |
| perft_full_d3 | 1.191x | 1.604x | 36/36 | 1.226x |
| polyglot_key | 0.996x | 36.385x | 12/12 | 16.813x |
| polyglot_recompute | 1.536x | 1.597x | 12/12 | 1.240x |
| repetition_key | 0.951x | 21.858x | 12/12 | 14.839x |
| replay_checked_native | 0.984x | 1.181x | 12/12 | 1.096x |
| replay_trusted_native | 1.000x | 0.777x | 3/12 | 0.516x |
| san_parse_all | 1.002x | 1.837x | 36/36 | 1.342x |
| san_render_native | 1.007x | 2.612x | 36/36 | 1.881x |
| snapshot | 1.028x | 1.137x | 23/36 | 0.665x |
| startpos | 0.902x | 1.204x | 5/6 | 0.789x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.516x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.533x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.580x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.654x |
| macos-15-native | snapshot | promotion | ferrum | 0.665x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.689x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.701x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.783x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.788x |
| macos-15-native | startpos | start | shakmaty | 0.789x |
| macos-15-native | materialize | tactical | ultra | 0.829x |
| macos-15-native | apply_children_full | promotion | cozy | 0.868x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.912x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.919x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.941x |
| macos-15-native | apply_children_full | tactical | cozy | 0.944x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.950x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.957x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.970x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.973x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.979x |
| macos-15-native | snapshot | start | ferrum | 0.982x |
| macos-15-native | snapshot | middlegame | ferrum | 0.990x |
| macos-15-native | materialize | kiwipete | ultra | 0.992x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.997x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.997x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.998x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3416 | shakmaty | 5208 | 1.525x |
| ubuntu-24.04-arm-native | 6479 | shakmaty | 9991 | 1.542x |
| ubuntu-24.04-native | 8534 | shakmaty | 9879 | 1.158x |
| ubuntu-24.04-pext | 4657 | shakmaty | 7274 | 1.562x |
| ubuntu-24.04-portable | 5375 | shakmaty | 7985 | 1.486x |
| windows-2025-native | 10000 | shakmaty | 13900 | 1.390x |

Cold wins: 6/6 (separate from native cases).
