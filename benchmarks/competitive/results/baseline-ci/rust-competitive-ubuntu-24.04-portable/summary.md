| Operation | shakmaty | cozy | ultra | chess | ferrum |
|---|---:|---:|---:|---:|---:|
| count | 2.68x (6) | 1.35x (6) | 1.18x (6) | 1.65x (6) | 1.21x (6) |
| materialize | 2.04x (6) | 1.65x (6) | 1.16x (6) | 2.33x (6) | 1.46x (6) |
| perft_bulk_d4 | 2.44x (6) | 1.29x (6) | 1.00x (6) | 1.55x (6) | 1.44x (6) |
| perft_full_d3 | 0.65x (6) | 0.85x (6) | 0.62x (6) | 0.96x (6) | 0.83x (6) |
| fen_parse | 1.36x (6) | 1.28x (6) | 1.06x (6) | 1.63x (6) | 1.10x (6) |
| fen_write_native_ep | 1.96x (6) | 6.64x (6) | 1.26x (6) | 20.49x (6) | 1.58x (6) |
| snapshot | 0.16x (6) | 0.11x (6) | 0.24x (6) | 0.10x (6) | 0.09x (6) |
| san_parse_all | 0.79x (6) | N/A | 1.64x (6) | N/A | N/A |
| san_render_native | 5.24x (6) | N/A | 1.13x (6) | N/A | N/A |
| apply_children_full | 0.81x (6) | 0.55x (6) | 1.65x (6) | 0.46x (6) | 0.50x (6) |
| make_unmake_full | N/A | N/A | 0.88x (6) | N/A | N/A |
| startpos | 0.02x (1) | 0.76x (1) | 1.01x (1) | 1.50x (1) | 1.09x (1) |
| corpus_count | 4.40x (1) | 1.47x (1) | 1.17x (1) | 1.93x (1) | 2.56x (1) |
| corpus_materialize | 2.40x (1) | 2.12x (1) | 1.48x (1) | 3.25x (1) | 1.89x (1) |
| corpus_uci_sorted | 0.86x (1) | 1.21x (1) | 0.97x (1) | 1.22x (1) | 0.67x (1) |
| replay_trusted_native | 0.43x (1) | 0.77x (1) | 1.04x (1) | 1.22x (1) | 1.22x (1) |
| replay_checked_native | 1.41x (1) | 0.68x (1) | 5.73x (1) | 5.90x (1) | 6.85x (1) |

Higher is better; parentheses show paired cases. Unsupported operations are N/A.
