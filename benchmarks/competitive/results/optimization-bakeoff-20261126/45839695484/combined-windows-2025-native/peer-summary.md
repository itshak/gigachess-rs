| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.75x | 1.74x | 1.68x | 2.68x | 2.12x | 30/30 |
| materialize | 1.01x | 2.05x | 1.38x | 1.16x | 2.36x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.97x | 1.62x | 1.45x | 2.45x | 2.26x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.21x | 0.97x | 1.65x | 1.47x | 24/30 |
| fen_parse | 1.00x | 1.30x | 1.42x | 1.35x | 2.07x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 3.96x | 7.70x | 1.60x | 31.66x | 2.03x | 30/30 |
| snapshot | 0.99x | 1.78x | 1.25x | 4.10x | 1.25x | 1.21x | 30/30 |
| san_parse_all | 1.01x | 1.92x | N/A | 4.71x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.30x | N/A | 3.31x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.89x | 1.27x | 8.02x | 1.49x | 1.79x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 84.06x | 133.74x | 199.20x | 145.52x | 5/5 |
| chess960_materialize | 1.01x | 1.70x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.86x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 18.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.47x | 1.79x | 1.29x | 2.37x | 2.14x | 9/10 |
| corpus_materialize | 1.04x | 1.90x | 1.41x | 1.21x | 2.88x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.87x | 2.41x | 1.96x | 2.28x | 1.50x | 10/10 |
| replay_trusted_native | 0.98x | 1.11x | 2.00x | 2.38x | 3.80x | 4.71x | 10/10 |
| replay_checked_native | 1.03x | 2.29x | 1.01x | 5.69x | 7.38x | 7.62x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
