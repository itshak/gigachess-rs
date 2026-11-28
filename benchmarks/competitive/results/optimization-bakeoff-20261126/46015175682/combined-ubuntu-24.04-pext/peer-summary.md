| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.70x | 1.96x | 1.70x | 2.57x | 2.00x | 30/30 |
| materialize | 1.06x | 2.31x | 1.72x | 1.81x | 2.73x | 1.96x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.09x | 1.61x | 1.27x | 2.47x | 1.96x | 29/30 |
| perft_full_d3 | 1.24x | 1.67x | 1.88x | 1.87x | 2.42x | 2.00x | 30/30 |
| fen_parse | 0.94x | 1.24x | 1.31x | 1.08x | 1.52x | 1.20x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 10.50x | 1.95x | 30.92x | 2.19x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 7.61x | 1.00x | 1.33x | 25/30 |
| san_parse_all | 1.12x | 1.92x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.22x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.21x | 1.36x | 4.14x | 1.39x | 1.70x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.25x | 103.14x | 182.64x | 231.73x | 196.71x | 5/5 |
| chess960_materialize | 1.01x | 1.77x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.55x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 35.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.67x | 1.63x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 18.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.23x | 6.26x | 2.33x | 1.32x | 2.89x | 2.62x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.30x | 1.08x | 1.71x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.24x | 2.98x | 2.23x | 2.93x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.49x | 1.56x | 2.16x | 2.50x | 9/10 |
| replay_checked_native | 0.99x | 2.14x | 1.11x | 9.37x | 8.81x | 10.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
