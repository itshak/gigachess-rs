| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 3.70x | 1.41x | 1.51x | 1.83x | 1.83x | 30/30 |
| materialize | 0.98x | 1.81x | 1.41x | 1.24x | 2.00x | 1.38x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.34x | 1.37x | 1.23x | 1.75x | 1.79x | 30/30 |
| perft_full_d3 | 0.94x | 1.09x | 1.18x | 0.98x | 1.46x | 1.37x | 25/30 |
| fen_parse | 1.01x | 1.24x | 1.32x | 1.29x | 1.74x | 1.40x | 30/30 |
| fen_write_native_ep | 0.95x | 4.28x | 8.93x | 1.49x | 28.25x | 2.25x | 30/30 |
| snapshot | 0.99x | 1.06x | 1.04x | 1.44x | 1.04x | 0.96x | 22/30 |
| san_parse_all | 0.97x | 1.79x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.31x | N/A | 2.46x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.07x | 1.21x | 1.19x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.24x | 71.21x | 120.69x | 157.98x | 127.01x | 5/5 |
| chess960_materialize | 0.98x | 1.55x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.59x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 3.78x | 1.49x | 1.18x | 1.71x | 1.89x | 10/10 |
| corpus_materialize | 0.98x | 1.51x | 1.45x | 1.16x | 1.80x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.15x | 2.97x | 2.11x | 2.92x | 1.48x | 10/10 |
| replay_trusted_native | 0.99x | 0.61x | 1.58x | 1.91x | 2.14x | 2.18x | 8/10 |
| replay_checked_native | 0.99x | 2.15x | 1.04x | 7.02x | 6.22x | 8.14x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
