| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 3.77x | 1.43x | 1.54x | 1.86x | 1.85x | 30/30 |
| materialize | 1.00x | 1.84x | 1.50x | 1.27x | 2.03x | 1.39x | 30/30 |
| perft_bulk_d4 | 0.80x | 2.98x | 1.22x | 1.10x | 1.55x | 1.59x | 30/30 |
| perft_full_d3 | 0.77x | 0.90x | 0.98x | 0.82x | 1.18x | 1.13x | 16/30 |
| fen_parse | 0.96x | 1.21x | 1.29x | 1.26x | 1.71x | 1.36x | 30/30 |
| fen_write_native_ep | 0.97x | 4.39x | 9.05x | 1.54x | 29.61x | 2.33x | 30/30 |
| snapshot | 1.40x | 1.69x | 1.33x | 2.13x | 1.32x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 1.86x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.72x | 8.59x | N/A | 1.87x | N/A | N/A | 12/12 |
| apply_children_full | 0.69x | 0.77x | 0.91x | 2.51x | 0.75x | 0.75x | 7/30 |
| make_unmake_full | 0.53x | N/A | N/A | 0.76x | N/A | N/A | 0/6 |
| startpos | 0.96x | 1.14x | 70.95x | 120.44x | 157.06x | 124.45x | 5/5 |
| chess960_materialize | 0.99x | 1.50x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.78x | 2.06x | 1.05x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 28.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.23x | 23.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.68x | 1.80x | 1.41x | 2.18x | 2.34x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.50x | 1.21x | 1.91x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.82x | 1.46x | 10/10 |
| replay_trusted_native | 0.79x | 0.40x | 1.04x | 1.26x | 1.39x | 1.43x | 8/10 |
| replay_checked_native | 0.75x | 1.51x | 0.73x | 4.90x | 4.47x | 5.70x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
