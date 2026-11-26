| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.31x | 1.47x | 1.45x | 2.13x | 1.94x | 29/30 |
| materialize | 1.11x | 1.97x | 1.35x | 1.30x | 2.35x | 1.58x | 30/30 |
| perft_bulk_d4 | 0.89x | 3.66x | 1.41x | 1.18x | 2.03x | 1.80x | 29/30 |
| perft_full_d3 | 1.05x | 1.32x | 1.41x | 1.28x | 1.87x | 1.79x | 30/30 |
| fen_parse | 0.99x | 1.26x | 1.43x | 1.27x | 2.27x | 1.46x | 30/30 |
| fen_write_native_ep | 1.01x | 4.26x | 8.43x | 1.69x | 31.83x | 1.93x | 30/30 |
| snapshot | 1.57x | 1.73x | 0.90x | 3.80x | 0.90x | 0.97x | 21/30 |
| san_parse_all | 0.95x | 2.10x | N/A | 5.49x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.61x | N/A | 3.60x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 2.20x | 1.34x | 9.54x | 1.45x | 1.60x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.16x | 1.33x | 85.86x | 167.14x | 239.20x | 171.57x | 5/5 |
| chess960_materialize | 1.01x | 1.48x | 1.24x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.77x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 26.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.84x | 4.65x | 1.53x | 1.22x | 2.11x | 2.27x | 9/10 |
| corpus_materialize | 0.89x | 1.60x | 1.21x | 1.11x | 1.99x | 1.32x | 10/10 |
| corpus_uci_sorted | 0.90x | 1.73x | 2.21x | 1.79x | 1.97x | 1.29x | 10/10 |
| replay_trusted_native | 1.00x | 0.75x | 1.53x | 1.85x | 2.31x | 2.68x | 8/10 |
| replay_checked_native | 0.95x | 2.34x | 1.05x | 8.01x | 7.71x | 10.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
