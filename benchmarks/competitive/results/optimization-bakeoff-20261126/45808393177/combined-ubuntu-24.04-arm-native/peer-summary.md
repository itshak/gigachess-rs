| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.93x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.83x | 1.45x | 1.25x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.42x | 1.39x | 1.27x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 1.01x | 1.19x | 1.29x | 1.07x | 1.58x | 1.50x | 29/30 |
| fen_parse | 0.99x | 1.25x | 1.34x | 1.26x | 1.76x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 4.46x | 9.27x | 1.56x | 29.93x | 2.37x | 30/30 |
| snapshot | 0.92x | 1.10x | 1.03x | 1.45x | 1.00x | 0.95x | 20/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.76x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.16x | 1.40x | 1.65x | 4.50x | 1.35x | 1.35x | 30/30 |
| make_unmake_full | 1.11x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.31x | 76.62x | 132.31x | 176.19x | 143.31x | 5/5 |
| chess960_materialize | 1.03x | 1.56x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.67x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 18.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 3.95x | 1.51x | 1.23x | 1.77x | 2.08x | 10/10 |
| corpus_materialize | 0.97x | 1.53x | 1.46x | 1.17x | 1.83x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.84x | 1.47x | 10/10 |
| replay_trusted_native | 1.22x | 0.64x | 1.67x | 2.03x | 2.28x | 2.34x | 8/10 |
| replay_checked_native | 0.99x | 2.00x | 0.97x | 6.55x | 5.85x | 7.77x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
