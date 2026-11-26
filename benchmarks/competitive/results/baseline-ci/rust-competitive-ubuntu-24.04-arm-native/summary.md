| Operation | shakmaty | cozy | ultra | chess | ferrum |
|---|---:|---:|---:|---:|---:|
| count | 3.05x (6) | 1.16x (6) | 1.25x (6) | 1.51x (6) | 1.50x (6) |
| materialize | 1.72x (6) | 1.42x (6) | 1.18x (6) | 1.89x (6) | 1.31x (6) |
| perft_bulk_d4 | 2.48x (6) | 1.04x (6) | 0.92x (6) | 1.30x (6) | 1.33x (6) |
| perft_full_d3 | 0.68x (6) | 0.80x (6) | 0.61x (6) | 0.89x (6) | 0.85x (6) |
| fen_parse | 1.18x (6) | 1.27x (6) | 1.25x (6) | 1.70x (6) | 1.35x (6) |
| fen_write_native_ep | 2.86x (6) | 6.16x (6) | 1.07x (6) | 20.06x (6) | 1.63x (6) |
| snapshot | 0.91x (6) | 0.92x (6) | 1.25x (6) | 0.93x (6) | 0.82x (6) |
| san_parse_all | 0.92x (6) | N/A | 2.15x (6) | N/A | N/A |
| san_render_native | 5.34x (6) | N/A | 1.14x (6) | N/A | N/A |
| apply_children_full | 0.71x (6) | 0.81x (6) | 2.29x (6) | 0.67x (6) | 0.69x (6) |
| make_unmake_full | N/A | N/A | 0.90x (6) | N/A | N/A |
| startpos | 0.01x (1) | 0.70x (1) | 1.19x (1) | 1.61x (1) | 1.28x (1) |
| corpus_count | 4.15x (1) | 1.46x (1) | 1.23x (1) | 1.77x (1) | 1.95x (1) |
| corpus_materialize | 1.48x (1) | 1.45x (1) | 1.12x (1) | 1.73x (1) | 1.24x (1) |
| corpus_uci_sorted | 0.83x (1) | 1.15x (1) | 0.82x (1) | 1.13x (1) | 0.58x (1) |
| replay_trusted_native | 0.25x (1) | 0.68x (1) | 0.90x (1) | 1.00x (1) | 1.03x (1) |
| replay_checked_native | 1.19x (1) | 0.58x (1) | 4.34x (1) | 3.88x (1) | 4.98x (1) |

Higher is better; parentheses show paired cases. Unsupported operations are N/A.
