| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 75 | 69 | 11 | 87 |
| ubuntu-24.04-arm-native | 79 | 79 | 12 | 87 |
| ubuntu-24.04-native | 81 | 80 | 12 | 87 |
| ubuntu-24.04-pext | 82 | 79 | 9 | 87 |
| ubuntu-24.04-portable | 77 | 74 | 11 | 87 |
| windows-2025-native | 83 | 78 | 12 | 87 |
| ALL | 477 | 459 | 67 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 0.986x | 1.265x | 35/36 | 0.993x |
| chess960_materialize | 0.984x | 1.456x | 12/12 | 1.315x |
| chess960_perft_d3 | 0.982x | 1.493x | 12/12 | 1.356x |
| corpus_count | 0.884x | 1.070x | 7/12 | 0.825x |
| corpus_materialize | 0.984x | 1.110x | 12/12 | 1.015x |
| corpus_uci_sorted | 0.998x | 1.486x | 12/12 | 1.362x |
| count | 0.966x | 1.436x | 35/36 | 0.990x |
| fen_parse | 1.015x | 1.184x | 36/36 | 1.009x |
| fen_write_native_ep | 1.000x | 1.787x | 36/36 | 1.441x |
| make_unmake_full | 1.011x | 1.307x | 36/36 | 1.067x |
| materialize | 0.999x | 1.368x | 33/36 | 0.912x |
| perft_bulk_d4 | 0.972x | 1.226x | 31/36 | 0.892x |
| perft_full_d3 | 0.985x | 1.326x | 36/36 | 1.142x |
| polyglot_key | 1.001x | 34.432x | 12/12 | 16.259x |
| polyglot_recompute | 0.997x | 1.008x | 7/12 | 0.896x |
| repetition_key | 0.978x | 19.810x | 12/12 | 14.305x |
| replay_checked_native | 1.006x | 1.168x | 12/12 | 1.085x |
| replay_trusted_native | 0.995x | 0.788x | 4/12 | 0.526x |
| san_parse_all | 0.983x | 1.845x | 36/36 | 1.428x |
| san_render_native | 1.003x | 2.576x | 36/36 | 1.891x |
| snapshot | 0.998x | 1.084x | 20/36 | 0.850x |
| startpos | 1.012x | 1.205x | 5/6 | 0.998x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.526x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.585x |
| ubuntu-24.04-native | replay_trusted_native | lichess300 | shakmaty | 0.664x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.679x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.679x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.691x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.694x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.815x |
| ubuntu-24.04-native | corpus_count | seeded | ultra | 0.825x |
| macos-15-native | snapshot | tactical | ferrum | 0.850x |
| ubuntu-24.04-native | perft_bulk_d4 | promotion | ultra | 0.892x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.893x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.896x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.904x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.906x |
| macos-15-native | materialize | kiwipete | ultra | 0.912x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.915x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.917x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | ferrum | 0.933x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.935x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.938x |
| ubuntu-24.04-arm-native | snapshot | promotion | ferrum | 0.940x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.941x |
| macos-15-native | materialize | tactical | ultra | 0.957x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.961x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.966x |
| ubuntu-24.04-portable | polyglot_recompute | lichess300 | shakmaty | 0.972x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.974x |
| macos-15-native | snapshot | middlegame | ferrum | 0.977x |
| windows-2025-native | polyglot_recompute | lichess300 | shakmaty | 0.988x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.988x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.990x |
| ubuntu-24.04-native | perft_bulk_d4 | kiwipete | ultra | 0.991x |
| macos-15-native | apply_children_full | kiwipete | chess | 0.993x |
| macos-15-native | snapshot | promotion | ferrum | 0.996x |
| windows-2025-native | snapshot | start | ferrum | 0.997x |
| ubuntu-24.04-pext | startpos | start | shakmaty | 0.998x |
| ubuntu-24.04-portable | snapshot | start | ferrum | 0.999x |
| macos-15-native | snapshot | kiwipete | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | endgame | ferrum | 0.999x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.999x |
| macos-15-native | materialize | middlegame | ultra | 1.000x |
| ubuntu-24.04-portable | snapshot | middlegame | ferrum | 1.000x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 3333 | shakmaty | 5666 | 1.700x |
| ubuntu-24.04-arm-native | 5696 | shakmaty | 9736 | 1.709x |
| ubuntu-24.04-native | 5320 | shakmaty | 5914 | 1.112x |
| ubuntu-24.04-pext | 5253 | shakmaty | 6700 | 1.275x |
| ubuntu-24.04-portable | 6857 | shakmaty | 7402 | 1.079x |
| windows-2025-native | 14100 | shakmaty | 18900 | 1.340x |

Cold wins: 6/6 (separate from native cases).
