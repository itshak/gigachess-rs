| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 70 | 67 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 75 | 66 | 16 | 87 |
| ubuntu-24.04-pext | 81 | 77 | 14 | 87 |
| ubuntu-24.04-portable | 80 | 75 | 8 | 87 |
| windows-2025-native | 77 | 75 | 11 | 87 |
| ALL | 461 | 437 | 72 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.008x | 1.129x | 31/36 | 0.622x |
| chess960_materialize | 1.008x | 1.343x | 12/12 | 1.237x |
| chess960_perft_d3 | 0.979x | 1.370x | 12/12 | 1.144x |
| corpus_count | 0.771x | 1.019x | 5/12 | 0.778x |
| corpus_materialize | 0.960x | 1.097x | 11/12 | 0.988x |
| corpus_uci_sorted | 0.991x | 1.511x | 12/12 | 1.369x |
| count | 0.976x | 1.354x | 35/36 | 0.956x |
| fen_parse | 0.976x | 1.192x | 36/36 | 1.006x |
| fen_write_native_ep | 1.002x | 1.821x | 36/36 | 1.439x |
| make_unmake_full | 0.999x | 1.226x | 34/36 | 0.894x |
| materialize | 0.962x | 1.315x | 34/36 | 0.950x |
| perft_bulk_d4 | 0.876x | 1.120x | 28/36 | 0.834x |
| perft_full_d3 | 0.980x | 1.249x | 32/36 | 0.902x |
| polyglot_key | 1.002x | 32.778x | 12/12 | 16.619x |
| polyglot_recompute | 1.016x | 1.046x | 10/12 | 0.896x |
| repetition_key | 0.905x | 17.872x | 12/12 | 14.337x |
| replay_checked_native | 0.955x | 1.040x | 8/12 | 0.840x |
| replay_trusted_native | 1.036x | 0.737x | 3/12 | 0.510x |
| san_parse_all | 0.966x | 1.955x | 36/36 | 1.419x |
| san_render_native | 0.954x | 2.418x | 36/36 | 1.622x |
| snapshot | 1.166x | 1.045x | 20/36 | 0.796x |
| startpos | 1.114x | 1.234x | 6/6 | 1.118x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.510x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.530x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.581x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.622x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.640x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.686x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.695x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.713x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.729x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.778x |
| macos-15-native | snapshot | start | cozy | 0.796x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.834x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.840x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.856x |
| macos-15-native | apply_children_full | endgame | shakmaty | 0.862x |
| windows-2025-native | replay_checked_native | seeded | cozy | 0.887x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.887x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.893x |
| macos-15-native | make_unmake_full | endgame | ultra | 0.894x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.902x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.906x |
| macos-15-native | apply_children_full | tactical | chess | 0.912x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.917x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.923x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.932x |
| windows-2025-native | perft_full_d3 | kiwipete | ultra | 0.934x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.935x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.950x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.950x |
| macos-15-native | materialize | middlegame | ultra | 0.950x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.951x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.954x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.956x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.957x |
| macos-15-native | corpus_count | seeded | ultra | 0.959x |
| windows-2025-native | materialize | kiwipete | ultra | 0.969x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.973x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.974x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.975x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.975x |
| macos-15-native | replay_checked_native | seeded | cozy | 0.986x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.988x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.990x |
| macos-15-native | snapshot | endgame | ferrum | 0.993x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.994x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.996x |
| windows-2025-native | replay_checked_native | lichess300 | cozy | 0.999x |
| ubuntu-24.04-native | perft_bulk_d4 | tactical | ultra | 0.999x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.999x |
| ubuntu-24.04-native | snapshot | promotion | chess | 0.999x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 1.000x |
| ubuntu-24.04-native | snapshot | start | chess | 1.000x |
| ubuntu-24.04-native | snapshot | kiwipete | chess | 1.000x |
| ubuntu-24.04-native | snapshot | endgame | chess | 1.000x |
| ubuntu-24.04-native | snapshot | middlegame | cozy | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 7250 | shakmaty | 10500 | 1.448x |
| ubuntu-24.04-arm-native | 6831 | shakmaty | 9023 | 1.321x |
| ubuntu-24.04-native | 5653 | shakmaty | 8124 | 1.437x |
| ubuntu-24.04-pext | 5238 | shakmaty | 6841 | 1.306x |
| ubuntu-24.04-portable | 5255 | shakmaty | 8373 | 1.593x |
| windows-2025-native | 19200 | shakmaty | 26000 | 1.354x |

Cold wins: 6/6 (separate from native cases).
