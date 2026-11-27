| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.05x | 1.68x | 1.66x | 2.46x | 2.00x | 30/30 |
| materialize | 1.01x | 2.18x | 1.70x | 1.25x | 2.44x | 1.57x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.32x | 1.64x | 1.29x | 2.49x | 2.22x | 30/30 |
| perft_full_d3 | 1.03x | 1.44x | 1.68x | 1.25x | 2.16x | 2.08x | 30/30 |
| fen_parse | 0.92x | 1.30x | 1.50x | 1.14x | 2.02x | 1.23x | 29/30 |
| fen_write_native_ep | 1.00x | 3.71x | 8.54x | 1.56x | 25.36x | 1.90x | 30/30 |
| snapshot | 1.01x | 2.49x | 1.50x | 4.48x | 1.50x | 1.49x | 30/30 |
| san_parse_all | 1.02x | 1.99x | N/A | 4.01x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.73x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 0.84x | 1.59x | 1.17x | 3.49x | 1.14x | 1.60x | 29/30 |
| make_unmake_full | 1.10x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.26x | 86.48x | 136.21x | 231.20x | 150.06x | 5/5 |
| chess960_materialize | 1.03x | 1.82x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.01x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.89x | 30.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.88x | 23.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.99x | 1.75x | 1.18x | 2.30x | 2.11x | 9/10 |
| corpus_materialize | 1.01x | 1.70x | 1.56x | 1.16x | 2.16x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.05x | 2.39x | 3.07x | 2.45x | 2.98x | 1.70x | 10/10 |
| replay_trusted_native | 0.98x | 0.73x | 1.38x | 1.50x | 2.35x | 2.86x | 8/10 |
| replay_checked_native | 0.99x | 2.62x | 1.22x | 8.98x | 9.36x | 10.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
