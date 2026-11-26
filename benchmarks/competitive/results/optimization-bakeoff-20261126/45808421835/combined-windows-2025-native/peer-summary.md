| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.33x | 1.57x | 1.51x | 2.27x | 1.91x | 30/30 |
| materialize | 0.99x | 1.95x | 1.29x | 1.09x | 2.19x | 1.47x | 29/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.52x | 1.35x | 2.37x | 2.11x | 30/30 |
| perft_full_d3 | 1.02x | 1.20x | 1.24x | 1.03x | 1.69x | 1.52x | 27/30 |
| fen_parse | 0.79x | 1.23x | 1.30x | 1.27x | 2.26x | 1.29x | 30/30 |
| fen_write_native_ep | 1.00x | 3.97x | 7.69x | 1.59x | 30.85x | 2.00x | 30/30 |
| snapshot | 0.49x | 1.33x | 1.12x | 1.08x | 1.11x | 1.10x | 30/30 |
| san_parse_all | 1.02x | 1.90x | N/A | 4.64x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.17x | N/A | 3.47x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.14x | 1.43x | 6.98x | 1.46x | 1.71x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 2.54x | 86.37x | 142.05x | 219.74x | 148.47x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.55x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 20.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 16.80x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.42x | 1.70x | 1.24x | 2.22x | 2.02x | 10/10 |
| corpus_materialize | 1.01x | 1.76x | 1.43x | 1.17x | 2.79x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.09x | 1.87x | 2.42x | 1.98x | 2.27x | 1.55x | 10/10 |
| replay_trusted_native | 1.04x | 0.83x | 1.36x | 1.72x | 2.81x | 3.60x | 9/10 |
| replay_checked_native | 1.17x | 2.15x | 0.93x | 5.37x | 6.54x | 8.14x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
