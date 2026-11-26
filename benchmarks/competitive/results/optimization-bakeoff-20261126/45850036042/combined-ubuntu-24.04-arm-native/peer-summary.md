| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.67x | 1.80x | 2.17x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.33x | 2.17x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.58x | 1.43x | 2.01x | 2.08x | 30/30 |
| perft_full_d3 | 0.99x | 1.46x | 1.59x | 1.33x | 1.95x | 1.84x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.33x | 1.26x | 1.74x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.49x | 9.39x | 1.59x | 30.18x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.15x | 1.02x | 1.49x | 1.00x | 0.94x | 20/30 |
| san_parse_all | 0.99x | 1.83x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.64x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.02x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.53x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.28x | 129.27x | 172.54x | 139.95x | 5/5 |
| chess960_materialize | 1.00x | 1.70x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.98x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.44x | 1.69x | 1.38x | 1.99x | 2.34x | 10/10 |
| corpus_materialize | 1.01x | 1.61x | 1.53x | 1.23x | 1.91x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.08x | 2.84x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.68x | 1.98x | 2.22x | 2.29x | 8/10 |
| replay_checked_native | 1.00x | 2.19x | 1.05x | 7.02x | 6.34x | 8.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
