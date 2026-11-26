| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.51x | 1.63x | 1.56x | 2.40x | 1.94x | 30/30 |
| materialize | 0.94x | 1.84x | 1.60x | 1.18x | 2.22x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.91x | 1.52x | 1.31x | 2.34x | 1.89x | 30/30 |
| perft_full_d3 | 0.82x | 1.28x | 1.05x | 0.91x | 1.33x | 1.28x | 23/30 |
| fen_parse | 1.04x | 1.41x | 1.44x | 1.32x | 2.12x | 1.39x | 30/30 |
| fen_write_native_ep | 0.99x | 4.25x | 9.51x | 1.77x | 28.03x | 2.11x | 30/30 |
| snapshot | 1.00x | 1.96x | 1.25x | 3.48x | 1.24x | 1.24x | 30/30 |
| san_parse_all | 0.93x | 1.91x | N/A | 4.01x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 9.51x | N/A | 2.00x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 2.15x | 1.23x | 4.47x | 1.38x | 1.51x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.16x | N/A | N/A | 5/6 |
| startpos | 0.86x | 1.07x | 66.76x | 123.77x | 189.54x | 137.27x | 5/5 |
| chess960_materialize | 0.92x | 1.47x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.90x | 2.52x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 27.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.33x | 1.80x | 1.34x | 2.60x | 2.36x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.51x | 1.19x | 2.00x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.47x | 3.20x | 2.47x | 3.09x | 1.74x | 10/10 |
| replay_trusted_native | 0.95x | 0.83x | 1.50x | 1.86x | 2.40x | 2.76x | 9/10 |
| replay_checked_native | 0.91x | 2.07x | 0.93x | 7.17x | 6.96x | 8.27x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
