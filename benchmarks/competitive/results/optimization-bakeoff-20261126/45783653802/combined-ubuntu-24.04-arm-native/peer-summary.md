| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.46x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 0.99x | 1.81x | 1.43x | 1.24x | 1.99x | 1.37x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.71x | 1.51x | 1.38x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.18x | 1.28x | 1.07x | 1.56x | 1.50x | 29/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.28x | 1.76x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 3.01x | 6.15x | 1.05x | 19.88x | 1.62x | 29/30 |
| snapshot | 0.94x | 1.19x | 0.96x | 1.53x | 0.93x | 0.95x | 12/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.85x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.11x | 1.30x | 3.63x | 1.07x | 1.07x | 27/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.18x | 73.24x | 127.18x | 164.72x | 131.35x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.63x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.76x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.86x | 1.85x | 1.47x | 2.18x | 2.40x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.49x | 1.19x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.09x | 2.83x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.50x | 1.31x | 1.61x | 1.76x | 1.81x | 8/10 |
| replay_checked_native | 1.00x | 2.05x | 0.99x | 6.72x | 5.95x | 7.74x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
