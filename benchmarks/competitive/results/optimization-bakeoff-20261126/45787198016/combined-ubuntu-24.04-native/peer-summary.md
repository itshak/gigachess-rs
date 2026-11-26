| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.97x | 1.71x | 1.43x | 2.32x | 1.74x | 30/30 |
| materialize | 1.00x | 2.11x | 1.59x | 1.64x | 2.65x | 1.84x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.80x | 1.56x | 1.18x | 2.44x | 1.86x | 30/30 |
| perft_full_d3 | 0.90x | 1.35x | 1.58x | 1.47x | 1.89x | 1.89x | 30/30 |
| fen_parse | 0.88x | 1.23x | 1.37x | 0.93x | 1.28x | 1.10x | 25/30 |
| fen_write_native_ep | 0.99x | 4.61x | 10.22x | 1.92x | 29.30x | 2.65x | 30/30 |
| snapshot | 0.97x | 2.25x | 1.75x | 3.00x | 1.75x | 1.50x | 30/30 |
| san_parse_all | 0.96x | 1.90x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.81x | 7.26x | N/A | 1.69x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.59x | 1.01x | 3.15x | 0.94x | 1.30x | 22/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.03x | N/A | N/A | 4/6 |
| startpos | 0.91x | 1.33x | 65.95x | 113.43x | 144.86x | 130.47x | 5/5 |
| chess960_materialize | 0.95x | 1.60x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.22x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.92x | 20.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.55x | 2.45x | 1.33x | 2.98x | 2.69x | 10/10 |
| corpus_materialize | 1.01x | 1.44x | 1.34x | 1.10x | 1.77x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.24x | 2.95x | 2.28x | 2.88x | 1.57x | 10/10 |
| replay_trusted_native | 0.78x | 0.57x | 1.00x | 0.96x | 1.37x | 1.61x | 6/10 |
| replay_checked_native | 0.73x | 1.46x | 0.82x | 6.18x | 5.95x | 7.23x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
