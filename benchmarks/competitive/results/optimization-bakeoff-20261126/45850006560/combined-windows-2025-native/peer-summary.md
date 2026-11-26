| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.55x | 1.65x | 1.60x | 2.38x | 2.00x | 30/30 |
| materialize | 0.99x | 2.06x | 1.35x | 1.14x | 2.30x | 1.46x | 30/30 |
| perft_bulk_d4 | 1.08x | 4.02x | 1.65x | 1.45x | 2.49x | 2.26x | 30/30 |
| perft_full_d3 | 0.86x | 1.23x | 1.28x | 1.06x | 1.75x | 1.59x | 30/30 |
| fen_parse | 0.94x | 1.28x | 1.32x | 1.27x | 2.15x | 1.36x | 30/30 |
| fen_write_native_ep | 1.01x | 3.98x | 7.70x | 1.64x | 31.36x | 2.02x | 30/30 |
| snapshot | 0.77x | 1.61x | 1.16x | 2.63x | 1.16x | 1.07x | 30/30 |
| san_parse_all | 0.89x | 1.72x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 8.54x | N/A | 3.05x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 2.50x | 1.95x | 7.62x | 1.95x | 2.14x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.08x | 81.65x | 134.73x | 211.45x | 149.84x | 5/5 |
| chess960_materialize | 0.90x | 1.55x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.70x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 21.11x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.94x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.29x | 1.76x | 1.29x | 2.31x | 2.09x | 9/10 |
| corpus_materialize | 0.96x | 1.78x | 1.34x | 1.14x | 2.75x | 1.48x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.83x | 2.34x | 1.95x | 2.25x | 1.47x | 10/10 |
| replay_trusted_native | 0.72x | 0.85x | 1.43x | 1.79x | 2.97x | 3.76x | 8/10 |
| replay_checked_native | 0.92x | 2.12x | 0.92x | 5.42x | 6.23x | 7.07x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
