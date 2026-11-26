| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.59x | 1.48x | 1.29x | 1.72x | 1.23x | 28/30 |
| materialize | 1.00x | 2.16x | 1.62x | 1.73x | 2.47x | 1.81x | 30/30 |
| perft_bulk_d4 | 0.92x | 2.65x | 1.42x | 1.05x | 1.76x | 1.57x | 28/30 |
| perft_full_d3 | 0.85x | 0.83x | 0.99x | 0.87x | 1.17x | 0.97x | 14/30 |
| fen_parse | 0.87x | 1.51x | 1.28x | 1.04x | 1.33x | 1.05x | 25/30 |
| fen_write_native_ep | 1.01x | 2.02x | 6.87x | 1.31x | 18.75x | 1.52x | 30/30 |
| snapshot | 0.82x | 1.19x | 0.91x | 2.46x | 0.91x | 0.82x | 12/30 |
| san_parse_all | 0.93x | 1.60x | N/A | 3.72x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 8.42x | N/A | 1.98x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 1.38x | 0.92x | 2.66x | 0.86x | 1.00x | 16/30 |
| make_unmake_full | 0.81x | N/A | N/A | 0.79x | N/A | N/A | 0/6 |
| startpos | 0.78x | 1.06x | 44.80x | 83.31x | 94.79x | 80.15x | 5/5 |
| chess960_materialize | 0.99x | 1.57x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 1.69x | 1.20x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 36.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.90x | 20.60x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.45x | 2.26x | 1.20x | 2.29x | 3.06x | 10/10 |
| corpus_materialize | 1.00x | 1.53x | 1.42x | 1.15x | 1.88x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.21x | 1.94x | 1.37x | 1.88x | 0.97x | 8/10 |
| replay_trusted_native | 0.89x | 0.53x | 1.03x | 1.00x | 1.36x | 1.25x | 6/10 |
| replay_checked_native | 0.82x | 1.37x | 0.78x | 6.00x | 5.87x | 6.69x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
