| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.44x | 1.73x | 1.71x | 2.34x | 1.93x | 30/30 |
| materialize | 1.00x | 2.15x | 1.58x | 1.44x | 2.37x | 1.57x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.61x | 1.43x | 1.24x | 2.00x | 1.70x | 29/30 |
| perft_full_d3 | 1.00x | 1.44x | 1.57x | 1.40x | 1.92x | 1.79x | 30/30 |
| fen_parse | 1.01x | 1.33x | 1.54x | 1.31x | 1.78x | 1.38x | 30/30 |
| fen_write_native_ep | 1.02x | 5.06x | 9.94x | 1.92x | 28.17x | 2.64x | 30/30 |
| snapshot | 1.00x | 2.07x | 1.13x | 4.67x | 1.87x | 1.35x | 30/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.10x | N/A | 2.43x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.09x | 1.25x | 4.63x | 1.31x | 1.47x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 88.73x | 175.66x | 225.19x | 174.79x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.53x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 31.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.06x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.67x | 1.75x | 1.07x | 2.13x | 2.02x | 10/10 |
| corpus_materialize | 1.03x | 1.54x | 1.43x | 1.15x | 1.81x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.20x | 2.91x | 2.19x | 2.77x | 1.49x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.61x | 1.85x | 2.28x | 2.39x | 9/10 |
| replay_checked_native | 1.00x | 2.27x | 1.10x | 8.44x | 7.82x | 9.32x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
