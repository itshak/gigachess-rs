| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.94x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.85x | 1.44x | 1.26x | 2.03x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.46x | 1.40x | 1.27x | 1.79x | 1.84x | 30/30 |
| perft_full_d3 | 1.01x | 1.19x | 1.29x | 1.07x | 1.58x | 1.49x | 29/30 |
| fen_parse | 1.01x | 1.26x | 1.35x | 1.28x | 1.78x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.46x | 9.37x | 1.57x | 29.94x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.09x | 1.00x | 1.45x | 0.99x | 0.95x | 16/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.68x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.39x | 1.64x | 4.45x | 1.33x | 1.34x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.52x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.10x | 129.24x | 171.56x | 140.33x | 5/5 |
| chess960_materialize | 0.98x | 1.56x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.66x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.18x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 3.96x | 1.51x | 1.20x | 1.76x | 1.97x | 10/10 |
| corpus_materialize | 1.01x | 1.53x | 1.44x | 1.15x | 1.80x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.62x | 1.95x | 2.18x | 2.25x | 8/10 |
| replay_checked_native | 1.00x | 2.17x | 1.06x | 7.04x | 6.31x | 8.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
