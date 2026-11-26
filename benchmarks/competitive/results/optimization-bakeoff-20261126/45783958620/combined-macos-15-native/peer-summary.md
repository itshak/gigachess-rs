| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.66x | 1.51x | 1.47x | 2.01x | 1.75x | 30/30 |
| materialize | 1.02x | 1.79x | 1.38x | 1.05x | 2.19x | 1.29x | 25/30 |
| perft_bulk_d4 | 1.06x | 4.27x | 1.49x | 1.27x | 1.83x | 1.89x | 30/30 |
| perft_full_d3 | 0.98x | 1.53x | 1.73x | 1.32x | 1.93x | 1.92x | 30/30 |
| fen_parse | 1.10x | 1.28x | 1.51x | 1.33x | 1.65x | 1.46x | 30/30 |
| fen_write_native_ep | 1.10x | 4.35x | 8.89x | 1.74x | 31.43x | 2.26x | 30/30 |
| snapshot | 1.10x | 1.70x | 1.17x | 2.40x | 1.04x | 0.95x | 22/30 |
| san_parse_all | 0.98x | 1.94x | N/A | 4.49x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.05x | N/A | 2.82x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 0.93x | 1.00x | 4.62x | 0.94x | 1.09x | 18/30 |
| make_unmake_full | 1.08x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.54x | 1.17x | 76.02x | 221.28x | 192.44x | 150.39x | 5/5 |
| chess960_materialize | 0.91x | 1.52x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 2.94x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 42.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 20.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.68x | 5.36x | 1.82x | 1.44x | 2.58x | 2.52x | 10/10 |
| corpus_materialize | 0.77x | 1.52x | 1.33x | 1.27x | 1.75x | 1.16x | 10/10 |
| corpus_uci_sorted | 0.87x | 1.89x | 2.50x | 1.79x | 2.24x | 1.42x | 10/10 |
| replay_trusted_native | 0.90x | 0.45x | 1.28x | 1.36x | 1.30x | 1.41x | 8/10 |
| replay_checked_native | 0.88x | 2.09x | 0.98x | 7.11x | 6.05x | 9.07x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
