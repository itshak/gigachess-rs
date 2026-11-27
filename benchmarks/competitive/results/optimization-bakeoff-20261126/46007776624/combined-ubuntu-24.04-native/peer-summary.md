| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.86x | 1.74x | 1.69x | 2.64x | 2.10x | 30/30 |
| materialize | 1.02x | 1.96x | 1.70x | 1.26x | 2.34x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.15x | 1.65x | 1.48x | 2.50x | 2.04x | 30/30 |
| perft_full_d3 | 1.23x | 1.94x | 1.86x | 1.62x | 2.42x | 2.35x | 30/30 |
| fen_parse | 0.98x | 1.22x | 1.28x | 1.05x | 1.91x | 1.15x | 28/30 |
| fen_write_native_ep | 0.98x | 4.28x | 9.58x | 1.67x | 27.51x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.25x | 4.04x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.04x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.32x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.33x | 1.33x | 4.64x | 1.48x | 1.62x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 79.36x | 137.37x | 219.48x | 146.35x | 5/5 |
| chess960_materialize | 0.99x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.85x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 27.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.81x | 0.84x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.98x | 19.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 5.78x | 1.92x | 1.45x | 2.81x | 2.53x | 10/10 |
| corpus_materialize | 1.02x | 1.59x | 1.60x | 1.16x | 2.03x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.40x | 3.19x | 2.48x | 3.02x | 1.72x | 10/10 |
| replay_trusted_native | 0.96x | 0.88x | 1.53x | 1.88x | 2.52x | 2.85x | 9/10 |
| replay_checked_native | 1.00x | 2.48x | 1.06x | 8.45x | 8.22x | 9.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
