| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.02x | 1.74x | 1.51x | 2.03x | 1.49x | 30/30 |
| materialize | 1.00x | 2.28x | 1.86x | 1.91x | 2.82x | 2.03x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.24x | 1.71x | 1.27x | 2.14x | 1.95x | 30/30 |
| perft_full_d3 | 1.16x | 1.59x | 1.89x | 1.66x | 2.23x | 1.87x | 30/30 |
| fen_parse | 0.96x | 1.52x | 1.34x | 1.17x | 1.47x | 1.21x | 30/30 |
| fen_write_native_ep | 1.01x | 2.98x | 10.15x | 1.95x | 27.85x | 2.58x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.26x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.93x | 1.79x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.27x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.24x | 1.48x | 4.35x | 1.38x | 1.60x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.35x | 61.85x | 110.87x | 126.95x | 111.53x | 5/5 |
| chess960_materialize | 0.98x | 1.82x | 1.85x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.14x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.63x | 1.57x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.88x | 20.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 4.91x | 2.46x | 1.34x | 2.49x | 3.44x | 10/10 |
| corpus_materialize | 0.99x | 1.50x | 1.40x | 1.13x | 1.81x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.93x | 2.96x | 2.21x | 2.85x | 1.54x | 10/10 |
| replay_trusted_native | 1.00x | 0.95x | 1.73x | 1.66x | 2.33x | 2.15x | 9/10 |
| replay_checked_native | 1.00x | 2.10x | 1.19x | 8.98x | 8.74x | 9.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
