| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.15x | 4.82x | 1.73x | 1.88x | 2.34x | 2.20x | 30/30 |
| materialize | 1.08x | 1.82x | 1.46x | 1.03x | 2.31x | 1.34x | 27/30 |
| perft_bulk_d4 | 1.08x | 4.65x | 1.69x | 1.28x | 2.08x | 1.97x | 30/30 |
| perft_full_d3 | 1.14x | 1.73x | 1.92x | 1.47x | 2.31x | 2.07x | 30/30 |
| fen_parse | 1.14x | 1.19x | 1.36x | 1.43x | 1.66x | 1.62x | 29/30 |
| fen_write_native_ep | 1.07x | 4.29x | 8.54x | 1.73x | 26.81x | 2.21x | 30/30 |
| snapshot | 0.99x | 1.78x | 1.02x | 2.17x | 1.02x | 1.08x | 24/30 |
| san_parse_all | 1.07x | 1.66x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 12.78x | N/A | 3.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.11x | 1.14x | 5.69x | 1.18x | 1.29x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.20x | 83.40x | 157.89x | 192.66x | 165.35x | 5/5 |
| chess960_materialize | 1.01x | 1.78x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 3.32x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.77x | 38.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.84x | 0.87x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 20.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.59x | 1.97x | 1.42x | 2.53x | 2.80x | 10/10 |
| corpus_materialize | 0.92x | 1.57x | 1.44x | 1.32x | 2.09x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.83x | 1.92x | 2.71x | 1.93x | 2.77x | 1.41x | 10/10 |
| replay_trusted_native | 0.84x | 0.68x | 1.45x | 1.66x | 1.82x | 1.90x | 8/10 |
| replay_checked_native | 0.85x | 2.64x | 1.09x | 9.20x | 7.25x | 9.55x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
