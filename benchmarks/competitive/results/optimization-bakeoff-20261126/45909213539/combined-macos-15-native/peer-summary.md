| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.18x | 5.25x | 1.87x | 1.90x | 2.45x | 2.25x | 30/30 |
| materialize | 0.98x | 2.27x | 1.65x | 1.03x | 2.44x | 1.41x | 27/30 |
| perft_bulk_d4 | 0.98x | 4.69x | 1.60x | 1.29x | 1.89x | 1.91x | 28/30 |
| perft_full_d3 | 0.92x | 1.47x | 1.57x | 1.24x | 1.81x | 1.88x | 30/30 |
| fen_parse | 1.07x | 1.27x | 1.38x | 1.32x | 1.64x | 1.34x | 27/30 |
| fen_write_native_ep | 1.07x | 4.29x | 8.71x | 1.68x | 27.10x | 2.22x | 30/30 |
| snapshot | 0.91x | 1.54x | 1.23x | 2.39x | 1.19x | 1.04x | 22/30 |
| san_parse_all | 1.01x | 1.95x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 11.70x | N/A | 2.61x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.18x | 1.32x | 5.86x | 1.23x | 1.45x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.22x | N/A | N/A | 5/6 |
| startpos | 1.25x | 1.69x | 73.60x | 146.26x | 180.94x | 156.69x | 5/5 |
| chess960_materialize | 0.93x | 1.81x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.24x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 40.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.82x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.84x | 18.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.98x | 1.97x | 1.32x | 2.16x | 2.48x | 10/10 |
| corpus_materialize | 1.11x | 1.56x | 1.49x | 1.21x | 2.20x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.10x | 1.83x | 2.46x | 2.20x | 2.55x | 1.46x | 10/10 |
| replay_trusted_native | 0.90x | 0.60x | 1.54x | 1.55x | 1.63x | 1.54x | 8/10 |
| replay_checked_native | 0.95x | 3.24x | 1.26x | 9.82x | 8.91x | 10.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
