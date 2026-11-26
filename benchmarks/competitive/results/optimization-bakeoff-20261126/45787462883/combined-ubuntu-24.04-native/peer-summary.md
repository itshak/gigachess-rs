| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.99x | 1.72x | 1.45x | 2.26x | 1.78x | 30/30 |
| materialize | 1.02x | 2.10x | 1.61x | 1.67x | 2.49x | 1.87x | 30/30 |
| perft_bulk_d4 | 0.86x | 3.31x | 1.36x | 1.03x | 2.03x | 1.63x | 28/30 |
| perft_full_d3 | 0.65x | 0.99x | 1.14x | 1.10x | 1.42x | 1.20x | 24/30 |
| fen_parse | 1.00x | 1.23x | 1.30x | 1.14x | 1.43x | 1.11x | 30/30 |
| fen_write_native_ep | 1.06x | 5.02x | 10.22x | 1.91x | 28.27x | 2.26x | 30/30 |
| snapshot | 0.71x | 1.58x | 1.24x | 2.83x | 1.25x | 1.15x | 30/30 |
| san_parse_all | 1.02x | 2.03x | N/A | 4.48x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 7.39x | N/A | 1.73x | N/A | N/A | 12/12 |
| apply_children_full | 0.58x | 1.03x | 0.63x | 1.90x | 0.61x | 0.80x | 8/30 |
| make_unmake_full | 0.67x | N/A | N/A | 0.75x | N/A | N/A | 1/6 |
| startpos | 0.91x | 1.28x | 80.83x | 148.24x | 178.70x | 145.09x | 5/5 |
| chess960_materialize | 0.94x | 1.58x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 1.98x | 1.20x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 39.20x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 19.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.53x | 2.47x | 1.34x | 2.99x | 2.73x | 10/10 |
| corpus_materialize | 0.99x | 1.42x | 1.33x | 1.10x | 1.73x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 3.11x | 2.23x | 2.99x | 1.58x | 10/10 |
| replay_trusted_native | 0.99x | 0.69x | 1.27x | 1.27x | 1.99x | 2.29x | 8/10 |
| replay_checked_native | 0.87x | 1.76x | 0.98x | 7.56x | 7.18x | 8.98x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
