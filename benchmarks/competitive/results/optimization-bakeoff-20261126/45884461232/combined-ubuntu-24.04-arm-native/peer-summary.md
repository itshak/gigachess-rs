| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.68x | 1.81x | 2.18x | 2.16x | 30/30 |
| materialize | 1.00x | 1.96x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.58x | 1.43x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.34x | 1.96x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.33x | 1.26x | 1.74x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.29x | 1.57x | 29.95x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.09x | 1.02x | 1.47x | 1.03x | 0.95x | 20/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.38x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.22x | 1.44x | 3.97x | 1.17x | 1.17x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.50x | 129.45x | 174.72x | 142.92x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.96x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.42x | 1.71x | 1.40x | 2.01x | 2.23x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.54x | 1.24x | 1.92x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.92x | 0.57x | 1.52x | 1.84x | 2.05x | 2.09x | 8/10 |
| replay_checked_native | 0.93x | 2.14x | 1.06x | 7.12x | 6.39x | 8.30x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
