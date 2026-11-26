| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.83x | 5.09x | 1.70x | 1.92x | 2.47x | 2.12x | 30/30 |
| materialize | 0.81x | 1.99x | 1.57x | 1.02x | 2.11x | 1.29x | 26/30 |
| perft_bulk_d4 | 0.89x | 4.14x | 1.51x | 1.27x | 1.98x | 1.92x | 29/30 |
| perft_full_d3 | 0.81x | 1.30x | 1.43x | 1.08x | 1.48x | 1.59x | 28/30 |
| fen_parse | 0.86x | 1.14x | 1.19x | 1.19x | 1.48x | 1.23x | 28/30 |
| fen_write_native_ep | 1.03x | 4.16x | 9.34x | 1.98x | 30.13x | 2.58x | 30/30 |
| snapshot | 0.96x | 1.77x | 1.17x | 2.48x | 1.30x | 0.90x | 22/30 |
| san_parse_all | 0.90x | 1.84x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 11.42x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 0.83x | 1.08x | 1.21x | 5.45x | 1.05x | 1.24x | 23/30 |
| make_unmake_full | 0.83x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 0.71x | 1.31x | 69.46x | 100.52x | 124.94x | 109.84x | 5/5 |
| chess960_materialize | 0.77x | 1.50x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.82x | 3.04x | 1.64x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 36.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 0.91x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 18.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.78x | 5.67x | 1.82x | 1.10x | 2.11x | 2.14x | 9/10 |
| corpus_materialize | 1.00x | 1.53x | 1.42x | 1.16x | 2.02x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.78x | 2.60x | 1.95x | 2.49x | 1.53x | 10/10 |
| replay_trusted_native | 0.92x | 0.59x | 1.61x | 1.71x | 2.10x | 1.81x | 8/10 |
| replay_checked_native | 0.79x | 2.39x | 1.07x | 8.38x | 7.33x | 7.77x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
