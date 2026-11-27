| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.96x | 1.54x | 1.34x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.57x | 1.43x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.97x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.26x | 1.74x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.36x | 1.58x | 29.08x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.09x | 1.45x | 1.09x | 0.96x | 24/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.39x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.26x | 1.49x | 4.03x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.31x | 129.03x | 169.93x | 137.37x | 5/5 |
| chess960_materialize | 1.01x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.40x | 1.68x | 1.38x | 1.98x | 2.19x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.52x | 1.22x | 1.89x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.08x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.02x | 0.64x | 1.68x | 2.05x | 2.30x | 2.33x | 8/10 |
| replay_checked_native | 0.99x | 2.27x | 1.11x | 7.50x | 6.70x | 8.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
