| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.11x | 1.48x | 1.44x | 2.22x | 1.79x | 30/30 |
| materialize | 0.95x | 1.87x | 1.25x | 1.06x | 2.09x | 1.35x | 28/30 |
| perft_bulk_d4 | 0.92x | 3.46x | 1.40x | 1.24x | 2.15x | 1.94x | 30/30 |
| perft_full_d3 | 0.92x | 1.21x | 1.26x | 1.03x | 1.69x | 1.50x | 26/30 |
| fen_parse | 1.07x | 1.39x | 1.51x | 1.49x | 2.17x | 1.49x | 30/30 |
| fen_write_native_ep | 0.93x | 3.63x | 7.09x | 1.49x | 28.66x | 1.85x | 30/30 |
| snapshot | 0.67x | 2.88x | 2.76x | 2.27x | 2.75x | 1.00x | 27/30 |
| san_parse_all | 0.97x | 1.86x | N/A | 4.61x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.30x | N/A | 3.27x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.71x | 1.17x | 7.20x | 1.34x | 1.61x | 30/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.13x | N/A | N/A | 5/6 |
| startpos | 1.00x | 2.76x | 80.05x | 143.47x | 202.36x | 144.19x | 5/5 |
| chess960_materialize | 0.96x | 1.55x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.52x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 20.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 3.88x | 1.55x | 1.13x | 2.03x | 1.85x | 9/10 |
| corpus_materialize | 0.99x | 1.66x | 1.29x | 1.11x | 2.61x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.86x | 2.31x | 1.90x | 2.21x | 1.46x | 10/10 |
| replay_trusted_native | 0.91x | 0.92x | 1.62x | 2.00x | 3.28x | 4.04x | 8/10 |
| replay_checked_native | 1.01x | 2.33x | 0.99x | 5.68x | 6.70x | 7.56x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
