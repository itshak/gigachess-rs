| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.07x | 1.46x | 1.45x | 2.16x | 1.79x | 30/30 |
| materialize | 0.97x | 1.83x | 1.56x | 1.22x | 2.12x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.74x | 1.50x | 1.29x | 2.24x | 1.89x | 30/30 |
| perft_full_d3 | 0.98x | 1.27x | 1.43x | 1.25x | 1.82x | 1.80x | 30/30 |
| fen_parse | 1.02x | 1.26x | 1.35x | 1.21x | 2.13x | 1.26x | 30/30 |
| fen_write_native_ep | 0.99x | 4.27x | 9.28x | 1.70x | 27.41x | 1.96x | 30/30 |
| snapshot | 1.00x | 1.58x | 1.19x | 2.50x | 1.19x | 1.03x | 30/30 |
| san_parse_all | 0.97x | 1.96x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.78x | N/A | 2.36x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.89x | 1.09x | 3.73x | 1.21x | 1.36x | 28/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 82.81x | 137.56x | 216.89x | 144.43x | 5/5 |
| chess960_materialize | 0.99x | 1.44x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.45x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 24.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 20.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.38x | 1.77x | 1.34x | 2.59x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.64x | 1.56x | 1.19x | 2.03x | 1.41x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.41x | 3.13x | 2.46x | 2.99x | 1.71x | 10/10 |
| replay_trusted_native | 0.98x | 0.70x | 1.29x | 1.54x | 2.04x | 2.28x | 8/10 |
| replay_checked_native | 1.00x | 2.29x | 1.03x | 7.64x | 7.33x | 8.86x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
