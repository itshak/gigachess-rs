| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.25x | 1.39x | 1.53x | 1.85x | 1.98x | 29/30 |
| materialize | 0.97x | 1.82x | 1.37x | 1.02x | 2.21x | 1.34x | 27/30 |
| perft_bulk_d4 | 1.27x | 4.05x | 1.52x | 1.33x | 1.90x | 1.82x | 30/30 |
| perft_full_d3 | 1.25x | 0.90x | 0.96x | 0.77x | 1.12x | 1.06x | 13/30 |
| fen_parse | 1.04x | 1.29x | 1.36x | 1.42x | 1.63x | 1.54x | 30/30 |
| fen_write_native_ep | 0.88x | 3.10x | 6.49x | 1.27x | 20.95x | 1.59x | 30/30 |
| snapshot | 1.64x | 2.16x | 1.45x | 2.77x | 1.37x | 1.23x | 29/30 |
| san_parse_all | 1.17x | 1.41x | N/A | 3.04x | N/A | N/A | 11/12 |
| san_render_native | 1.61x | 11.30x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.11x | 0.85x | 0.90x | 4.44x | 0.88x | 0.96x | 10/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.06x | N/A | N/A | 3/6 |
| startpos | 1.16x | 1.53x | 68.22x | 137.83x | 171.01x | 143.93x | 5/5 |
| chess960_materialize | 1.18x | 1.46x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.44x | 3.20x | 1.65x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 36.91x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.91x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.20x | 17.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 5.83x | 2.10x | 1.23x | 2.49x | 2.38x | 10/10 |
| corpus_materialize | 0.97x | 1.33x | 1.26x | 1.19x | 1.80x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.37x | 1.16x | 1.49x | 1.15x | 1.44x | 0.86x | 8/10 |
| replay_trusted_native | 0.97x | 0.40x | 0.97x | 1.04x | 1.12x | 1.09x | 4/10 |
| replay_checked_native | 1.14x | 1.72x | 0.78x | 6.32x | 5.74x | 7.47x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
