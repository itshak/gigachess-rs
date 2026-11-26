| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.74x | 1.68x | 1.62x | 2.51x | 2.03x | 30/30 |
| materialize | 1.00x | 1.96x | 1.66x | 1.25x | 2.26x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.07x | 1.61x | 1.42x | 2.49x | 2.02x | 30/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.52x | 1.36x | 1.96x | 1.91x | 30/30 |
| fen_parse | 1.09x | 1.38x | 1.50x | 1.22x | 2.20x | 1.26x | 30/30 |
| fen_write_native_ep | 1.00x | 4.15x | 10.33x | 1.74x | 28.66x | 2.20x | 30/30 |
| snapshot | 1.03x | 1.97x | 1.25x | 3.55x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.08x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.57x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.34x | 1.34x | 4.66x | 1.49x | 1.64x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.72x | 146.77x | 232.54x | 146.11x | 5/5 |
| chess960_materialize | 0.99x | 1.56x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.79x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 27.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.41x | 1.79x | 1.38x | 2.62x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.53x | 1.14x | 1.99x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.40x | 3.23x | 2.43x | 3.09x | 1.80x | 10/10 |
| replay_trusted_native | 0.97x | 0.88x | 1.59x | 1.91x | 2.63x | 2.86x | 9/10 |
| replay_checked_native | 1.00x | 2.47x | 1.14x | 8.29x | 8.39x | 9.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
