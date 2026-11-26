| Platform | Wins | >=1.03x | >=2x | Total |
|---|---:|---:|---:|---:|
| macos-15-native | 74 | 71 | 12 | 87 |
| ubuntu-24.04-arm-native | 76 | 76 | 10 | 87 |
| ubuntu-24.04-native | 77 | 75 | 13 | 87 |
| ubuntu-24.04-pext | 78 | 74 | 9 | 87 |
| ubuntu-24.04-portable | 80 | 74 | 11 | 87 |
| windows-2025-native | 77 | 73 | 11 | 87 |
| ALL | 462 | 443 | 66 | 522 |

| Operation | vs paired baseline | vs fastest peer | Wins | Worst case |
|---|---:|---:|---:|---:|
| apply_children_full | 1.129x | 1.270x | 36/36 | 1.042x |
| chess960_materialize | 1.037x | 1.371x | 12/12 | 1.213x |
| chess960_perft_d3 | 0.997x | 1.363x | 12/12 | 1.089x |
| corpus_count | 0.826x | 1.082x | 6/12 | 0.857x |
| corpus_materialize | 0.953x | 1.122x | 12/12 | 1.027x |
| corpus_uci_sorted | 0.976x | 1.487x | 12/12 | 1.258x |
| count | 1.021x | 1.314x | 33/36 | 0.932x |
| fen_parse | 1.010x | 1.162x | 34/36 | 0.960x |
| fen_write_native_ep | 1.004x | 1.761x | 36/36 | 1.417x |
| make_unmake_full | 1.053x | 1.304x | 36/36 | 1.001x |
| materialize | 1.066x | 1.314x | 33/36 | 0.914x |
| perft_bulk_d4 | 0.907x | 1.141x | 29/36 | 0.877x |
| perft_full_d3 | 0.998x | 1.239x | 31/36 | 0.874x |
| polyglot_key | 1.041x | 30.768x | 12/12 | 16.512x |
| polyglot_recompute | 1.008x | 1.012x | 7/12 | 0.885x |
| repetition_key | 0.936x | 18.053x | 12/12 | 14.126x |
| replay_checked_native | 0.998x | 1.008x | 6/12 | 0.921x |
| replay_trusted_native | 1.127x | 0.787x | 2/12 | 0.543x |
| san_parse_all | 0.980x | 1.915x | 36/36 | 1.416x |
| san_render_native | 1.012x | 2.478x | 36/36 | 1.691x |
| snapshot | 1.181x | 1.058x | 23/36 | 0.684x |
| startpos | 1.074x | 1.266x | 6/6 | 1.199x |

