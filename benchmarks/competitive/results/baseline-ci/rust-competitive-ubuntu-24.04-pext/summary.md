| Operation | shakmaty | cozy | ultra | chess | ferrum |
|---|---:|---:|---:|---:|---:|
| count | 4.23x (6) | 1.44x (6) | 1.50x (6) | 2.37x (6) | 1.71x (6) |
| materialize | 2.04x (6) | 1.71x (6) | 1.14x (6) | 2.12x (6) | 1.40x (6) |
| perft_bulk_d4 | 3.12x (6) | 1.23x (6) | 1.03x (6) | 1.95x (6) | 1.62x (6) |
| perft_full_d3 | 0.78x (6) | 0.83x (6) | 0.61x (6) | 1.15x (6) | 0.92x (6) |
| fen_parse | 1.30x (6) | 1.43x (6) | 1.19x (6) | 1.79x (6) | 1.25x (6) |
| fen_write_native_ep | 2.75x (6) | 6.54x (6) | 1.22x (6) | 18.55x (6) | 1.43x (6) |
| snapshot | 1.25x (6) | 1.04x (6) | 2.37x (6) | 1.03x (6) | 0.86x (6) |
| san_parse_all | 1.05x (6) | N/A | 1.81x (6) | N/A | N/A |
| san_render_native | 5.54x (6) | N/A | 1.08x (6) | N/A | N/A |
| apply_children_full | 1.04x (6) | 0.69x (6) | 2.57x (6) | 0.76x (6) | 0.86x (6) |
| make_unmake_full | N/A | N/A | 0.84x (6) | N/A | N/A |
| startpos | 0.01x (1) | 0.74x (1) | 1.13x (1) | 1.62x (1) | 1.19x (1) |
| corpus_count | 4.13x (1) | 1.53x (1) | 1.54x (1) | 2.60x (1) | 1.80x (1) |
| corpus_materialize | 2.14x (1) | 2.09x (1) | 1.29x (1) | 4.05x (1) | 1.76x (1) |
| corpus_uci_sorted | 0.91x (1) | 1.23x (1) | 0.92x (1) | 1.17x (1) | 0.66x (1) |
| replay_trusted_native | 0.57x (1) | 0.77x (1) | 1.05x (1) | 1.85x (1) | 2.33x (1) |
| replay_checked_native | 1.93x (1) | 0.65x (1) | 3.28x (1) | 4.18x (1) | 4.73x (1) |

Higher is better; parentheses show paired cases. Unsupported operations are N/A.
