| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.54x | 1.53x | 1.56x | 2.59x | 1.77x | 30/30 |
| materialize | 0.97x | 2.17x | 1.69x | 1.16x | 2.23x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.90x | 3.76x | 1.45x | 1.21x | 2.31x | 1.90x | 30/30 |
| perft_full_d3 | 0.96x | 0.81x | 0.83x | 0.64x | 1.16x | 0.94x | 8/30 |
| fen_parse | 1.00x | 1.25x | 1.40x | 1.07x | 1.99x | 1.29x | 30/30 |
| fen_write_native_ep | 0.99x | 2.70x | 6.44x | 1.19x | 17.94x | 1.37x | 30/30 |
| snapshot | 0.81x | 1.38x | 0.98x | 2.29x | 1.00x | 0.91x | 16/30 |
| san_parse_all | 0.88x | 1.88x | N/A | 3.20x | N/A | N/A | 12/12 |
| san_render_native | 0.89x | 9.96x | N/A | 1.99x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.18x | 0.79x | 2.96x | 0.87x | 0.99x | 14/30 |
| make_unmake_full | 0.79x | N/A | N/A | 0.77x | N/A | N/A | 0/6 |
| startpos | 0.76x | 0.89x | 65.76x | 99.70x | 160.97x | 116.93x | 4/5 |
| chess960_materialize | 1.04x | 1.68x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.90x | 2.64x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 21.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 15.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.70x | 1.81x | 1.39x | 2.52x | 2.08x | 10/10 |
| corpus_materialize | 0.98x | 1.95x | 1.78x | 1.26x | 2.87x | 1.54x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.37x | 1.85x | 1.37x | 1.76x | 0.96x | 8/10 |
| replay_trusted_native | 0.91x | 0.58x | 0.98x | 1.10x | 1.93x | 2.33x | 7/10 |
| replay_checked_native | 0.84x | 2.16x | 0.77x | 4.46x | 5.35x | 6.06x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
