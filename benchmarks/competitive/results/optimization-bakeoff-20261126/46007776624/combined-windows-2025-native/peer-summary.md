| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 5.02x | 1.81x | 1.73x | 2.59x | 2.19x | 30/30 |
| materialize | 1.02x | 2.02x | 1.34x | 1.15x | 2.33x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.08x | 4.13x | 1.71x | 1.48x | 2.63x | 2.32x | 30/30 |
| perft_full_d3 | 1.34x | 1.97x | 2.04x | 1.70x | 3.18x | 2.46x | 30/30 |
| fen_parse | 0.97x | 1.28x | 1.40x | 1.30x | 2.12x | 1.41x | 30/30 |
| fen_write_native_ep | 1.02x | 3.99x | 7.62x | 1.59x | 31.07x | 2.02x | 30/30 |
| snapshot | 0.98x | 1.77x | 1.23x | 4.06x | 1.23x | 1.19x | 30/30 |
| san_parse_all | 0.98x | 1.95x | N/A | 4.64x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 9.37x | N/A | 3.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.93x | 1.31x | 8.08x | 1.52x | 1.68x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 0.98x | 0.97x | 84.15x | 130.92x | 200.41x | 142.73x | 4/5 |
| chess960_materialize | 1.00x | 1.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.94x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.79x | 0.76x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.70x | 1.91x | 1.43x | 2.55x | 2.28x | 10/10 |
| corpus_materialize | 0.99x | 1.87x | 1.39x | 1.25x | 2.93x | 1.67x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.95x | 2.35x | 1.92x | 2.26x | 1.46x | 10/10 |
| replay_trusted_native | 1.02x | 1.12x | 1.95x | 2.52x | 3.88x | 4.78x | 10/10 |
| replay_checked_native | 0.97x | 2.46x | 1.09x | 6.13x | 7.43x | 8.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
