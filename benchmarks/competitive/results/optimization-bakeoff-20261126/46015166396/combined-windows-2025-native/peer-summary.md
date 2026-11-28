| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.87x | 1.79x | 1.76x | 2.76x | 2.18x | 30/30 |
| materialize | 1.01x | 2.08x | 1.41x | 1.17x | 2.42x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.08x | 1.66x | 1.48x | 2.59x | 2.29x | 30/30 |
| perft_full_d3 | 1.30x | 1.97x | 2.05x | 1.75x | 2.73x | 2.46x | 30/30 |
| fen_parse | 0.99x | 1.36x | 1.43x | 1.40x | 2.25x | 1.43x | 30/30 |
| fen_write_native_ep | 0.91x | 3.66x | 7.11x | 1.48x | 28.20x | 1.82x | 30/30 |
| snapshot | 1.03x | 1.78x | 1.27x | 4.23x | 1.28x | 1.17x | 30/30 |
| san_parse_all | 0.99x | 1.97x | N/A | 4.81x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.56x | N/A | 3.39x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.95x | 1.37x | 8.37x | 1.55x | 1.70x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.84x | 0.99x | 70.21x | 115.19x | 174.57x | 130.53x | 4/5 |
| chess960_materialize | 0.99x | 1.63x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.01x | 1.70x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 22.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.23x | 1.28x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.82x | 1.89x | 1.43x | 2.62x | 2.29x | 10/10 |
| corpus_materialize | 1.00x | 1.86x | 1.45x | 1.20x | 2.88x | 1.56x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.82x | 2.32x | 1.91x | 2.23x | 1.56x | 10/10 |
| replay_trusted_native | 1.18x | 1.20x | 2.15x | 2.64x | 4.21x | 5.26x | 10/10 |
| replay_checked_native | 0.99x | 2.68x | 1.18x | 6.58x | 7.87x | 8.86x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
