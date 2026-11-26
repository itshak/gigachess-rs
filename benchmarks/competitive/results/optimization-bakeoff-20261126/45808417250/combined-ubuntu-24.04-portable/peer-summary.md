| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.68x | 1.51x | 1.31x | 1.77x | 1.25x | 28/30 |
| materialize | 1.00x | 2.20x | 1.87x | 1.79x | 2.80x | 1.96x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.97x | 1.57x | 1.17x | 1.96x | 1.76x | 30/30 |
| perft_full_d3 | 1.00x | 1.54x | 1.80x | 1.62x | 2.21x | 1.81x | 30/30 |
| fen_parse | 1.04x | 1.59x | 1.49x | 1.15x | 1.56x | 1.20x | 30/30 |
| fen_write_native_ep | 1.00x | 2.98x | 9.84x | 2.04x | 26.93x | 2.23x | 30/30 |
| snapshot | 0.80x | 1.21x | 1.00x | 2.70x | 1.00x | 1.00x | 22/30 |
| san_parse_all | 1.01x | 1.91x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.41x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.95x | 1.26x | 3.66x | 1.19x | 1.39x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.28x | 57.13x | 104.40x | 130.77x | 100.44x | 5/5 |
| chess960_materialize | 0.93x | 1.56x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.90x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.68x | 14.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.64x | 2.33x | 1.27x | 2.35x | 3.18x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.43x | 1.16x | 1.88x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.97x | 2.95x | 2.25x | 2.92x | 1.57x | 10/10 |
| replay_trusted_native | 0.95x | 0.74x | 1.42x | 1.44x | 1.96x | 1.78x | 8/10 |
| replay_checked_native | 0.98x | 1.95x | 1.06x | 8.31x | 8.25x | 9.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
