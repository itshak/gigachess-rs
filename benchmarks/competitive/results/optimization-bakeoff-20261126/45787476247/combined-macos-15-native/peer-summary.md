| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.52x | 1.53x | 1.65x | 2.04x | 1.89x | 30/30 |
| materialize | 0.85x | 1.87x | 1.42x | 0.97x | 2.16x | 1.18x | 25/30 |
| perft_bulk_d4 | 1.01x | 4.55x | 1.75x | 1.34x | 2.06x | 1.94x | 30/30 |
| perft_full_d3 | 0.81x | 1.45x | 1.54x | 1.16x | 1.74x | 1.87x | 29/30 |
| fen_parse | 0.91x | 1.22x | 1.34x | 1.23x | 1.82x | 1.39x | 30/30 |
| fen_write_native_ep | 0.94x | 4.66x | 9.69x | 1.77x | 29.20x | 2.45x | 30/30 |
| snapshot | 0.96x | 1.71x | 1.09x | 2.25x | 1.12x | 1.13x | 25/30 |
| san_parse_all | 0.95x | 1.88x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 11.29x | N/A | 2.60x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 0.97x | 1.00x | 5.02x | 0.95x | 1.11x | 19/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.09x | N/A | N/A | 4/6 |
| startpos | 0.99x | 1.37x | 81.99x | 162.88x | 193.20x | 149.09x | 5/5 |
| chess960_materialize | 0.88x | 1.50x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 3.22x | 1.71x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 35.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.86x | 0.88x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.01x | 21.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.88x | 6.38x | 2.02x | 1.16x | 2.38x | 2.92x | 10/10 |
| corpus_materialize | 0.89x | 1.46x | 1.37x | 1.05x | 1.86x | 1.13x | 10/10 |
| corpus_uci_sorted | 0.94x | 1.60x | 2.49x | 1.85x | 2.52x | 1.34x | 10/10 |
| replay_trusted_native | 0.90x | 0.62x | 1.47x | 1.78x | 1.74x | 1.75x | 8/10 |
| replay_checked_native | 0.90x | 1.88x | 0.89x | 7.14x | 6.05x | 8.47x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
