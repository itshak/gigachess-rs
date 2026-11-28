| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.16x | 5.42x | 1.84x | 1.81x | 2.53x | 2.28x | 30/30 |
| materialize | 1.19x | 1.96x | 1.56x | 1.14x | 2.29x | 1.45x | 28/30 |
| perft_bulk_d4 | 1.18x | 4.26x | 1.56x | 1.33x | 2.11x | 1.93x | 30/30 |
| perft_full_d3 | 1.37x | 1.77x | 2.01x | 1.52x | 2.25x | 2.22x | 30/30 |
| fen_parse | 1.07x | 1.32x | 1.32x | 1.43x | 1.73x | 1.48x | 30/30 |
| fen_write_native_ep | 1.19x | 4.18x | 8.57x | 1.77x | 29.37x | 2.27x | 30/30 |
| snapshot | 0.98x | 1.64x | 1.03x | 2.18x | 1.13x | 0.90x | 23/30 |
| san_parse_all | 1.06x | 1.82x | N/A | 3.78x | N/A | N/A | 12/12 |
| san_render_native | 1.12x | 12.62x | N/A | 2.91x | N/A | N/A | 12/12 |
| apply_children_full | 1.16x | 1.23x | 1.28x | 6.18x | 1.20x | 1.35x | 28/30 |
| make_unmake_full | 1.12x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.20x | 67.87x | 129.58x | 160.87x | 137.74x | 5/5 |
| chess960_materialize | 1.07x | 1.76x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.12x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 36.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.62x | 1.54x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 18.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 6.44x | 2.06x | 1.13x | 2.33x | 2.54x | 10/10 |
| corpus_materialize | 0.97x | 1.41x | 1.19x | 1.07x | 1.79x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.89x | 1.70x | 2.46x | 1.84x | 2.43x | 1.45x | 10/10 |
| replay_trusted_native | 0.91x | 0.75x | 1.67x | 1.68x | 1.98x | 1.67x | 8/10 |
| replay_checked_native | 1.13x | 2.65x | 1.22x | 10.17x | 8.82x | 10.50x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
