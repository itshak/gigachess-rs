| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.46x | 1.69x | 1.82x | 2.19x | 2.18x | 30/30 |
| materialize | 1.01x | 1.98x | 1.54x | 1.36x | 2.18x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.04x | 1.64x | 1.48x | 2.10x | 2.15x | 30/30 |
| perft_full_d3 | 1.12x | 1.67x | 1.80x | 1.50x | 2.20x | 2.10x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.28x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.01x | 4.43x | 9.38x | 1.57x | 29.96x | 2.41x | 30/30 |
| snapshot | 1.00x | 1.04x | 1.01x | 1.45x | 1.01x | 0.96x | 17/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.40x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.01x | 1.19x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 76.33x | 124.50x | 173.52x | 138.97x | 5/5 |
| chess960_materialize | 1.01x | 1.73x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.00x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.54x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.86x | 1.86x | 1.49x | 2.17x | 2.42x | 10/10 |
| corpus_materialize | 1.01x | 1.63x | 1.56x | 1.25x | 1.92x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.93x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.65x | 2.00x | 2.26x | 2.27x | 8/10 |
| replay_checked_native | 1.01x | 2.33x | 1.13x | 7.65x | 6.81x | 8.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
