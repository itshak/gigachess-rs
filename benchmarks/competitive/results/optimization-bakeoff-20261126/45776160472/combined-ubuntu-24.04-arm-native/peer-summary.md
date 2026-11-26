| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.65x | 1.40x | 1.50x | 1.81x | 1.81x | 30/30 |
| materialize | 1.01x | 1.77x | 1.40x | 1.24x | 1.96x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.51x | 1.44x | 1.30x | 1.83x | 1.88x | 30/30 |
| perft_full_d3 | 1.00x | 0.83x | 0.90x | 0.75x | 1.09x | 1.05x | 11/30 |
| fen_parse | 0.99x | 1.24x | 1.33x | 1.26x | 1.76x | 1.39x | 30/30 |
| fen_write_native_ep | 1.51x | 4.51x | 9.40x | 1.59x | 29.40x | 2.46x | 30/30 |
| snapshot | 1.11x | 1.21x | 1.05x | 1.60x | 1.05x | 1.02x | 30/30 |
| san_parse_all | 1.00x | 1.80x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.16x | N/A | 2.42x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 0.89x | 1.05x | 2.87x | 0.86x | 0.86x | 10/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.21x | 73.86x | 124.76x | 171.09x | 136.34x | 5/5 |
| chess960_materialize | 1.00x | 1.41x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.41x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.51x | 1.75x | 1.40x | 2.12x | 2.28x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.49x | 1.21x | 1.88x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.41x | 1.95x | 1.39x | 1.89x | 0.97x | 8/10 |
| replay_trusted_native | 0.98x | 0.41x | 1.07x | 1.31x | 1.44x | 1.49x | 8/10 |
| replay_checked_native | 0.99x | 1.83x | 0.90x | 6.15x | 5.45x | 6.99x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
