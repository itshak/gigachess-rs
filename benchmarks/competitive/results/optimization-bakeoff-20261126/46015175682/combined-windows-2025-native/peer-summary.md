| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.88x | 1.81x | 1.73x | 2.59x | 2.19x | 30/30 |
| materialize | 1.01x | 2.03x | 1.35x | 1.15x | 2.35x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.08x | 1.74x | 1.47x | 2.52x | 2.30x | 30/30 |
| perft_full_d3 | 1.33x | 1.97x | 2.39x | 1.70x | 2.76x | 2.44x | 30/30 |
| fen_parse | 1.02x | 1.37x | 1.48x | 1.38x | 2.31x | 1.52x | 30/30 |
| fen_write_native_ep | 1.01x | 3.99x | 7.70x | 1.59x | 31.06x | 2.07x | 30/30 |
| snapshot | 1.00x | 1.80x | 1.26x | 4.13x | 1.26x | 1.16x | 30/30 |
| san_parse_all | 1.00x | 1.97x | N/A | 4.86x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.38x | N/A | 3.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 2.02x | 1.36x | 8.54x | 1.60x | 1.78x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 82.93x | 131.55x | 202.73x | 149.15x | 4/5 |
| chess960_materialize | 1.03x | 1.65x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.80x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.20x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.31x | 1.22x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 4.79x | 1.89x | 1.40x | 2.55x | 2.28x | 10/10 |
| corpus_materialize | 1.03x | 1.78x | 1.40x | 1.19x | 2.89x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.88x | 2.39x | 1.96x | 2.31x | 1.50x | 10/10 |
| replay_trusted_native | 1.07x | 1.15x | 2.02x | 2.46x | 3.98x | 4.92x | 10/10 |
| replay_checked_native | 1.05x | 2.49x | 1.10x | 6.13x | 7.33x | 8.29x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
