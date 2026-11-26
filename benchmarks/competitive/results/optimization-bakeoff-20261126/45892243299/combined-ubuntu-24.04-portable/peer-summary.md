| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.20x | 1.63x | 1.48x | 2.07x | 1.43x | 30/30 |
| materialize | 0.98x | 2.39x | 1.83x | 1.34x | 2.45x | 1.69x | 30/30 |
| perft_bulk_d4 | 0.96x | 2.95x | 1.59x | 1.24x | 1.98x | 1.71x | 30/30 |
| perft_full_d3 | 0.98x | 1.38x | 1.69x | 1.27x | 1.91x | 1.79x | 30/30 |
| fen_parse | 1.01x | 1.38x | 1.41x | 1.11x | 2.03x | 1.21x | 30/30 |
| fen_write_native_ep | 0.98x | 2.57x | 8.72x | 1.69x | 25.31x | 1.98x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 2.88x | 1.14x | 0.98x | 24/30 |
| san_parse_all | 1.00x | 2.19x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.80x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.43x | 1.50x | 5.53x | 1.42x | 1.46x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 72.76x | 106.53x | 171.93x | 113.49x | 5/5 |
| chess960_materialize | 0.93x | 1.78x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.39x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 21.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.16x | 1.78x | 1.25x | 2.02x | 1.83x | 10/10 |
| corpus_materialize | 0.97x | 1.99x | 1.78x | 1.32x | 2.94x | 1.75x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.06x | 2.94x | 2.21x | 2.76x | 1.60x | 10/10 |
| replay_trusted_native | 1.01x | 1.08x | 2.20x | 2.37x | 3.48x | 3.19x | 10/10 |
| replay_checked_native | 1.01x | 2.81x | 1.17x | 6.38x | 6.86x | 8.14x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
