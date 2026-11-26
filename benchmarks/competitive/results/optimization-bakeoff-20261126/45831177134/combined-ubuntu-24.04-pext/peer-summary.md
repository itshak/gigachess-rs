| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.30x | 1.49x | 1.51x | 2.36x | 1.76x | 30/30 |
| materialize | 1.03x | 1.88x | 1.57x | 1.20x | 2.17x | 1.42x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.60x | 1.40x | 1.25x | 2.15x | 1.70x | 30/30 |
| perft_full_d3 | 1.05x | 1.30x | 1.49x | 1.30x | 1.92x | 1.77x | 30/30 |
| fen_parse | 0.92x | 1.25x | 1.23x | 1.03x | 1.91x | 1.19x | 28/30 |
| fen_write_native_ep | 1.03x | 4.27x | 9.81x | 1.70x | 27.58x | 2.08x | 30/30 |
| snapshot | 1.00x | 1.98x | 1.25x | 3.63x | 1.25x | 1.28x | 30/30 |
| san_parse_all | 0.98x | 2.04x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.28x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.35x | 1.35x | 4.64x | 1.48x | 1.64x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.25x | 139.75x | 225.59x | 154.19x | 5/5 |
| chess960_materialize | 0.99x | 1.50x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.72x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.09x | 1.66x | 1.29x | 2.45x | 2.14x | 10/10 |
| corpus_materialize | 0.99x | 1.52x | 1.44x | 1.09x | 2.01x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.41x | 3.17x | 2.37x | 3.06x | 1.68x | 10/10 |
| replay_trusted_native | 1.05x | 0.92x | 1.73x | 1.98x | 2.61x | 2.90x | 9/10 |
| replay_checked_native | 0.97x | 2.24x | 1.00x | 7.38x | 7.46x | 8.69x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
