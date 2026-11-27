| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 5.16x | 1.77x | 1.82x | 2.39x | 2.13x | 30/30 |
| materialize | 1.04x | 2.03x | 1.55x | 1.09x | 2.31x | 1.37x | 27/30 |
| perft_bulk_d4 | 1.08x | 4.35x | 1.61x | 1.31x | 2.06x | 1.99x | 30/30 |
| perft_full_d3 | 1.07x | 1.76x | 1.91x | 1.50x | 2.25x | 2.31x | 30/30 |
| fen_parse | 1.02x | 1.22x | 1.35x | 1.35x | 1.76x | 1.49x | 30/30 |
| fen_write_native_ep | 0.95x | 4.14x | 9.17x | 1.78x | 29.69x | 2.33x | 30/30 |
| snapshot | 0.86x | 1.50x | 1.01x | 2.16x | 1.00x | 0.92x | 23/30 |
| san_parse_all | 0.89x | 1.67x | N/A | 3.50x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.86x | N/A | 2.82x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.13x | 1.25x | 5.69x | 1.19x | 1.32x | 29/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.18x | 67.78x | 136.16x | 154.67x | 132.77x | 5/5 |
| chess960_materialize | 1.05x | 1.76x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 3.23x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.87x | 0.87x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.97x | 19.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.52x | 2.11x | 1.28x | 2.49x | 2.71x | 10/10 |
| corpus_materialize | 1.00x | 1.49x | 1.41x | 1.14x | 1.90x | 1.28x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.88x | 2.61x | 1.94x | 2.52x | 1.41x | 10/10 |
| replay_trusted_native | 0.96x | 0.77x | 1.86x | 1.75x | 1.82x | 1.84x | 8/10 |
| replay_checked_native | 1.02x | 2.74x | 1.28x | 10.72x | 8.55x | 11.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
