| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 76 | 74 | 13 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 86 | 85 | 12 | 87 |
| ubuntu-24.04-pext | 76 | 73 | 13 | 87 |
| ubuntu-24.04-portable | 81 | 76 | 12 | 87 |
| windows-2025-native | 82 | 79 | 13 | 87 |
| ALL | 480 | 466 | 75 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.009x | 1.292x | 36/36 | 1.045x |
| chess960_materialize | 1.007x | 1.431x | 12/12 | 1.195x |
| chess960_perft_d3 | 0.991x | 1.508x | 12/12 | 1.368x |
| corpus_count | 1.011x | 1.238x | 11/12 | 0.878x |
| corpus_materialize | 1.005x | 1.134x | 12/12 | 1.040x |
| corpus_uci_sorted | 1.008x | 1.505x | 12/12 | 1.310x |
| count | 0.999x | 1.459x | 36/36 | 1.025x |
| fen_parse | 1.029x | 1.215x | 35/36 | 0.991x |
| fen_write_native_ep | 0.997x | 1.749x | 36/36 | 1.291x |
| make_unmake_full | 0.975x | 1.289x | 35/36 | 0.995x |
| materialize | 0.988x | 1.331x | 32/36 | 0.698x |
| perft_bulk_d4 | 0.995x | 1.293x | 34/36 | 0.922x |
| perft_full_d3 | 1.011x | 1.336x | 36/36 | 1.052x |
| polyglot_key | 0.998x | 33.336x | 12/12 | 16.189x |
| polyglot_recompute | 1.001x | 1.027x | 8/12 | 0.970x |
| repetition_key | 0.983x | 20.845x | 12/12 | 14.163x |
| replay_checked_native | 0.998x | 1.147x | 12/12 | 1.042x |
| replay_trusted_native | 1.031x | 0.808x | 4/12 | 0.522x |
| san_parse_all | 1.016x | 1.937x | 36/36 | 1.537x |
| san_render_native | 1.003x | 2.591x | 36/36 | 1.851x |
| snapshot | 1.008x | 1.018x | 16/36 | 0.826x |
| startpos | 0.981x | 1.206x | 5/6 | 0.999x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.522x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.595x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.658x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.677x |
| macos-15-native | materialize | kiwipete | ultra | 0.698x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.722x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.733x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.786x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.826x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.845x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.878x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.922x |
| macos-15-native | materialize | start | ultra | 0.940x |
| macos-15-native | materialize | middlegame | ultra | 0.941x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.942x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.943x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.945x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.948x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.948x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.950x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.970x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.973x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.979x |
| macos-15-native | materialize | tactical | ultra | 0.981x |
| ubuntu-24.04-pext | snapshot | kiwipete | chess | 0.986x |
| windows-2025-native | snapshot | kiwipete | chess | 0.987x |
| ubuntu-24.04-pext | snapshot | endgame | chess | 0.989x |
| windows-2025-native | snapshot | promotion | chess | 0.990x |
| ubuntu-24.04-pext | fen_parse | start | ultra | 0.991x |
| ubuntu-24.04-pext | snapshot | tactical | chess | 0.992x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.994x |
| ubuntu-24.04-pext | snapshot | promotion | cozy | 0.994x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.995x |
| ubuntu-24.04-pext | snapshot | start | cozy | 0.995x |
| windows-2025-native | snapshot | middlegame | ferrum | 0.996x |
| macos-15-native | snapshot | middlegame | ferrum | 0.996x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.999x |
| macos-15-native | snapshot | endgame | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 1.000x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 4250 | ferrum | 6417 | 1.510x |
| ubuntu-24.04-arm-native | 5407 | shakmaty | 7056 | 1.305x |
| ubuntu-24.04-native | 11953 | chess | 17242 | 1.442x |
| ubuntu-24.04-pext | 6470 | shakmaty | 7691 | 1.189x |
| ubuntu-24.04-portable | 5167 | shakmaty | 6740 | 1.304x |
| windows-2025-native | 16400 | shakmaty | 19200 | 1.171x |

Cold wins: 6/6 (separate from native cases).
