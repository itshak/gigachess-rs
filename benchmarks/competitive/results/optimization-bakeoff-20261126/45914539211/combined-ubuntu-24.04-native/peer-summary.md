| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.92x | 1.75x | 1.70x | 2.63x | 2.12x | 30/30 |
| materialize | 1.02x | 1.97x | 1.69x | 1.27x | 2.27x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.17x | 1.66x | 1.43x | 2.49x | 2.06x | 30/30 |
| perft_full_d3 | 1.24x | 1.87x | 1.88x | 1.63x | 2.42x | 2.37x | 30/30 |
| fen_parse | 1.06x | 1.31x | 1.46x | 1.18x | 2.18x | 1.25x | 30/30 |
| fen_write_native_ep | 1.01x | 4.32x | 10.39x | 1.79x | 29.35x | 2.11x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.25x | 3.53x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.09x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.38x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.39x | 1.38x | 4.80x | 1.52x | 1.68x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.56x | 147.10x | 227.61x | 147.02x | 5/5 |
| chess960_materialize | 1.01x | 1.61x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.82x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.74x | 1.91x | 1.46x | 2.79x | 2.56x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.54x | 1.15x | 2.01x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.45x | 3.24x | 2.43x | 3.14x | 1.74x | 10/10 |
| replay_trusted_native | 0.97x | 0.87x | 1.59x | 1.90x | 2.57x | 2.96x | 9/10 |
| replay_checked_native | 1.00x | 2.42x | 1.08x | 8.26x | 8.15x | 9.72x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
