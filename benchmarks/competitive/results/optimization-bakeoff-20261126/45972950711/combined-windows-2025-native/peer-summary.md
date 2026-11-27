| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.80x | 1.78x | 1.71x | 2.68x | 2.16x | 30/30 |
| materialize | 1.02x | 2.01x | 1.34x | 1.15x | 2.33x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.96x | 1.64x | 1.45x | 2.45x | 2.24x | 30/30 |
| perft_full_d3 | 0.99x | 1.47x | 1.51x | 1.22x | 2.07x | 1.85x | 30/30 |
| fen_parse | 1.02x | 1.38x | 1.49x | 1.42x | 2.19x | 1.57x | 30/30 |
| fen_write_native_ep | 1.05x | 3.98x | 7.60x | 1.57x | 32.17x | 2.05x | 30/30 |
| snapshot | 1.00x | 1.79x | 1.25x | 4.11x | 1.25x | 1.17x | 30/30 |
| san_parse_all | 1.00x | 1.99x | N/A | 4.82x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.18x | N/A | 3.34x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.94x | 1.31x | 8.24x | 1.54x | 1.71x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.25x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.00x | 91.50x | 146.88x | 199.86x | 150.73x | 4/5 |
| chess960_materialize | 1.00x | 1.66x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.02x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 20.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 19.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.48x | 1.79x | 1.36x | 2.37x | 2.15x | 10/10 |
| corpus_materialize | 1.00x | 1.93x | 1.46x | 1.22x | 2.92x | 1.58x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.87x | 2.42x | 2.01x | 2.26x | 1.49x | 10/10 |
| replay_trusted_native | 0.96x | 1.03x | 1.86x | 2.27x | 3.91x | 5.12x | 9/10 |
| replay_checked_native | 0.97x | 2.40x | 1.07x | 5.98x | 7.50x | 8.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
