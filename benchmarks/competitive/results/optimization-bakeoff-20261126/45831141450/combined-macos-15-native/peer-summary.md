| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.24x | 1.36x | 1.47x | 1.75x | 1.61x | 27/30 |
| materialize | 1.07x | 2.04x | 1.57x | 1.04x | 2.29x | 1.40x | 27/30 |
| perft_bulk_d4 | 0.99x | 3.48x | 1.30x | 0.99x | 1.61x | 1.56x | 26/30 |
| perft_full_d3 | 0.97x | 1.49x | 1.67x | 1.25x | 1.88x | 1.89x | 29/30 |
| fen_parse | 0.95x | 1.29x | 1.41x | 1.50x | 1.69x | 1.48x | 30/30 |
| fen_write_native_ep | 1.00x | 4.20x | 8.51x | 1.64x | 26.73x | 2.10x | 30/30 |
| snapshot | 1.11x | 1.68x | 1.28x | 2.56x | 1.15x | 1.01x | 27/30 |
| san_parse_all | 1.00x | 1.77x | N/A | 3.70x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.99x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.12x | 1.18x | 5.75x | 1.06x | 1.23x | 27/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.18x | N/A | N/A | 5/6 |
| startpos | 1.45x | 1.71x | 97.60x | 197.76x | 243.36x | 191.43x | 5/5 |
| chess960_materialize | 0.96x | 1.69x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.13x | 2.81x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 41.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.93x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.26x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 5.12x | 1.59x | 1.01x | 1.86x | 1.94x | 9/10 |
| corpus_materialize | 1.03x | 1.44x | 1.40x | 1.11x | 1.72x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.09x | 2.04x | 2.97x | 1.97x | 2.73x | 1.46x | 10/10 |
| replay_trusted_native | 0.92x | 0.66x | 1.53x | 1.63x | 2.03x | 1.57x | 8/10 |
| replay_checked_native | 1.02x | 2.66x | 1.36x | 11.28x | 8.40x | 10.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
