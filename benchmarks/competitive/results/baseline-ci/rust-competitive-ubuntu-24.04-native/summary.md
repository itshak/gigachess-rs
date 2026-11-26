| Operation | shakmaty | cozy | ultra | chess | ferrum |
|---|---:|---:|---:|---:|---:|
| count | 3.39x (6) | 1.44x (6) | 1.23x (6) | 1.92x (6) | 1.49x (6) |
| materialize | 1.94x (6) | 1.51x (6) | 1.53x (6) | 2.31x (6) | 1.74x (6) |
| perft_bulk_d4 | 2.77x (6) | 1.18x (6) | 0.89x (6) | 1.77x (6) | 1.40x (6) |
| perft_full_d3 | 0.61x (6) | 0.73x (6) | 0.65x (6) | 0.86x (6) | 0.73x (6) |
| fen_parse | 1.30x (6) | 1.48x (6) | 1.14x (6) | 1.51x (6) | 1.09x (6) |
| fen_write_native_ep | 2.87x (6) | 6.58x (6) | 1.28x (6) | 20.28x (6) | 1.75x (6) |
| snapshot | 1.20x (6) | 0.60x (6) | 2.79x (6) | 0.60x (6) | 0.80x (6) |
| san_parse_all | 0.94x (6) | N/A | 2.08x (6) | N/A | N/A |
| san_render_native | 4.56x (6) | N/A | 1.08x (6) | N/A | N/A |
| apply_children_full | 1.06x (6) | 0.66x (6) | 1.99x (6) | 0.68x (6) | 0.82x (6) |
| make_unmake_full | N/A | N/A | 0.76x (6) | N/A | N/A |
| startpos | 0.01x (1) | 0.54x (1) | 0.95x (1) | 1.28x (1) | 0.98x (1) |
| corpus_count | 7.57x (1) | 2.28x (1) | 1.20x (1) | 3.00x (1) | 2.79x (1) |
| corpus_materialize | 1.46x (1) | 1.38x (1) | 1.08x (1) | 1.70x (1) | 1.22x (1) |
| corpus_uci_sorted | 0.89x (1) | 1.19x (1) | 0.87x (1) | 1.17x (1) | 0.62x (1) |
| replay_trusted_native | 0.44x (1) | 0.69x (1) | 0.88x (1) | 1.20x (1) | 1.53x (1) |
| replay_checked_native | 1.27x (1) | 0.58x (1) | 6.73x (1) | 6.43x (1) | 7.76x (1) |

Higher is better; parentheses show paired cases. Unsupported operations are N/A.