| Platform | Operation | Case | Fastest peer | Ratio |
|---|---|---|---|---:|
| macos-15-native | replay_trusted_native | lichess300 | shakmaty | 0.543x |
| ubuntu-24.04-arm-native | replay_trusted_native | lichess300 | shakmaty | 0.585x |
| windows-2025-native | replay_trusted_native | lichess300 | shakmaty | 0.645x |
| ubuntu-24.04-pext | replay_trusted_native | lichess300 | shakmaty | 0.654x |
| windows-2025-native | snapshot | middlegame | cozy | 0.684x |
| ubuntu-24.04-arm-native | replay_trusted_native | seeded | shakmaty | 0.708x |
| ubuntu-24.04-portable | replay_trusted_native | lichess300 | shakmaty | 0.709x |
| windows-2025-native | snapshot | kiwipete | cozy | 0.765x |
| macos-15-native | corpus_count | seeded | ultra | 0.857x |
| ubuntu-24.04-native | perft_full_d3 | promotion | ultra | 0.874x |
| windows-2025-native | replay_trusted_native | seeded | shakmaty | 0.877x |
| ubuntu-24.04-pext | perft_bulk_d4 | promotion | ultra | 0.877x |
| ubuntu-24.04-native | polyglot_recompute | lichess300 | shakmaty | 0.885x |
| ubuntu-24.04-native | perft_full_d3 | kiwipete | ultra | 0.886x |
| ubuntu-24.04-portable | perft_bulk_d4 | promotion | ultra | 0.886x |
| macos-15-native | snapshot | promotion | ferrum | 0.901x |
| macos-15-native | perft_bulk_d4 | promotion | ultra | 0.907x |
| macos-15-native | materialize | tactical | ultra | 0.914x |
| macos-15-native | replay_trusted_native | seeded | shakmaty | 0.915x |
| windows-2025-native | perft_bulk_d4 | promotion | ultra | 0.917x |
| ubuntu-24.04-pext | replay_checked_native | seeded | cozy | 0.921x |
| ubuntu-24.04-portable | replay_trusted_native | seeded | shakmaty | 0.923x |
| ubuntu-24.04-native | corpus_count | lichess300 | ultra | 0.928x |
| macos-15-native | replay_checked_native | lichess300 | cozy | 0.931x |
| macos-15-native | count | promotion | cozy | 0.932x |
| ubuntu-24.04-arm-native | perft_full_d3 | promotion | ultra | 0.934x |
| ubuntu-24.04-arm-native | snapshot | kiwipete | chess | 0.937x |
| ubuntu-24.04-arm-native | snapshot | promotion | chess | 0.937x |
| ubuntu-24.04-native | perft_full_d3 | middlegame | ultra | 0.937x |
| macos-15-native | snapshot | middlegame | ferrum | 0.940x |
| ubuntu-24.04-arm-native | snapshot | endgame | ferrum | 0.947x |
| ubuntu-24.04-arm-native | snapshot | middlegame | ferrum | 0.948x |
| ubuntu-24.04-arm-native | snapshot | start | ferrum | 0.953x |
| ubuntu-24.04-pext | corpus_count | lichess300 | ultra | 0.953x |
| ubuntu-24.04-arm-native | snapshot | tactical | ferrum | 0.956x |
| ubuntu-24.04-pext | perft_bulk_d4 | start | ultra | 0.959x |
| ubuntu-24.04-native | fen_parse | start | ultra | 0.960x |
| ubuntu-24.04-portable | count | promotion | ferrum | 0.961x |
| windows-2025-native | snapshot | tactical | cozy | 0.963x |
| ubuntu-24.04-portable | polyglot_recompute | seeded | shakmaty | 0.963x |
| ubuntu-24.04-arm-native | replay_checked_native | lichess300 | cozy | 0.964x |
| windows-2025-native | corpus_count | lichess300 | ultra | 0.965x |
| ubuntu-24.04-native | fen_parse | kiwipete | ultra | 0.969x |
| ubuntu-24.04-pext | replay_checked_native | lichess300 | cozy | 0.970x |
| macos-15-native | perft_bulk_d4 | start | ultra | 0.971x |
| macos-15-native | materialize | middlegame | ultra | 0.974x |
| ubuntu-24.04-pext | corpus_count | seeded | ultra | 0.975x |
| ubuntu-24.04-pext | replay_trusted_native | seeded | shakmaty | 0.979x |
| ubuntu-24.04-native | replay_checked_native | seeded | cozy | 0.981x |
| ubuntu-24.04-arm-native | replay_checked_native | seeded | cozy | 0.983x |
| macos-15-native | corpus_count | lichess300 | ultra | 0.983x |
| ubuntu-24.04-native | perft_full_d3 | tactical | ultra | 0.984x |
| macos-15-native | polyglot_recompute | lichess300 | shakmaty | 0.990x |
| windows-2025-native | polyglot_recompute | seeded | shakmaty | 0.990x |
| windows-2025-native | count | middlegame | cozy | 0.991x |
| ubuntu-24.04-pext | polyglot_recompute | lichess300 | shakmaty | 0.992x |
| ubuntu-24.04-portable | perft_bulk_d4 | kiwipete | ultra | 0.996x |
| ubuntu-24.04-native | materialize | kiwipete | ultra | 0.996x |
| ubuntu-24.04-portable | snapshot | kiwipete | ferrum | 0.997x |
| windows-2025-native | snapshot | promotion | cozy | 0.998x |

| Platform | Giga median (ns) | Fastest peer | Peer median (ns) | Ratio |
|---|---:|---|---:|---:|
| macos-15-native | 5542 | shakmaty | 8500 | 1.534x |
| ubuntu-24.04-arm-native | 5096 | shakmaty | 8737 | 1.714x |
| ubuntu-24.04-native | 6139 | shakmaty | 6780 | 1.104x |
| ubuntu-24.04-pext | 5280 | shakmaty | 8121 | 1.538x |
| ubuntu-24.04-portable | 5435 | shakmaty | 5696 | 1.048x |
| windows-2025-native | 19800 | shakmaty | 29400 | 1.485x |

Cold wins: 6/6 (separate from native cases).
