| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.35x | 1.73x | 1.49x | 2.26x | 1.50x | 30/30 |
| materialize | 0.96x | 2.39x | 1.76x | 1.32x | 2.72x | 1.73x | 30/30 |
| perft_bulk_d4 | 1.09x | 3.47x | 1.76x | 1.35x | 2.31x | 1.99x | 30/30 |
| perft_full_d3 | 1.21x | 1.76x | 2.18x | 1.66x | 2.42x | 2.19x | 30/30 |
| fen_parse | 0.98x | 1.52x | 1.50x | 1.36x | 1.84x | 1.22x | 30/30 |
| fen_write_native_ep | 1.00x | 2.77x | 8.39x | 1.45x | 25.69x | 2.03x | 30/30 |
| snapshot | 0.99x | 1.64x | 1.23x | 2.82x | 1.16x | 1.05x | 27/30 |
| san_parse_all | 0.99x | 1.91x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.84x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.28x | 1.53x | 5.56x | 1.33x | 1.37x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.28x | 58.64x | 86.31x | 114.16x | 78.13x | 5/5 |
| chess960_materialize | 0.99x | 1.89x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.38x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 31.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 4.67x | 1.98x | 1.21x | 2.10x | 3.13x | 9/10 |
| corpus_materialize | 1.03x | 1.79x | 1.65x | 1.24x | 2.37x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.04x | 2.83x | 2.31x | 2.81x | 1.75x | 10/10 |
| replay_trusted_native | 1.03x | 0.77x | 1.72x | 1.75x | 2.33x | 2.11x | 9/10 |
| replay_checked_native | 0.88x | 1.92x | 1.13x | 7.52x | 8.59x | 8.95x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
