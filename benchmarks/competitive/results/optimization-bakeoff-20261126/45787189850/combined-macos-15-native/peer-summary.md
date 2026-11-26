| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.30x | 1.32x | 1.50x | 1.99x | 1.75x | 30/30 |
| materialize | 1.00x | 2.00x | 1.54x | 1.03x | 2.28x | 1.27x | 27/30 |
| perft_bulk_d4 | 1.10x | 4.21x | 1.63x | 1.37x | 2.03x | 1.98x | 30/30 |
| perft_full_d3 | 1.10x | 1.59x | 1.73x | 1.33x | 2.07x | 2.07x | 30/30 |
| fen_parse | 0.96x | 1.22x | 1.46x | 1.40x | 1.75x | 1.46x | 30/30 |
| fen_write_native_ep | 0.95x | 4.05x | 8.34x | 1.63x | 28.93x | 2.09x | 30/30 |
| snapshot | 0.93x | 1.65x | 1.14x | 2.47x | 1.18x | 1.00x | 27/30 |
| san_parse_all | 0.92x | 1.94x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 0.88x | 11.15x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 0.97x | 1.01x | 4.67x | 0.97x | 1.05x | 18/30 |
| make_unmake_full | 1.08x | N/A | N/A | 1.14x | N/A | N/A | 5/6 |
| startpos | 0.87x | 1.23x | 66.42x | 126.74x | 151.55x | 129.52x | 5/5 |
| chess960_materialize | 1.07x | 1.62x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.27x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 38.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.72x | 0.89x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.05x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 6.22x | 1.83x | 1.29x | 2.46x | 2.63x | 10/10 |
| corpus_materialize | 0.85x | 1.34x | 1.34x | 1.16x | 2.40x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.08x | 2.19x | 3.62x | 2.41x | 3.23x | 1.75x | 10/10 |
| replay_trusted_native | 0.75x | 0.53x | 1.38x | 1.26x | 1.40x | 1.50x | 8/10 |
| replay_checked_native | 0.93x | 2.10x | 0.94x | 7.00x | 6.46x | 8.39x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
