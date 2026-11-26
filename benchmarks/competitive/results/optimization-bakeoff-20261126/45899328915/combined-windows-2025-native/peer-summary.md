| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.54x | 1.64x | 1.61x | 2.48x | 1.98x | 30/30 |
| materialize | 0.97x | 2.03x | 1.32x | 1.13x | 2.28x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.03x | 1.64x | 1.46x | 2.51x | 2.25x | 30/30 |
| perft_full_d3 | 1.12x | 1.68x | 1.72x | 1.39x | 2.38x | 2.14x | 30/30 |
| fen_parse | 0.91x | 1.31x | 1.38x | 1.33x | 2.00x | 1.41x | 30/30 |
| fen_write_native_ep | 0.92x | 3.66x | 7.05x | 1.52x | 29.95x | 1.86x | 30/30 |
| snapshot | 0.71x | 1.46x | 1.09x | 2.37x | 1.06x | 0.97x | 21/30 |
| san_parse_all | 0.93x | 1.83x | N/A | 4.53x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.43x | N/A | 3.46x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.93x | 1.82x | 8.46x | 1.77x | 1.68x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 0.77x | 0.83x | 61.90x | 106.73x | 154.93x | 116.36x | 4/5 |
| chess960_materialize | 0.93x | 1.59x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.74x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 21.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 18.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.37x | 1.78x | 1.32x | 2.36x | 2.16x | 10/10 |
| corpus_materialize | 0.98x | 1.79x | 1.39x | 1.22x | 2.85x | 1.56x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.83x | 2.35x | 1.94x | 2.24x | 1.47x | 10/10 |
| replay_trusted_native | 0.89x | 0.98x | 1.68x | 2.09x | 3.44x | 4.41x | 9/10 |
| replay_checked_native | 0.91x | 2.26x | 0.99x | 5.78x | 7.09x | 8.27x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
