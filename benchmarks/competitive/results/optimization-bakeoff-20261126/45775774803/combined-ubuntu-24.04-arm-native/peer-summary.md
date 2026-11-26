| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.67x | 1.40x | 1.50x | 1.81x | 1.81x | 30/30 |
| materialize | 1.00x | 1.77x | 1.41x | 1.22x | 1.96x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.52x | 1.43x | 1.30x | 1.84x | 1.88x | 30/30 |
| perft_full_d3 | 0.99x | 0.83x | 0.90x | 0.75x | 1.09x | 1.05x | 11/30 |
| fen_parse | 0.99x | 1.23x | 1.33x | 1.26x | 1.76x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 2.97x | 6.16x | 1.04x | 19.62x | 1.58x | 28/30 |
| snapshot | 1.06x | 1.21x | 1.06x | 1.60x | 1.05x | 1.02x | 30/30 |
| san_parse_all | 1.01x | 1.78x | N/A | 4.17x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.14x | N/A | 2.44x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 0.89x | 1.05x | 2.91x | 0.86x | 0.86x | 10/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.21x | 73.70x | 124.88x | 167.06x | 134.17x | 5/5 |
| chess960_materialize | 1.00x | 1.40x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.42x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.63x | 1.76x | 1.42x | 2.17x | 2.29x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.49x | 1.19x | 1.89x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.96x | 1.39x | 1.89x | 0.98x | 8/10 |
| replay_trusted_native | 1.00x | 0.41x | 1.08x | 1.34x | 1.44x | 1.51x | 8/10 |
| replay_checked_native | 1.02x | 1.90x | 0.92x | 6.16x | 5.61x | 7.15x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
