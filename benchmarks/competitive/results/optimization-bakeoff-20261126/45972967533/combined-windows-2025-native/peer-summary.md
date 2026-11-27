| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.89x | 1.74x | 1.67x | 2.45x | 2.18x | 30/30 |
| materialize | 1.03x | 2.05x | 1.40x | 1.40x | 2.43x | 1.64x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.24x | 1.71x | 1.44x | 2.44x | 2.20x | 30/30 |
| perft_full_d3 | 1.22x | 1.69x | 1.80x | 1.67x | 2.35x | 2.26x | 30/30 |
| fen_parse | 1.03x | 1.32x | 1.50x | 1.49x | 2.27x | 1.53x | 30/30 |
| fen_write_native_ep | 0.96x | 3.91x | 7.88x | 1.50x | 30.30x | 1.91x | 30/30 |
| snapshot | 1.03x | 1.89x | 1.06x | 4.01x | 1.01x | 1.01x | 24/30 |
| san_parse_all | 1.00x | 2.13x | N/A | 5.80x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.75x | N/A | 3.82x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.31x | 1.42x | 9.65x | 1.47x | 1.65x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.30x | 188.63x | 243.45x | 173.47x | 5/5 |
| chess960_materialize | 1.02x | 1.71x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.85x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 26.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 22.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.39x | 1.84x | 1.42x | 2.58x | 2.45x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.30x | 1.24x | 2.08x | 1.51x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.71x | 2.29x | 1.84x | 2.16x | 1.38x | 10/10 |
| replay_trusted_native | 0.95x | 0.86x | 1.61x | 2.22x | 2.55x | 2.92x | 8/10 |
| replay_checked_native | 1.00x | 2.63x | 1.07x | 8.89x | 8.38x | 10.00x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
