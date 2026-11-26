| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.83x | 4.67x | 1.39x | 1.53x | 1.96x | 1.74x | 28/30 |
| materialize | 1.09x | 2.14x | 1.68x | 1.26x | 2.61x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.08x | 4.16x | 1.69x | 1.47x | 2.24x | 2.12x | 30/30 |
| perft_full_d3 | 0.95x | 1.48x | 1.63x | 1.29x | 1.89x | 1.87x | 30/30 |
| fen_parse | 1.10x | 1.33x | 1.57x | 1.49x | 1.80x | 1.45x | 30/30 |
| fen_write_native_ep | 0.96x | 3.95x | 8.84x | 1.62x | 26.41x | 2.24x | 30/30 |
| snapshot | 0.94x | 1.87x | 1.35x | 2.51x | 1.28x | 1.10x | 26/30 |
| san_parse_all | 0.94x | 1.83x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 12.22x | N/A | 2.78x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.15x | 1.30x | 6.02x | 1.19x | 1.39x | 28/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.79x | 0.86x | 70.31x | 147.84x | 172.21x | 125.53x | 4/5 |
| chess960_materialize | 1.16x | 1.67x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.97x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 40.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.79x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.11x | 19.56x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.71x | 2.05x | 1.14x | 2.64x | 2.54x | 10/10 |
| corpus_materialize | 0.99x | 1.49x | 1.51x | 1.11x | 1.78x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.89x | 2.17x | 2.78x | 2.28x | 2.62x | 1.52x | 10/10 |
| replay_trusted_native | 0.77x | 0.56x | 1.31x | 1.44x | 1.61x | 1.59x | 8/10 |
| replay_checked_native | 0.81x | 2.43x | 1.20x | 8.62x | 7.00x | 9.31x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
