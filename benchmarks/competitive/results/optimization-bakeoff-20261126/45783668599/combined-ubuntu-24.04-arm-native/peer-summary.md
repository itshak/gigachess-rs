| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.86x | 1.46x | 1.58x | 1.90x | 1.89x | 30/30 |
| materialize | 1.00x | 1.83x | 1.43x | 1.24x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.53x | 1.38x | 1.95x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.15x | 1.25x | 1.05x | 1.50x | 1.46x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.26x | 1.78x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 2.97x | 6.02x | 1.02x | 19.63x | 1.60x | 28/30 |
| snapshot | 1.00x | 1.14x | 1.03x | 1.59x | 1.01x | 1.03x | 23/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.73x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.12x | 1.33x | 3.60x | 1.07x | 1.07x | 27/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.16x | 66.73x | 112.77x | 154.27x | 123.33x | 5/5 |
| chess960_materialize | 1.01x | 1.51x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.59x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.71x | 1.83x | 1.45x | 2.14x | 2.40x | 10/10 |
| corpus_materialize | 1.01x | 1.57x | 1.50x | 1.20x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.92x | 2.09x | 2.86x | 1.47x | 10/10 |
| replay_trusted_native | 1.00x | 0.50x | 1.32x | 1.61x | 1.78x | 1.83x | 8/10 |
| replay_checked_native | 1.00x | 2.01x | 0.99x | 6.61x | 5.89x | 7.71x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
