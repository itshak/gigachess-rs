| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.17x | 1.43x | 1.45x | 2.04x | 1.90x | 30/30 |
| materialize | 0.99x | 1.95x | 1.40x | 1.22x | 2.26x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.85x | 1.49x | 1.33x | 2.15x | 1.99x | 30/30 |
| perft_full_d3 | 0.97x | 1.32x | 1.42x | 1.26x | 1.79x | 1.73x | 30/30 |
| fen_parse | 1.06x | 1.32x | 1.47x | 1.37x | 2.29x | 1.45x | 30/30 |
| fen_write_native_ep | 1.00x | 2.62x | 5.17x | 1.00x | 19.72x | 1.22x | 28/30 |
| snapshot | 0.59x | 1.32x | 1.14x | 1.93x | 1.17x | 0.85x | 24/30 |
| san_parse_all | 0.97x | 1.93x | N/A | 5.37x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 11.11x | N/A | 3.65x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.96x | 1.18x | 7.92x | 1.30x | 1.44x | 29/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.14x | 73.77x | 145.41x | 218.67x | 146.81x | 5/5 |
| chess960_materialize | 0.92x | 1.59x | 1.20x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.34x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 26.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 5.11x | 1.51x | 1.18x | 2.19x | 2.07x | 9/10 |
| corpus_materialize | 0.95x | 1.70x | 1.31x | 1.15x | 2.19x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.74x | 2.13x | 1.81x | 2.06x | 1.41x | 10/10 |
| replay_trusted_native | 0.97x | 0.63x | 1.37x | 1.72x | 2.11x | 2.46x | 8/10 |
| replay_checked_native | 0.95x | 2.43x | 0.95x | 7.43x | 7.30x | 9.23x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
