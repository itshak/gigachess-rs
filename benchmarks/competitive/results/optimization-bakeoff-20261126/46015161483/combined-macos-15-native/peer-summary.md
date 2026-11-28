| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 4.89x | 1.74x | 1.74x | 2.31x | 2.17x | 30/30 |
| materialize | 1.08x | 1.86x | 1.45x | 1.02x | 2.17x | 1.30x | 27/30 |
| perft_bulk_d4 | 1.22x | 4.39x | 1.68x | 1.41x | 2.16x | 2.08x | 30/30 |
| perft_full_d3 | 1.31x | 1.74x | 1.98x | 1.50x | 2.20x | 2.15x | 30/30 |
| fen_parse | 1.02x | 1.21x | 1.30x | 1.28x | 1.61x | 1.44x | 30/30 |
| fen_write_native_ep | 1.17x | 4.14x | 8.58x | 1.70x | 26.77x | 2.17x | 30/30 |
| snapshot | 1.09x | 1.70x | 1.23x | 2.48x | 1.19x | 1.16x | 28/30 |
| san_parse_all | 1.06x | 1.73x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 1.09x | 11.94x | N/A | 2.75x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.16x | 1.20x | 5.49x | 1.19x | 1.28x | 30/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.52x | 70.97x | 143.71x | 163.64x | 137.70x | 5/5 |
| chess960_materialize | 1.01x | 1.69x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 3.23x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 37.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.42x | 1.46x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 19.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.31x | 6.64x | 2.39x | 1.35x | 2.63x | 2.83x | 10/10 |
| corpus_materialize | 1.01x | 1.44x | 1.35x | 1.08x | 2.01x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.16x | 1.87x | 2.67x | 1.96x | 2.53x | 1.42x | 10/10 |
| replay_trusted_native | 1.12x | 0.69x | 1.92x | 1.95x | 2.05x | 2.15x | 8/10 |
| replay_checked_native | 1.00x | 2.77x | 1.30x | 9.79x | 8.79x | 10.53x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
