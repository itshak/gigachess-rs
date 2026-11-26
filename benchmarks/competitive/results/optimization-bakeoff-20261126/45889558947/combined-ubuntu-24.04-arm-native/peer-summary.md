| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.43x | 1.67x | 1.81x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.96x | 1.59x | 1.34x | 2.16x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.87x | 1.56x | 1.42x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.61x | 1.34x | 1.99x | 1.86x | 30/30 |
| fen_parse | 0.98x | 1.22x | 1.30x | 1.26x | 1.71x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.32x | 1.58x | 30.15x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.09x | 1.04x | 1.47x | 1.06x | 0.95x | 22/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.49x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.07x | 1.20x | 1.21x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.81x | 128.67x | 169.06x | 135.52x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.45x | 1.71x | 1.37x | 2.00x | 2.19x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.53x | 1.23x | 1.90x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.62x | 1.65x | 2.00x | 2.27x | 2.28x | 8/10 |
| replay_checked_native | 1.00x | 2.31x | 1.13x | 7.61x | 6.79x | 8.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
