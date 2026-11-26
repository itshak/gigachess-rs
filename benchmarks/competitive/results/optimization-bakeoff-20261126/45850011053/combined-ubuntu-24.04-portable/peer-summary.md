| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 3.03x | 1.57x | 1.40x | 1.95x | 1.38x | 30/30 |
| materialize | 0.99x | 2.11x | 1.86x | 1.34x | 2.47x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.94x | 2.95x | 1.60x | 1.24x | 1.99x | 1.72x | 30/30 |
| perft_full_d3 | 0.94x | 1.30x | 1.49x | 1.19x | 1.78x | 1.63x | 30/30 |
| fen_parse | 0.80x | 1.26x | 1.23x | 0.97x | 1.77x | 1.03x | 23/30 |
| fen_write_native_ep | 0.99x | 2.86x | 9.42x | 1.65x | 27.58x | 2.06x | 30/30 |
| snapshot | 1.00x | 2.06x | 1.59x | 3.07x | 1.00x | 1.07x | 27/30 |
| san_parse_all | 0.92x | 1.91x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 9.84x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 2.02x | 1.43x | 4.14x | 1.35x | 1.35x | 30/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 64.30x | 115.60x | 182.15x | 113.91x | 5/5 |
| chess960_materialize | 0.95x | 1.63x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.15x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 23.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 17.96x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 3.73x | 1.86x | 1.25x | 2.05x | 2.47x | 10/10 |
| corpus_materialize | 0.96x | 1.55x | 1.55x | 1.14x | 1.99x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.24x | 3.24x | 2.76x | 3.13x | 1.75x | 10/10 |
| replay_trusted_native | 0.86x | 0.76x | 1.43x | 1.66x | 2.22x | 2.06x | 8/10 |
| replay_checked_native | 0.95x | 2.03x | 1.00x | 6.96x | 7.06x | 7.95x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
