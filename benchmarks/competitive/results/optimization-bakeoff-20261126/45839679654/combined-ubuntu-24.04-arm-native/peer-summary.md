| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.28x | 1.63x | 1.78x | 2.12x | 2.11x | 30/30 |
| materialize | 0.99x | 1.93x | 1.51x | 1.34x | 2.13x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.70x | 1.51x | 1.37x | 1.93x | 1.98x | 30/30 |
| perft_full_d3 | 1.03x | 0.95x | 1.04x | 0.86x | 1.27x | 1.20x | 19/30 |
| fen_parse | 1.03x | 1.25x | 1.35x | 1.29x | 1.77x | 1.43x | 30/30 |
| fen_write_native_ep | 0.99x | 4.54x | 9.32x | 1.57x | 29.83x | 2.37x | 30/30 |
| snapshot | 0.99x | 1.04x | 1.08x | 1.45x | 1.10x | 0.96x | 23/30 |
| san_parse_all | 0.92x | 1.72x | N/A | 3.98x | N/A | N/A | 12/12 |
| san_render_native | 0.89x | 10.39x | N/A | 2.24x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.32x | 1.57x | 4.26x | 1.27x | 1.27x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.22x | 70.92x | 117.86x | 156.92x | 126.91x | 5/5 |
| chess960_materialize | 0.98x | 1.68x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.80x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.31x | 1.67x | 1.39x | 1.95x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.50x | 1.23x | 1.88x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 0.97x | 0.59x | 1.57x | 1.91x | 2.13x | 2.18x | 8/10 |
| replay_checked_native | 0.96x | 2.07x | 1.01x | 6.91x | 6.04x | 7.81x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
