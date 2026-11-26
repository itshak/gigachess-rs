| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.69x | 1.69x | 1.62x | 2.12x | 1.83x | 30/30 |
| materialize | 0.95x | 2.07x | 1.54x | 1.02x | 2.32x | 1.31x | 27/30 |
| perft_bulk_d4 | 0.91x | 4.38x | 1.58x | 1.27x | 1.93x | 1.92x | 30/30 |
| perft_full_d3 | 0.74x | 1.23x | 1.34x | 1.05x | 1.52x | 1.49x | 27/30 |
| fen_parse | 0.99x | 1.39x | 1.40x | 1.35x | 1.66x | 1.59x | 30/30 |
| fen_write_native_ep | 0.87x | 3.22x | 6.48x | 1.20x | 19.67x | 1.56x | 30/30 |
| snapshot | 1.29x | 1.99x | 1.35x | 2.65x | 1.22x | 1.07x | 26/30 |
| san_parse_all | 1.02x | 1.85x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 0.88x | 12.68x | N/A | 2.91x | N/A | N/A | 12/12 |
| apply_children_full | 0.81x | 0.90x | 1.05x | 5.30x | 0.99x | 1.13x | 20/30 |
| make_unmake_full | 0.84x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.60x | 95.77x | 200.71x | 207.44x | 168.06x | 5/5 |
| chess960_materialize | 0.95x | 1.58x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.89x | 3.33x | 1.71x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 43.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.79x | 0.83x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.94x | 18.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 6.25x | 2.23x | 1.29x | 2.20x | 2.65x | 10/10 |
| corpus_materialize | 0.96x | 1.61x | 1.45x | 1.13x | 2.04x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.86x | 1.87x | 2.44x | 1.76x | 2.35x | 1.32x | 10/10 |
| replay_trusted_native | 0.83x | 0.56x | 1.46x | 1.64x | 1.85x | 1.63x | 8/10 |
| replay_checked_native | 0.85x | 2.27x | 1.09x | 8.00x | 6.85x | 8.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
