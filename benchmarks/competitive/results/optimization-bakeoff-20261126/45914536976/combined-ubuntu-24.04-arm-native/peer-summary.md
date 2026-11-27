| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.67x | 1.81x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.52x | 1.33x | 2.16x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.84x | 1.57x | 1.42x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.61x | 1.33x | 1.97x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.32x | 1.29x | 1.74x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.31x | 1.59x | 29.27x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.09x | 1.45x | 1.09x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.38x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.23x | 1.46x | 3.95x | 1.18x | 1.18x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.16x | 133.12x | 171.81x | 140.34x | 5/5 |
| chess960_materialize | 1.02x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.98x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.06x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.44x | 1.70x | 1.35x | 1.99x | 2.22x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.54x | 1.23x | 1.92x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.10x | 2.85x | 1.47x | 10/10 |
| replay_trusted_native | 0.99x | 0.63x | 1.63x | 1.97x | 2.23x | 2.27x | 8/10 |
| replay_checked_native | 1.00x | 2.31x | 1.13x | 7.55x | 6.85x | 8.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
