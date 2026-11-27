| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.28x | 1.79x | 1.72x | 2.60x | 2.21x | 30/30 |
| materialize | 1.01x | 2.11x | 1.38x | 1.29x | 2.35x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.18x | 1.66x | 1.39x | 2.42x | 2.16x | 30/30 |
| perft_full_d3 | 1.20x | 1.77x | 1.84x | 1.65x | 2.41x | 2.36x | 30/30 |
| fen_parse | 0.96x | 1.44x | 1.46x | 1.39x | 2.45x | 1.62x | 30/30 |
| fen_write_native_ep | 0.91x | 3.87x | 7.54x | 1.48x | 28.90x | 1.83x | 30/30 |
| snapshot | 1.00x | 1.97x | 1.06x | 4.29x | 1.07x | 1.03x | 25/30 |
| san_parse_all | 0.94x | 2.03x | N/A | 5.35x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.99x | N/A | 3.81x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.39x | 1.49x | 10.13x | 1.53x | 1.72x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.30x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.25x | 79.73x | 170.53x | 258.96x | 177.77x | 5/5 |
| chess960_materialize | 1.00x | 1.77x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.90x | 2.81x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 30.20x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.89x | 21.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 6.00x | 1.98x | 1.47x | 2.69x | 2.59x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.27x | 1.26x | 2.19x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.74x | 2.27x | 1.85x | 2.11x | 1.39x | 10/10 |
| replay_trusted_native | 1.02x | 0.91x | 1.68x | 2.18x | 2.79x | 3.21x | 9/10 |
| replay_checked_native | 0.95x | 2.31x | 1.03x | 8.34x | 7.96x | 9.56x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
