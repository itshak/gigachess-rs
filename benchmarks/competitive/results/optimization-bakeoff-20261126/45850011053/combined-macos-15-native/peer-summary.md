| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.84x | 4.82x | 1.74x | 1.76x | 2.26x | 1.88x | 30/30 |
| materialize | 0.83x | 2.03x | 1.56x | 1.10x | 2.39x | 1.46x | 29/30 |
| perft_bulk_d4 | 0.85x | 4.13x | 1.64x | 1.22x | 1.81x | 1.89x | 29/30 |
| perft_full_d3 | 0.78x | 1.43x | 1.56x | 1.21x | 1.74x | 1.85x | 30/30 |
| fen_parse | 0.86x | 1.18x | 1.23x | 1.25x | 1.55x | 1.39x | 29/30 |
| fen_write_native_ep | 0.87x | 4.28x | 8.71x | 1.82x | 28.20x | 2.24x | 30/30 |
| snapshot | 0.81x | 1.33x | 0.85x | 1.89x | 0.94x | 0.80x | 16/30 |
| san_parse_all | 0.93x | 1.71x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 11.84x | N/A | 2.88x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.09x | 1.24x | 5.46x | 1.16x | 1.34x | 24/30 |
| make_unmake_full | 0.80x | N/A | N/A | 1.17x | N/A | N/A | 5/6 |
| startpos | 0.98x | 1.20x | 68.24x | 135.35x | 160.03x | 134.32x | 5/5 |
| chess960_materialize | 0.96x | 1.75x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 3.16x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 36.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 19.68x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.77x | 1.89x | 1.09x | 2.29x | 2.48x | 9/10 |
| corpus_materialize | 1.12x | 1.46x | 1.41x | 1.39x | 2.15x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.79x | 2.55x | 1.96x | 2.47x | 1.50x | 10/10 |
| replay_trusted_native | 0.92x | 0.56x | 1.51x | 1.49x | 1.57x | 1.78x | 8/10 |
| replay_checked_native | 1.00x | 2.45x | 1.14x | 8.29x | 8.15x | 9.42x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
