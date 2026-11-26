| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 5.02x | 1.71x | 1.75x | 2.47x | 2.04x | 30/30 |
| materialize | 0.98x | 2.03x | 1.60x | 1.09x | 2.36x | 1.41x | 27/30 |
| perft_bulk_d4 | 0.97x | 4.18x | 1.51x | 1.29x | 1.95x | 1.83x | 30/30 |
| perft_full_d3 | 0.97x | 1.56x | 1.72x | 1.33x | 1.94x | 1.98x | 30/30 |
| fen_parse | 0.96x | 1.20x | 1.40x | 1.26x | 1.89x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.18x | 13.98x | 1.80x | 28.86x | 2.24x | 30/30 |
| snapshot | 0.98x | 1.74x | 1.17x | 2.46x | 1.18x | 1.00x | 27/30 |
| san_parse_all | 0.96x | 1.67x | N/A | 3.57x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.32x | N/A | 2.83x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 1.21x | 1.25x | 5.94x | 1.19x | 1.38x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.20x | 67.90x | 134.52x | 185.31x | 138.42x | 5/5 |
| chess960_materialize | 0.98x | 1.90x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 3.25x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 41.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 20.79x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.78x | 1.90x | 1.27x | 2.26x | 2.49x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.43x | 1.15x | 1.91x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.81x | 2.61x | 1.92x | 2.51x | 1.41x | 10/10 |
| replay_trusted_native | 0.93x | 0.63x | 1.85x | 1.67x | 1.86x | 1.81x | 8/10 |
| replay_checked_native | 0.98x | 2.66x | 1.21x | 9.56x | 8.37x | 10.54x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
