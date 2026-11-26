| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.91x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.83x | 1.45x | 1.26x | 2.03x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.44x | 1.41x | 1.26x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.19x | 1.29x | 1.08x | 1.59x | 1.49x | 29/30 |
| fen_parse | 1.00x | 1.27x | 1.34x | 1.28x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.43x | 9.23x | 1.58x | 29.62x | 2.43x | 30/30 |
| snapshot | 1.01x | 1.15x | 1.02x | 1.47x | 1.00x | 0.94x | 20/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.69x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.27x | 1.49x | 4.14x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.47x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.29x | 128.80x | 171.29x | 137.18x | 5/5 |
| chess960_materialize | 1.00x | 1.58x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.64x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 3.94x | 1.57x | 1.24x | 1.90x | 1.98x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.47x | 1.17x | 1.87x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.09x | 2.91x | 2.06x | 2.85x | 1.47x | 10/10 |
| replay_trusted_native | 1.00x | 0.55x | 1.43x | 1.75x | 1.96x | 1.98x | 8/10 |
| replay_checked_native | 1.00x | 2.09x | 1.02x | 6.84x | 6.28x | 7.91x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
