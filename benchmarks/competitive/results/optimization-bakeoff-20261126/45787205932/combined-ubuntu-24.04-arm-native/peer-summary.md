| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.46x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.01x | 1.83x | 1.45x | 1.26x | 2.04x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.71x | 1.52x | 1.38x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.01x | 1.17x | 1.27x | 1.07x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.35x | 1.28x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.32x | 1.57x | 29.81x | 2.45x | 30/30 |
| snapshot | 1.08x | 1.29x | 0.94x | 1.62x | 0.92x | 1.01x | 18/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.79x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.15x | 1.33x | 3.76x | 1.11x | 1.12x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.18x | 76.11x | 132.55x | 173.63x | 140.08x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.62x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.80x | 1.87x | 1.50x | 2.15x | 2.37x | 10/10 |
| corpus_materialize | 0.99x | 1.58x | 1.51x | 1.22x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.77x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.51x | 1.35x | 1.62x | 1.78x | 1.83x | 8/10 |
| replay_checked_native | 1.01x | 2.05x | 0.99x | 6.74x | 5.95x | 7.72x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
