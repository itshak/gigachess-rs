| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.56x | 1.36x | 1.47x | 1.83x | 1.72x | 29/30 |
| materialize | 1.02x | 1.97x | 1.43x | 1.02x | 2.34x | 1.43x | 27/30 |
| perft_bulk_d4 | 0.82x | 3.45x | 1.29x | 1.07x | 1.51x | 1.52x | 28/30 |
| perft_full_d3 | 1.00x | 1.76x | 1.71x | 1.44x | 2.15x | 2.11x | 30/30 |
| fen_parse | 0.99x | 1.18x | 1.42x | 1.34x | 1.75x | 1.51x | 30/30 |
| fen_write_native_ep | 0.94x | 4.28x | 8.91x | 1.85x | 28.33x | 2.45x | 30/30 |
| snapshot | 1.04x | 1.79x | 1.24x | 2.54x | 1.41x | 1.23x | 27/30 |
| san_parse_all | 0.92x | 1.66x | N/A | 3.70x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 12.31x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.12x | 1.18x | 5.81x | 1.15x | 1.36x | 28/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.28x | 89.94x | 168.30x | 246.78x | 188.93x | 5/5 |
| chess960_materialize | 0.86x | 1.64x | 1.18x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.85x | 2.60x | 1.17x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.78x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.78x | 16.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.69x | 5.05x | 1.46x | 0.93x | 1.87x | 1.73x | 9/10 |
| corpus_materialize | 0.84x | 1.34x | 1.26x | 0.99x | 1.74x | 1.16x | 9/10 |
| corpus_uci_sorted | 0.82x | 1.99x | 2.66x | 1.94x | 2.49x | 1.56x | 10/10 |
| replay_trusted_native | 0.93x | 0.61x | 1.50x | 1.80x | 1.73x | 1.65x | 8/10 |
| replay_checked_native | 1.04x | 2.63x | 1.15x | 8.15x | 7.50x | 8.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
