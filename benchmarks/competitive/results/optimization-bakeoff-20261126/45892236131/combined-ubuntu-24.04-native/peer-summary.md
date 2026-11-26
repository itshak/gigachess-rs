| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.52x | 1.65x | 1.59x | 2.49x | 1.91x | 30/30 |
| materialize | 0.96x | 2.16x | 1.70x | 1.14x | 2.17x | 1.47x | 29/30 |
| perft_bulk_d4 | 0.77x | 3.23x | 1.26x | 1.03x | 1.98x | 1.72x | 27/30 |
| perft_full_d3 | 0.79x | 1.20x | 1.25x | 0.93x | 1.72x | 1.46x | 25/30 |
| fen_parse | 0.99x | 1.45x | 1.62x | 1.20x | 2.41x | 1.26x | 30/30 |
| fen_write_native_ep | 0.87x | 3.51x | 8.67x | 1.59x | 24.57x | 1.78x | 30/30 |
| snapshot | 0.99x | 1.90x | 1.25x | 3.16x | 2.96x | 1.24x | 30/30 |
| san_parse_all | 0.85x | 1.90x | N/A | 3.28x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 9.91x | N/A | 1.93x | N/A | N/A | 12/12 |
| apply_children_full | 0.85x | 1.66x | 1.30x | 4.20x | 1.51x | 1.81x | 30/30 |
| make_unmake_full | 0.75x | N/A | N/A | 0.89x | N/A | N/A | 1/6 |
| startpos | 1.25x | 1.50x | 116.27x | 184.59x | 310.92x | 184.32x | 5/5 |
| chess960_materialize | 0.95x | 1.93x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.81x | 2.52x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 22.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 18.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.32x | 1.68x | 1.23x | 2.27x | 1.95x | 9/10 |
| corpus_materialize | 0.98x | 1.98x | 1.77x | 1.26x | 2.82x | 1.61x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.33x | 3.14x | 2.34x | 2.91x | 1.68x | 10/10 |
| replay_trusted_native | 0.76x | 0.85x | 1.37x | 1.60x | 2.89x | 3.66x | 8/10 |
| replay_checked_native | 0.86x | 2.45x | 0.93x | 5.38x | 6.11x | 7.45x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
