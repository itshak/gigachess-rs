| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.69x | 1.55x | 1.35x | 1.78x | 1.33x | 29/30 |
| materialize | 1.00x | 2.13x | 1.74x | 1.77x | 2.64x | 1.88x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.71x | 1.43x | 1.06x | 1.78x | 1.62x | 28/30 |
| perft_full_d3 | 1.00x | 1.50x | 1.77x | 1.55x | 2.05x | 1.75x | 30/30 |
| fen_parse | 1.00x | 1.52x | 1.43x | 1.18x | 1.48x | 1.19x | 30/30 |
| fen_write_native_ep | 1.00x | 2.97x | 10.29x | 1.86x | 29.43x | 2.57x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.51x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.36x | 3.96x | 1.31x | 1.49x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.11x | 109.89x | 122.18x | 103.95x | 5/5 |
| chess960_materialize | 1.00x | 1.56x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.96x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 43.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 20.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.68x | 1.83x | 1.00x | 1.85x | 2.57x | 9/10 |
| corpus_materialize | 1.00x | 1.44x | 1.35x | 1.08x | 1.74x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.92x | 2.95x | 2.16x | 2.88x | 1.52x | 10/10 |
| replay_trusted_native | 1.00x | 0.91x | 1.66x | 1.59x | 2.20x | 2.04x | 9/10 |
| replay_checked_native | 1.00x | 1.90x | 1.04x | 7.99x | 7.71x | 8.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
