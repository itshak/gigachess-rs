| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 69 | 67 | 11 | 87 |
| ubuntu-24.04-arm-native | 77 | 71 | 10 | 87 |
| ubuntu-24.04-native | 84 | 82 | 8 | 87 |
| ubuntu-24.04-pext | 85 | 80 | 8 | 87 |
| ubuntu-24.04-portable | 78 | 71 | 10 | 87 |
| windows-2025-native | 73 | 71 | 11 | 87 |
| ALL | 466 | 442 | 58 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.002x | 1.052x | 26/36 | 0.878x |
| chess960_materialize | 0.996x | 1.350x | 12/12 | 1.163x |
| chess960_perft_d3 | 0.971x | 1.345x | 12/12 | 1.077x |
| corpus_count | 0.920x | 1.198x | 10/12 | 0.681x |
| corpus_materialize | 1.017x | 1.156x | 12/12 | 1.053x |
| corpus_uci_sorted | 1.007x | 1.486x | 12/12 | 1.358x |
| count | 0.967x | 1.300x | 34/36 | 0.641x |
| fen_parse | 1.010x | 1.198x | 35/36 | 0.910x |
| fen_write_native_ep | 0.973x | 1.192x | 34/36 | 0.960x |
| make_unmake_full | 0.990x | 1.262x | 35/36 | 0.890x |
| materialize | 0.977x | 1.273x | 33/36 | 0.873x |
| perft_bulk_d4 | 0.989x | 1.247x | 36/36 | 1.024x |
| perft_full_d3 | 0.987x | 1.257x | 30/36 | 0.853x |
| polyglot_key | 0.995x | 35.092x | 12/12 | 16.370x |
| polyglot_recompute | 1.004x | 1.014x | 8/12 | 0.869x |
| repetition_key | 1.009x | 21.753x | 12/12 | 14.010x |
| replay_checked_native | 1.016x | 1.014x | 8/12 | 0.828x |
| replay_trusted_native | 0.997x | 0.617x | 0/12 | 0.440x |
| san_parse_all | 0.988x | 1.849x | 36/36 | 1.296x |
| san_render_native | 1.006x | 2.540x | 36/36 | 1.923x |
| snapshot | 1.123x | 1.105x | 27/36 | 0.714x |
| startpos | 1.055x | 1.204x | 6/6 | 1.119x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.440x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.474x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.476x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.486x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.506x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.630x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.633x |
| windows-2025-native | count | endgame | cozy | 0.641x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.681x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.712x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.714x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.719x |
| windows-2025-native | corpus_count | seeded | ultra | 0.725x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.803x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.828x |
| ubuntu-24.04-native | replay_trusted_native | seeded | shakmaty | 0.835x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.853x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.869x |
| macos-15-native | materialize | kiwipete | ultra | 0.873x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.876x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.878x |
| macos-15-native | materialize | tactical | ultra | 0.885x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.890x |
| macos-15-native | apply_children_full | tactical | chess | 0.895x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.898x |
| ubuntu-24.04-portable | apply_children_full | promotion | chess | 0.903x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.909x |
| windows-2025-native | fen_parse | start | cozy | 0.910x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.910x |
| macos-15-native | apply_children_full | middlegame | shakmaty | 0.915x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.918x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.929x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.932x |
| windows-2025-native | materialize | kiwipete | ultra | 0.932x |
| macos-15-native | apply_children_full | promotion | shakmaty | 0.940x |
| ubuntu-24.04-portable | replay_checked_native | seeded | cozy | 0.951x |
| ubuntu-24.04-arm-native | fen_write_native_ep | kiwipete | ultra | 0.960x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.960x |
| ubuntu-24.04-arm-native | perft_full_d3 | tactical | ultra | 0.963x |
| ubuntu-24.04-arm-native | apply_children_full | start | chess | 0.963x |
| macos-15-native | snapshot | endgame | ferrum | 0.964x |
| ubuntu-24.04-portable | count | start | ferrum | 0.970x |
| windows-2025-native | perft_full_d3 | middlegame | ultra | 0.976x |
| macos-15-native | snapshot | start | ferrum | 0.979x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | ferrum | 0.980x |
| macos-15-native | snapshot | promotion | ferrum | 0.980x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.980x |
| ubuntu-24.04-arm-native | apply_children_full | kiwipete | chess | 0.988x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.989x |
| ubuntu-24.04-arm-native | fen_write_native_ep | middlegame | ultra | 0.991x |
| macos-15-native | snapshot | tactical | ferrum | 0.991x |
| macos-15-native | snapshot | middlegame | ferrum | 0.991x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.992x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.997x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.998x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 9000 | shakmaty | 7417 | 0.824x |
| ubuntu-24.04-arm-native | 11721 | shakmaty | 7496 | 0.640x |
| ubuntu-24.04-native | 9064 | shakmaty | 6374 | 0.703x |
| ubuntu-24.04-pext | 7824 | shakmaty | 7938 | 1.015x |
| ubuntu-24.04-portable | 12749 | shakmaty | 9374 | 0.735x |
| windows-2025-native | 19800 | shakmaty | 15100 | 0.763x |

Cold wins: 1/6 (separate from native cases).
