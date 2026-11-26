| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.94x | 1.49x | 1.63x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.84x | 1.48x | 1.26x | 2.04x | 1.41x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.44x | 1.40x | 1.27x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.19x | 1.29x | 1.07x | 1.58x | 1.50x | 29/30 |
| fen_parse | 1.00x | 1.27x | 1.35x | 1.29x | 1.79x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.37x | 1.58x | 29.67x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.10x | 1.01x | 1.45x | 1.02x | 0.95x | 19/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.71x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.28x | 1.51x | 4.15x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.28x | 129.51x | 172.29x | 140.48x | 5/5 |
| chess960_materialize | 1.00x | 1.61x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.67x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.94x | 1.52x | 1.23x | 1.77x | 1.96x | 10/10 |
| corpus_materialize | 1.00x | 1.53x | 1.45x | 1.16x | 1.81x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.90x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 0.99x | 0.54x | 1.42x | 1.74x | 1.96x | 1.97x | 8/10 |
| replay_checked_native | 1.01x | 2.12x | 1.03x | 6.86x | 6.18x | 8.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
