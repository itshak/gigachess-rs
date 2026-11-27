| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 73 | 72 | 13 | 87 |
| ubuntu-24.04-arm-native | 78 | 76 | 13 | 87 |
| ubuntu-24.04-native | 84 | 80 | 10 | 87 |
| ubuntu-24.04-pext | 86 | 86 | 16 | 87 |
| ubuntu-24.04-portable | 86 | 79 | 12 | 87 |
| windows-2025-native | 83 | 82 | 12 | 87 |
| ALL | 490 | 475 | 76 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.012x | 1.259x | 34/36 | 0.947x |
| chess960_materialize | 0.979x | 1.520x | 12/12 | 1.290x |
| chess960_perft_d3 | 0.995x | 1.553x | 12/12 | 1.332x |
| corpus_count | 1.113x | 1.373x | 12/12 | 1.046x |
| corpus_materialize | 0.981x | 1.194x | 12/12 | 1.015x |
| corpus_uci_sorted | 0.997x | 1.582x | 12/12 | 1.118x |
| count | 1.035x | 1.552x | 36/36 | 1.044x |
| fen_parse | 1.034x | 1.165x | 36/36 | 1.020x |
| fen_write_native_ep | 1.008x | 1.709x | 36/36 | 1.082x |
| make_unmake_full | 0.985x | 1.301x | 36/36 | 1.011x |
| materialize | 1.006x | 1.320x | 33/36 | 0.804x |
| perft_bulk_d4 | 1.052x | 1.345x | 33/36 | 0.839x |
| perft_full_d3 | 1.182x | 1.531x | 36/36 | 1.069x |
| polyglot_key | 0.979x | 31.597x | 12/12 | 16.658x |
| polyglot_recompute | 0.992x | 1.042x | 9/12 | 0.906x |
| repetition_key | 0.989x | 23.106x | 12/12 | 14.637x |
| replay_checked_native | 0.962x | 1.164x | 11/12 | 0.963x |
| replay_trusted_native | 0.999x | 0.845x | 5/12 | 0.563x |
| san_parse_all | 1.005x | 1.969x | 36/36 | 1.445x |
| san_render_native | 0.993x | 2.507x | 36/36 | 1.496x |
| snapshot | 1.152x | 1.058x | 25/36 | 0.503x |
| startpos | 0.951x | 1.140x | 4/6 | 0.932x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | snapshot | promotion | ferrum | 0.503x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.563x |
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.586x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.642x |
| macos-15-native | snapshot | endgame | chess | 0.687x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.710x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.714x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.725x |
| macos-15-native | materialize | kiwipete | ultra | 0.804x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.825x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.839x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.906x |
| macos-15-native | materialize | endgame | ultra | 0.916x |
| macos-15-native | materialize | tactical | ultra | 0.917x |
| macos-15-native | snapshot | kiwipete | cozy | 0.927x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.930x |
| macos-15-native | startpos | start | shakmaty | 0.932x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.941x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.946x |
| ubuntu-24.04-arm-native | apply_children_full | promotion | chess | 0.947x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.963x |
| macos-15-native | snapshot | start | ferrum | 0.978x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.979x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.979x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.981x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.983x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.987x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.987x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.987x |
| macos-15-native | apply_children_full | tactical | cozy | 0.988x |
| windows-2025-native | startpos | start | shakmaty | 0.988x |
| macos-15-native | snapshot | middlegame | ferrum | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6500 | shakmaty | 8792 | 1.353x |
| ubuntu-24.04-arm-native | 4556 | shakmaty | 4260 | 0.935x |
| ubuntu-24.04-native | 5162 | shakmaty | 7158 | 1.387x |
| ubuntu-24.04-pext | 6910 | cozy | 10316 | 1.493x |
| ubuntu-24.04-portable | 8306 | shakmaty | 11983 | 1.443x |
| windows-2025-native | 20900 | shakmaty | 28500 | 1.364x |

Cold wins: 5/6 (separate from native cases).
