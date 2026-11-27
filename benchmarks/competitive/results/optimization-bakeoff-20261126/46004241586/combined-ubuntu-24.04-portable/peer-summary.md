| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.05x | 1.74x | 1.52x | 2.06x | 1.45x | 30/30 |
| materialize | 1.00x | 2.29x | 1.93x | 1.90x | 2.91x | 2.03x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.19x | 1.70x | 1.27x | 2.14x | 1.89x | 30/30 |
| perft_full_d3 | 1.17x | 1.59x | 1.86x | 1.66x | 2.27x | 1.85x | 30/30 |
| fen_parse | 0.91x | 1.57x | 1.36x | 1.07x | 1.36x | 1.24x | 30/30 |
| fen_write_native_ep | 1.00x | 3.03x | 10.13x | 1.89x | 27.40x | 2.29x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.24x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 0.99x | 1.95x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 9.05x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.19x | 1.49x | 4.19x | 1.34x | 1.58x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.73x | 108.00x | 124.70x | 120.73x | 5/5 |
| chess960_materialize | 0.95x | 1.70x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.16x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.17x | 24.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 4.86x | 2.44x | 1.34x | 2.46x | 3.33x | 10/10 |
| corpus_materialize | 0.99x | 1.49x | 1.40x | 1.13x | 1.81x | 1.22x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.95x | 3.02x | 2.22x | 2.95x | 1.57x | 10/10 |
| replay_trusted_native | 0.99x | 0.88x | 1.62x | 1.59x | 2.24x | 2.11x | 9/10 |
| replay_checked_native | 1.00x | 2.07x | 1.14x | 8.99x | 8.69x | 10.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
