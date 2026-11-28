| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 5.09x | 1.75x | 1.81x | 2.76x | 2.06x | 30/30 |
| materialize | 1.02x | 1.99x | 1.67x | 1.31x | 2.33x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.29x | 1.65x | 1.49x | 2.56x | 2.02x | 30/30 |
| perft_full_d3 | 1.23x | 1.63x | 1.86x | 1.63x | 2.39x | 2.21x | 30/30 |
| fen_parse | 1.02x | 1.33x | 1.44x | 1.18x | 2.14x | 1.35x | 30/30 |
| fen_write_native_ep | 0.99x | 4.16x | 9.95x | 1.69x | 27.81x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.97x | 1.25x | 4.14x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.98x | 2.03x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.43x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.32x | 1.32x | 4.63x | 1.47x | 1.62x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.44x | 141.36x | 221.07x | 144.41x | 5/5 |
| chess960_materialize | 1.01x | 1.61x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.91x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 29.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.28x | 1.32x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 21.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.06x | 1.98x | 1.52x | 3.00x | 2.51x | 10/10 |
| corpus_materialize | 0.97x | 1.59x | 1.52x | 1.19x | 2.04x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.46x | 3.24x | 2.50x | 3.10x | 1.77x | 10/10 |
| replay_trusted_native | 0.96x | 0.88x | 1.57x | 1.89x | 2.49x | 2.67x | 9/10 |
| replay_checked_native | 0.96x | 2.45x | 1.08x | 8.54x | 8.25x | 9.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
