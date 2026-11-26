| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.49x | 1.63x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.84x | 1.44x | 1.26x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.41x | 1.39x | 1.26x | 1.78x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.19x | 1.30x | 1.07x | 1.58x | 1.50x | 29/30 |
| fen_parse | 1.01x | 1.27x | 1.36x | 1.33x | 1.79x | 1.45x | 30/30 |
| fen_write_native_ep | 1.00x | 4.49x | 9.43x | 1.59x | 30.15x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.05x | 1.06x | 1.44x | 1.10x | 0.96x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 12.44x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.28x | 1.52x | 4.12x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.47x | 129.24x | 169.52x | 137.53x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.69x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.96x | 1.52x | 1.22x | 1.84x | 1.95x | 10/10 |
| corpus_materialize | 1.00x | 1.53x | 1.45x | 1.16x | 1.83x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.54x | 1.42x | 1.72x | 1.94x | 1.97x | 8/10 |
| replay_checked_native | 0.99x | 2.08x | 1.03x | 6.76x | 6.11x | 7.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
