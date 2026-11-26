| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.87x | 1.45x | 1.32x | 1.80x | 1.27x | 29/30 |
| materialize | 0.89x | 2.11x | 1.65x | 1.14x | 2.14x | 1.52x | 30/30 |
| perft_bulk_d4 | 0.95x | 2.70x | 1.40x | 1.10x | 1.77x | 1.51x | 29/30 |
| perft_full_d3 | 0.97x | 1.23x | 1.45x | 1.11x | 1.71x | 1.58x | 29/30 |
| fen_parse | 1.15x | 1.64x | 1.61x | 1.21x | 1.99x | 1.44x | 30/30 |
| fen_write_native_ep | 0.92x | 2.42x | 8.18x | 1.63x | 24.03x | 1.78x | 30/30 |
| snapshot | 1.08x | 1.88x | 1.30x | 3.11x | 1.22x | 0.99x | 25/30 |
| san_parse_all | 0.97x | 2.16x | N/A | 3.74x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 10.36x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.16x | 2.46x | 1.52x | 5.90x | 1.47x | 1.46x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.22x | 1.60x | 88.69x | 126.67x | 208.26x | 143.32x | 5/5 |
| chess960_materialize | 0.90x | 1.66x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.17x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 22.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 2.83x | 1.60x | 1.11x | 1.73x | 1.66x | 9/10 |
| corpus_materialize | 1.00x | 1.88x | 1.73x | 1.24x | 2.79x | 1.59x | 10/10 |
| corpus_uci_sorted | 1.06x | 2.13x | 3.06x | 2.29x | 2.89x | 1.65x | 10/10 |
| replay_trusted_native | 1.05x | 1.04x | 2.09x | 2.27x | 3.33x | 3.06x | 10/10 |
| replay_checked_native | 0.98x | 2.31x | 1.03x | 5.59x | 6.34x | 7.24x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
