| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.15x | 1.34x | 1.37x | 1.93x | 1.64x | 29/30 |
| materialize | 0.96x | 1.98x | 1.39x | 1.00x | 2.09x | 1.32x | 27/30 |
| perft_bulk_d4 | 0.81x | 3.42x | 1.37x | 1.12x | 1.67x | 1.64x | 28/30 |
| perft_full_d3 | 0.83x | 1.40x | 1.69x | 1.23x | 1.83x | 1.77x | 28/30 |
| fen_parse | 0.99x | 1.24x | 1.31x | 1.42x | 1.56x | 1.49x | 30/30 |
| fen_write_native_ep | 0.87x | 4.17x | 9.91x | 1.99x | 29.48x | 2.20x | 30/30 |
| snapshot | 0.88x | 1.95x | 1.07x | 2.42x | 1.23x | 1.06x | 26/30 |
| san_parse_all | 0.91x | 1.97x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 10.98x | N/A | 2.88x | N/A | N/A | 12/12 |
| apply_children_full | 0.68x | 0.70x | 0.75x | 3.58x | 0.72x | 0.80x | 10/30 |
| make_unmake_full | 0.52x | N/A | N/A | 0.64x | N/A | N/A | 1/6 |
| startpos | 0.75x | 0.93x | 70.80x | 123.52x | 146.52x | 112.67x | 4/5 |
| chess960_materialize | 1.05x | 1.79x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.82x | 2.42x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.82x | 34.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.89x | 0.93x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.84x | 22.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 6.40x | 2.47x | 1.49x | 2.84x | 3.03x | 10/10 |
| corpus_materialize | 0.86x | 1.42x | 1.45x | 1.14x | 1.88x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.85x | 1.69x | 2.38x | 1.88x | 2.47x | 1.38x | 10/10 |
| replay_trusted_native | 0.70x | 0.43x | 1.28x | 1.24x | 1.22x | 1.26x | 8/10 |
| replay_checked_native | 0.82x | 1.64x | 0.71x | 5.91x | 5.24x | 6.41x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
