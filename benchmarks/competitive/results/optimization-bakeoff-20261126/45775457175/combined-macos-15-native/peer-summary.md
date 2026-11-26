| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.36x | 1.44x | 1.43x | 1.87x | 1.67x | 30/30 |
| materialize | 1.07x | 1.93x | 1.37x | 0.99x | 2.33x | 1.24x | 26/30 |
| perft_bulk_d4 | 1.14x | 4.15x | 1.52x | 1.23x | 1.93x | 1.85x | 30/30 |
| perft_full_d3 | 1.12x | 0.91x | 1.02x | 0.78x | 1.14x | 1.17x | 16/30 |
| fen_parse | 1.02x | 1.25x | 1.32x | 1.28x | 1.56x | 1.32x | 30/30 |
| fen_write_native_ep | 1.08x | 3.16x | 6.73x | 1.37x | 21.51x | 1.67x | 30/30 |
| snapshot | 1.10x | 1.65x | 1.11x | 2.36x | 1.12x | 0.95x | 25/30 |
| san_parse_all | 1.03x | 1.60x | N/A | 3.41x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 11.78x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 0.77x | 0.80x | 3.97x | 0.79x | 0.89x | 6/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.00x | N/A | N/A | 2/6 |
| startpos | 0.91x | 1.18x | 76.56x | 137.08x | 161.77x | 130.57x | 5/5 |
| chess960_materialize | 0.95x | 1.41x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 3.15x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 38.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 18.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.21x | 2.19x | 1.35x | 2.50x | 2.86x | 10/10 |
| corpus_materialize | 0.88x | 1.49x | 1.43x | 1.14x | 1.83x | 1.21x | 9/10 |
| corpus_uci_sorted | 0.89x | 1.41x | 1.87x | 1.22x | 1.69x | 0.92x | 9/10 |
| replay_trusted_native | 0.79x | 0.42x | 1.14x | 1.09x | 1.20x | 1.17x | 7/10 |
| replay_checked_native | 0.84x | 1.97x | 1.01x | 7.64x | 6.69x | 8.62x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
