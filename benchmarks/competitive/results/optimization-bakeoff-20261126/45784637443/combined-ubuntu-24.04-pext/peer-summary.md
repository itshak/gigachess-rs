| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.93x | 1.65x | 1.67x | 2.67x | 1.92x | 30/30 |
| materialize | 1.00x | 2.23x | 1.72x | 1.18x | 2.26x | 1.46x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.26x | 1.65x | 1.37x | 2.67x | 2.15x | 30/30 |
| perft_full_d3 | 1.00x | 1.23x | 1.28x | 0.97x | 1.84x | 1.45x | 26/30 |
| fen_parse | 1.08x | 1.43x | 1.61x | 1.19x | 2.01x | 1.45x | 30/30 |
| fen_write_native_ep | 1.00x | 4.04x | 9.90x | 1.86x | 27.14x | 2.09x | 30/30 |
| snapshot | 1.23x | 1.91x | 1.25x | 3.21x | 1.38x | 1.31x | 30/30 |
| san_parse_all | 1.00x | 2.23x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.51x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.72x | 1.15x | 3.92x | 1.24x | 1.46x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.91x | 1.00x | 82.46x | 117.94x | 170.43x | 135.49x | 5/5 |
| chess960_materialize | 1.05x | 1.89x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 3.04x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 18.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.08x | 1.92x | 1.48x | 2.67x | 2.21x | 10/10 |
| corpus_materialize | 0.99x | 2.01x | 1.80x | 1.30x | 2.91x | 1.63x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.35x | 3.17x | 2.38x | 2.93x | 1.71x | 10/10 |
| replay_trusted_native | 1.01x | 0.75x | 1.24x | 1.46x | 2.62x | 3.19x | 8/10 |
| replay_checked_native | 1.00x | 2.54x | 0.94x | 5.49x | 6.11x | 7.28x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
