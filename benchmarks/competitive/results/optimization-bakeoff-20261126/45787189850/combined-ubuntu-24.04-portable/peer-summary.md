| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.95x | 1.51x | 1.36x | 1.93x | 1.34x | 30/30 |
| materialize | 1.01x | 2.00x | 1.78x | 1.26x | 2.35x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.95x | 1.59x | 1.22x | 1.98x | 1.71x | 30/30 |
| perft_full_d3 | 1.01x | 1.50x | 1.72x | 1.36x | 2.06x | 1.87x | 30/30 |
| fen_parse | 1.01x | 1.60x | 1.56x | 1.19x | 2.41x | 1.28x | 30/30 |
| fen_write_native_ep | 0.99x | 2.79x | 9.07x | 1.77x | 26.50x | 2.04x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.05x | 3.03x | 1.03x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 2.05x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.59x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.96x | 1.33x | 3.91x | 1.23x | 1.18x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.30x | 65.09x | 110.95x | 197.61x | 112.95x | 5/5 |
| chess960_materialize | 0.98x | 1.49x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.06x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 27.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 21.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.96x | 1.99x | 1.32x | 2.17x | 2.60x | 10/10 |
| corpus_materialize | 1.02x | 1.64x | 1.64x | 1.20x | 2.11x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.19x | 3.15x | 2.42x | 3.03x | 1.73x | 10/10 |
| replay_trusted_native | 1.03x | 0.72x | 1.38x | 1.63x | 2.16x | 1.98x | 8/10 |
| replay_checked_native | 1.01x | 2.27x | 1.06x | 7.46x | 7.54x | 8.57x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
