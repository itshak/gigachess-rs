| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.89x | 4.63x | 1.55x | 1.45x | 2.17x | 1.89x | 30/30 |
| materialize | 0.95x | 2.03x | 1.35x | 1.15x | 2.25x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.83x | 3.51x | 1.37x | 1.10x | 2.07x | 1.86x | 28/30 |
| perft_full_d3 | 0.70x | 1.01x | 1.28x | 0.94x | 1.53x | 1.58x | 22/30 |
| fen_parse | 0.91x | 1.34x | 1.46x | 1.45x | 2.11x | 1.75x | 30/30 |
| fen_write_native_ep | 0.94x | 3.40x | 6.67x | 1.43x | 29.15x | 1.92x | 30/30 |
| snapshot | 0.93x | 2.30x | 1.45x | 7.35x | 1.45x | 1.36x | 30/30 |
| san_parse_all | 0.91x | 1.78x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.87x | 10.11x | N/A | 3.56x | N/A | N/A | 12/12 |
| apply_children_full | 0.81x | 1.73x | 1.28x | 7.68x | 1.27x | 1.76x | 30/30 |
| make_unmake_full | 0.87x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.90x | 0.91x | 76.70x | 125.25x | 178.83x | 139.63x | 4/5 |
| chess960_materialize | 0.82x | 1.61x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.61x | 1.97x | 1.03x | N/A | N/A | N/A | 3/4 |
| polyglot_key | 0.94x | 37.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.97x | 26.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.49x | 1.66x | 1.14x | 2.17x | 2.08x | 9/10 |
| corpus_materialize | 1.02x | 1.66x | 1.33x | 1.14x | 2.19x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.07x | 1.76x | 2.10x | 1.82x | 2.06x | 1.47x | 10/10 |
| replay_trusted_native | 1.01x | 0.78x | 1.47x | 1.69x | 2.56x | 3.19x | 9/10 |
| replay_checked_native | 0.88x | 2.17x | 1.00x | 7.94x | 7.86x | 9.86x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
