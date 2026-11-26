| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.93x | 1.50x | 1.62x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.84x | 1.44x | 1.26x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.45x | 1.40x | 1.28x | 1.78x | 1.82x | 30/30 |
| perft_full_d3 | 0.99x | 1.19x | 1.29x | 1.06x | 1.57x | 1.48x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.32x | 1.29x | 1.74x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.43x | 1.63x | 29.98x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.11x | 1.02x | 1.46x | 1.00x | 0.95x | 17/30 |
| san_parse_all | 1.01x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.72x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.28x | 1.51x | 4.12x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.46x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.51x | 129.56x | 168.81x | 135.70x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.71x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 25.99x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 18.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.93x | 1.51x | 1.21x | 1.76x | 1.94x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.44x | 1.15x | 1.80x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.89x | 2.07x | 2.83x | 1.45x | 10/10 |
| replay_trusted_native | 0.99x | 0.55x | 1.42x | 1.71x | 1.94x | 1.96x | 8/10 |
| replay_checked_native | 1.00x | 2.10x | 1.03x | 6.88x | 6.12x | 7.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
