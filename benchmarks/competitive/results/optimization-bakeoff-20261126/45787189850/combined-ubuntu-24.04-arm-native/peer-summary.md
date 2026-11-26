| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.47x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 0.99x | 1.81x | 1.43x | 1.24x | 1.98x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.77x | 1.53x | 1.38x | 1.95x | 2.00x | 30/30 |
| perft_full_d3 | 0.99x | 1.15x | 1.24x | 1.04x | 1.50x | 1.46x | 28/30 |
| fen_parse | 1.01x | 1.27x | 1.36x | 1.33x | 1.80x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.56x | 9.31x | 1.58x | 29.69x | 2.38x | 30/30 |
| snapshot | 0.91x | 1.11x | 0.99x | 1.44x | 0.99x | 0.89x | 14/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.82x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.09x | 1.29x | 3.57x | 1.06x | 1.05x | 27/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.19x | 70.90x | 123.93x | 159.55x | 129.54x | 5/5 |
| chess960_materialize | 1.00x | 1.50x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.63x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.79x | 1.83x | 1.46x | 2.17x | 2.37x | 10/10 |
| corpus_materialize | 0.99x | 1.56x | 1.49x | 1.19x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.95x | 2.07x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.51x | 1.31x | 1.61x | 1.75x | 1.84x | 8/10 |
| replay_checked_native | 0.98x | 1.99x | 0.98x | 6.54x | 5.81x | 7.53x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
