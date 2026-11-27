| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 3.28x | 1.64x | 1.65x | 2.06x | 1.46x | 30/30 |
| materialize | 0.99x | 2.44x | 1.83x | 1.47x | 2.44x | 1.71x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.06x | 1.63x | 1.37x | 2.04x | 1.75x | 30/30 |
| perft_full_d3 | 1.02x | 1.42x | 1.68x | 1.31x | 1.95x | 1.84x | 30/30 |
| fen_parse | 0.98x | 1.77x | 1.58x | 1.27x | 2.01x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 2.63x | 9.44x | 1.73x | 26.18x | 2.00x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 2.81x | 1.13x | 0.97x | 24/30 |
| san_parse_all | 1.03x | 2.24x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.71x | N/A | 2.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.42x | 1.51x | 5.49x | 1.42x | 1.44x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 73.50x | 112.67x | 160.28x | 115.26x | 5/5 |
| chess960_materialize | 0.99x | 1.95x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.44x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.22x | 1.77x | 1.25x | 2.03x | 1.84x | 10/10 |
| corpus_materialize | 1.04x | 2.15x | 1.81x | 1.32x | 3.02x | 1.71x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.06x | 2.94x | 2.22x | 2.86x | 1.59x | 10/10 |
| replay_trusted_native | 0.98x | 1.06x | 2.13x | 2.35x | 3.39x | 3.11x | 10/10 |
| replay_checked_native | 1.01x | 2.68x | 1.16x | 6.39x | 7.21x | 8.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
