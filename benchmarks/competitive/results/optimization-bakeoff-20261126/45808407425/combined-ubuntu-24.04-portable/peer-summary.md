| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.63x | 1.51x | 1.31x | 1.74x | 1.28x | 29/30 |
| materialize | 0.99x | 2.24x | 1.82x | 1.76x | 2.70x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.93x | 1.56x | 1.15x | 1.91x | 1.75x | 30/30 |
| perft_full_d3 | 0.93x | 1.45x | 1.69x | 1.50x | 2.04x | 1.70x | 30/30 |
| fen_parse | 0.94x | 1.46x | 1.30x | 1.13x | 1.38x | 1.11x | 30/30 |
| fen_write_native_ep | 1.01x | 3.00x | 9.96x | 1.77x | 27.94x | 2.54x | 30/30 |
| snapshot | 1.13x | 1.62x | 1.25x | 3.25x | 1.25x | 1.12x | 30/30 |
| san_parse_all | 0.95x | 1.80x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 8.67x | N/A | 2.05x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.83x | 1.19x | 3.47x | 1.11x | 1.30x | 29/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.07x | N/A | N/A | 5/6 |
| startpos | 1.17x | 1.50x | 75.26x | 128.89x | 144.41x | 119.00x | 5/5 |
| chess960_materialize | 0.99x | 1.62x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 1.82x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 43.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 24.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.63x | 2.34x | 1.28x | 2.38x | 3.24x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.43x | 1.16x | 1.87x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 3.04x | 2.20x | 2.93x | 1.55x | 10/10 |
| replay_trusted_native | 0.89x | 0.73x | 1.37x | 1.34x | 1.81x | 1.68x | 7/10 |
| replay_checked_native | 0.93x | 1.87x | 1.05x | 7.93x | 7.86x | 8.81x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
