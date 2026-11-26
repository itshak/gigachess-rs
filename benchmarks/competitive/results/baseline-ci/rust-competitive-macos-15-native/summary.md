| Operation | shakmaty | cozy | ultra | chess | ferrum |
|---|---:|---:|---:|---:|---:|
| count | 3.47x (6) | 1.20x (6) | 1.21x (6) | 1.52x (6) | 1.41x (6) |
| materialize | 1.69x (6) | 1.33x (6) | 0.95x (6) | 1.94x (6) | 1.15x (6) |
| perft_bulk_d4 | 3.14x (6) | 1.18x (6) | 0.84x (6) | 1.37x (6) | 1.29x (6) |
| perft_full_d3 | 0.66x (6) | 0.75x (6) | 0.55x (6) | 0.84x (6) | 0.89x (6) |
| fen_parse | 1.15x (6) | 1.15x (6) | 1.18x (6) | 1.42x (6) | 1.33x (6) |
| fen_write_native_ep | 2.88x (6) | 6.57x (6) | 1.34x (6) | 22.13x (6) | 1.72x (6) |
| snapshot | 1.28x (6) | 0.72x (6) | 1.90x (6) | 0.83x (6) | 0.70x (6) |
| san_parse_all | 1.02x (6) | N/A | 2.04x (6) | N/A | N/A |
| san_render_native | 6.18x (6) | N/A | 1.48x (6) | N/A | N/A |
| apply_children_full | 0.68x (6) | 0.69x (6) | 3.31x (6) | 0.65x (6) | 0.75x (6) |
| make_unmake_full | N/A | N/A | 0.88x (6) | N/A | N/A |
| startpos | 0.01x (1) | 0.59x (1) | 1.21x (1) | 1.39x (1) | 2.12x (1) |
| corpus_count | 3.02x (1) | 0.78x (1) | 0.71x (1) | 1.14x (1) | 1.55x (1) |
| corpus_materialize | 1.45x (1) | 1.35x (1) | 0.85x (1) | 1.73x (1) | 0.84x (1) |
| corpus_uci_sorted | 0.78x (1) | 1.10x (1) | 0.80x (1) | 1.00x (1) | 0.59x (1) |
| replay_trusted_native | 0.36x (1) | 0.75x (1) | 0.80x (1) | 0.77x (1) | 0.94x (1) |
| replay_checked_native | 1.25x (1) | 0.54x (1) | 5.43x (1) | 5.33x (1) | 5.93x (1) |

Higher is better; parentheses show paired cases. Unsupported operations are N/A.
