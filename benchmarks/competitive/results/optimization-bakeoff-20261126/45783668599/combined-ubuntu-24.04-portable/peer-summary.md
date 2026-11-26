| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.67x | 1.51x | 1.31x | 1.82x | 1.26x | 28/30 |
| materialize | 0.99x | 2.28x | 1.76x | 1.80x | 2.74x | 2.07x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.93x | 1.57x | 1.16x | 1.98x | 1.74x | 30/30 |
| perft_full_d3 | 0.99x | 1.50x | 1.73x | 1.57x | 2.11x | 1.76x | 30/30 |
| fen_parse | 1.00x | 1.64x | 1.49x | 1.17x | 1.65x | 1.24x | 30/30 |
| fen_write_native_ep | 0.97x | 1.90x | 6.31x | 1.30x | 17.48x | 1.41x | 30/30 |
| snapshot | 1.00x | 1.45x | 1.12x | 3.01x | 1.11x | 1.00x | 24/30 |
| san_parse_all | 1.00x | 1.88x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.25x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.85x | 1.18x | 3.45x | 1.11x | 1.29x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.76x | 101.22x | 130.20x | 105.58x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.93x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 42.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.01x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 22.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.63x | 2.34x | 1.26x | 2.34x | 3.17x | 10/10 |
| corpus_materialize | 1.00x | 1.53x | 1.43x | 1.15x | 1.85x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.97x | 2.92x | 2.22x | 2.86x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.68x | 1.29x | 1.32x | 1.79x | 1.64x | 8/10 |
| replay_checked_native | 0.98x | 1.90x | 1.09x | 8.30x | 8.20x | 9.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
