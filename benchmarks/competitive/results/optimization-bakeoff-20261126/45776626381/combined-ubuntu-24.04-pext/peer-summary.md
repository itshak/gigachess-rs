| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.35x | 1.50x | 1.56x | 2.33x | 1.78x | 30/30 |
| materialize | 1.03x | 1.99x | 1.67x | 1.34x | 2.30x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.02x | 1.55x | 1.37x | 2.40x | 1.90x | 30/30 |
| perft_full_d3 | 1.53x | 1.29x | 1.42x | 1.25x | 1.83x | 1.71x | 30/30 |
| fen_parse | 1.01x | 1.36x | 1.48x | 1.15x | 2.19x | 1.34x | 30/30 |
| fen_write_native_ep | 0.99x | 2.35x | 5.34x | 0.95x | 16.04x | 1.10x | 25/30 |
| snapshot | 1.15x | 1.16x | 1.14x | 1.86x | 1.14x | 0.86x | 24/30 |
| san_parse_all | 1.05x | 2.07x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 10.92x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.17x | 1.96x | 1.10x | 3.92x | 1.25x | 1.33x | 28/30 |
| make_unmake_full | 1.24x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.16x | 1.33x | 81.29x | 139.29x | 234.46x | 153.68x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.64x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 25.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 19.80x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 6.01x | 1.89x | 1.51x | 2.80x | 2.51x | 10/10 |
| corpus_materialize | 1.01x | 1.70x | 1.61x | 1.22x | 2.21x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.75x | 2.46x | 3.22x | 2.39x | 3.12x | 1.70x | 10/10 |
| replay_trusted_native | 1.20x | 0.71x | 1.26x | 1.53x | 2.13x | 2.40x | 8/10 |
| replay_checked_native | 1.05x | 2.26x | 0.98x | 7.45x | 7.48x | 8.88x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
