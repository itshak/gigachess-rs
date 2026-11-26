| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.91x | 1.48x | 1.59x | 1.92x | 1.92x | 30/30 |
| materialize | 1.01x | 1.84x | 1.43x | 1.26x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.91x | 3.41x | 1.39x | 1.26x | 1.78x | 1.82x | 30/30 |
| perft_full_d3 | 0.98x | 1.15x | 1.25x | 1.04x | 1.54x | 1.45x | 29/30 |
| fen_parse | 1.01x | 1.26x | 1.35x | 1.30x | 1.78x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.56x | 9.32x | 1.59x | 29.99x | 2.45x | 30/30 |
| snapshot | 0.91x | 1.10x | 1.03x | 1.45x | 1.03x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.81x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 1.29x | 1.53x | 4.15x | 1.24x | 1.24x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 1.08x | 1.29x | 76.18x | 130.38x | 173.59x | 140.42x | 5/5 |
| chess960_materialize | 1.04x | 1.59x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.68x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.87x | 1.48x | 1.19x | 1.76x | 1.92x | 10/10 |
| corpus_materialize | 0.98x | 1.53x | 1.45x | 1.17x | 1.82x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 3.12x | 2.09x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.06x | 0.55x | 1.49x | 1.74x | 1.96x | 1.99x | 8/10 |
| replay_checked_native | 1.03x | 2.06x | 1.01x | 6.84x | 6.05x | 7.84x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
