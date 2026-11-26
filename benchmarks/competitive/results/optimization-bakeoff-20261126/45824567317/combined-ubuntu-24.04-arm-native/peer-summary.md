| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.84x | 1.46x | 1.26x | 2.05x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.41x | 1.40x | 1.27x | 1.80x | 1.84x | 30/30 |
| perft_full_d3 | 1.01x | 1.19x | 1.29x | 1.07x | 1.58x | 1.50x | 29/30 |
| fen_parse | 1.02x | 1.26x | 1.35x | 1.31x | 1.79x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.45x | 9.37x | 1.60x | 30.31x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.09x | 1.44x | 1.08x | 0.96x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 12.36x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.40x | 1.66x | 4.50x | 1.34x | 1.34x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.53x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.57x | 130.88x | 175.28x | 142.03x | 5/5 |
| chess960_materialize | 1.00x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.67x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.88x | 1.51x | 1.18x | 1.75x | 1.94x | 10/10 |
| corpus_materialize | 1.00x | 1.53x | 1.47x | 1.17x | 1.83x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.83x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.61x | 1.62x | 1.95x | 2.20x | 2.23x | 8/10 |
| replay_checked_native | 1.05x | 2.28x | 1.11x | 7.43x | 6.64x | 8.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
