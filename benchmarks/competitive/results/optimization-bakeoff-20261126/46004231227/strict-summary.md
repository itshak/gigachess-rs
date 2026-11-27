| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 72 | 16 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 84 | 81 | 9 | 87 |
| ubuntu-24.04-pext | 85 | 85 | 17 | 87 |
| ubuntu-24.04-portable | 80 | 79 | 14 | 87 |
| windows-2025-native | 85 | 82 | 13 | 87 |
| ALL | 487 | 478 | 81 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.007x | 1.258x | 34/36 | 0.790x |
| chess960_materialize | 1.020x | 1.486x | 12/12 | 1.106x |
| chess960_perft_d3 | 1.066x | 1.624x | 12/12 | 1.406x |
| corpus_count | 1.021x | 1.254x | 9/12 | 0.975x |
| corpus_materialize | 1.010x | 1.181x | 12/12 | 1.036x |
| corpus_uci_sorted | 1.031x | 1.495x | 12/12 | 1.211x |
| count | 0.999x | 1.484x | 36/36 | 1.136x |
| fen_parse | 1.012x | 1.214x | 35/36 | 0.988x |
| fen_write_native_ep | 1.003x | 1.729x | 36/36 | 1.210x |
| make_unmake_full | 1.011x | 1.368x | 36/36 | 1.078x |
| materialize | 0.994x | 1.289x | 36/36 | 1.003x |
| perft_bulk_d4 | 1.010x | 1.327x | 34/36 | 0.895x |
| perft_full_d3 | 1.017x | 1.299x | 36/36 | 1.096x |
| polyglot_key | 1.092x | 26.911x | 12/12 | 16.189x |
| polyglot_recompute | 1.042x | 1.006x | 7/12 | 0.862x |
| repetition_key | 1.077x | 20.377x | 12/12 | 14.021x |
| replay_checked_native | 1.048x | 1.166x | 12/12 | 1.051x |
| replay_trusted_native | 1.049x | 0.892x | 7/12 | 0.536x |
| san_parse_all | 1.014x | 1.987x | 36/36 | 1.370x |
| san_render_native | 1.018x | 2.574x | 36/36 | 1.540x |
| snapshot | 0.982x | 1.069x | 19/36 | 0.536x |
| startpos | 1.023x | 1.269x | 6/6 | 1.001x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.536x |
| macos-15-native | snapshot | promotion | ferrum | 0.536x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.584x |
| macos-15-native | snapshot | kiwipete | cozy | 0.690x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.694x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.709x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.738x |
| macos-15-native | apply_children_full | tactical | shakmaty | 0.790x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.862x |
| macos-15-native | snapshot | endgame | chess | 0.881x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.895x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.900x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.904x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.912x |
| macos-15-native | apply_children_full | promotion | ferrum | 0.921x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.929x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.954x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.957x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.959x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 0.963x |
| macos-15-native | snapshot | tactical | ferrum | 0.963x |
| macos-15-native | snapshot | start | ferrum | 0.963x |
| ubuntu-24.04-portable | snapshot | promotion | ferrum | 0.966x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.970x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.971x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.972x |
| ubuntu-24.04-portable | snapshot | tactical | ferrum | 0.972x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.975x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.983x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.984x |
| macos-15-native | fen_parse | kiwipete | ultra | 0.988x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.994x |
| macos-15-native | polyglot_recompute | seeded | shakmaty | 0.995x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 6042 | shakmaty | 10917 | 1.807x |
| ubuntu-24.04-arm-native | 5512 | shakmaty | 8319 | 1.509x |
| ubuntu-24.04-native | 6664 | shakmaty | 6511 | 0.977x |
| ubuntu-24.04-pext | 6490 | shakmaty | 9284 | 1.431x |
| ubuntu-24.04-portable | 6660 | shakmaty | 7721 | 1.159x |
| windows-2025-native | 12700 | shakmaty | 15600 | 1.228x |

Cold wins: 5/6 (separate from native cases).
