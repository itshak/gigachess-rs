| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 4.46x | 1.57x | 1.63x | 1.97x | 1.77x | 30/30 |
| materialize | 0.91x | 1.94x | 1.53x | 1.03x | 2.38x | 1.39x | 26/30 |
| perft_bulk_d4 | 0.83x | 4.10x | 1.50x | 1.27x | 1.91x | 1.78x | 29/30 |
| perft_full_d3 | 0.87x | 1.46x | 1.59x | 1.20x | 1.85x | 1.88x | 29/30 |
| fen_parse | 0.81x | 1.06x | 1.10x | 1.14x | 1.39x | 1.15x | 26/30 |
| fen_write_native_ep | 1.00x | 4.50x | 9.88x | 1.77x | 29.92x | 2.37x | 30/30 |
| snapshot | 1.10x | 1.82x | 1.21x | 2.82x | 1.16x | 1.10x | 24/30 |
| san_parse_all | 0.90x | 1.82x | N/A | 4.08x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 11.99x | N/A | 3.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 1.03x | 1.09x | 5.17x | 1.03x | 1.26x | 25/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.68x | 2.12x | 91.78x | 222.23x | 229.47x | 199.09x | 5/5 |
| chess960_materialize | 0.86x | 1.47x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 3.02x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.11x | 43.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.90x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.96x | 17.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 5.47x | 2.08x | 1.36x | 2.30x | 2.75x | 10/10 |
| corpus_materialize | 0.84x | 1.47x | 1.45x | 1.17x | 1.89x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.75x | 1.80x | 2.39x | 1.79x | 2.34x | 1.32x | 10/10 |
| replay_trusted_native | 0.70x | 0.54x | 1.30x | 1.55x | 1.88x | 1.49x | 8/10 |
| replay_checked_native | 0.87x | 2.36x | 1.04x | 8.77x | 7.31x | 7.71x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
