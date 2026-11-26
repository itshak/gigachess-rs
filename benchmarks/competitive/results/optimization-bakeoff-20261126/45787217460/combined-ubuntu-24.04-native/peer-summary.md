| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.20x | 1.50x | 1.47x | 2.24x | 1.81x | 30/30 |
| materialize | 0.98x | 1.86x | 1.60x | 1.18x | 2.13x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.78x | 1.51x | 1.29x | 2.24x | 1.89x | 30/30 |
| perft_full_d3 | 1.00x | 1.30x | 1.47x | 1.28x | 1.88x | 1.89x | 30/30 |
| fen_parse | 1.08x | 1.31x | 1.52x | 1.32x | 2.24x | 1.29x | 30/30 |
| fen_write_native_ep | 0.99x | 3.95x | 9.70x | 1.73x | 28.52x | 2.00x | 30/30 |
| snapshot | 0.76x | 1.17x | 1.14x | 1.86x | 1.14x | 0.86x | 24/30 |
| san_parse_all | 0.97x | 2.06x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 10.70x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.97x | 1.14x | 3.95x | 1.26x | 1.40x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.14x | 67.99x | 131.88x | 199.77x | 126.23x | 5/5 |
| chess960_materialize | 1.00x | 1.41x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.50x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 30.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 22.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.34x | 1.81x | 1.37x | 2.58x | 2.38x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.59x | 1.20x | 2.05x | 1.40x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.41x | 3.16x | 2.42x | 3.06x | 1.74x | 10/10 |
| replay_trusted_native | 0.92x | 0.67x | 1.21x | 1.46x | 1.94x | 2.25x | 8/10 |
| replay_checked_native | 1.00x | 2.26x | 1.01x | 8.01x | 7.60x | 9.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
