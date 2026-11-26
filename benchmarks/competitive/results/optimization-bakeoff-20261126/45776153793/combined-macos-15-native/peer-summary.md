| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.50x | 1.49x | 1.51x | 2.00x | 1.73x | 30/30 |
| materialize | 1.06x | 2.07x | 1.37x | 1.01x | 2.32x | 1.29x | 27/30 |
| perft_bulk_d4 | 0.94x | 3.88x | 1.43x | 1.20x | 1.78x | 1.73x | 30/30 |
| perft_full_d3 | 0.93x | 0.84x | 0.92x | 0.69x | 1.03x | 1.05x | 9/30 |
| fen_parse | 0.90x | 1.14x | 1.21x | 1.28x | 1.52x | 1.32x | 30/30 |
| fen_write_native_ep | 0.95x | 3.05x | 6.39x | 1.28x | 20.05x | 1.62x | 30/30 |
| snapshot | 0.78x | 1.28x | 0.86x | 1.91x | 0.87x | 0.80x | 12/30 |
| san_parse_all | 0.97x | 1.60x | N/A | 3.26x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 11.44x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 0.68x | 0.74x | 3.55x | 0.72x | 0.82x | 6/30 |
| make_unmake_full | 0.83x | N/A | N/A | 0.85x | N/A | N/A | 0/6 |
| startpos | 0.83x | 1.00x | 60.23x | 120.48x | 143.69x | 115.12x | 5/5 |
| chess960_materialize | 0.88x | 1.33x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.75x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 34.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.22x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 17.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 6.62x | 2.14x | 1.21x | 2.37x | 2.71x | 10/10 |
| corpus_materialize | 1.09x | 1.60x | 1.49x | 1.31x | 1.99x | 1.50x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.20x | 1.86x | 1.22x | 1.53x | 0.91x | 8/10 |
| replay_trusted_native | 0.87x | 0.40x | 0.93x | 1.05x | 1.13x | 1.18x | 4/10 |
| replay_checked_native | 0.82x | 1.68x | 0.77x | 6.08x | 5.30x | 6.67x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
