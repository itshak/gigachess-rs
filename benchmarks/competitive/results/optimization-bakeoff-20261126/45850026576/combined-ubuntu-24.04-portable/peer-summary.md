| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.19x | 1.63x | 1.47x | 2.05x | 1.43x | 30/30 |
| materialize | 1.04x | 2.41x | 1.92x | 1.34x | 2.57x | 1.71x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.05x | 1.58x | 1.25x | 1.99x | 1.70x | 30/30 |
| perft_full_d3 | 1.00x | 1.43x | 1.69x | 1.31x | 1.97x | 1.86x | 30/30 |
| fen_parse | 0.87x | 1.44x | 1.35x | 1.07x | 1.67x | 1.21x | 28/30 |
| fen_write_native_ep | 0.99x | 2.63x | 8.91x | 1.73x | 26.01x | 1.89x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.22x | 2.81x | 1.13x | 0.98x | 24/30 |
| san_parse_all | 0.98x | 2.22x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.93x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 2.28x | 1.37x | 5.42x | 1.32x | 1.36x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 72.52x | 102.93x | 154.06x | 112.15x | 5/5 |
| chess960_materialize | 1.04x | 1.91x | 1.68x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.42x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 21.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 20.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.20x | 1.76x | 1.24x | 1.93x | 1.84x | 10/10 |
| corpus_materialize | 1.00x | 2.07x | 1.84x | 1.39x | 3.06x | 1.76x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.98x | 2.25x | 2.94x | 1.61x | 10/10 |
| replay_trusted_native | 0.99x | 0.94x | 1.87x | 2.04x | 2.97x | 2.75x | 8/10 |
| replay_checked_native | 0.99x | 2.37x | 1.06x | 5.77x | 6.76x | 7.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
