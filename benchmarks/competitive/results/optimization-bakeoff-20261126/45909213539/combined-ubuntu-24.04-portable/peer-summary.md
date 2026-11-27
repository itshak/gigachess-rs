| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.19x | 1.64x | 1.47x | 2.05x | 1.42x | 30/30 |
| materialize | 1.00x | 2.42x | 1.88x | 1.34x | 2.49x | 1.72x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.06x | 1.64x | 1.29x | 2.04x | 1.75x | 30/30 |
| perft_full_d3 | 1.00x | 1.42x | 1.71x | 1.31x | 1.96x | 1.83x | 30/30 |
| fen_parse | 1.04x | 1.79x | 1.61x | 1.28x | 2.02x | 1.52x | 30/30 |
| fen_write_native_ep | 1.01x | 2.64x | 8.91x | 1.72x | 26.14x | 1.97x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 2.89x | 1.14x | 1.00x | 25/30 |
| san_parse_all | 0.99x | 2.18x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.69x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.42x | 1.54x | 5.73x | 1.42x | 1.47x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 72.59x | 108.37x | 168.86x | 126.31x | 5/5 |
| chess960_materialize | 1.08x | 1.89x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.47x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.24x | 1.78x | 1.24x | 1.98x | 1.85x | 10/10 |
| corpus_materialize | 0.98x | 2.04x | 1.81x | 1.33x | 2.96x | 1.73x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.05x | 2.95x | 2.21x | 2.80x | 1.59x | 10/10 |
| replay_trusted_native | 1.00x | 1.09x | 2.23x | 2.36x | 3.46x | 3.26x | 10/10 |
| replay_checked_native | 1.00x | 2.71x | 1.16x | 6.29x | 7.17x | 8.00x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
