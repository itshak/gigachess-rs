| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.16x | 1.73x | 1.76x | 2.53x | 2.09x | 30/30 |
| materialize | 0.99x | 2.20x | 1.69x | 1.23x | 2.46x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.30x | 1.62x | 1.31x | 2.47x | 2.26x | 30/30 |
| perft_full_d3 | 1.32x | 1.84x | 2.18x | 1.64x | 2.76x | 2.71x | 30/30 |
| fen_parse | 0.91x | 1.29x | 1.44x | 1.12x | 1.87x | 1.14x | 29/30 |
| fen_write_native_ep | 0.96x | 3.91x | 8.55x | 1.41x | 25.11x | 1.84x | 30/30 |
| snapshot | 1.02x | 2.49x | 1.49x | 4.49x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 0.99x | 1.80x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.15x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.91x | 1.41x | 4.37x | 1.37x | 1.93x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.25x | 87.03x | 136.58x | 216.67x | 141.71x | 5/5 |
| chess960_materialize | 0.98x | 1.79x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.09x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 35.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 22.79x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.00x | 1.76x | 1.20x | 2.36x | 2.18x | 9/10 |
| corpus_materialize | 0.99x | 1.70x | 1.57x | 1.16x | 2.22x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.38x | 3.04x | 2.47x | 3.04x | 1.69x | 10/10 |
| replay_trusted_native | 1.02x | 0.75x | 1.42x | 1.57x | 2.51x | 2.94x | 9/10 |
| replay_checked_native | 0.97x | 2.59x | 1.16x | 8.95x | 9.16x | 11.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
