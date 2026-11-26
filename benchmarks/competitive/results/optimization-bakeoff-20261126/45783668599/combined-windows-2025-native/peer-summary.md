| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.31x | 1.50x | 1.45x | 2.12x | 1.87x | 30/30 |
| materialize | 1.01x | 1.93x | 1.31x | 1.20x | 2.20x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.88x | 1.53x | 1.32x | 2.19x | 1.98x | 30/30 |
| perft_full_d3 | 0.95x | 1.27x | 1.34x | 1.21x | 1.76x | 1.69x | 30/30 |
| fen_parse | 1.14x | 1.41x | 1.53x | 1.37x | 2.34x | 1.52x | 30/30 |
| fen_write_native_ep | 1.02x | 2.50x | 5.05x | 1.03x | 19.85x | 1.29x | 28/30 |
| snapshot | 1.00x | 1.62x | 1.19x | 2.45x | 1.22x | 1.03x | 30/30 |
| san_parse_all | 1.00x | 1.99x | N/A | 5.35x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.88x | N/A | 3.70x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.91x | 1.19x | 7.71x | 1.26x | 1.40x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.09x | 66.97x | 128.37x | 194.63x | 143.70x | 5/5 |
| chess960_materialize | 1.07x | 1.51x | 1.27x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.55x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 27.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.39x | 1.79x | 1.35x | 2.47x | 2.45x | 10/10 |
| corpus_materialize | 1.03x | 1.63x | 1.31x | 1.17x | 2.05x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.09x | 1.78x | 2.21x | 1.81x | 2.16x | 1.41x | 10/10 |
| replay_trusted_native | 1.03x | 0.68x | 1.26x | 1.66x | 2.06x | 2.49x | 8/10 |
| replay_checked_native | 1.02x | 2.20x | 0.98x | 7.39x | 7.41x | 8.97x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
