| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.86x | 1.79x | 1.72x | 2.76x | 2.17x | 30/30 |
| materialize | 0.98x | 1.99x | 1.32x | 1.12x | 2.33x | 1.48x | 29/30 |
| perft_bulk_d4 | 1.02x | 4.12x | 1.67x | 1.47x | 2.51x | 2.31x | 30/30 |
| perft_full_d3 | 1.32x | 2.19x | 2.06x | 1.70x | 2.78x | 2.45x | 30/30 |
| fen_parse | 0.97x | 1.35x | 1.52x | 1.38x | 2.15x | 1.47x | 30/30 |
| fen_write_native_ep | 0.96x | 3.81x | 7.68x | 1.53x | 29.90x | 1.92x | 30/30 |
| snapshot | 1.00x | 1.79x | 1.25x | 4.12x | 1.27x | 1.12x | 30/30 |
| san_parse_all | 1.02x | 1.98x | N/A | 4.80x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.41x | N/A | 3.37x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.96x | 1.34x | 8.50x | 1.54x | 1.75x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 87.50x | 132.57x | 203.75x | 144.38x | 5/5 |
| chess960_materialize | 0.99x | 1.63x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.87x | 1.62x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.42x | 1.77x | 1.31x | 2.37x | 2.14x | 9/10 |
| corpus_materialize | 1.00x | 1.79x | 1.38x | 1.21x | 2.82x | 1.52x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.86x | 2.37x | 1.96x | 2.26x | 1.50x | 10/10 |
| replay_trusted_native | 1.03x | 1.14x | 2.06x | 2.47x | 4.03x | 4.89x | 10/10 |
| replay_checked_native | 1.00x | 2.51x | 1.12x | 6.71x | 7.88x | 8.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
