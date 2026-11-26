| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.34x | 1.48x | 1.50x | 1.93x | 1.68x | 30/30 |
| materialize | 0.98x | 1.92x | 1.48x | 1.03x | 2.35x | 1.32x | 28/30 |
| perft_bulk_d4 | 0.99x | 4.36x | 1.62x | 1.32x | 2.06x | 1.98x | 30/30 |
| perft_full_d3 | 0.97x | 1.58x | 1.73x | 1.33x | 1.97x | 2.04x | 30/30 |
| fen_parse | 0.97x | 1.24x | 1.39x | 1.54x | 1.63x | 1.57x | 30/30 |
| fen_write_native_ep | 0.91x | 2.87x | 6.33x | 1.17x | 19.30x | 1.51x | 30/30 |
| snapshot | 0.92x | 1.60x | 1.07x | 2.23x | 1.05x | 0.93x | 22/30 |
| san_parse_all | 0.95x | 1.79x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.29x | N/A | 2.62x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 0.98x | 1.06x | 4.97x | 1.00x | 1.16x | 22/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.20x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.23x | 69.40x | 156.93x | 166.38x | 138.97x | 5/5 |
| chess960_materialize | 0.92x | 1.54x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 3.24x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 36.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 7.04x | 2.25x | 1.38x | 2.74x | 3.03x | 10/10 |
| corpus_materialize | 0.99x | 1.52x | 1.38x | 1.09x | 1.86x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.81x | 2.70x | 1.88x | 2.43x | 1.36x | 10/10 |
| replay_trusted_native | 0.99x | 0.59x | 1.46x | 1.59x | 1.63x | 1.62x | 8/10 |
| replay_checked_native | 1.03x | 2.13x | 0.97x | 7.65x | 6.73x | 8.10x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
