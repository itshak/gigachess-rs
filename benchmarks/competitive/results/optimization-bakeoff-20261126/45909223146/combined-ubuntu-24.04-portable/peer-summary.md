| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.05x | 1.73x | 1.51x | 1.99x | 1.44x | 30/30 |
| materialize | 1.00x | 2.30x | 1.84x | 1.90x | 2.80x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.20x | 1.70x | 1.27x | 2.11x | 1.89x | 30/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.64x | 1.42x | 1.85x | 1.62x | 30/30 |
| fen_parse | 1.00x | 1.72x | 1.45x | 1.17x | 1.50x | 1.27x | 30/30 |
| fen_write_native_ep | 1.00x | 3.06x | 10.01x | 1.95x | 29.22x | 2.30x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 30/30 |
| san_parse_all | 0.93x | 1.79x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.57x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.26x | 1.45x | 4.34x | 1.38x | 1.58x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.21x | 106.18x | 121.25x | 111.17x | 5/5 |
| chess960_materialize | 1.00x | 1.81x | 1.75x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.16x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 41.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 21.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.16x | 4.80x | 2.40x | 1.31x | 2.45x | 3.29x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.41x | 1.14x | 1.82x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.96x | 3.03x | 2.24x | 2.95x | 1.58x | 10/10 |
| replay_trusted_native | 1.00x | 0.88x | 1.69x | 1.61x | 2.26x | 2.08x | 9/10 |
| replay_checked_native | 1.01x | 2.07x | 1.17x | 8.86x | 8.78x | 9.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
