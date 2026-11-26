| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 3.00x | 1.54x | 1.39x | 1.97x | 1.38x | 30/30 |
| materialize | 0.85x | 1.68x | 1.49x | 1.07x | 2.00x | 1.30x | 27/30 |
| perft_bulk_d4 | 1.01x | 2.94x | 1.60x | 1.24x | 1.99x | 1.72x | 30/30 |
| perft_full_d3 | 0.98x | 1.42x | 1.67x | 1.31x | 1.98x | 1.83x | 30/30 |
| fen_parse | 0.98x | 1.42x | 1.44x | 1.21x | 2.20x | 1.30x | 30/30 |
| fen_write_native_ep | 1.01x | 3.03x | 10.03x | 1.71x | 28.09x | 2.01x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.22x | 3.05x | 1.03x | 0.98x | 23/30 |
| san_parse_all | 1.03x | 2.04x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.59x | N/A | 2.34x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.07x | 1.42x | 4.15x | 1.31x | 1.25x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 66.41x | 117.20x | 178.58x | 111.81x | 5/5 |
| chess960_materialize | 1.02x | 1.48x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 2.25x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 25.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.89x | 18.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 3.67x | 1.86x | 1.23x | 2.05x | 2.44x | 10/10 |
| corpus_materialize | 0.89x | 1.45x | 1.49x | 1.07x | 1.91x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.14x | 3.21x | 2.43x | 3.07x | 1.76x | 10/10 |
| replay_trusted_native | 1.04x | 0.79x | 1.62x | 1.75x | 2.36x | 2.16x | 8/10 |
| replay_checked_native | 1.03x | 2.14x | 1.09x | 7.30x | 7.63x | 8.47x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
