| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.00x | 1.72x | 1.45x | 2.25x | 1.77x | 30/30 |
| materialize | 0.92x | 1.91x | 1.55x | 1.60x | 2.38x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.74x | 1.54x | 1.17x | 2.32x | 1.84x | 30/30 |
| perft_full_d3 | 0.92x | 1.39x | 1.56x | 1.55x | 2.08x | 1.69x | 30/30 |
| fen_parse | 0.91x | 1.17x | 1.28x | 1.11x | 1.47x | 1.13x | 30/30 |
| fen_write_native_ep | 1.01x | 4.84x | 10.55x | 1.95x | 30.59x | 2.18x | 30/30 |
| snapshot | 0.67x | 1.64x | 1.32x | 2.79x | 1.32x | 1.23x | 30/30 |
| san_parse_all | 0.93x | 1.79x | N/A | 4.06x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 8.60x | N/A | 2.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 1.75x | 1.12x | 3.14x | 1.11x | 1.32x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 1.29x | 2.17x | 117.05x | 245.79x | 281.56x | 226.77x | 5/5 |
| chess960_materialize | 0.97x | 1.62x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.17x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 41.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 22.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.60x | 2.49x | 1.35x | 3.02x | 2.74x | 10/10 |
| corpus_materialize | 1.02x | 1.46x | 1.36x | 1.12x | 1.77x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.32x | 3.17x | 2.26x | 3.08x | 1.59x | 10/10 |
| replay_trusted_native | 0.90x | 0.71x | 1.28x | 1.33x | 1.86x | 2.17x | 7/10 |
| replay_checked_native | 0.91x | 1.87x | 0.97x | 7.99x | 7.59x | 9.09x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
