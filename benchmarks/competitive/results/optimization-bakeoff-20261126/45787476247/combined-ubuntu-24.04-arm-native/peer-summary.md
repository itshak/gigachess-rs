| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.83x | 1.47x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 1.01x | 1.82x | 1.42x | 1.26x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.52x | 1.39x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.27x | 1.07x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.26x | 1.35x | 1.29x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.62x | 9.32x | 1.59x | 29.58x | 2.39x | 30/30 |
| snapshot | 1.11x | 1.25x | 0.93x | 1.59x | 0.89x | 1.01x | 18/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.78x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.11x | 1.28x | 3.65x | 1.06x | 1.07x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.08x | 1.18x | 76.62x | 131.66x | 170.79x | 137.81x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.63x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 24.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.78x | 1.84x | 1.49x | 2.16x | 2.38x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.50x | 1.23x | 1.87x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.09x | 2.79x | 1.46x | 10/10 |
| replay_trusted_native | 0.98x | 0.49x | 1.29x | 1.59x | 1.73x | 1.79x | 8/10 |
| replay_checked_native | 1.00x | 2.06x | 0.99x | 6.72x | 5.97x | 7.73x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
