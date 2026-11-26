| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.72x | 1.88x | 1.63x | 2.47x | 1.90x | 30/30 |
| materialize | 0.93x | 2.12x | 1.64x | 1.74x | 2.59x | 1.83x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.90x | 1.54x | 1.21x | 2.37x | 1.88x | 29/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.51x | 1.52x | 1.97x | 1.61x | 30/30 |
| fen_parse | 0.94x | 1.34x | 1.31x | 1.09x | 1.50x | 1.07x | 30/30 |
| fen_write_native_ep | 0.96x | 4.89x | 10.32x | 2.06x | 29.49x | 2.43x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.10x | 1.93x | N/A | 4.48x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.16x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.03x | 1.28x | 3.91x | 1.21x | 1.65x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.11x | 1.25x | 101.02x | 185.18x | 233.48x | 176.80x | 5/5 |
| chess960_materialize | 1.02x | 1.76x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.50x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 19.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.09x | 1.88x | 1.07x | 2.31x | 2.09x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.32x | 1.10x | 1.73x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.29x | 3.15x | 2.28x | 3.03x | 1.58x | 10/10 |
| replay_trusted_native | 1.01x | 0.92x | 1.58x | 1.62x | 2.28x | 2.62x | 9/10 |
| replay_checked_native | 1.02x | 2.05x | 1.06x | 8.71x | 8.26x | 9.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
