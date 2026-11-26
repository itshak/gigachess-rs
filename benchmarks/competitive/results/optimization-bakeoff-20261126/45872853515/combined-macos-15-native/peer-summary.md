| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 5.48x | 1.66x | 1.60x | 2.36x | 2.10x | 30/30 |
| materialize | 1.00x | 2.01x | 1.61x | 1.16x | 2.28x | 1.56x | 29/30 |
| perft_bulk_d4 | 1.03x | 3.95x | 1.60x | 1.18x | 1.88x | 1.85x | 30/30 |
| perft_full_d3 | 1.22x | 1.76x | 1.87x | 1.52x | 2.36x | 2.21x | 30/30 |
| fen_parse | 1.09x | 1.37x | 1.57x | 1.38x | 1.97x | 1.58x | 30/30 |
| fen_write_native_ep | 1.03x | 4.26x | 14.88x | 1.81x | 29.25x | 2.28x | 30/30 |
| snapshot | 1.18x | 1.67x | 1.22x | 2.78x | 1.23x | 0.93x | 25/30 |
| san_parse_all | 1.08x | 1.72x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 1.14x | 14.03x | N/A | 2.93x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.07x | 1.17x | 5.94x | 1.14x | 1.29x | 26/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.28x | N/A | N/A | 5/6 |
| startpos | 1.23x | 1.18x | 106.29x | 177.64x | 288.99x | 177.51x | 5/5 |
| chess960_materialize | 0.66x | 1.77x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.72x | 2.77x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.70x | 34.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.86x | 1.16x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.93x | 18.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.91x | 1.93x | 1.26x | 2.18x | 2.76x | 10/10 |
| corpus_materialize | 0.73x | 1.62x | 1.43x | 1.03x | 1.70x | 1.15x | 9/10 |
| corpus_uci_sorted | 0.74x | 1.71x | 2.22x | 1.71x | 2.39x | 1.32x | 10/10 |
| replay_trusted_native | 0.75x | 0.65x | 1.59x | 1.72x | 1.84x | 1.98x | 8/10 |
| replay_checked_native | 0.73x | 2.43x | 1.05x | 8.54x | 6.93x | 9.11x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
