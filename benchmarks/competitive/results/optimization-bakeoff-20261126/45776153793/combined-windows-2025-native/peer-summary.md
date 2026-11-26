| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.17x | 1.49x | 1.46x | 2.12x | 1.83x | 30/30 |
| materialize | 0.96x | 1.82x | 1.28x | 1.19x | 2.06x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.90x | 3.46x | 1.35x | 1.25x | 2.05x | 1.77x | 30/30 |
| perft_full_d3 | 0.90x | 0.70x | 0.77x | 0.68x | 0.97x | 0.95x | 2/30 |
| fen_parse | 0.96x | 1.14x | 1.29x | 1.16x | 2.01x | 1.39x | 30/30 |
| fen_write_native_ep | 0.98x | 2.69x | 5.03x | 1.01x | 19.27x | 1.22x | 27/30 |
| snapshot | 0.86x | 1.38x | 0.96x | 2.11x | 0.92x | 0.89x | 13/30 |
| san_parse_all | 0.86x | 1.60x | N/A | 4.39x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 9.61x | N/A | 3.30x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.18x | 0.74x | 4.86x | 0.76x | 0.86x | 12/30 |
| make_unmake_full | 0.83x | N/A | N/A | 0.83x | N/A | N/A | 0/6 |
| startpos | 0.43x | 0.48x | 30.77x | 55.54x | 86.18x | 70.90x | 4/5 |
| chess960_materialize | 0.93x | 1.37x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.87x | 2.13x | 1.18x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 29.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.88x | 18.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.63x | 1.79x | 1.34x | 2.43x | 2.41x | 10/10 |
| corpus_materialize | 0.91x | 1.51x | 1.22x | 1.12x | 1.86x | 1.32x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.22x | 1.54x | 1.31x | 1.46x | 1.00x | 9/10 |
| replay_trusted_native | 0.90x | 0.47x | 0.98x | 1.26x | 1.53x | 1.75x | 7/10 |
| replay_checked_native | 0.79x | 1.68x | 0.75x | 6.07x | 5.42x | 6.72x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
