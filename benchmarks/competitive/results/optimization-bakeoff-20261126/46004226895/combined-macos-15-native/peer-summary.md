| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.93x | 1.65x | 1.82x | 2.22x | 2.06x | 30/30 |
| materialize | 1.07x | 1.91x | 1.51x | 1.10x | 2.23x | 1.40x | 29/30 |
| perft_bulk_d4 | 1.21x | 4.53x | 1.66x | 1.36x | 2.05x | 1.92x | 30/30 |
| perft_full_d3 | 1.38x | 1.87x | 1.92x | 1.70x | 2.36x | 2.35x | 30/30 |
| fen_parse | 1.05x | 1.29x | 1.31x | 1.31x | 1.73x | 1.51x | 28/30 |
| fen_write_native_ep | 0.98x | 4.44x | 9.23x | 1.84x | 27.77x | 2.23x | 30/30 |
| snapshot | 1.03x | 1.77x | 1.18x | 2.46x | 1.14x | 0.90x | 23/30 |
| san_parse_all | 1.10x | 1.76x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.76x | N/A | 2.87x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 1.12x | 1.26x | 6.40x | 1.21x | 1.49x | 28/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.24x | 1.16x | 73.36x | 137.11x | 175.70x | 168.97x | 5/5 |
| chess960_materialize | 1.26x | 1.93x | 1.78x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.23x | 3.70x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.34x | 37.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.12x | 0.95x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.19x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.38x | 6.17x | 2.11x | 1.28x | 2.41x | 3.15x | 10/10 |
| corpus_materialize | 1.15x | 1.43x | 1.39x | 1.18x | 2.01x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.74x | 2.42x | 1.88x | 2.45x | 1.49x | 10/10 |
| replay_trusted_native | 1.41x | 0.64x | 1.82x | 1.87x | 2.14x | 1.80x | 8/10 |
| replay_checked_native | 1.04x | 2.63x | 1.21x | 8.44x | 8.69x | 8.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
