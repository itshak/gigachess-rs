| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 67 | 11 | 87 |
| ubuntu-24.04-arm-native | 76 | 68 | 10 | 87 |
| ubuntu-24.04-native | 73 | 71 | 13 | 87 |
| ubuntu-24.04-pext | 82 | 77 | 8 | 87 |
| ubuntu-24.04-portable | 76 | 66 | 11 | 87 |
| windows-2025-native | 73 | 71 | 11 | 87 |
| ALL | 449 | 420 | 64 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.964x | 1.036x | 24/36 | 0.626x |
| chess960_materialize | 0.984x | 1.339x | 12/12 | 1.155x |
| chess960_perft_d3 | 0.976x | 1.433x | 12/12 | 1.263x |
| corpus_count | 0.986x | 1.349x | 12/12 | 1.154x |
| corpus_materialize | 0.992x | 1.171x | 12/12 | 1.103x |
| corpus_uci_sorted | 0.977x | 1.545x | 12/12 | 1.290x |
| count | 0.994x | 1.349x | 36/36 | 1.025x |
| fen_parse | 1.002x | 1.238x | 36/36 | 1.042x |
| fen_write_native_ep | 0.982x | 1.156x | 31/36 | 0.886x |
| make_unmake_full | 0.973x | 1.255x | 36/36 | 1.026x |
| materialize | 0.986x | 1.186x | 31/36 | 0.784x |
| perft_bulk_d4 | 0.980x | 1.276x | 36/36 | 1.087x |
| perft_full_d3 | 0.807x | 0.955x | 12/36 | 0.664x |
| polyglot_key | 0.973x | 27.091x | 12/12 | 15.554x |
| polyglot_recompute | 0.961x | 0.991x | 6/12 | 0.778x |
| repetition_key | 0.988x | 18.681x | 12/12 | 14.280x |
| replay_checked_native | 0.968x | 1.011x | 7/12 | 0.883x |
| replay_trusted_native | 0.960x | 0.655x | 0/12 | 0.483x |
| san_parse_all | 0.997x | 1.896x | 36/36 | 1.280x |
| san_render_native | 1.012x | 2.564x | 36/36 | 1.625x |
| snapshot | 1.263x | 1.059x | 32/36 | 0.943x |
| startpos | 1.014x | 1.229x | 6/6 | 1.082x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.483x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.501x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.501x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.586x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.606x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.619x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.625x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.626x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.631x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.664x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.689x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.706x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.778x |
| macos-15-native | materialize | tactical | ultra | 0.784x |
| macos-15-native | materialize | kiwipete | ultra | 0.789x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.797x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.807x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.807x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.816x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.820x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.827x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.841x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.855x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.861x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.864x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.878x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.883x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.885x |
| ubuntu-24.04-portable | fen_write_native_ep | kiwipete | ultra | 0.886x |
| ubuntu-24.04-native | perft_full_d3 | endgame | ultra | 0.891x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.894x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.903x |
| ubuntu-24.04-portable | perft_full_d3 | promotion | ultra | 0.904x |
| ubuntu-24.04-portable | fen_write_native_ep | middlegame | ultra | 0.905x |
| ubuntu-24.04-native | perft_full_d3 | start | ultra | 0.916x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.918x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.923x |
| macos-15-native | perft_full_d3 | kiwipete | shakmaty | 0.936x |
| ubuntu-24.04-native | apply_children_full | promotion | cozy | 0.938x |
| ubuntu-24.04-portable | fen_write_native_ep | promotion | ultra | 0.938x |
| macos-15-native | snapshot | middlegame | ferrum | 0.943x |
| ubuntu-24.04-native | apply_children_full | tactical | cozy | 0.947x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.950x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.951x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.959x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.961x |
| macos-15-native | snapshot | tactical | ferrum | 0.961x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.961x |
| ubuntu-24.04-arm-native | perft_full_d3 | kiwipete | ultra | 0.967x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.968x |
| windows-2025-native | perft_full_d3 | endgame | ultra | 0.969x |
| ubuntu-24.04-portable | perft_full_d3 | middlegame | ultra | 0.971x |
| ubuntu-24.04-pext | perft_full_d3 | tactical | shakmaty | 0.973x |
| ubuntu-24.04-portable | perft_full_d3 | kiwipete | ultra | 0.973x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.974x |
| ubuntu-24.04-pext | perft_full_d3 | kiwipete | shakmaty | 0.974x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.976x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.976x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.978x |
| ubuntu-24.04-portable | fen_write_native_ep | tactical | ultra | 0.979x |
| macos-15-native | snapshot | start | ferrum | 0.980x |
| macos-15-native | materialize | middlegame | ultra | 0.987x |
| ubuntu-24.04-arm-native | perft_full_d3 | middlegame | ultra | 0.988x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.988x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.991x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.991x |
| windows-2025-native | materialize | kiwipete | ultra | 0.992x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.993x |
| ubuntu-24.04-portable | perft_full_d3 | tactical | ultra | 0.994x |
| ubuntu-24.04-native | replay_checked_native | lichess300 | cozy | 0.997x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.997x |
| windows-2025-native | perft_full_d3 | start | ultra | 0.997x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 18084 | shakmaty | 12750 | 0.705x |
| ubuntu-24.04-arm-native | 10920 | shakmaty | 7544 | 0.691x |
| ubuntu-24.04-native | 12419 | shakmaty | 8733 | 0.703x |
| ubuntu-24.04-pext | 9222 | shakmaty | 7686 | 0.833x |
| ubuntu-24.04-portable | 12584 | shakmaty | 11431 | 0.908x |
| windows-2025-native | 25600 | shakmaty | 20500 | 0.801x |

Cold wins: 0/6 (separate from native cases).
