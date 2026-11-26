| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 70 | 13 | 87 |
| ubuntu-24.04-arm-native | 84 | 83 | 11 | 87 |
| ubuntu-24.04-native | 77 | 74 | 11 | 87 |
| ubuntu-24.04-pext | 81 | 78 | 9 | 87 |
| ubuntu-24.04-portable | 83 | 78 | 10 | 87 |
| windows-2025-native | 74 | 71 | 11 | 87 |
| ALL | 474 | 454 | 65 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.885x | 1.147x | 28/36 | 0.834x |
| chess960_materialize | 0.942x | 1.363x | 12/12 | 1.044x |
| chess960_perft_d3 | 0.927x | 1.368x | 12/12 | 1.259x |
| corpus_count | 0.959x | 1.063x | 7/12 | 0.798x |
| corpus_materialize | 1.015x | 1.129x | 12/12 | 1.042x |
| corpus_uci_sorted | 1.053x | 1.504x | 12/12 | 1.275x |
| count | 0.921x | 1.350x | 35/36 | 0.976x |
| fen_parse | 0.993x | 1.156x | 32/36 | 0.968x |
| fen_write_native_ep | 0.931x | 1.650x | 36/36 | 1.229x |
| make_unmake_full | 0.953x | 1.214x | 34/36 | 0.924x |
| materialize | 0.953x | 1.337x | 33/36 | 0.876x |
| perft_bulk_d4 | 0.928x | 1.167x | 31/36 | 0.800x |
| perft_full_d3 | 0.850x | 1.128x | 30/36 | 0.852x |
| polyglot_key | 1.034x | 37.229x | 12/12 | 18.431x |
| polyglot_recompute | 1.003x | 1.020x | 9/12 | 0.874x |
| repetition_key | 1.023x | 22.829x | 12/12 | 17.374x |
| replay_checked_native | 0.965x | 1.078x | 10/12 | 0.988x |
| replay_trusted_native | 1.011x | 0.768x | 4/12 | 0.492x |
| san_parse_all | 0.991x | 1.807x | 36/36 | 1.402x |
| san_render_native | 0.974x | 2.522x | 36/36 | 1.765x |
| snapshot | 0.951x | 1.171x | 36/36 | 1.051x |
| startpos | 1.263x | 1.545x | 5/6 | 0.863x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.492x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.556x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.602x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.622x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.644x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.645x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.646x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.775x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.798x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.800x |
| windows-2025-native | apply_children_full | promotion | cozy | 0.834x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.844x |
| macos-15-native | perft_full_d3 | middlegame | ultra | 0.852x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.862x |
| windows-2025-native | startpos | start | shakmaty | 0.863x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.874x |
| macos-15-native | materialize | middlegame | ultra | 0.876x |
| windows-2025-native | apply_children_full | kiwipete | cozy | 0.888x |
| macos-15-native | perft_full_d3 | promotion | ultra | 0.899x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.900x |
| windows-2025-native | apply_children_full | endgame | cozy | 0.905x |
| macos-15-native | materialize | kiwipete | ultra | 0.915x |
| windows-2025-native | perft_full_d3 | start | ultra | 0.916x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.923x |
| macos-15-native | make_unmake_full | promotion | ultra | 0.924x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.927x |
| windows-2025-native | perft_full_d3 | promotion | ultra | 0.931x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.934x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.942x |
| ubuntu-24.04-pext | apply_children_full | promotion | cozy | 0.951x |
| macos-15-native | make_unmake_full | tactical | ultra | 0.954x |
| windows-2025-native | apply_children_full | middlegame | cozy | 0.960x |
| windows-2025-native | apply_children_full | tactical | cozy | 0.963x |
| macos-15-native | perft_full_d3 | tactical | ultra | 0.965x |
| ubuntu-24.04-pext | apply_children_full | tactical | chess | 0.967x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.967x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.968x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.975x |
| windows-2025-native | perft_full_d3 | tactical | ultra | 0.975x |
| macos-15-native | materialize | tactical | ultra | 0.975x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.976x |
| ubuntu-24.04-native | fen_parse | middlegame | ultra | 0.979x |
| ubuntu-24.04-portable | fen_parse | start | ultra | 0.986x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.988x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.988x |
| windows-2025-native | apply_children_full | start | cozy | 0.991x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.995x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.999x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5333 | shakmaty | 10166 | 1.906x |
| ubuntu-24.04-arm-native | 5695 | chess | 10616 | 1.864x |
| ubuntu-24.04-native | 7605 | shakmaty | 7660 | 1.007x |
| ubuntu-24.04-pext | 6283 | chess | 7947 | 1.265x |
| ubuntu-24.04-portable | 5864 | shakmaty | 6941 | 1.184x |
| windows-2025-native | 10600 | shakmaty | 13200 | 1.245x |

Cold wins: 6/6 (separate from native cases).
