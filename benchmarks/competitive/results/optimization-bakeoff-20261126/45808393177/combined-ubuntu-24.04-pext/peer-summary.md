| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.17x | 1.71x | 1.50x | 2.26x | 1.74x | 30/30 |
| materialize | 1.01x | 2.05x | 1.51x | 1.71x | 2.40x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.84x | 3.40x | 1.34x | 1.05x | 2.05x | 1.72x | 28/30 |
| perft_full_d3 | 0.95x | 1.47x | 1.66x | 1.63x | 2.10x | 1.74x | 30/30 |
| fen_parse | 0.94x | 1.31x | 1.35x | 1.09x | 1.36x | 1.07x | 30/30 |
| fen_write_native_ep | 1.00x | 5.06x | 10.07x | 1.90x | 29.01x | 2.52x | 30/30 |
| snapshot | 1.42x | 1.67x | 1.33x | 4.00x | 1.33x | 1.34x | 30/30 |
| san_parse_all | 0.97x | 1.91x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.21x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 2.01x | 1.27x | 3.88x | 1.19x | 1.65x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.23x | 1.25x | 119.41x | 187.32x | 213.50x | 177.22x | 5/5 |
| chess960_materialize | 0.97x | 1.57x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.31x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 39.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.69x | 4.66x | 1.74x | 0.96x | 2.10x | 1.91x | 8/10 |
| corpus_materialize | 0.95x | 1.37x | 1.26x | 1.06x | 1.68x | 1.14x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.31x | 2.96x | 2.25x | 2.93x | 1.54x | 10/10 |
| replay_trusted_native | 0.96x | 0.80x | 1.36x | 1.40x | 1.95x | 2.26x | 7/10 |
| replay_checked_native | 0.87x | 1.87x | 0.95x | 7.69x | 7.36x | 8.69x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
