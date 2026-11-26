| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.72x | 1.52x | 1.32x | 1.77x | 1.30x | 29/30 |
| materialize | 1.00x | 2.27x | 1.77x | 1.82x | 2.75x | 1.96x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.97x | 1.56x | 1.17x | 1.96x | 1.77x | 30/30 |
| perft_full_d3 | 1.00x | 1.58x | 1.86x | 1.59x | 2.23x | 1.82x | 30/30 |
| fen_parse | 0.99x | 1.51x | 1.44x | 1.19x | 1.64x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 3.15x | 10.47x | 1.88x | 29.19x | 2.58x | 30/30 |
| snapshot | 0.89x | 1.46x | 1.12x | 3.02x | 1.12x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.49x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.78x | 1.20x | 3.40x | 1.10x | 1.29x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.37x | 57.35x | 108.39x | 141.36x | 102.45x | 5/5 |
| chess960_materialize | 1.07x | 1.66x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.88x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.11x | 23.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.62x | 2.31x | 1.28x | 2.35x | 3.23x | 10/10 |
| corpus_materialize | 1.01x | 1.53x | 1.43x | 1.17x | 1.86x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.93x | 3.06x | 2.22x | 3.02x | 1.54x | 10/10 |
| replay_trusted_native | 1.00x | 0.71x | 1.34x | 1.31x | 1.78x | 1.65x | 8/10 |
| replay_checked_native | 0.91x | 1.77x | 1.01x | 7.71x | 7.49x | 8.54x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
