| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.02x | 1.62x | 1.47x | 2.02x | 1.42x | 30/30 |
| materialize | 1.00x | 2.17x | 1.80x | 1.28x | 2.38x | 1.63x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.88x | 1.59x | 1.25x | 1.96x | 1.70x | 30/30 |
| perft_full_d3 | 0.98x | 1.35x | 1.64x | 1.26x | 1.91x | 1.76x | 30/30 |
| fen_parse | 1.17x | 1.78x | 1.57x | 1.32x | 2.04x | 1.44x | 30/30 |
| fen_write_native_ep | 0.99x | 2.68x | 9.26x | 1.69x | 25.87x | 1.96x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 2.87x | 1.14x | 0.98x | 24/30 |
| san_parse_all | 1.00x | 2.01x | N/A | 4.01x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.66x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 0.78x | 1.79x | 1.19x | 4.54x | 1.11x | 1.14x | 27/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 73.02x | 109.78x | 159.14x | 113.59x | 5/5 |
| chess960_materialize | 0.95x | 1.65x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.36x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.11x | 1.77x | 1.25x | 1.98x | 1.83x | 10/10 |
| corpus_materialize | 0.99x | 1.88x | 1.78x | 1.28x | 2.96x | 1.67x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.21x | 3.16x | 2.41x | 2.98x | 1.78x | 10/10 |
| replay_trusted_native | 0.93x | 0.98x | 1.99x | 2.11x | 3.23x | 2.90x | 9/10 |
| replay_checked_native | 0.97x | 2.27x | 1.12x | 5.95x | 7.19x | 7.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
