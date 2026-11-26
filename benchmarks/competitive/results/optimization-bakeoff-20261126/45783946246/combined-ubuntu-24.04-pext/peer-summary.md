| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.53x | 1.52x | 1.50x | 2.23x | 1.81x | 30/30 |
| materialize | 0.94x | 1.92x | 1.52x | 1.10x | 2.16x | 1.37x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.13x | 1.57x | 1.25x | 2.41x | 2.18x | 30/30 |
| perft_full_d3 | 0.93x | 1.01x | 1.18x | 0.89x | 1.53x | 1.45x | 20/30 |
| fen_parse | 0.91x | 1.16x | 1.30x | 1.00x | 1.71x | 1.01x | 24/30 |
| fen_write_native_ep | 1.00x | 3.87x | 8.59x | 1.49x | 25.52x | 2.09x | 30/30 |
| snapshot | 1.01x | 1.95x | 1.13x | 3.37x | 1.41x | 1.12x | 30/30 |
| san_parse_all | 0.99x | 1.88x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.44x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.57x | 2.14x | 1.68x | 4.26x | 1.71x | 1.96x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 83.78x | 143.35x | 222.04x | 142.11x | 5/5 |
| chess960_materialize | 0.99x | 1.54x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.69x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 36.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 28.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.40x | 1.89x | 1.29x | 2.50x | 2.33x | 10/10 |
| corpus_materialize | 0.94x | 1.79x | 1.70x | 1.24x | 2.37x | 1.47x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.33x | 2.97x | 2.40x | 3.00x | 1.65x | 10/10 |
| replay_trusted_native | 0.99x | 0.65x | 1.20x | 1.41x | 2.11x | 2.69x | 8/10 |
| replay_checked_native | 0.99x | 1.88x | 0.88x | 6.75x | 6.94x | 8.52x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
