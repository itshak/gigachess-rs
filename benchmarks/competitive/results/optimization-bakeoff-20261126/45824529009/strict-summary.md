| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 71 | 67 | 14 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 80 | 73 | 10 | 87 |
| ubuntu-24.04-pext | 75 | 73 | 12 | 87 |
| ubuntu-24.04-portable | 79 | 78 | 12 | 87 |
| windows-2025-native | 81 | 76 | 12 | 87 |
| ALL | 464 | 444 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.015x | 1.266x | 34/36 | 0.938x |
| chess960_materialize | 0.996x | 1.381x | 12/12 | 1.251x |
| chess960_perft_d3 | 1.006x | 1.378x | 12/12 | 1.206x |
| corpus_count | 0.981x | 1.052x | 6/12 | 0.818x |
| corpus_materialize | 1.029x | 1.106x | 12/12 | 1.002x |
| corpus_uci_sorted | 0.979x | 1.533x | 12/12 | 1.158x |
| count | 1.004x | 1.321x | 36/36 | 1.021x |
| fen_parse | 1.030x | 1.155x | 30/36 | 0.938x |
| fen_write_native_ep | 1.009x | 1.767x | 36/36 | 1.423x |
| make_unmake_full | 1.023x | 1.287x | 35/36 | 0.984x |
| materialize | 1.024x | 1.280x | 33/36 | 0.902x |
| perft_bulk_d4 | 1.016x | 1.127x | 32/36 | 0.852x |
| perft_full_d3 | 1.019x | 1.297x | 35/36 | 0.945x |
| polyglot_key | 1.026x | 31.780x | 12/12 | 15.959x |
| polyglot_recompute | 0.988x | 0.985x | 5/12 | 0.817x |
| repetition_key | 0.988x | 17.773x | 12/12 | 13.945x |
| replay_checked_native | 1.030x | 1.068x | 9/12 | 0.958x |
| replay_trusted_native | 0.982x | 0.791x | 4/12 | 0.507x |
| san_parse_all | 1.002x | 1.940x | 36/36 | 1.542x |
| san_render_native | 1.013x | 2.536x | 36/36 | 1.901x |
| snapshot | 0.991x | 1.024x | 19/36 | 0.656x |
| startpos | 0.959x | 1.252x | 6/6 | 1.113x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.507x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.575x |
| macos-15-native | snapshot | tactical | ferrum | 0.656x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.671x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.672x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.721x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.726x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.806x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.817x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.818x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.830x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.837x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.852x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.869x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.881x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.900x |
| macos-15-native | materialize | kiwipete | ultra | 0.902x |
| macos-15-native | materialize | promotion | ultra | 0.905x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.910x |
| macos-15-native | corpus_count | seeded | ultra | 0.917x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.938x |
| macos-15-native | apply_children_full | promotion | cozy | 0.938x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.944x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.945x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.946x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.946x |
| macos-15-native | materialize | middlegame | ultra | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.949x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.958x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.959x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.962x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.965x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.966x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.967x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.973x |
| ubuntu-24.04-pext | fen_parse | middlegame | ultra | 0.974x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.975x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.975x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.976x |
| ubuntu-24.04-native | polyglot_recompute | seeded | shakmaty | 0.981x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.981x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.982x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.984x |
| ubuntu-24.04-native | make_unmake_full | tactical | ultra | 0.984x |
| ubuntu-24.04-pext | fen_parse | promotion | ultra | 0.986x |
| macos-15-native | snapshot | start | ferrum | 0.987x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.989x |
| windows-2025-native | snapshot | start | ferrum | 0.990x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.991x |
| ubuntu-24.04-pext | fen_parse | endgame | ultra | 0.992x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.992x |
| ubuntu-24.04-pext | fen_parse | tactical | ultra | 0.999x |
| ubuntu-24.04-pext | fen_parse | kiwipete | ultra | 1.000x |
| windows-2025-native | snapshot | endgame | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4500 | shakmaty | 6375 | 1.417x |
| ubuntu-24.04-arm-native | 6720 | shakmaty | 8871 | 1.320x |
| ubuntu-24.04-native | 6395 | ferrum | 9682 | 1.514x |
| ubuntu-24.04-pext | 5546 | shakmaty | 7581 | 1.367x |
| ubuntu-24.04-portable | 7735 | shakmaty | 10810 | 1.398x |
| windows-2025-native | 17600 | shakmaty | 27700 | 1.574x |

Cold wins: 6/6 (separate from native cases).
