| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.19x | 3.67x | 1.40x | 1.50x | 1.81x | 1.81x | 30/30 |
| materialize | 1.03x | 1.77x | 1.43x | 1.22x | 1.98x | 1.36x | 30/30 |
| perft_bulk_d4 | 1.46x | 3.57x | 1.45x | 1.32x | 1.86x | 1.90x | 30/30 |
| perft_full_d3 | 1.25x | 0.84x | 0.89x | 0.76x | 1.08x | 1.04x | 10/30 |
| fen_parse | 1.03x | 1.24x | 1.33x | 1.29x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 3.02x | 6.29x | 1.06x | 19.58x | 1.58x | 29/30 |
| snapshot | 1.22x | 1.17x | 0.95x | 1.50x | 0.91x | 0.95x | 12/30 |
| san_parse_all | 1.94x | 1.80x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 2.08x | 11.14x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.27x | 0.89x | 1.04x | 2.89x | 0.86x | 0.87x | 10/30 |
| make_unmake_full | 1.27x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.55x | 1.18x | 73.41x | 123.97x | 163.02x | 131.55x | 5/5 |
| chess960_materialize | 1.10x | 1.40x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.45x | 2.42x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 26.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.31x | 18.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 4.63x | 1.77x | 1.41x | 2.09x | 2.29x | 10/10 |
| corpus_materialize | 1.07x | 1.58x | 1.52x | 1.21x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.70x | 1.40x | 1.98x | 1.39x | 1.89x | 0.98x | 8/10 |
| replay_trusted_native | 1.28x | 0.42x | 1.08x | 1.30x | 1.43x | 1.48x | 8/10 |
| replay_checked_native | 1.34x | 1.83x | 0.88x | 5.94x | 5.31x | 6.90x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
