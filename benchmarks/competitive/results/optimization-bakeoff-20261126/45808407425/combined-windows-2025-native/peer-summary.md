| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.13x | 1.73x | 1.50x | 2.06x | 1.81x | 30/30 |
| materialize | 0.98x | 1.98x | 1.36x | 1.54x | 2.33x | 1.57x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.81x | 1.62x | 1.17x | 2.14x | 1.84x | 30/30 |
| perft_full_d3 | 0.92x | 1.47x | 1.64x | 1.49x | 2.12x | 1.72x | 30/30 |
| fen_parse | 0.89x | 1.07x | 1.37x | 1.16x | 1.57x | 1.32x | 30/30 |
| fen_write_native_ep | 0.99x | 3.87x | 7.61x | 1.70x | 29.66x | 2.17x | 30/30 |
| snapshot | 0.96x | 2.21x | 1.56x | 3.97x | 1.64x | 1.40x | 30/30 |
| san_parse_all | 0.93x | 1.63x | N/A | 4.91x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 8.67x | N/A | 3.66x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.69x | 1.12x | 8.25x | 1.17x | 1.33x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 1.20x | 1.26x | 77.73x | 156.60x | 207.16x | 178.77x | 5/5 |
| chess960_materialize | 0.94x | 1.46x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.20x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 31.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.92x | 23.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.73x | 2.71x | 1.34x | 2.99x | 2.91x | 10/10 |
| corpus_materialize | 0.99x | 1.42x | 1.23x | 1.09x | 1.77x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.79x | 2.17x | 1.84x | 2.11x | 1.53x | 10/10 |
| replay_trusted_native | 0.86x | 0.67x | 1.24x | 1.30x | 1.76x | 2.01x | 7/10 |
| replay_checked_native | 0.92x | 1.94x | 1.00x | 7.87x | 7.61x | 8.84x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
