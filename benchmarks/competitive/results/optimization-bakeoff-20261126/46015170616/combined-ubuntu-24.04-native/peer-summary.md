| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 5.04x | 1.79x | 1.67x | 2.46x | 2.12x | 30/30 |
| materialize | 1.02x | 2.15x | 1.67x | 1.22x | 2.42x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.40x | 1.68x | 1.32x | 2.52x | 2.37x | 30/30 |
| perft_full_d3 | 1.66x | 1.74x | 2.03x | 1.53x | 2.65x | 2.58x | 30/30 |
| fen_parse | 0.92x | 1.28x | 1.43x | 1.13x | 1.90x | 1.15x | 29/30 |
| fen_write_native_ep | 1.00x | 3.79x | 8.97x | 1.48x | 25.88x | 1.92x | 30/30 |
| snapshot | 1.03x | 2.48x | 1.49x | 4.49x | 1.49x | 1.50x | 30/30 |
| san_parse_all | 1.00x | 1.77x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.98x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.93x | 1.42x | 4.08x | 1.38x | 1.93x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.24x | 84.90x | 156.84x | 228.65x | 155.56x | 5/5 |
| chess960_materialize | 0.98x | 1.76x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.89x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 36.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 23.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.23x | 1.88x | 1.25x | 2.43x | 2.32x | 9/10 |
| corpus_materialize | 1.03x | 1.71x | 1.56x | 1.16x | 2.23x | 1.40x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.33x | 3.03x | 2.42x | 2.97x | 1.66x | 10/10 |
| replay_trusted_native | 1.03x | 0.78x | 1.46x | 1.63x | 2.52x | 3.03x | 9/10 |
| replay_checked_native | 1.02x | 2.60x | 1.25x | 9.11x | 9.22x | 11.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
