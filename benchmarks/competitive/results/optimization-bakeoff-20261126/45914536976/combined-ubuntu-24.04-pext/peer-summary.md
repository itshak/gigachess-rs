| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.85x | 1.66x | 1.69x | 2.62x | 1.97x | 30/30 |
| materialize | 1.00x | 1.96x | 1.66x | 1.27x | 2.31x | 1.50x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.27x | 1.59x | 1.43x | 2.50x | 1.94x | 30/30 |
| perft_full_d3 | 1.01x | 1.39x | 1.52x | 1.34x | 2.20x | 1.79x | 30/30 |
| fen_parse | 0.96x | 1.41x | 1.36x | 1.14x | 2.15x | 1.35x | 30/30 |
| fen_write_native_ep | 0.99x | 4.26x | 9.79x | 1.70x | 27.83x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.96x | 1.26x | 3.75x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.01x | 2.11x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.49x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.41x | 1.38x | 4.78x | 1.54x | 1.69x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 80.81x | 140.46x | 226.36x | 158.09x | 5/5 |
| chess960_materialize | 0.97x | 1.57x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.94x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 25.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.71x | 1.81x | 1.43x | 2.68x | 2.36x | 10/10 |
| corpus_materialize | 0.99x | 1.65x | 1.56x | 1.18x | 2.15x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.51x | 3.21x | 2.46x | 3.16x | 1.82x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.66x | 1.93x | 2.56x | 2.83x | 9/10 |
| replay_checked_native | 0.97x | 2.54x | 1.11x | 8.31x | 8.34x | 9.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
