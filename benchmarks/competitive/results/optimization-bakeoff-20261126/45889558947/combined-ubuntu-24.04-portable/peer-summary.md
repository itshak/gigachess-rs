| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.11x | 1.62x | 1.39x | 1.98x | 1.38x | 30/30 |
| materialize | 1.00x | 2.33x | 1.80x | 1.34x | 2.70x | 1.66x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.16x | 1.63x | 1.24x | 2.04x | 1.83x | 30/30 |
| perft_full_d3 | 1.01x | 1.43x | 1.79x | 1.34x | 1.99x | 1.81x | 30/30 |
| fen_parse | 1.02x | 1.49x | 1.45x | 1.16x | 1.77x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 2.67x | 8.40x | 1.49x | 25.71x | 2.00x | 30/30 |
| snapshot | 0.99x | 1.62x | 1.16x | 2.65x | 1.15x | 1.00x | 26/30 |
| san_parse_all | 0.98x | 1.77x | N/A | 4.03x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.62x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.25x | 1.50x | 4.93x | 1.32x | 1.37x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.29x | 59.42x | 82.09x | 118.88x | 95.27x | 5/5 |
| chess960_materialize | 0.95x | 1.80x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.30x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 31.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.24x | 1.78x | 1.08x | 1.94x | 2.77x | 9/10 |
| corpus_materialize | 0.96x | 1.73x | 1.59x | 1.19x | 2.28x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.03x | 2.85x | 2.31x | 2.86x | 1.68x | 10/10 |
| replay_trusted_native | 0.96x | 0.72x | 1.58x | 1.65x | 2.18x | 2.03x | 8/10 |
| replay_checked_native | 1.04x | 2.28x | 1.24x | 8.67x | 8.54x | 10.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
