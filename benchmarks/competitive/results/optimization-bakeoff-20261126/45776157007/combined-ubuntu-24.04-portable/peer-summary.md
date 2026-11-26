| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 2.69x | 1.39x | 1.24x | 1.74x | 1.24x | 27/30 |
| materialize | 1.00x | 1.97x | 1.75x | 1.21x | 2.32x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.79x | 1.50x | 1.17x | 1.89x | 1.63x | 30/30 |
| perft_full_d3 | 1.00x | 0.93x | 1.06x | 0.87x | 1.28x | 1.20x | 18/30 |
| fen_parse | 0.94x | 1.40x | 1.46x | 1.16x | 2.19x | 1.24x | 30/30 |
| fen_write_native_ep | 0.99x | 1.59x | 5.27x | 0.96x | 15.18x | 1.11x | 25/30 |
| snapshot | 1.00x | 1.72x | 1.05x | 3.05x | 1.01x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.44x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.55x | 1.07x | 3.17x | 0.96x | 1.00x | 21/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 0.97x | 1.29x | 63.88x | 106.71x | 181.45x | 110.37x | 5/5 |
| chess960_materialize | 0.98x | 1.44x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.91x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 23.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 18.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 3.61x | 1.82x | 1.20x | 1.98x | 2.40x | 10/10 |
| corpus_materialize | 0.98x | 1.61x | 1.62x | 1.18x | 2.08x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.81x | 2.24x | 3.29x | 2.50x | 3.17x | 1.85x | 10/10 |
| replay_trusted_native | 0.99x | 0.61x | 1.21x | 1.34x | 1.77x | 1.64x | 8/10 |
| replay_checked_native | 1.02x | 2.06x | 1.03x | 7.08x | 7.09x | 8.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
