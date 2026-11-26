| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.16x | 1.50x | 1.44x | 2.20x | 1.78x | 30/30 |
| materialize | 1.01x | 1.84x | 1.57x | 1.17x | 2.12x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.50x | 1.39x | 1.19x | 2.10x | 1.81x | 30/30 |
| perft_full_d3 | 1.04x | 1.24x | 1.45x | 1.31x | 1.87x | 1.85x | 30/30 |
| fen_parse | 1.01x | 1.32x | 1.48x | 1.17x | 2.18x | 1.26x | 30/30 |
| fen_write_native_ep | 1.01x | 4.06x | 9.96x | 1.83x | 28.86x | 2.05x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.29x | 4.15x | 1.29x | 1.29x | 30/30 |
| san_parse_all | 1.00x | 2.08x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.57x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.25x | 1.29x | 4.48x | 1.44x | 1.57x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.22x | 143.66x | 230.08x | 148.91x | 5/5 |
| chess960_materialize | 1.00x | 1.49x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.62x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 26.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 19.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.89x | 1.64x | 1.24x | 2.31x | 2.16x | 10/10 |
| corpus_materialize | 0.99x | 1.51x | 1.46x | 1.11x | 1.97x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.38x | 3.19x | 2.38x | 3.06x | 1.77x | 10/10 |
| replay_trusted_native | 1.03x | 0.86x | 1.59x | 1.95x | 2.54x | 2.85x | 9/10 |
| replay_checked_native | 1.00x | 2.39x | 1.03x | 7.79x | 7.69x | 9.24x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
