| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.89x | 1.46x | 1.33x | 1.80x | 1.29x | 29/30 |
| materialize | 0.95x | 2.12x | 1.63x | 1.23x | 2.18x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.83x | 1.51x | 1.19x | 1.90x | 1.63x | 30/30 |
| perft_full_d3 | 1.02x | 1.25x | 1.49x | 1.15x | 1.74x | 1.80x | 30/30 |
| fen_parse | 0.84x | 1.37x | 1.37x | 1.07x | 1.77x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 1.86x | 6.23x | 1.20x | 18.86x | 1.46x | 30/30 |
| snapshot | 1.00x | 1.72x | 1.21x | 2.82x | 1.14x | 0.92x | 24/30 |
| san_parse_all | 0.99x | 2.22x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.25x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.92x | 1.17x | 4.50x | 1.10x | 1.12x | 25/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.31x | 73.30x | 101.94x | 156.57x | 112.18x | 5/5 |
| chess960_materialize | 1.00x | 1.80x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.17x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.91x | 0.90x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 21.34x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.11x | 1.75x | 1.23x | 1.91x | 1.84x | 10/10 |
| corpus_materialize | 1.00x | 1.95x | 1.77x | 1.30x | 2.83x | 1.65x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.06x | 2.96x | 2.26x | 2.78x | 1.61x | 10/10 |
| replay_trusted_native | 1.00x | 0.69x | 1.36x | 1.49x | 2.18x | 1.98x | 8/10 |
| replay_checked_native | 0.99x | 2.20x | 0.98x | 5.34x | 5.89x | 7.39x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
