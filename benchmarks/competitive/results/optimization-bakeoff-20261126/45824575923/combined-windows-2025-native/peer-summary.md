| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 5.05x | 1.72x | 1.63x | 2.44x | 2.13x | 30/30 |
| materialize | 1.02x | 2.12x | 1.36x | 1.19x | 2.41x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.34x | 1.54x | 1.38x | 2.38x | 2.23x | 30/30 |
| perft_full_d3 | 1.04x | 1.44x | 1.52x | 1.29x | 2.00x | 2.04x | 30/30 |
| fen_parse | 0.98x | 1.28x | 1.47x | 1.49x | 2.06x | 1.49x | 30/30 |
| fen_write_native_ep | 0.99x | 3.69x | 7.07x | 1.51x | 31.51x | 2.06x | 30/30 |
| snapshot | 1.00x | 2.57x | 1.50x | 5.46x | 1.51x | 1.51x | 30/30 |
| san_parse_all | 1.00x | 1.95x | N/A | 4.72x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.69x | N/A | 3.80x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.81x | 1.34x | 8.55x | 1.36x | 1.85x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.25x | 89.09x | 183.50x | 232.12x | 168.26x | 5/5 |
| chess960_materialize | 1.09x | 1.69x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.12x | 2.84x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.20x | 38.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 23.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.91x | 1.74x | 1.12x | 2.29x | 2.14x | 9/10 |
| corpus_materialize | 1.04x | 1.65x | 1.26x | 1.06x | 2.22x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.63x | 1.97x | 1.67x | 1.93x | 1.37x | 10/10 |
| replay_trusted_native | 1.06x | 0.72x | 1.41x | 1.66x | 2.43x | 2.91x | 8/10 |
| replay_checked_native | 1.03x | 2.41x | 1.15x | 8.45x | 8.52x | 10.52x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
