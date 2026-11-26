| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.95x | 1.70x | 1.71x | 2.63x | 1.99x | 30/30 |
| materialize | 1.01x | 1.99x | 1.67x | 1.27x | 2.33x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.20x | 1.59x | 1.46x | 2.47x | 1.92x | 30/30 |
| perft_full_d3 | 1.00x | 1.50x | 1.52x | 1.33x | 1.97x | 1.79x | 30/30 |
| fen_parse | 0.98x | 1.35x | 1.46x | 1.16x | 2.18x | 1.28x | 30/30 |
| fen_write_native_ep | 0.97x | 3.88x | 9.31x | 1.69x | 28.37x | 2.16x | 30/30 |
| snapshot | 1.00x | 1.97x | 1.25x | 3.61x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.04x | 2.13x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.56x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.33x | 1.33x | 4.69x | 1.49x | 1.64x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 77.87x | 147.73x | 239.43x | 156.77x | 5/5 |
| chess960_materialize | 1.01x | 1.63x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.86x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 25.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 19.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.66x | 1.85x | 1.42x | 2.77x | 2.32x | 10/10 |
| corpus_materialize | 0.97x | 1.62x | 1.52x | 1.17x | 2.11x | 1.33x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.44x | 3.24x | 2.48x | 3.13x | 1.75x | 10/10 |
| replay_trusted_native | 0.99x | 0.86x | 1.52x | 1.88x | 2.51x | 2.73x | 9/10 |
| replay_checked_native | 1.02x | 2.31x | 1.02x | 7.97x | 7.80x | 8.96x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
