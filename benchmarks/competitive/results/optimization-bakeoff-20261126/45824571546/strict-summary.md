| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 72 | 70 | 11 | 87 |
| ubuntu-24.04-arm-native | 78 | 77 | 11 | 87 |
| ubuntu-24.04-native | 77 | 76 | 12 | 87 |
| ubuntu-24.04-pext | 79 | 74 | 12 | 87 |
| ubuntu-24.04-portable | 80 | 73 | 9 | 87 |
| windows-2025-native | 80 | 78 | 11 | 87 |
| ALL | 466 | 448 | 66 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.000x | 1.205x | 35/36 | 0.824x |
| chess960_materialize | 1.013x | 1.369x | 12/12 | 1.152x |
| chess960_perft_d3 | 1.064x | 1.328x | 12/12 | 1.094x |
| corpus_count | 0.973x | 0.950x | 4/12 | 0.735x |
| corpus_materialize | 1.011x | 1.083x | 11/12 | 0.981x |
| corpus_uci_sorted | 0.962x | 1.498x | 12/12 | 1.316x |
| count | 0.988x | 1.315x | 35/36 | 0.946x |
| fen_parse | 1.044x | 1.219x | 36/36 | 1.044x |
| fen_write_native_ep | 1.023x | 1.751x | 36/36 | 1.454x |
| make_unmake_full | 0.999x | 1.277x | 35/36 | 0.969x |
| materialize | 0.996x | 1.260x | 32/36 | 0.630x |
| perft_bulk_d4 | 1.007x | 1.070x | 22/36 | 0.784x |
| perft_full_d3 | 1.033x | 1.382x | 34/36 | 0.948x |
| polyglot_key | 0.996x | 36.151x | 12/12 | 17.154x |
| polyglot_recompute | 1.027x | 1.061x | 10/12 | 0.984x |
| repetition_key | 0.964x | 19.586x | 12/12 | 14.329x |
| replay_checked_native | 0.970x | 1.111x | 12/12 | 1.005x |
| replay_trusted_native | 0.962x | 0.730x | 3/12 | 0.526x |
| san_parse_all | 0.983x | 1.745x | 36/36 | 1.337x |
| san_render_native | 1.022x | 2.638x | 36/36 | 1.851x |
| snapshot | 1.048x | 1.124x | 24/36 | 0.801x |
| startpos | 1.078x | 1.148x | 5/6 | 0.999x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.526x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.538x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.558x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.600x |
| macos-15-native | materialize | kiwipete | ultra | 0.630x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.638x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.639x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.649x |
| windows-2025-native | corpus_count | seeded | ultra | 0.735x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.739x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.784x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.796x |
| macos-15-native | snapshot | endgame | ferrum | 0.801x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.804x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.823x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.824x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.833x |
| ubuntu-24.04-portable | corpus_count | lichess300 | ultra | 0.848x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.858x |
| macos-15-native | snapshot | start | ferrum | 0.883x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.892x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.894x |
| windows-2025-native | perft_bulk_d4 | start | ultra | 0.899x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.925x |
| macos-15-native | materialize | promotion | ultra | 0.925x |
| macos-15-native | materialize | middlegame | ultra | 0.927x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.935x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.937x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.937x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.938x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.939x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.940x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.940x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.941x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.946x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.948x |
| ubuntu-24.04-native | perft_bulk_d4 | start | ultra | 0.951x |
| macos-15-native | materialize | tactical | ultra | 0.952x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.955x |
| macos-15-native | snapshot | middlegame | ferrum | 0.961x |
| windows-2025-native | perft_bulk_d4 | tactical | ultra | 0.963x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.963x |
| ubuntu-24.04-pext | make_unmake_full | tactical | ultra | 0.969x |
| windows-2025-native | perft_bulk_d4 | kiwipete | ultra | 0.971x |
| ubuntu-24.04-native | corpus_materialize | seeded | ultra | 0.981x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.982x |
| ubuntu-24.04-pext | polyglot_recompute | seeded | shakmaty | 0.984x |
| macos-15-native | snapshot | promotion | ferrum | 0.984x |
| ubuntu-24.04-pext | perft_bulk_d4 | kiwipete | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.990x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.991x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.994x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.995x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.998x |
| ubuntu-24.04-native | startpos | start | shakmaty | 0.999x |
| ubuntu-24.04-native | perft_bulk_d4 | tactical | ultra | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5334 | shakmaty | 10417 | 1.953x |
| ubuntu-24.04-arm-native | 5568 | shakmaty | 8807 | 1.582x |
| ubuntu-24.04-native | 7084 | shakmaty | 10142 | 1.432x |
| ubuntu-24.04-pext | 5458 | chess | 8005 | 1.467x |
| ubuntu-24.04-portable | 6470 | shakmaty | 9274 | 1.433x |
| windows-2025-native | 18900 | shakmaty | 23700 | 1.254x |

Cold wins: 6/6 (separate from native cases).
