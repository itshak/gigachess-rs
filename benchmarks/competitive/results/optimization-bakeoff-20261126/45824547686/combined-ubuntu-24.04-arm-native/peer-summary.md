| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.89x | 1.49x | 1.60x | 1.93x | 1.92x | 30/30 |
| materialize | 1.00x | 1.83x | 1.44x | 1.26x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.41x | 1.39x | 1.26x | 1.77x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.18x | 1.28x | 1.07x | 1.57x | 1.49x | 29/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.27x | 1.76x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.60x | 9.37x | 1.61x | 30.33x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.10x | 1.06x | 1.45x | 1.05x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.75x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.38x | 1.64x | 4.44x | 1.33x | 1.33x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 76.35x | 128.15x | 176.21x | 142.03x | 5/5 |
| chess960_materialize | 0.98x | 1.55x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.66x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 25.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.91x | 1.51x | 1.23x | 1.77x | 1.95x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.45x | 1.17x | 1.81x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.09x | 2.89x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 0.98x | 0.59x | 1.58x | 1.92x | 2.14x | 2.18x | 8/10 |
| replay_checked_native | 1.00x | 2.16x | 1.06x | 7.21x | 6.34x | 8.22x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
