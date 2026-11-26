| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.32x | 1.65x | 1.77x | 2.14x | 2.15x | 30/30 |
| materialize | 1.00x | 1.95x | 1.52x | 1.35x | 2.14x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.73x | 1.52x | 1.37x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 0.94x | 1.38x | 1.50x | 1.26x | 1.85x | 1.74x | 30/30 |
| fen_parse | 1.01x | 1.23x | 1.32x | 1.26x | 1.74x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.47x | 9.22x | 1.61x | 29.74x | 2.39x | 30/30 |
| snapshot | 1.09x | 1.27x | 0.92x | 1.61x | 0.92x | 1.00x | 13/30 |
| san_parse_all | 0.95x | 1.77x | N/A | 4.05x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 10.68x | N/A | 2.35x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.16x | 1.34x | 3.79x | 1.12x | 1.12x | 28/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 0.81x | 0.96x | 62.00x | 106.34x | 143.44x | 117.58x | 4/5 |
| chess960_materialize | 0.99x | 1.70x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.81x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 26.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 20.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.40x | 1.67x | 1.36x | 1.98x | 2.16x | 10/10 |
| corpus_materialize | 0.98x | 1.58x | 1.49x | 1.21x | 1.88x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 0.94x | 0.58x | 1.55x | 1.86x | 2.03x | 2.13x | 8/10 |
| replay_checked_native | 0.95x | 2.06x | 1.00x | 6.74x | 5.96x | 7.71x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
