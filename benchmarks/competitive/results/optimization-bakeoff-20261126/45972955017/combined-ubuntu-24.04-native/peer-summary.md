| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.32x | 1.84x | 1.56x | 2.42x | 1.88x | 30/30 |
| materialize | 0.89x | 1.96x | 1.48x | 1.70x | 2.28x | 1.80x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.71x | 1.52x | 1.15x | 2.27x | 1.81x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.57x | 1.50x | 1.98x | 1.70x | 30/30 |
| fen_parse | 0.99x | 1.48x | 1.35x | 1.17x | 1.47x | 1.24x | 30/30 |
| fen_write_native_ep | 0.97x | 4.46x | 9.71x | 1.84x | 27.66x | 2.46x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.12x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.15x | 1.35x | 4.17x | 1.35x | 1.71x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.11x | 87.94x | 160.78x | 197.45x | 171.51x | 5/5 |
| chess960_materialize | 0.86x | 1.50x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.52x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 42.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.23x | 25.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.35x | 2.02x | 1.11x | 2.45x | 2.24x | 9/10 |
| corpus_materialize | 1.02x | 1.40x | 1.30x | 1.07x | 1.72x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 3.00x | 2.24x | 2.93x | 1.57x | 10/10 |
| replay_trusted_native | 1.03x | 0.88x | 1.61x | 1.61x | 2.25x | 2.65x | 9/10 |
| replay_checked_native | 1.01x | 2.17x | 1.20x | 9.18x | 8.88x | 10.53x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
