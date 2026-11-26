| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.76x | 1.68x | 1.63x | 2.53x | 2.03x | 30/30 |
| materialize | 1.00x | 1.95x | 1.65x | 1.25x | 2.28x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.78x | 1.49x | 1.32x | 2.25x | 1.88x | 30/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.51x | 1.32x | 1.94x | 1.89x | 30/30 |
| fen_parse | 1.11x | 1.40x | 1.45x | 1.21x | 2.24x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.04x | 9.31x | 1.70x | 27.37x | 2.19x | 30/30 |
| snapshot | 0.78x | 1.56x | 0.97x | 2.54x | 1.18x | 0.97x | 18/30 |
| san_parse_all | 0.99x | 2.00x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.15x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 2.21x | 1.26x | 4.42x | 1.40x | 1.53x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.43x | 148.82x | 228.86x | 159.24x | 5/5 |
| chess960_materialize | 1.03x | 1.62x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.74x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.89x | 25.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 19.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.37x | 1.77x | 1.36x | 2.61x | 2.36x | 10/10 |
| corpus_materialize | 1.01x | 1.59x | 1.53x | 1.17x | 2.01x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.37x | 3.16x | 2.40x | 3.00x | 1.76x | 10/10 |
| replay_trusted_native | 0.94x | 0.85x | 1.50x | 1.88x | 2.46x | 2.85x | 8/10 |
| replay_checked_native | 0.99x | 2.47x | 1.06x | 8.29x | 8.34x | 9.86x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
