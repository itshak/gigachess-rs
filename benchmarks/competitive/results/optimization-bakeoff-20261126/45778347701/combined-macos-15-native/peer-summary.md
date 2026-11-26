| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.35x | 1.58x | 1.52x | 2.01x | 1.79x | 30/30 |
| materialize | 0.89x | 1.88x | 1.38x | 0.99x | 2.20x | 1.23x | 27/30 |
| perft_bulk_d4 | 0.95x | 3.77x | 1.42x | 1.16x | 1.76x | 1.72x | 27/30 |
| perft_full_d3 | 0.99x | 1.53x | 1.65x | 1.32x | 1.88x | 2.21x | 29/30 |
| fen_parse | 0.96x | 1.07x | 1.18x | 1.20x | 1.37x | 1.22x | 25/30 |
| fen_write_native_ep | 1.05x | 2.99x | 6.63x | 1.18x | 20.27x | 1.52x | 29/30 |
| snapshot | 1.16x | 1.80x | 1.40x | 2.74x | 1.34x | 1.11x | 28/30 |
| san_parse_all | 0.97x | 1.81x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.77x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 1.15x | 1.03x | 1.12x | 5.41x | 1.11x | 1.20x | 19/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.06x | N/A | N/A | 3/6 |
| startpos | 0.54x | 0.64x | 44.46x | 79.05x | 98.95x | 80.42x | 4/5 |
| chess960_materialize | 1.04x | 1.55x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.67x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.18x | 42.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.18x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 20.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 5.94x | 2.19x | 1.17x | 2.53x | 2.58x | 10/10 |
| corpus_materialize | 1.16x | 1.68x | 1.52x | 1.17x | 2.06x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.18x | 1.92x | 2.48x | 1.85x | 2.33x | 1.35x | 10/10 |
| replay_trusted_native | 1.01x | 0.54x | 1.29x | 1.36x | 1.43x | 1.41x | 8/10 |
| replay_checked_native | 1.21x | 2.17x | 1.04x | 8.28x | 7.48x | 8.45x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
