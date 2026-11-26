| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.63x | 1.70x | 1.64x | 2.80x | 2.07x | 30/30 |
| materialize | 0.99x | 2.02x | 1.37x | 1.15x | 2.47x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.06x | 1.65x | 1.46x | 2.51x | 2.28x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.53x | 1.25x | 2.10x | 1.96x | 30/30 |
| fen_parse | 0.95x | 1.30x | 1.42x | 1.35x | 2.26x | 1.42x | 30/30 |
| fen_write_native_ep | 1.01x | 3.95x | 7.59x | 1.62x | 33.62x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.84x | 1.25x | 4.11x | 1.25x | 1.16x | 30/30 |
| san_parse_all | 0.98x | 1.95x | N/A | 4.71x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.50x | N/A | 3.45x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.95x | 1.31x | 9.22x | 1.54x | 1.78x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 105.20x | 133.77x | 221.10x | 144.67x | 4/5 |
| chess960_materialize | 1.00x | 1.68x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.90x | 1.66x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.79x | 1.85x | 1.35x | 2.49x | 2.23x | 10/10 |
| corpus_materialize | 1.01x | 1.87x | 1.40x | 1.19x | 2.88x | 1.53x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.88x | 2.43x | 2.00x | 2.29x | 1.50x | 10/10 |
| replay_trusted_native | 0.99x | 1.07x | 1.85x | 2.32x | 3.65x | 4.55x | 9/10 |
| replay_checked_native | 1.01x | 2.60x | 1.09x | 6.38x | 7.68x | 8.29x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
