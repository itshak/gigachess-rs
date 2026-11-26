| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.36x | 1.55x | 1.52x | 2.10x | 1.92x | 30/30 |
| materialize | 0.93x | 1.97x | 1.34x | 1.23x | 2.14x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.81x | 3.28x | 1.29x | 1.15x | 1.93x | 1.61x | 30/30 |
| perft_full_d3 | 0.92x | 1.32x | 1.37x | 1.21x | 1.81x | 1.68x | 29/30 |
| fen_parse | 1.04x | 1.36x | 1.37x | 1.37x | 2.22x | 1.43x | 30/30 |
| fen_write_native_ep | 0.95x | 2.55x | 5.06x | 0.97x | 18.70x | 1.30x | 26/30 |
| snapshot | 1.28x | 1.88x | 1.25x | 3.62x | 1.26x | 1.35x | 30/30 |
| san_parse_all | 0.79x | 1.68x | N/A | 4.62x | N/A | N/A | 12/12 |
| san_render_native | 0.87x | 9.80x | N/A | 3.47x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.62x | 1.03x | 6.96x | 1.06x | 1.24x | 24/30 |
| make_unmake_full | 0.79x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 1.17x | 1.58x | 83.77x | 181.19x | 241.95x | 184.18x | 5/5 |
| chess960_materialize | 0.96x | 1.50x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.80x | 2.18x | 1.11x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 30.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.84x | 0.93x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 21.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.18x | 1.78x | 1.46x | 2.43x | 2.31x | 10/10 |
| corpus_materialize | 0.92x | 1.71x | 1.27x | 1.11x | 1.95x | 1.33x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.77x | 2.29x | 1.78x | 2.14x | 1.48x | 10/10 |
| replay_trusted_native | 0.83x | 0.56x | 1.09x | 1.42x | 1.86x | 2.18x | 8/10 |
| replay_checked_native | 0.83x | 1.88x | 0.92x | 6.80x | 6.23x | 8.08x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
