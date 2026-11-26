| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.90x | 1.48x | 1.33x | 1.87x | 1.30x | 29/30 |
| materialize | 1.05x | 2.33x | 1.79x | 1.28x | 2.39x | 1.66x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.78x | 1.49x | 1.17x | 1.86x | 1.59x | 30/30 |
| perft_full_d3 | 1.00x | 1.28x | 1.52x | 1.17x | 1.74x | 1.64x | 30/30 |
| fen_parse | 1.11x | 1.71x | 1.53x | 1.15x | 2.19x | 1.43x | 30/30 |
| fen_write_native_ep | 1.00x | 2.64x | 9.16x | 1.80x | 26.42x | 1.97x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.22x | 2.88x | 1.14x | 0.98x | 24/30 |
| san_parse_all | 1.03x | 2.23x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.99x | N/A | 2.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.33x | 4.98x | 1.26x | 1.30x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 75.45x | 104.59x | 173.12x | 119.52x | 5/5 |
| chess960_materialize | 1.01x | 1.90x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.26x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 21.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 2.92x | 1.66x | 1.16x | 1.88x | 1.72x | 10/10 |
| corpus_materialize | 0.99x | 1.88x | 1.75x | 1.26x | 2.77x | 1.64x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.02x | 2.91x | 2.18x | 2.73x | 1.57x | 10/10 |
| replay_trusted_native | 0.99x | 0.96x | 1.90x | 2.07x | 3.11x | 2.81x | 8/10 |
| replay_checked_native | 1.09x | 2.68x | 1.17x | 6.34x | 6.93x | 8.09x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
