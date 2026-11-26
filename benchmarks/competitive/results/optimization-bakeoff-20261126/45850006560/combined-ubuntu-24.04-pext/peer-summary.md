| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.44x | 1.86x | 1.61x | 2.49x | 1.89x | 30/30 |
| materialize | 0.99x | 2.24x | 1.66x | 1.78x | 2.67x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.70x | 1.47x | 1.15x | 2.28x | 1.90x | 29/30 |
| perft_full_d3 | 0.94x | 1.27x | 1.43x | 1.43x | 1.83x | 1.50x | 30/30 |
| fen_parse | 0.89x | 1.30x | 1.24x | 1.05x | 1.40x | 1.00x | 24/30 |
| fen_write_native_ep | 0.99x | 5.06x | 10.73x | 2.04x | 30.90x | 2.49x | 30/30 |
| snapshot | 0.67x | 1.23x | 1.00x | 2.80x | 1.00x | 1.00x | 23/30 |
| san_parse_all | 1.00x | 1.75x | N/A | 4.04x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 8.98x | N/A | 2.00x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.98x | 1.27x | 3.66x | 1.31x | 1.56x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 0.69x | 0.91x | 61.17x | 110.28x | 138.50x | 106.81x | 4/5 |
| chess960_materialize | 0.88x | 1.61x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 2.30x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 40.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.78x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 5.29x | 1.96x | 1.12x | 2.44x | 2.23x | 10/10 |
| corpus_materialize | 0.98x | 1.37x | 1.28x | 1.08x | 1.69x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 3.10x | 2.26x | 3.02x | 1.56x | 10/10 |
| replay_trusted_native | 0.90x | 0.80x | 1.44x | 1.49x | 2.07x | 2.37x | 9/10 |
| replay_checked_native | 0.97x | 1.95x | 1.01x | 8.32x | 7.88x | 9.29x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
