| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.31x | 1.85x | 1.57x | 2.37x | 1.90x | 30/30 |
| materialize | 1.00x | 2.24x | 1.75x | 1.75x | 2.72x | 1.92x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.71x | 1.53x | 1.16x | 2.27x | 1.82x | 29/30 |
| perft_full_d3 | 1.01x | 1.33x | 1.46x | 1.43x | 1.86x | 1.58x | 30/30 |
| fen_parse | 1.01x | 1.36x | 1.39x | 1.08x | 1.57x | 1.25x | 30/30 |
| fen_write_native_ep | 1.00x | 4.70x | 10.59x | 2.15x | 30.88x | 2.69x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.68x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 0.92x | 1.77x | N/A | 4.06x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.26x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.09x | 1.29x | 3.96x | 1.32x | 1.62x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.26x | 98.76x | 171.33x | 231.37x | 196.34x | 5/5 |
| chess960_materialize | 1.01x | 1.75x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.59x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.53x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 23.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 5.43x | 2.04x | 1.11x | 2.47x | 2.23x | 10/10 |
| corpus_materialize | 1.00x | 1.41x | 1.29x | 1.07x | 1.69x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 3.01x | 2.28x | 2.95x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.89x | 1.51x | 1.55x | 2.18x | 2.62x | 9/10 |
| replay_checked_native | 1.01x | 2.01x | 1.04x | 8.56x | 8.20x | 9.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
