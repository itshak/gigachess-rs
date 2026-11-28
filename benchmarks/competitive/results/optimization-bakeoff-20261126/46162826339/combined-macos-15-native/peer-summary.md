| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 5.34x | 1.72x | 1.89x | 2.36x | 2.07x | 30/30 |
| materialize | 0.95x | 1.92x | 1.52x | 1.18x | 2.23x | 1.52x | 29/30 |
| perft_bulk_d4 | 0.94x | 4.01x | 1.44x | 1.25x | 1.89x | 1.76x | 30/30 |
| perft_full_d3 | 1.15x | 1.72x | 1.88x | 1.41x | 2.10x | 2.11x | 30/30 |
| fen_parse | 1.04x | 1.29x | 1.43x | 1.44x | 1.68x | 1.60x | 30/30 |
| fen_write_native_ep | 1.09x | 4.17x | 8.85x | 1.76x | 27.26x | 2.16x | 30/30 |
| snapshot | 1.11x | 1.60x | 1.13x | 2.40x | 1.14x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.74x | N/A | 3.66x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.77x | N/A | 3.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.15x | 1.30x | 6.20x | 1.26x | 1.39x | 29/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.98x | 0.87x | 51.64x | 116.79x | 143.66x | 126.33x | 4/5 |
| chess960_materialize | 0.83x | 1.70x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 3.42x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 37.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.45x | 1.46x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.16x | 21.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.44x | 5.95x | 2.41x | 1.24x | 2.45x | 2.72x | 9/10 |
| corpus_materialize | 1.17x | 1.51x | 1.50x | 1.30x | 2.18x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.07x | 1.86x | 2.74x | 2.32x | 2.80x | 1.50x | 10/10 |
| replay_trusted_native | 0.91x | 0.70x | 1.88x | 1.73x | 1.85x | 1.86x | 8/10 |
| replay_checked_native | 0.89x | 2.83x | 1.22x | 10.25x | 9.79x | 11.66x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
