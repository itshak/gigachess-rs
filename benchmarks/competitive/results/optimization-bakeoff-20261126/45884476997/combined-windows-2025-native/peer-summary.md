| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.79x | 1.82x | 1.65x | 2.57x | 2.21x | 30/30 |
| materialize | 0.98x | 2.02x | 1.34x | 1.16x | 2.34x | 1.52x | 29/30 |
| perft_bulk_d4 | 1.00x | 4.00x | 1.64x | 1.46x | 2.46x | 2.24x | 30/30 |
| perft_full_d3 | 1.26x | 1.94x | 2.00x | 1.62x | 2.67x | 2.35x | 30/30 |
| fen_parse | 0.91x | 1.34x | 1.40x | 1.37x | 2.01x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.06x | 7.79x | 1.66x | 32.07x | 2.05x | 30/30 |
| snapshot | 0.95x | 1.71x | 1.23x | 3.94x | 1.23x | 1.13x | 29/30 |
| san_parse_all | 0.99x | 2.00x | N/A | 4.74x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 10.40x | N/A | 3.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.09x | 1.34x | 8.46x | 1.59x | 1.74x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 85.89x | 136.39x | 196.32x | 141.79x | 4/5 |
| chess960_materialize | 1.02x | 1.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.92x | 1.62x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.43x | 1.77x | 1.30x | 2.36x | 2.14x | 9/10 |
| corpus_materialize | 1.01x | 1.78x | 1.39x | 1.19x | 2.83x | 1.51x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.89x | 2.40x | 1.96x | 2.26x | 1.49x | 10/10 |
| replay_trusted_native | 0.85x | 0.91x | 1.62x | 1.96x | 3.21x | 3.86x | 9/10 |
| replay_checked_native | 0.95x | 2.34x | 1.03x | 6.10x | 7.04x | 8.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
