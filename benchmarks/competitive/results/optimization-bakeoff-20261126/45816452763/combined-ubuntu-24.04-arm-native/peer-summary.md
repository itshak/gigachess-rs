| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 0.99x | 1.83x | 1.42x | 1.25x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.44x | 1.40x | 1.27x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 0.93x | 1.11x | 1.20x | 0.99x | 1.47x | 1.39x | 26/30 |
| fen_parse | 0.99x | 1.24x | 1.32x | 1.26x | 1.74x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 4.56x | 9.15x | 1.59x | 30.31x | 2.44x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.07x | 1.44x | 1.08x | 0.96x | 24/30 |
| san_parse_all | 0.99x | 1.82x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 11.16x | N/A | 2.42x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.20x | 1.43x | 3.88x | 1.16x | 1.16x | 29/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.34x | 126.26x | 176.99x | 144.81x | 5/5 |
| chess960_materialize | 0.91x | 1.45x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.50x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.03x | 1.51x | 1.21x | 1.77x | 1.94x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.45x | 1.17x | 1.81x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.90x | 2.08x | 2.82x | 1.45x | 10/10 |
| replay_trusted_native | 0.89x | 0.50x | 1.33x | 1.56x | 1.74x | 1.77x | 8/10 |
| replay_checked_native | 0.92x | 1.96x | 0.94x | 6.43x | 5.69x | 7.36x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
