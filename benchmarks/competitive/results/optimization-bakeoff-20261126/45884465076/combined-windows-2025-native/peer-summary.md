| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.00x | 1.71x | 1.62x | 2.42x | 2.19x | 30/30 |
| materialize | 1.02x | 2.16x | 1.42x | 1.20x | 2.49x | 1.57x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.19x | 1.58x | 1.38x | 2.40x | 2.23x | 30/30 |
| perft_full_d3 | 1.03x | 1.45x | 1.70x | 1.31x | 2.04x | 2.09x | 30/30 |
| fen_parse | 1.01x | 1.37x | 1.58x | 1.64x | 2.26x | 1.66x | 30/30 |
| fen_write_native_ep | 0.98x | 3.71x | 6.95x | 1.48x | 30.23x | 1.98x | 30/30 |
| snapshot | 1.00x | 2.50x | 1.50x | 5.44x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 1.00x | 1.95x | N/A | 4.52x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.11x | N/A | 4.00x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.87x | 1.37x | 8.82x | 1.43x | 1.90x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 70.10x | 141.51x | 196.52x | 149.31x | 5/5 |
| chess960_materialize | 0.98x | 1.80x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.95x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 37.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 28.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.79x | 1.75x | 1.16x | 2.32x | 2.19x | 9/10 |
| corpus_materialize | 1.03x | 1.66x | 1.33x | 1.11x | 2.32x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.64x | 1.99x | 1.74x | 1.95x | 1.37x | 10/10 |
| replay_trusted_native | 1.01x | 0.74x | 1.43x | 1.69x | 2.56x | 3.05x | 9/10 |
| replay_checked_native | 1.00x | 2.59x | 1.18x | 9.88x | 9.14x | 11.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
