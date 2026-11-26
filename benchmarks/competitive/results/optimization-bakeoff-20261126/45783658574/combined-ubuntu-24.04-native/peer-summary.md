| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.01x | 1.72x | 1.46x | 2.30x | 1.78x | 30/30 |
| materialize | 1.00x | 2.09x | 1.67x | 1.67x | 2.66x | 1.84x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.87x | 1.59x | 1.21x | 2.39x | 1.92x | 30/30 |
| perft_full_d3 | 0.99x | 1.49x | 1.72x | 1.66x | 2.18x | 1.85x | 30/30 |
| fen_parse | 1.15x | 1.45x | 1.44x | 1.18x | 1.67x | 1.19x | 30/30 |
| fen_write_native_ep | 0.98x | 2.96x | 6.61x | 1.35x | 19.28x | 1.59x | 30/30 |
| snapshot | 0.71x | 1.58x | 1.25x | 2.84x | 1.26x | 1.16x | 30/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.46x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.15x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.02x | 1.41x | 3.59x | 1.56x | 1.66x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.91x | 1.29x | 80.78x | 147.89x | 189.92x | 144.74x | 5/5 |
| chess960_materialize | 1.06x | 1.57x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.29x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.22x | 24.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.44x | 2.44x | 1.34x | 2.96x | 2.74x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.34x | 1.10x | 1.77x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.26x | 3.06x | 2.29x | 3.00x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.72x | 1.24x | 1.28x | 1.83x | 2.10x | 8/10 |
| replay_checked_native | 1.00x | 2.02x | 1.04x | 8.52x | 8.31x | 9.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
