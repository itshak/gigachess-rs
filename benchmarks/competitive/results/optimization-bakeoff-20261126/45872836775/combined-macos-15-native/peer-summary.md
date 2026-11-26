| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.85x | 4.91x | 1.71x | 1.86x | 2.57x | 1.90x | 30/30 |
| materialize | 0.84x | 1.98x | 1.63x | 1.04x | 2.28x | 1.36x | 27/30 |
| perft_bulk_d4 | 0.87x | 3.81x | 1.56x | 1.20x | 1.95x | 1.80x | 29/30 |
| perft_full_d3 | 0.88x | 1.54x | 1.63x | 1.24x | 2.12x | 1.93x | 29/30 |
| fen_parse | 0.90x | 1.24x | 1.59x | 1.42x | 1.62x | 1.37x | 30/30 |
| fen_write_native_ep | 0.86x | 4.31x | 9.14x | 1.77x | 28.55x | 2.16x | 30/30 |
| snapshot | 0.80x | 1.63x | 1.08x | 2.23x | 1.13x | 1.02x | 23/30 |
| san_parse_all | 0.94x | 1.74x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 13.14x | N/A | 2.88x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.17x | 1.16x | 5.68x | 1.10x | 1.27x | 29/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 0.85x | 0.96x | 53.68x | 118.29x | 131.83x | 107.05x | 4/5 |
| chess960_materialize | 1.06x | 1.66x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 3.19x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.20x | 40.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.92x | 19.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 6.40x | 2.18x | 1.44x | 2.23x | 2.67x | 9/10 |
| corpus_materialize | 1.24x | 1.47x | 1.39x | 1.18x | 1.96x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.14x | 1.87x | 2.73x | 1.98x | 2.50x | 1.50x | 10/10 |
| replay_trusted_native | 1.18x | 0.62x | 1.58x | 1.53x | 1.67x | 1.61x | 8/10 |
| replay_checked_native | 1.19x | 2.62x | 1.23x | 10.62x | 9.35x | 10.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
