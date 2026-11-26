| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.87x | 1.47x | 1.59x | 1.91x | 1.91x | 30/30 |
| materialize | 1.02x | 1.84x | 1.44x | 1.27x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.90x | 3.33x | 1.36x | 1.24x | 1.75x | 1.79x | 30/30 |
| perft_full_d3 | 0.95x | 1.10x | 1.20x | 1.01x | 1.45x | 1.39x | 25/30 |
| fen_parse | 0.95x | 1.18x | 1.27x | 1.21x | 1.68x | 1.33x | 30/30 |
| fen_write_native_ep | 0.99x | 2.92x | 6.03x | 1.03x | 19.30x | 1.55x | 27/30 |
| snapshot | 1.31x | 1.47x | 1.31x | 1.96x | 1.27x | 1.28x | 30/30 |
| san_parse_all | 0.97x | 1.78x | N/A | 4.09x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 9.78x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.01x | 1.19x | 3.29x | 0.97x | 0.98x | 18/30 |
| make_unmake_full | 0.83x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.17x | 70.80x | 120.32x | 164.53x | 133.21x | 5/5 |
| chess960_materialize | 1.01x | 1.53x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 2.37x | 1.20x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 26.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.18x | 21.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.92x | 1.87x | 1.50x | 2.21x | 2.46x | 10/10 |
| corpus_materialize | 1.01x | 1.60x | 1.51x | 1.22x | 1.89x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.93x | 2.09x | 2.83x | 1.46x | 10/10 |
| replay_trusted_native | 0.89x | 0.46x | 1.17x | 1.42x | 1.58x | 1.63x | 8/10 |
| replay_checked_native | 0.89x | 1.81x | 0.88x | 6.00x | 5.29x | 6.91x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
