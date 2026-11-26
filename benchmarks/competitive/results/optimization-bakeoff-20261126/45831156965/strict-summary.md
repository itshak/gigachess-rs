| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 68 | 12 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 79 | 75 | 10 | 87 |
| ubuntu-24.04-pext | 79 | 76 | 14 | 87 |
| ubuntu-24.04-portable | 78 | 71 | 13 | 87 |
| windows-2025-native | 79 | 76 | 11 | 87 |
| ALL | 467 | 443 | 71 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.003x | 1.233x | 34/36 | 0.941x |
| chess960_materialize | 1.022x | 1.329x | 12/12 | 1.155x |
| chess960_perft_d3 | 1.039x | 1.358x | 12/12 | 1.185x |
| corpus_count | 1.019x | 0.992x | 6/12 | 0.726x |
| corpus_materialize | 1.024x | 1.077x | 11/12 | 0.997x |
| corpus_uci_sorted | 1.037x | 1.536x | 12/12 | 1.408x |
| count | 1.011x | 1.297x | 35/36 | 0.956x |
| fen_parse | 1.019x | 1.152x | 35/36 | 0.968x |
| fen_write_native_ep | 0.985x | 1.809x | 36/36 | 1.424x |
| make_unmake_full | 1.017x | 1.278x | 36/36 | 1.063x |
| materialize | 1.022x | 1.290x | 33/36 | 0.853x |
| perft_bulk_d4 | 1.012x | 1.118x | 26/36 | 0.815x |
| perft_full_d3 | 1.008x | 1.325x | 35/36 | 0.935x |
| polyglot_key | 1.002x | 33.746x | 12/12 | 15.522x |
| polyglot_recompute | 1.023x | 1.018x | 8/12 | 0.931x |
| repetition_key | 1.041x | 21.681x | 12/12 | 14.361x |
| replay_checked_native | 1.016x | 1.092x | 12/12 | 1.014x |
| replay_trusted_native | 1.085x | 0.782x | 4/12 | 0.515x |
| san_parse_all | 1.004x | 1.833x | 36/36 | 1.241x |
| san_render_native | 1.003x | 2.565x | 36/36 | 1.486x |
| snapshot | 0.991x | 1.036x | 18/36 | 0.873x |
| startpos | 1.035x | 1.198x | 6/6 | 1.010x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.515x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.571x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.625x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.660x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.676x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.684x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.699x |
| windows-2025-native | corpus_count | seeded | ultra | 0.726x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.768x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.815x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.817x |
| macos-15-native | materialize | middlegame | ultra | 0.853x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.856x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.857x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.872x |
| macos-15-native | snapshot | promotion | ferrum | 0.873x |
| macos-15-native | perft_bulk_d4 | tactical | ultra | 0.893x |
| windows-2025-native | perft_bulk_d4 | start | ultra | 0.895x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.902x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.913x |
| macos-15-native | materialize | tactical | ultra | 0.922x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.931x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.935x |
| ubuntu-24.04-portable | corpus_count | seeded | ultra | 0.937x |
| macos-15-native | apply_children_full | tactical | chess | 0.941x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.944x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.950x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.951x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.956x |
| macos-15-native | apply_children_full | kiwipete | shakmaty | 0.959x |
| macos-15-native | fen_parse | endgame | shakmaty | 0.968x |
| ubuntu-24.04-pext | snapshot | tactical | cozy | 0.971x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.973x |
| macos-15-native | materialize | kiwipete | ultra | 0.973x |
| ubuntu-24.04-pext | snapshot | kiwipete | ferrum | 0.974x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.975x |
| ubuntu-24.04-pext | snapshot | middlegame | cozy | 0.975x |
| ubuntu-24.04-pext | snapshot | promotion | ferrum | 0.976x |
| ubuntu-24.04-pext | snapshot | endgame | ferrum | 0.976x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.977x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.986x |
| windows-2025-native | perft_bulk_d4 | tactical | ultra | 0.990x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.990x |
| ubuntu-24.04-native | perft_bulk_d4 | tactical | ultra | 0.991x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.993x |
| windows-2025-native | snapshot | tactical | ferrum | 0.994x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.994x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.996x |
| macos-15-native | corpus_materialize | seeded | ultra | 0.997x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.998x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6583 | cozy | 9708 | 1.475x |
| ubuntu-24.04-arm-native | 6808 | chess | 8504 | 1.249x |
| ubuntu-24.04-native | 7272 | shakmaty | 10724 | 1.475x |
| ubuntu-24.04-pext | 7434 | cozy | 11252 | 1.514x |
| ubuntu-24.04-portable | 5109 | shakmaty | 7675 | 1.502x |
| windows-2025-native | 17000 | shakmaty | 21600 | 1.271x |

Cold wins: 6/6 (separate from native cases).
