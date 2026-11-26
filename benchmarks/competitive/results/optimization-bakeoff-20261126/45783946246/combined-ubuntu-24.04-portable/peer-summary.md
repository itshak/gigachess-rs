| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.73x | 1.51x | 1.31x | 1.74x | 1.26x | 29/30 |
| materialize | 0.79x | 1.85x | 1.42x | 1.47x | 2.18x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.94x | 1.56x | 1.16x | 1.94x | 1.75x | 30/30 |
| perft_full_d3 | 0.99x | 1.51x | 1.80x | 1.57x | 2.19x | 1.81x | 30/30 |
| fen_parse | 1.00x | 1.35x | 1.35x | 1.08x | 1.40x | 1.07x | 30/30 |
| fen_write_native_ep | 0.99x | 3.05x | 10.06x | 2.10x | 27.94x | 2.55x | 30/30 |
| snapshot | 1.11x | 1.75x | 1.25x | 3.26x | 1.26x | 1.00x | 29/30 |
| san_parse_all | 1.11x | 1.93x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.38x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.81x | 1.18x | 3.40x | 1.07x | 1.30x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.96x | 110.37x | 127.63x | 99.99x | 5/5 |
| chess960_materialize | 0.94x | 1.44x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.88x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 40.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.20x | 24.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.63x | 2.30x | 1.28x | 2.37x | 3.17x | 10/10 |
| corpus_materialize | 0.96x | 1.45x | 1.36x | 1.10x | 1.77x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.91x | 2.91x | 2.16x | 2.87x | 1.52x | 10/10 |
| replay_trusted_native | 1.00x | 0.69x | 1.36x | 1.36x | 1.83x | 1.65x | 8/10 |
| replay_checked_native | 1.00x | 1.95x | 1.09x | 8.39x | 8.36x | 9.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
