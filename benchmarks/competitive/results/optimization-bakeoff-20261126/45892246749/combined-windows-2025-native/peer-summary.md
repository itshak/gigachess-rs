| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.53x | 1.65x | 1.63x | 2.46x | 2.00x | 30/30 |
| materialize | 0.97x | 1.93x | 1.31x | 1.09x | 2.22x | 1.42x | 29/30 |
| perft_bulk_d4 | 0.79x | 3.13x | 1.27x | 1.13x | 1.92x | 1.76x | 30/30 |
| perft_full_d3 | 0.99x | 1.48x | 1.53x | 1.29x | 2.08x | 1.84x | 30/30 |
| fen_parse | 0.87x | 1.20x | 1.34x | 1.22x | 1.91x | 1.31x | 30/30 |
| fen_write_native_ep | 0.94x | 3.70x | 7.08x | 1.49x | 28.76x | 1.90x | 30/30 |
| snapshot | 0.99x | 2.02x | 1.22x | 3.09x | 1.22x | 1.05x | 29/30 |
| san_parse_all | 0.86x | 1.67x | N/A | 4.07x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.72x | N/A | 3.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.86x | 1.67x | 1.13x | 7.23x | 1.30x | 1.48x | 29/30 |
| make_unmake_full | 0.71x | N/A | N/A | 0.93x | N/A | N/A | 1/6 |
| startpos | 0.86x | 0.86x | 72.36x | 113.00x | 169.61x | 124.13x | 4/5 |
| chess960_materialize | 0.95x | 1.74x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.80x | 2.33x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 23.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 18.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.19x | 1.71x | 1.28x | 2.29x | 2.07x | 9/10 |
| corpus_materialize | 0.92x | 1.76x | 1.32x | 1.13x | 2.62x | 1.50x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.88x | 2.36x | 1.97x | 2.25x | 1.49x | 10/10 |
| replay_trusted_native | 0.73x | 0.82x | 1.37x | 1.69x | 2.77x | 3.70x | 8/10 |
| replay_checked_native | 0.82x | 2.07x | 0.89x | 5.28x | 6.17x | 7.18x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
