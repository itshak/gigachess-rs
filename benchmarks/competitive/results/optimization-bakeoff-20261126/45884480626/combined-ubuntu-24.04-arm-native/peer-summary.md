| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.41x | 1.69x | 1.81x | 2.20x | 2.18x | 30/30 |
| materialize | 1.01x | 1.95x | 1.54x | 1.36x | 2.17x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.91x | 1.59x | 1.44x | 2.03x | 2.08x | 30/30 |
| perft_full_d3 | 1.15x | 1.70x | 1.84x | 1.54x | 2.26x | 2.13x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.32x | 1.25x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.32x | 1.57x | 29.83x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.12x | 1.02x | 1.49x | 1.02x | 0.94x | 23/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.43x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.08x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.29x | 128.51x | 175.49x | 143.07x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.48x | 1.71x | 1.37x | 2.02x | 2.34x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.52x | 1.23x | 1.92x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.87x | 1.49x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.65x | 1.99x | 2.24x | 2.33x | 8/10 |
| replay_checked_native | 1.01x | 2.35x | 1.13x | 7.70x | 6.84x | 9.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
