| Operation | shakmaty | cozy | ultra | chess | ferrum |
|---|---:|---:|---:|---:|---:|
| count | 4.41x (6) | 1.45x (6) | 1.36x (6) | 2.06x (6) | 1.78x (6) |
| materialize | 1.88x (6) | 1.23x (6) | 1.11x (6) | 2.11x (6) | 1.43x (6) |
| perft_bulk_d4 | 3.30x (6) | 1.24x (6) | 1.05x (6) | 1.92x (6) | 1.72x (6) |
| perft_full_d3 | 0.66x (6) | 0.80x (6) | 0.60x (6) | 0.94x (6) | 0.97x (6) |
| fen_parse | 1.32x (6) | 1.50x (6) | 1.55x (6) | 2.15x (6) | 1.52x (6) |
| fen_write_native_ep | 3.19x (6) | 6.01x (6) | 1.26x (6) | 26.04x (6) | 1.73x (6) |
| snapshot | 1.04x (6) | 1.00x (6) | 0.36x (6) | 1.00x (6) | 1.00x (6) |
| san_parse_all | 0.91x (6) | N/A | 2.13x (6) | N/A | N/A |
| san_render_native | 5.19x (6) | N/A | 1.68x (6) | N/A | N/A |
| apply_children_full | 1.26x (6) | 0.96x (6) | 4.99x (6) | 0.81x (6) | 1.11x (6) |
| make_unmake_full | N/A | N/A | 0.92x (6) | N/A | N/A |
| startpos | 0.05x (1) | 0.77x (1) | 1.43x (1) | 1.92x (1) | 1.43x (1) |
| corpus_count | 6.96x (1) | 1.45x (1) | 1.34x (1) | 2.12x (1) | 2.03x (1) |
| corpus_materialize | 2.44x (1) | 1.78x (1) | 1.35x (1) | 3.33x (1) | 1.91x (1) |
| corpus_uci_sorted | 0.95x (1) | 1.12x (1) | 1.00x (1) | 1.12x (1) | 0.80x (1) |
| replay_trusted_native | 0.48x (1) | 0.71x (1) | 1.13x (1) | 1.57x (1) | 2.17x (1) |
| replay_checked_native | 1.55x (1) | 0.60x (1) | 5.85x (1) | 6.09x (1) | 7.63x (1) |

Higher is better; parentheses show paired cases. Unsupported operations are N/A.
