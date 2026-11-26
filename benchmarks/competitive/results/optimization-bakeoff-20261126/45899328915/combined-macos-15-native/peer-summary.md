| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 4.64x | 1.57x | 1.59x | 2.22x | 1.84x | 30/30 |
| materialize | 1.00x | 2.07x | 1.63x | 1.07x | 2.25x | 1.43x | 28/30 |
| perft_bulk_d4 | 1.01x | 4.21x | 1.53x | 1.25x | 1.94x | 1.80x | 29/30 |
| perft_full_d3 | 0.86x | 1.46x | 1.66x | 1.21x | 1.73x | 1.84x | 30/30 |
| fen_parse | 0.98x | 1.24x | 1.39x | 1.40x | 1.73x | 1.54x | 30/30 |
| fen_write_native_ep | 0.92x | 3.95x | 8.49x | 1.64x | 25.94x | 2.13x | 30/30 |
| snapshot | 1.02x | 1.64x | 1.13x | 2.33x | 1.18x | 1.17x | 27/30 |
| san_parse_all | 1.00x | 1.81x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 12.35x | N/A | 2.85x | N/A | N/A | 12/12 |
| apply_children_full | 0.46x | 0.54x | 0.59x | 2.59x | 0.56x | 0.63x | 6/30 |
| make_unmake_full | 0.84x | N/A | N/A | 1.17x | N/A | N/A | 5/6 |
| startpos | 0.65x | 0.77x | 49.61x | 106.03x | 131.91x | 114.94x | 4/5 |
| chess960_materialize | 0.82x | 1.64x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.76x | 2.78x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 37.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.74x | 1.99x | 1.26x | 2.28x | 2.53x | 10/10 |
| corpus_materialize | 0.91x | 1.72x | 1.49x | 1.11x | 1.98x | 1.32x | 10/10 |
| corpus_uci_sorted | 0.96x | 2.09x | 2.79x | 1.92x | 2.60x | 1.41x | 10/10 |
| replay_trusted_native | 0.96x | 0.61x | 1.63x | 1.71x | 1.71x | 1.73x | 8/10 |
| replay_checked_native | 0.98x | 2.61x | 1.24x | 9.38x | 8.54x | 9.78x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
