| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 2.97x | 1.50x | 1.38x | 1.92x | 1.35x | 30/30 |
| materialize | 0.92x | 2.02x | 1.79x | 1.28x | 2.36x | 1.54x | 30/30 |
| perft_bulk_d4 | 0.91x | 2.81x | 1.50x | 1.18x | 1.88x | 1.65x | 30/30 |
| perft_full_d3 | 0.95x | 1.32x | 1.50x | 1.22x | 1.80x | 1.62x | 30/30 |
| fen_parse | 1.11x | 1.55x | 1.47x | 1.33x | 2.25x | 1.28x | 30/30 |
| fen_write_native_ep | 0.95x | 2.75x | 9.23x | 1.56x | 25.30x | 1.93x | 30/30 |
| snapshot | 1.20x | 2.07x | 1.36x | 3.69x | 1.22x | 1.18x | 30/30 |
| san_parse_all | 0.94x | 1.97x | N/A | 4.11x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.03x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 2.20x | 1.48x | 4.42x | 1.35x | 1.30x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.60x | 2.11x | 107.00x | 198.44x | 280.95x | 173.71x | 5/5 |
| chess960_materialize | 0.91x | 1.50x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.16x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 29.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 21.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 3.75x | 1.84x | 1.24x | 2.04x | 2.45x | 10/10 |
| corpus_materialize | 0.97x | 1.61x | 1.57x | 1.17x | 2.05x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.27x | 3.34x | 2.52x | 3.23x | 1.78x | 10/10 |
| replay_trusted_native | 0.82x | 0.73x | 1.48x | 1.68x | 2.17x | 2.01x | 8/10 |
| replay_checked_native | 0.86x | 2.00x | 1.00x | 7.01x | 7.02x | 7.90x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
