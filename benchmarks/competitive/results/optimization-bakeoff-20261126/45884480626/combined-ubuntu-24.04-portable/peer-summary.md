| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.87x | 1.64x | 1.43x | 1.88x | 1.37x | 30/30 |
| materialize | 0.99x | 2.37x | 1.84x | 1.90x | 2.77x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.99x | 1.59x | 1.19x | 1.96x | 1.78x | 29/30 |
| perft_full_d3 | 1.21x | 1.65x | 1.91x | 1.68x | 2.24x | 1.91x | 30/30 |
| fen_parse | 1.11x | 1.79x | 1.51x | 1.18x | 1.59x | 1.23x | 30/30 |
| fen_write_native_ep | 1.02x | 2.99x | 9.76x | 1.79x | 27.59x | 2.36x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.01x | 1.93x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.38x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.26x | 1.45x | 4.25x | 1.39x | 1.59x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.59x | 106.88x | 132.51x | 107.31x | 5/5 |
| chess960_materialize | 1.00x | 1.81x | 1.77x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.12x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 43.91x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 21.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.14x | 2.08x | 1.14x | 2.11x | 2.84x | 10/10 |
| corpus_materialize | 1.00x | 1.47x | 1.40x | 1.13x | 1.83x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.95x | 2.96x | 2.23x | 2.90x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.89x | 1.64x | 1.62x | 2.25x | 2.05x | 9/10 |
| replay_checked_native | 1.01x | 2.12x | 1.15x | 8.99x | 8.94x | 10.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
