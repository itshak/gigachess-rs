| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.76x | 1.42x | 1.31x | 1.83x | 1.34x | 29/30 |
| materialize | 1.01x | 2.19x | 1.75x | 1.25x | 2.28x | 1.65x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.64x | 1.41x | 1.10x | 1.76x | 1.51x | 30/30 |
| perft_full_d3 | 0.93x | 0.75x | 0.88x | 0.67x | 1.03x | 0.96x | 6/30 |
| fen_parse | 0.91x | 1.58x | 1.47x | 1.19x | 1.91x | 1.24x | 30/30 |
| fen_write_native_ep | 1.00x | 1.88x | 6.21x | 1.20x | 18.41x | 1.42x | 30/30 |
| snapshot | 0.75x | 1.22x | 0.92x | 2.10x | 0.85x | 0.75x | 12/30 |
| san_parse_all | 0.90x | 1.82x | N/A | 3.17x | N/A | N/A | 12/12 |
| san_render_native | 0.89x | 9.50x | N/A | 2.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.84x | 1.26x | 0.77x | 2.92x | 0.72x | 0.76x | 12/30 |
| make_unmake_full | 0.76x | N/A | N/A | 0.79x | N/A | N/A | 0/6 |
| startpos | 0.74x | 0.97x | 54.26x | 80.97x | 120.87x | 78.47x | 4/5 |
| chess960_materialize | 1.02x | 1.66x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 1.96x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 22.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.83x | 16.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.05x | 1.70x | 1.19x | 1.90x | 1.79x | 10/10 |
| corpus_materialize | 0.98x | 1.96x | 1.79x | 1.28x | 2.92x | 1.68x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.24x | 1.78x | 1.34x | 1.70x | 0.97x | 8/10 |
| replay_trusted_native | 0.94x | 0.52x | 1.03x | 1.15x | 1.65x | 1.50x | 7/10 |
| replay_checked_native | 0.87x | 1.95x | 0.83x | 4.57x | 5.01x | 5.92x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
