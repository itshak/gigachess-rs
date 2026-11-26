| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.97x | 1.70x | 1.58x | 2.49x | 2.06x | 30/30 |
| materialize | 1.05x | 2.15x | 1.40x | 1.19x | 2.48x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.22x | 1.54x | 1.37x | 2.44x | 2.20x | 30/30 |
| perft_full_d3 | 0.89x | 1.25x | 1.43x | 1.13x | 1.79x | 1.83x | 30/30 |
| fen_parse | 0.94x | 1.26x | 1.49x | 1.48x | 2.02x | 1.46x | 30/30 |
| fen_write_native_ep | 1.00x | 3.60x | 7.29x | 1.50x | 31.26x | 2.04x | 30/30 |
| snapshot | 1.01x | 2.52x | 1.51x | 5.57x | 1.51x | 1.50x | 30/30 |
| san_parse_all | 0.96x | 1.86x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 0.85x | 9.14x | N/A | 3.26x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.80x | 1.31x | 8.51x | 1.35x | 1.88x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.23x | 1.25x | 87.97x | 172.55x | 229.38x | 176.23x | 5/5 |
| chess960_materialize | 0.91x | 1.63x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.77x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 32.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.83x | 23.49x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.79x | 1.69x | 1.13x | 2.23x | 2.15x | 9/10 |
| corpus_materialize | 0.97x | 1.64x | 1.30x | 1.07x | 2.21x | 1.33x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.57x | 1.87x | 1.67x | 1.82x | 1.33x | 10/10 |
| replay_trusted_native | 1.04x | 0.71x | 1.39x | 1.67x | 2.44x | 2.92x | 8/10 |
| replay_checked_native | 0.94x | 2.18x | 1.04x | 7.79x | 7.60x | 9.42x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
