| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 4.30x | 1.52x | 1.74x | 2.08x | 1.80x | 30/30 |
| materialize | 0.90x | 1.93x | 1.51x | 1.19x | 2.11x | 1.40x | 27/30 |
| perft_bulk_d4 | 0.98x | 3.71x | 1.42x | 1.15x | 1.75x | 1.60x | 29/30 |
| perft_full_d3 | 0.91x | 1.58x | 1.75x | 1.35x | 2.10x | 2.06x | 28/30 |
| fen_parse | 0.91x | 1.25x | 1.34x | 1.28x | 1.58x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.05x | 9.23x | 1.89x | 29.19x | 2.25x | 30/30 |
| snapshot | 0.99x | 1.91x | 1.11x | 2.41x | 1.18x | 1.03x | 24/30 |
| san_parse_all | 0.97x | 1.79x | N/A | 3.78x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 12.59x | N/A | 2.79x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.16x | 1.25x | 6.29x | 1.24x | 1.40x | 27/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.22x | 1.19x | 86.49x | 215.81x | 218.85x | 182.71x | 5/5 |
| chess960_materialize | 0.87x | 1.51x | 1.29x | N/A | N/A | N/A | 3/4 |
| chess960_perft_d3 | 1.00x | 2.59x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 36.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.78x | 0.85x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.89x | 23.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.88x | 5.09x | 1.56x | 0.96x | 1.93x | 2.02x | 9/10 |
| corpus_materialize | 1.00x | 1.59x | 1.48x | 1.23x | 2.11x | 1.32x | 10/10 |
| corpus_uci_sorted | 0.90x | 1.57x | 2.43x | 1.62x | 2.28x | 1.28x | 10/10 |
| replay_trusted_native | 1.03x | 0.77x | 1.90x | 1.87x | 2.03x | 2.04x | 8/10 |
| replay_checked_native | 0.87x | 2.65x | 1.18x | 8.15x | 7.95x | 9.75x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
