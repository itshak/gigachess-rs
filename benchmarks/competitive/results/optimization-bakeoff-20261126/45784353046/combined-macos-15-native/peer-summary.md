| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.07x | 1.28x | 1.43x | 1.97x | 1.70x | 28/30 |
| materialize | 1.04x | 1.86x | 1.43x | 0.98x | 2.05x | 1.16x | 25/30 |
| perft_bulk_d4 | 1.03x | 3.75x | 1.66x | 1.29x | 1.93x | 1.82x | 30/30 |
| perft_full_d3 | 1.11x | 1.63x | 1.82x | 1.43x | 2.16x | 2.17x | 30/30 |
| fen_parse | 1.03x | 1.27x | 1.43x | 1.32x | 1.66x | 1.41x | 30/30 |
| fen_write_native_ep | 1.10x | 4.06x | 8.69x | 1.76x | 28.74x | 2.18x | 30/30 |
| snapshot | 1.03x | 2.02x | 1.02x | 2.37x | 1.16x | 0.96x | 23/30 |
| san_parse_all | 1.14x | 1.94x | N/A | 4.81x | N/A | N/A | 12/12 |
| san_render_native | 1.14x | 11.22x | N/A | 2.84x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 0.96x | 1.06x | 4.51x | 0.97x | 1.10x | 18/30 |
| make_unmake_full | 1.11x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.52x | 1.48x | 72.03x | 171.77x | 180.50x | 163.62x | 5/5 |
| chess960_materialize | 0.99x | 1.73x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.96x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 42.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.78x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.97x | 19.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.76x | 4.92x | 1.78x | 1.16x | 1.98x | 2.34x | 10/10 |
| corpus_materialize | 0.97x | 1.47x | 1.42x | 1.12x | 1.90x | 1.31x | 9/10 |
| corpus_uci_sorted | 1.07x | 1.94x | 2.86x | 2.18x | 2.69x | 1.60x | 10/10 |
| replay_trusted_native | 0.83x | 0.50x | 1.16x | 1.14x | 1.19x | 1.42x | 8/10 |
| replay_checked_native | 0.99x | 2.26x | 1.01x | 7.68x | 6.17x | 8.45x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
