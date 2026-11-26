| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.90x | 1.49x | 1.64x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.82x | 1.44x | 1.25x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.41x | 1.39x | 1.27x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.19x | 1.29x | 1.07x | 1.58x | 1.49x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.34x | 1.28x | 1.78x | 1.43x | 30/30 |
| fen_write_native_ep | 1.00x | 4.41x | 9.39x | 1.59x | 30.18x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.03x | 1.46x | 1.03x | 0.95x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.64x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.26x | 1.50x | 4.13x | 1.22x | 1.22x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.32x | 127.99x | 176.64x | 143.76x | 5/5 |
| chess960_materialize | 1.00x | 1.58x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.65x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.01x | 1.53x | 1.20x | 1.78x | 1.96x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.46x | 1.16x | 1.82x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.55x | 1.48x | 1.73x | 1.94x | 1.98x | 8/10 |
| replay_checked_native | 1.00x | 2.12x | 1.02x | 6.84x | 6.11x | 7.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
