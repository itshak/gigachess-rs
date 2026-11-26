| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.50x | 1.53x | 1.44x | 2.29x | 1.85x | 30/30 |
| materialize | 0.97x | 2.03x | 1.32x | 1.17x | 2.40x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.79x | 1.44x | 1.23x | 2.18x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.39x | 1.64x | 1.27x | 2.00x | 2.05x | 30/30 |
| fen_parse | 0.84x | 1.16x | 1.28x | 1.33x | 1.91x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 3.65x | 7.12x | 1.55x | 31.58x | 2.07x | 30/30 |
| snapshot | 0.97x | 2.49x | 1.50x | 5.40x | 1.50x | 1.49x | 30/30 |
| san_parse_all | 0.97x | 1.94x | N/A | 4.67x | N/A | N/A | 12/12 |
| san_render_native | 1.16x | 9.82x | N/A | 3.59x | N/A | N/A | 12/12 |
| apply_children_full | 1.15x | 1.80x | 1.35x | 8.73x | 1.39x | 1.84x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 0.34x | 0.44x | 33.53x | 66.26x | 88.32x | 64.79x | 4/5 |
| chess960_materialize | 1.00x | 1.63x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.72x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 37.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.24x | 29.54x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.38x | 1.58x | 1.07x | 2.11x | 1.98x | 9/10 |
| corpus_materialize | 0.95x | 1.54x | 1.17x | 0.99x | 2.07x | 1.27x | 9/10 |
| corpus_uci_sorted | 0.98x | 1.66x | 1.94x | 1.70x | 1.93x | 1.38x | 10/10 |
| replay_trusted_native | 1.00x | 0.72x | 1.38x | 1.63x | 2.42x | 2.95x | 8/10 |
| replay_checked_native | 0.99x | 2.45x | 1.11x | 8.50x | 8.49x | 10.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
