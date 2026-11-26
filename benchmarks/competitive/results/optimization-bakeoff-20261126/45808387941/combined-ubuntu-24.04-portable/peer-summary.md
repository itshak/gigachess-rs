| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.92x | 1.51x | 1.33x | 1.86x | 1.29x | 29/30 |
| materialize | 1.03x | 2.26x | 1.80x | 1.28x | 2.59x | 1.62x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.91x | 1.48x | 1.13x | 1.79x | 1.64x | 30/30 |
| perft_full_d3 | 0.99x | 1.47x | 1.79x | 1.37x | 2.01x | 1.79x | 30/30 |
| fen_parse | 1.01x | 1.65x | 1.60x | 1.22x | 1.99x | 1.33x | 30/30 |
| fen_write_native_ep | 1.00x | 2.77x | 8.43x | 1.53x | 24.99x | 1.90x | 30/30 |
| snapshot | 1.01x | 1.62x | 1.16x | 2.63x | 1.15x | 1.00x | 27/30 |
| san_parse_all | 0.98x | 1.74x | N/A | 3.81x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.11x | N/A | 2.41x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.97x | 1.36x | 4.97x | 1.19x | 1.22x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.29x | 60.09x | 80.27x | 120.24x | 87.28x | 5/5 |
| chess960_materialize | 1.15x | 1.71x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.09x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 32.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.79x | 20.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 4.02x | 1.70x | 1.03x | 1.80x | 2.61x | 9/10 |
| corpus_materialize | 0.90x | 1.63x | 1.51x | 1.11x | 2.12x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.96x | 2.88x | 2.22x | 2.70x | 1.61x | 10/10 |
| replay_trusted_native | 0.97x | 0.61x | 1.40x | 1.41x | 1.92x | 1.78x | 8/10 |
| replay_checked_native | 0.95x | 1.74x | 0.98x | 6.81x | 6.67x | 7.86x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
