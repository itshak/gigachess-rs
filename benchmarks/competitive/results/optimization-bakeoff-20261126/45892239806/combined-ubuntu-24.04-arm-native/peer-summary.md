| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 3.98x | 1.52x | 1.64x | 1.97x | 1.96x | 30/30 |
| materialize | 0.97x | 1.89x | 1.51x | 1.30x | 2.09x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.64x | 1.47x | 1.33x | 1.87x | 1.92x | 30/30 |
| perft_full_d3 | 0.80x | 1.19x | 1.29x | 1.07x | 1.60x | 1.49x | 30/30 |
| fen_parse | 0.98x | 1.21x | 1.30x | 1.23x | 1.71x | 1.35x | 30/30 |
| fen_write_native_ep | 0.95x | 4.26x | 8.87x | 1.51x | 28.45x | 2.28x | 30/30 |
| snapshot | 1.24x | 1.43x | 1.04x | 1.87x | 1.04x | 1.12x | 28/30 |
| san_parse_all | 0.98x | 1.83x | N/A | 4.21x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 11.79x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.19x | 1.39x | 3.91x | 1.15x | 1.16x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.14x | 1.35x | 86.53x | 149.27x | 201.46x | 159.75x | 5/5 |
| chess960_materialize | 0.96x | 1.64x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.79x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 26.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.15x | 21.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 4.13x | 1.58x | 1.27x | 1.85x | 2.05x | 10/10 |
| corpus_materialize | 0.96x | 1.54x | 1.47x | 1.18x | 1.83x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.15x | 2.98x | 2.13x | 2.85x | 1.50x | 10/10 |
| replay_trusted_native | 0.97x | 0.61x | 1.60x | 1.96x | 2.18x | 2.21x | 8/10 |
| replay_checked_native | 0.89x | 2.07x | 1.00x | 6.77x | 6.01x | 7.78x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
