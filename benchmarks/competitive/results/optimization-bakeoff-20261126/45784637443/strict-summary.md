| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 69 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 71 | 11 | 87 |
| ubuntu-24.04-native | 83 | 76 | 9 | 87 |
| ubuntu-24.04-pext | 79 | 75 | 13 | 87 |
| ubuntu-24.04-portable | 74 | 72 | 15 | 87 |
| windows-2025-native | 80 | 79 | 11 | 87 |
| ALL | 464 | 442 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.015x | 1.081x | 28/36 | 0.720x |
| chess960_materialize | 1.078x | 1.475x | 12/12 | 1.224x |
| chess960_perft_d3 | 1.041x | 1.459x | 12/12 | 1.178x |
| corpus_count | 0.999x | 1.342x | 12/12 | 1.038x |
| corpus_materialize | 1.023x | 1.208x | 12/12 | 1.036x |
| corpus_uci_sorted | 0.998x | 1.510x | 12/12 | 1.217x |
| count | 1.041x | 1.395x | 36/36 | 1.010x |
| fen_parse | 1.050x | 1.209x | 36/36 | 1.041x |
| fen_write_native_ep | 1.018x | 1.735x | 36/36 | 1.424x |
| make_unmake_full | 1.011x | 1.277x | 36/36 | 1.034x |
| materialize | 1.013x | 1.236x | 33/36 | 0.860x |
| perft_bulk_d4 | 1.032x | 1.311x | 36/36 | 1.087x |
| perft_full_d3 | 1.007x | 1.147x | 27/36 | 0.852x |
| polyglot_key | 0.973x | 28.475x | 12/12 | 15.635x |
| polyglot_recompute | 1.001x | 1.007x | 7/12 | 0.873x |
| repetition_key | 1.039x | 20.456x | 12/12 | 14.013x |
| replay_checked_native | 1.008x | 0.973x | 5/12 | 0.838x |
| replay_trusted_native | 0.983x | 0.636x | 0/12 | 0.471x |
| san_parse_all | 1.021x | 1.980x | 36/36 | 1.422x |
| san_render_native | 1.016x | 2.561x | 36/36 | 1.681x |
| snapshot | 1.036x | 1.060x | 22/36 | 0.787x |
| startpos | 0.963x | 1.221x | 6/6 | 1.002x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.471x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.487x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.517x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.519x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.536x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.613x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.638x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.652x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.719x |
| macos-15-native | apply_children_full | tactical | chess | 0.720x |
| macos-15-native | snapshot | start | ferrum | 0.787x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.791x |
| macos-15-native | snapshot | kiwipete | cozy | 0.792x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.793x |
| macos-15-native | snapshot | middlegame | chess | 0.801x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.838x |
| ubuntu-24.04-pext | perft_full_d3 | promotion | ultra | 0.852x |
| macos-15-native | materialize | tactical | ultra | 0.860x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.866x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.867x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.873x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.873x |
| macos-15-native | apply_children_full | promotion | cozy | 0.874x |
| ubuntu-24.04-pext | perft_full_d3 | middlegame | ultra | 0.876x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.879x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.881x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.884x |
| macos-15-native | materialize | kiwipete | ultra | 0.886x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | ultra | 0.889x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.890x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.898x |
| macos-15-native | apply_children_full | middlegame | chess | 0.909x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.917x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.927x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | ultra | 0.929x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.934x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.936x |
| macos-15-native | materialize | middlegame | ultra | 0.942x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.946x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.952x |
| macos-15-native | snapshot | promotion | ferrum | 0.955x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.958x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.961x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.969x |
| macos-15-native | perft_full_d3 | tactical | ultra | 0.969x |
| ubuntu-24.04-portable | replay_checked_native | lichess300 | cozy | 0.972x |
| macos-15-native | snapshot | endgame | ferrum | 0.974x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.976x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.982x |
| ubuntu-24.04-portable | apply_children_full | tactical | chess | 0.984x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.985x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.986x |
| ubuntu-24.04-arm-native | snapshot | promotion | cozy | 0.993x |
| ubuntu-24.04-arm-native | snapshot | tactical | cozy | 0.994x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.995x |
| ubuntu-24.04-native | snapshot | endgame | ferrum | 0.997x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.998x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 9125 | cozy | 11167 | 1.224x |
| ubuntu-24.04-arm-native | 8504 | shakmaty | 8416 | 0.990x |
| ubuntu-24.04-native | 8287 | shakmaty | 6677 | 0.806x |
| ubuntu-24.04-pext | 10065 | shakmaty | 10125 | 1.006x |
| ubuntu-24.04-portable | 10215 | shakmaty | 10446 | 1.023x |
| windows-2025-native | 15600 | shakmaty | 16100 | 1.032x |

Cold wins: 4/6 (separate from native cases).
