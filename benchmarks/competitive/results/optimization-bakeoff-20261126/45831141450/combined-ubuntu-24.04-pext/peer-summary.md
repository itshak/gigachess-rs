| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.27x | 1.45x | 1.48x | 2.26x | 1.73x | 30/30 |
| materialize | 1.03x | 2.00x | 1.61x | 1.24x | 2.31x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.49x | 1.33x | 1.20x | 2.10x | 1.63x | 30/30 |
| perft_full_d3 | 0.92x | 1.18x | 1.31x | 1.17x | 1.72x | 1.59x | 30/30 |
| fen_parse | 1.08x | 1.38x | 1.57x | 1.30x | 2.26x | 1.38x | 30/30 |
| fen_write_native_ep | 0.98x | 3.90x | 9.39x | 1.64x | 25.82x | 1.95x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.25x | 3.50x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.12x | N/A | 4.46x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.36x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.23x | 1.28x | 4.44x | 1.42x | 1.57x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.51x | 1.87x | 117.47x | 217.48x | 331.91x | 223.27x | 5/5 |
| chess960_materialize | 0.98x | 1.50x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.54x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 28.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 21.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.91x | 1.61x | 1.26x | 2.38x | 2.06x | 10/10 |
| corpus_materialize | 1.02x | 1.55x | 1.48x | 1.14x | 1.97x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.06x | 2.55x | 3.42x | 2.53x | 3.35x | 1.87x | 10/10 |
| replay_trusted_native | 0.85x | 0.74x | 1.33x | 1.63x | 2.15x | 2.35x | 8/10 |
| replay_checked_native | 0.99x | 2.28x | 0.99x | 7.87x | 7.62x | 8.94x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
