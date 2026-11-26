| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.33x | 1.46x | 1.47x | 1.92x | 1.75x | 27/30 |
| materialize | 1.01x | 1.98x | 1.59x | 1.16x | 2.23x | 1.47x | 29/30 |
| perft_bulk_d4 | 0.99x | 3.77x | 1.34x | 1.12x | 1.77x | 1.70x | 29/30 |
| perft_full_d3 | 0.94x | 1.50x | 1.66x | 1.26x | 1.81x | 1.87x | 30/30 |
| fen_parse | 0.97x | 1.24x | 1.29x | 1.30x | 1.67x | 1.45x | 30/30 |
| fen_write_native_ep | 1.04x | 4.33x | 9.20x | 1.85x | 28.53x | 2.26x | 30/30 |
| snapshot | 0.93x | 1.76x | 0.98x | 2.38x | 1.23x | 1.08x | 23/30 |
| san_parse_all | 1.00x | 1.73x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 12.51x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.15x | 1.30x | 6.28x | 1.25x | 1.30x | 29/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.39x | 1.77x | 101.57x | 181.64x | 299.53x | 198.42x | 5/5 |
| chess960_materialize | 0.80x | 1.28x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.34x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.84x | 32.91x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.12x | 24.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.77x | 4.72x | 1.57x | 0.96x | 2.14x | 2.00x | 8/10 |
| corpus_materialize | 0.89x | 1.32x | 1.38x | 1.18x | 1.85x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.89x | 1.72x | 2.69x | 1.81x | 2.14x | 1.35x | 10/10 |
| replay_trusted_native | 1.14x | 0.70x | 1.93x | 1.87x | 2.17x | 2.03x | 8/10 |
| replay_checked_native | 0.92x | 2.33x | 1.16x | 7.85x | 6.96x | 8.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
