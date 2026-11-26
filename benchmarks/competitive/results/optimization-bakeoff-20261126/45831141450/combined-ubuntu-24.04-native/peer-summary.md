| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 3.93x | 1.42x | 1.38x | 2.12x | 1.70x | 30/30 |
| materialize | 0.96x | 1.76x | 1.52x | 1.13x | 2.07x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.37x | 1.37x | 1.18x | 2.06x | 1.69x | 30/30 |
| perft_full_d3 | 0.92x | 1.15x | 1.32x | 1.13x | 1.67x | 1.63x | 30/30 |
| fen_parse | 0.93x | 1.32x | 1.54x | 1.15x | 2.07x | 1.24x | 30/30 |
| fen_write_native_ep | 0.92x | 3.83x | 9.62x | 1.59x | 25.96x | 1.91x | 30/30 |
| snapshot | 1.01x | 2.00x | 1.25x | 3.63x | 1.26x | 1.26x | 30/30 |
| san_parse_all | 1.01x | 2.08x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.11x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.23x | 1.27x | 4.48x | 1.42x | 1.56x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.50x | 1.88x | 116.66x | 201.72x | 359.12x | 232.84x | 5/5 |
| chess960_materialize | 0.96x | 1.43x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.48x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.66x | 1.55x | 1.17x | 2.22x | 2.05x | 9/10 |
| corpus_materialize | 0.98x | 1.49x | 1.46x | 1.09x | 1.90x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.56x | 3.39x | 2.52x | 3.39x | 1.86x | 10/10 |
| replay_trusted_native | 0.87x | 0.73x | 1.38x | 1.58x | 2.12x | 2.47x | 8/10 |
| replay_checked_native | 1.04x | 2.28x | 1.04x | 7.82x | 7.71x | 9.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
