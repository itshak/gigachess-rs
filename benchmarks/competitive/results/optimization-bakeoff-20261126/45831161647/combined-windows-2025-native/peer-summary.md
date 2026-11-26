| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.45x | 1.53x | 1.43x | 2.28x | 1.88x | 30/30 |
| materialize | 1.03x | 2.06x | 1.35x | 1.15x | 2.39x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.87x | 1.44x | 1.26x | 2.19x | 2.04x | 30/30 |
| perft_full_d3 | 0.98x | 1.34x | 1.59x | 1.22x | 1.91x | 2.11x | 30/30 |
| fen_parse | 1.07x | 1.38x | 1.59x | 1.70x | 2.26x | 1.58x | 30/30 |
| fen_write_native_ep | 0.96x | 3.49x | 6.85x | 1.43x | 30.28x | 1.95x | 30/30 |
| snapshot | 0.97x | 2.42x | 1.49x | 5.29x | 1.45x | 1.46x | 30/30 |
| san_parse_all | 1.00x | 1.97x | N/A | 4.70x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.71x | N/A | 3.82x | N/A | N/A | 12/12 |
| apply_children_full | 0.72x | 1.31x | 0.96x | 6.34x | 0.99x | 1.35x | 25/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.00x | 74.19x | 149.85x | 190.72x | 148.72x | 5/5 |
| chess960_materialize | 0.98x | 1.72x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.90x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 36.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 29.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.47x | 1.66x | 1.09x | 2.16x | 2.08x | 9/10 |
| corpus_materialize | 1.03x | 1.57x | 1.21x | 0.99x | 2.10x | 1.34x | 9/10 |
| corpus_uci_sorted | 1.13x | 1.61x | 2.02x | 1.75x | 1.97x | 1.41x | 10/10 |
| replay_trusted_native | 0.90x | 0.67x | 1.30x | 1.57x | 2.31x | 2.78x | 8/10 |
| replay_checked_native | 0.98x | 2.45x | 1.08x | 8.24x | 8.77x | 11.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
