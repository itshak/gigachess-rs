| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.11x | 1.77x | 1.49x | 2.44x | 1.83x | 30/30 |
| materialize | 0.97x | 2.14x | 1.62x | 1.68x | 2.61x | 1.96x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.64x | 1.50x | 1.14x | 2.40x | 1.80x | 29/30 |
| perft_full_d3 | 0.91x | 1.28x | 1.41x | 1.36x | 1.89x | 1.52x | 30/30 |
| fen_parse | 1.00x | 1.33x | 1.59x | 1.15x | 1.46x | 1.14x | 30/30 |
| fen_write_native_ep | 0.92x | 4.35x | 9.30x | 1.87x | 26.23x | 2.08x | 30/30 |
| snapshot | 0.67x | 1.35x | 0.87x | 3.02x | 0.87x | 1.01x | 17/30 |
| san_parse_all | 0.98x | 1.85x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 8.18x | N/A | 1.91x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.11x | 1.37x | 3.86x | 1.35x | 1.59x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.19x | 78.63x | 146.90x | 170.18x | 143.08x | 5/5 |
| chess960_materialize | 0.94x | 1.79x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.37x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 40.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 20.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 4.89x | 1.83x | 1.01x | 2.24x | 2.05x | 9/10 |
| corpus_materialize | 1.02x | 1.40x | 1.28x | 1.07x | 1.70x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 2.97x | 2.22x | 2.88x | 1.55x | 10/10 |
| replay_trusted_native | 0.98x | 0.83x | 1.56x | 1.54x | 2.14x | 2.50x | 9/10 |
| replay_checked_native | 0.90x | 1.99x | 1.08x | 8.20x | 7.81x | 9.34x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
