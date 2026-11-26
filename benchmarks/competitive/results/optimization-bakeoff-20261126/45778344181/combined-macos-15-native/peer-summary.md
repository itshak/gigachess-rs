| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.36x | 1.45x | 1.50x | 1.94x | 1.75x | 30/30 |
| materialize | 1.01x | 1.96x | 1.50x | 1.03x | 2.31x | 1.30x | 27/30 |
| perft_bulk_d4 | 0.91x | 3.98x | 1.46x | 1.20x | 1.85x | 1.77x | 30/30 |
| perft_full_d3 | 0.96x | 1.51x | 1.67x | 1.28x | 1.88x | 1.94x | 30/30 |
| fen_parse | 0.93x | 1.13x | 1.22x | 1.20x | 1.47x | 1.24x | 30/30 |
| fen_write_native_ep | 1.00x | 3.05x | 6.45x | 1.26x | 20.58x | 1.62x | 30/30 |
| snapshot | 0.99x | 1.64x | 1.11x | 2.34x | 1.11x | 0.97x | 25/30 |
| san_parse_all | 0.99x | 1.77x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.86x | N/A | 2.78x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 0.99x | 1.06x | 5.00x | 1.02x | 1.16x | 21/30 |
| make_unmake_full | 0.85x | N/A | N/A | 1.04x | N/A | N/A | 2/6 |
| startpos | 1.01x | 1.18x | 66.10x | 131.64x | 156.89x | 127.79x | 5/5 |
| chess960_materialize | 1.02x | 1.58x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.90x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 38.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 20.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.52x | 2.15x | 1.28x | 2.53x | 2.74x | 10/10 |
| corpus_materialize | 1.03x | 1.47x | 1.40x | 1.14x | 1.91x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.84x | 2.65x | 1.88x | 2.47x | 1.40x | 10/10 |
| replay_trusted_native | 0.95x | 0.54x | 1.38x | 1.39x | 1.49x | 1.46x | 8/10 |
| replay_checked_native | 1.06x | 2.16x | 1.02x | 7.68x | 6.84x | 8.32x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
