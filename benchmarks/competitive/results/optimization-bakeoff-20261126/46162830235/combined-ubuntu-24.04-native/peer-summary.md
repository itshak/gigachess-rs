| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 5.03x | 1.82x | 1.66x | 2.45x | 2.11x | 30/30 |
| materialize | 0.99x | 2.09x | 1.67x | 1.18x | 2.38x | 1.57x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.44x | 1.82x | 1.32x | 2.51x | 2.38x | 30/30 |
| perft_full_d3 | 1.20x | 2.01x | 2.50x | 1.49x | 2.58x | 2.51x | 30/30 |
| fen_parse | 0.92x | 1.29x | 1.44x | 1.15x | 2.11x | 1.28x | 30/30 |
| fen_write_native_ep | 0.95x | 3.98x | 8.93x | 1.48x | 24.38x | 1.84x | 30/30 |
| snapshot | 0.99x | 2.49x | 1.50x | 8.19x | 1.50x | 1.49x | 30/30 |
| san_parse_all | 1.00x | 1.77x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.88x | N/A | 2.35x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.92x | 1.41x | 4.11x | 1.38x | 1.94x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 83.49x | 142.27x | 213.52x | 151.79x | 5/5 |
| chess960_materialize | 1.00x | 1.74x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.89x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 35.72x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.63x | 1.57x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 27.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 6.20x | 1.90x | 1.24x | 2.51x | 2.33x | 9/10 |
| corpus_materialize | 1.04x | 1.71x | 1.62x | 1.16x | 2.32x | 1.42x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.31x | 3.01x | 2.35x | 3.04x | 1.66x | 10/10 |
| replay_trusted_native | 1.03x | 0.78x | 1.45x | 1.61x | 2.61x | 3.54x | 9/10 |
| replay_checked_native | 1.00x | 2.74x | 1.19x | 8.80x | 9.17x | 11.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
