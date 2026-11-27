| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.10x | 1.80x | 1.81x | 2.79x | 2.06x | 30/30 |
| materialize | 1.01x | 2.24x | 1.82x | 1.23x | 2.29x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.36x | 1.69x | 1.42x | 2.96x | 2.20x | 30/30 |
| perft_full_d3 | 1.35x | 2.05x | 2.11x | 1.60x | 4.03x | 2.41x | 30/30 |
| fen_parse | 0.86x | 1.32x | 1.31x | 1.12x | 1.84x | 1.07x | 29/30 |
| fen_write_native_ep | 1.00x | 4.11x | 9.93x | 1.88x | 29.19x | 2.35x | 30/30 |
| snapshot | 1.00x | 1.91x | 1.25x | 6.31x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.23x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.48x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.95x | 1.28x | 4.71x | 1.48x | 1.68x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 107.69x | 148.25x | 229.76x | 143.18x | 5/5 |
| chess960_materialize | 1.02x | 1.91x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.17x | 1.63x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 4.99x | 1.92x | 1.48x | 2.68x | 2.20x | 10/10 |
| corpus_materialize | 0.99x | 1.94x | 1.79x | 1.26x | 2.93x | 1.54x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.35x | 3.19x | 2.37x | 2.98x | 1.67x | 10/10 |
| replay_trusted_native | 1.01x | 1.20x | 2.14x | 2.36x | 4.07x | 4.68x | 10/10 |
| replay_checked_native | 1.06x | 2.96x | 1.12x | 6.43x | 7.39x | 8.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
