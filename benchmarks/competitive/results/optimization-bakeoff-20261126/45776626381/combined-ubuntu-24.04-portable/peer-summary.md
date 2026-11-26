| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 2.93x | 1.49x | 1.36x | 1.97x | 1.34x | 29/30 |
| materialize | 1.05x | 2.02x | 1.78x | 1.28x | 2.40x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.02x | 1.60x | 1.24x | 1.97x | 1.72x | 30/30 |
| perft_full_d3 | 1.58x | 1.47x | 1.68x | 1.39x | 2.04x | 1.82x | 30/30 |
| fen_parse | 0.95x | 1.57x | 1.52x | 1.20x | 2.16x | 1.27x | 30/30 |
| fen_write_native_ep | 0.98x | 1.54x | 5.10x | 0.91x | 14.68x | 1.10x | 24/30 |
| snapshot | 1.00x | 1.69x | 1.20x | 3.01x | 1.00x | 1.07x | 26/30 |
| san_parse_all | 1.07x | 2.10x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.05x | N/A | 2.36x | N/A | N/A | 12/12 |
| apply_children_full | 1.29x | 1.95x | 1.33x | 3.90x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.25x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.26x | 67.88x | 106.41x | 175.01x | 109.86x | 5/5 |
| chess960_materialize | 1.05x | 1.51x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 2.11x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 27.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 20.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 3.90x | 1.95x | 1.28x | 2.15x | 2.58x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.62x | 1.18x | 2.11x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.81x | 2.19x | 3.19x | 2.45x | 3.08x | 1.81x | 10/10 |
| replay_trusted_native | 1.20x | 0.70x | 1.35x | 1.56x | 2.07x | 1.94x | 8/10 |
| replay_checked_native | 1.06x | 2.10x | 1.04x | 7.43x | 7.43x | 8.39x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
