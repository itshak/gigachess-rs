| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.30x | 1.49x | 1.48x | 2.13x | 1.87x | 30/30 |
| materialize | 0.97x | 1.91x | 1.31x | 1.20x | 2.22x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.89x | 1.54x | 1.37x | 2.20x | 1.95x | 30/30 |
| perft_full_d3 | 0.93x | 1.26x | 1.33x | 1.19x | 1.73x | 1.67x | 30/30 |
| fen_parse | 1.08x | 1.40x | 1.49x | 1.39x | 2.31x | 1.50x | 30/30 |
| fen_write_native_ep | 0.93x | 3.94x | 7.83x | 1.47x | 29.69x | 1.87x | 30/30 |
| snapshot | 0.74x | 1.52x | 1.10x | 2.31x | 1.12x | 0.98x | 25/30 |
| san_parse_all | 0.97x | 1.84x | N/A | 5.18x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.55x | N/A | 3.62x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.36x | 1.48x | 9.79x | 1.54x | 1.72x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.70x | 0.90x | 56.13x | 108.69x | 164.57x | 119.27x | 4/5 |
| chess960_materialize | 0.99x | 1.57x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.72x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 24.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.12x | 23.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.18x | 1.74x | 1.34x | 2.35x | 2.31x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.28x | 1.15x | 2.01x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.73x | 2.17x | 1.78x | 2.08x | 1.48x | 10/10 |
| replay_trusted_native | 0.91x | 0.79x | 1.50x | 1.95x | 2.42x | 2.95x | 9/10 |
| replay_checked_native | 0.83x | 2.14x | 0.89x | 6.99x | 6.91x | 8.17x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
