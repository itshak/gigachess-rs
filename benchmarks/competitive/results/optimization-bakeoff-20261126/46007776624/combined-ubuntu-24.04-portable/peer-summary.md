| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.03x | 1.74x | 1.51x | 2.06x | 1.45x | 30/30 |
| materialize | 1.09x | 2.42x | 1.88x | 1.92x | 2.86x | 2.03x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.21x | 1.70x | 1.27x | 2.14x | 1.91x | 29/30 |
| perft_full_d3 | 1.18x | 1.61x | 1.94x | 1.66x | 2.29x | 1.87x | 30/30 |
| fen_parse | 0.97x | 1.69x | 1.43x | 1.14x | 1.48x | 1.24x | 30/30 |
| fen_write_native_ep | 0.98x | 2.96x | 9.93x | 1.97x | 28.65x | 2.28x | 30/30 |
| snapshot | 0.99x | 1.74x | 1.25x | 3.24x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.50x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.05x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.13x | 1.51x | 4.25x | 1.36x | 1.57x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.36x | 109.97x | 123.03x | 112.40x | 5/5 |
| chess960_materialize | 1.06x | 1.84x | 1.79x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.15x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 0.86x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.86x | 19.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 4.88x | 2.45x | 1.34x | 2.47x | 3.35x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.39x | 1.13x | 1.81x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.97x | 3.03x | 2.23x | 2.99x | 1.54x | 10/10 |
| replay_trusted_native | 1.01x | 0.91x | 1.74x | 1.65x | 2.33x | 2.15x | 9/10 |
| replay_checked_native | 0.99x | 2.09x | 1.16x | 8.99x | 8.60x | 9.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
