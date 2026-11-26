| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.19x | 3.65x | 1.40x | 1.49x | 1.81x | 1.81x | 30/30 |
| materialize | 1.03x | 1.77x | 1.38x | 1.23x | 1.96x | 1.37x | 30/30 |
| perft_bulk_d4 | 1.44x | 3.59x | 1.45x | 1.32x | 1.87x | 1.91x | 30/30 |
| perft_full_d3 | 1.73x | 1.13x | 1.23x | 1.05x | 1.48x | 1.44x | 27/30 |
| fen_parse | 1.03x | 1.24x | 1.35x | 1.29x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 0.98x | 2.93x | 6.07x | 1.04x | 19.44x | 1.57x | 28/30 |
| snapshot | 1.31x | 1.25x | 0.93x | 1.59x | 0.91x | 1.01x | 18/30 |
| san_parse_all | 2.03x | 1.86x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 2.15x | 11.48x | N/A | 2.46x | N/A | N/A | 12/12 |
| apply_children_full | 1.23x | 0.87x | 1.00x | 2.80x | 0.83x | 0.84x | 10/30 |
| make_unmake_full | 1.34x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.46x | 1.06x | 68.29x | 112.44x | 155.14x | 123.74x | 5/5 |
| chess960_materialize | 1.17x | 1.48x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.52x | 2.53x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 24.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.91x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.36x | 18.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 4.60x | 1.76x | 1.45x | 2.11x | 2.30x | 10/10 |
| corpus_materialize | 1.06x | 1.56x | 1.48x | 1.23x | 1.88x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.65x | 1.35x | 1.87x | 1.34x | 1.79x | 0.94x | 8/10 |
| replay_trusted_native | 1.39x | 0.45x | 1.16x | 1.42x | 1.55x | 1.61x | 8/10 |
| replay_checked_native | 1.34x | 1.83x | 0.89x | 6.03x | 5.37x | 6.89x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
