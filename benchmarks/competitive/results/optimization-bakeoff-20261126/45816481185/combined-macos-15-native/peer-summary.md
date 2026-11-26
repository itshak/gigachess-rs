| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 4.49x | 1.51x | 1.58x | 2.07x | 1.85x | 30/30 |
| materialize | 0.98x | 1.95x | 1.47x | 1.02x | 2.26x | 1.41x | 27/30 |
| perft_bulk_d4 | 0.98x | 3.63x | 1.34x | 1.09x | 1.71x | 1.70x | 29/30 |
| perft_full_d3 | 0.95x | 1.58x | 1.77x | 1.33x | 2.06x | 2.00x | 30/30 |
| fen_parse | 0.90x | 1.26x | 1.28x | 1.31x | 1.64x | 1.36x | 30/30 |
| fen_write_native_ep | 0.93x | 4.23x | 9.16x | 1.75x | 27.83x | 2.20x | 30/30 |
| snapshot | 0.97x | 1.69x | 1.11x | 2.36x | 1.10x | 0.97x | 24/30 |
| san_parse_all | 1.03x | 1.75x | N/A | 3.66x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.70x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.04x | 1.09x | 5.12x | 1.06x | 1.21x | 26/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.29x | 75.40x | 155.79x | 185.17x | 156.28x | 5/5 |
| chess960_materialize | 0.95x | 1.73x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.61x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 40.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.91x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.84x | 17.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 5.10x | 1.59x | 0.91x | 1.91x | 1.95x | 8/10 |
| corpus_materialize | 1.03x | 1.43x | 1.40x | 1.11x | 1.84x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.86x | 2.60x | 1.94x | 2.57x | 1.41x | 10/10 |
| replay_trusted_native | 0.94x | 0.57x | 1.45x | 1.56x | 1.68x | 1.72x | 8/10 |
| replay_checked_native | 0.94x | 2.42x | 1.05x | 8.19x | 7.07x | 8.88x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
