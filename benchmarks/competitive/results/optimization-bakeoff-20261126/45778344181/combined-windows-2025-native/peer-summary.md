| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.36x | 1.44x | 1.36x | 2.05x | 1.77x | 30/30 |
| materialize | 0.97x | 2.04x | 1.40x | 1.17x | 2.30x | 1.53x | 30/30 |
| perft_bulk_d4 | 0.88x | 3.53x | 1.32x | 1.12x | 2.00x | 1.89x | 30/30 |
| perft_full_d3 | 0.87x | 1.08x | 1.28x | 0.99x | 1.59x | 1.72x | 25/30 |
| fen_parse | 0.93x | 1.36x | 1.50x | 1.54x | 2.16x | 1.56x | 30/30 |
| fen_write_native_ep | 1.02x | 3.13x | 6.07x | 1.28x | 26.48x | 1.78x | 30/30 |
| snapshot | 0.30x | 1.31x | 1.00x | 1.04x | 1.00x | 1.00x | 24/30 |
| san_parse_all | 0.90x | 1.81x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 0.89x | 9.95x | N/A | 3.40x | N/A | N/A | 12/12 |
| apply_children_full | 0.69x | 1.23x | 0.91x | 5.43x | 0.91x | 1.26x | 18/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 0.97x | 3.46x | 91.77x | 178.63x | 244.01x | 183.31x | 5/5 |
| chess960_materialize | 1.12x | 1.45x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.38x | 1.18x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 34.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.57x | 14.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 6.12x | 1.81x | 1.22x | 2.37x | 2.27x | 10/10 |
| corpus_materialize | 1.02x | 1.75x | 1.45x | 1.27x | 2.51x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.10x | 1.75x | 2.20x | 1.83x | 2.13x | 1.51x | 10/10 |
| replay_trusted_native | 0.91x | 0.67x | 1.10x | 1.32x | 1.98x | 2.51x | 7/10 |
| replay_checked_native | 1.02x | 1.91x | 0.95x | 6.82x | 6.87x | 8.83x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
