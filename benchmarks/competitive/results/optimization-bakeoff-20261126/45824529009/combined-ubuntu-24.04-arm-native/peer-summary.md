| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.94x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.84x | 1.46x | 1.26x | 2.04x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.47x | 1.41x | 1.29x | 1.80x | 1.85x | 30/30 |
| perft_full_d3 | 1.00x | 1.19x | 1.28x | 1.07x | 1.57x | 1.49x | 29/30 |
| fen_parse | 1.00x | 1.23x | 1.33x | 1.25x | 1.75x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.27x | 1.56x | 29.83x | 2.39x | 30/30 |
| snapshot | 1.01x | 1.11x | 1.05x | 1.45x | 1.05x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.75x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.37x | 1.62x | 4.40x | 1.32x | 1.32x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.47x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.22x | 129.60x | 173.84x | 140.59x | 5/5 |
| chess960_materialize | 0.99x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.71x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 3.91x | 1.50x | 1.20x | 1.77x | 1.98x | 10/10 |
| corpus_materialize | 0.99x | 1.53x | 1.45x | 1.16x | 1.81x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.07x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.03x | 0.62x | 1.66x | 2.01x | 2.23x | 2.29x | 8/10 |
| replay_checked_native | 0.96x | 2.10x | 1.02x | 6.96x | 6.14x | 8.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
