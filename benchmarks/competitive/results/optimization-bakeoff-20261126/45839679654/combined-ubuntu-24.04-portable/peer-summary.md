| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 2.98x | 1.53x | 1.34x | 1.90x | 1.33x | 29/30 |
| materialize | 0.97x | 2.25x | 1.75x | 1.30x | 2.57x | 1.61x | 30/30 |
| perft_bulk_d4 | 0.91x | 2.79x | 1.44x | 1.10x | 1.77x | 1.60x | 28/30 |
| perft_full_d3 | 0.87x | 0.98x | 1.24x | 0.95x | 1.39x | 1.26x | 22/30 |
| fen_parse | 0.91x | 1.37x | 1.36x | 1.07x | 1.68x | 1.19x | 29/30 |
| fen_write_native_ep | 0.99x | 2.72x | 8.62x | 1.45x | 25.12x | 1.95x | 30/30 |
| snapshot | 1.00x | 1.63x | 1.16x | 2.64x | 1.15x | 1.00x | 25/30 |
| san_parse_all | 0.93x | 1.79x | N/A | 3.52x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 8.65x | N/A | 1.88x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.03x | 1.36x | 5.07x | 1.19x | 1.23x | 29/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 60.09x | 92.61x | 115.72x | 84.13x | 5/5 |
| chess960_materialize | 0.88x | 1.61x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 2.03x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 29.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.69x | 14.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.07x | 1.72x | 1.05x | 1.84x | 2.69x | 9/10 |
| corpus_materialize | 0.96x | 1.69x | 1.52x | 1.17x | 2.22x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.13x | 2.89x | 2.28x | 2.93x | 1.62x | 10/10 |
| replay_trusted_native | 0.95x | 0.65x | 1.51x | 1.49x | 2.00x | 1.82x | 8/10 |
| replay_checked_native | 0.82x | 1.73x | 0.99x | 6.71x | 6.62x | 7.78x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
