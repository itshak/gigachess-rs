| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.21x | 1.76x | 1.72x | 2.57x | 2.09x | 30/30 |
| materialize | 1.07x | 2.11x | 1.66x | 1.25x | 2.40x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.40x | 1.66x | 1.34x | 2.54x | 2.31x | 30/30 |
| perft_full_d3 | 1.31x | 1.84x | 2.17x | 1.62x | 2.79x | 2.66x | 30/30 |
| fen_parse | 0.93x | 1.30x | 1.47x | 1.15x | 1.95x | 1.25x | 29/30 |
| fen_write_native_ep | 1.00x | 3.90x | 8.83x | 1.48x | 25.76x | 1.92x | 30/30 |
| snapshot | 1.01x | 2.50x | 1.50x | 4.50x | 1.49x | 1.50x | 30/30 |
| san_parse_all | 1.01x | 1.81x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.89x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.91x | 1.40x | 4.07x | 1.37x | 1.91x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.25x | 83.58x | 135.79x | 218.45x | 143.24x | 5/5 |
| chess960_materialize | 0.98x | 1.77x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.95x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 37.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.61x | 1.66x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.83x | 23.49x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 6.48x | 1.88x | 1.29x | 2.51x | 2.32x | 9/10 |
| corpus_materialize | 1.00x | 1.70x | 1.59x | 1.15x | 2.23x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.34x | 3.06x | 2.38x | 3.05x | 1.69x | 10/10 |
| replay_trusted_native | 1.03x | 0.77x | 1.44x | 1.65x | 2.56x | 3.01x | 9/10 |
| replay_checked_native | 1.00x | 2.66x | 1.22x | 9.10x | 9.31x | 11.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
