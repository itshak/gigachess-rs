| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.14x | 1.43x | 1.39x | 2.03x | 1.76x | 30/30 |
| materialize | 0.97x | 1.84x | 1.25x | 1.15x | 2.12x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.79x | 1.52x | 1.33x | 2.23x | 1.95x | 30/30 |
| perft_full_d3 | 1.08x | 0.83x | 0.86x | 0.76x | 1.13x | 1.08x | 11/30 |
| fen_parse | 0.96x | 1.15x | 1.25x | 1.18x | 1.98x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 2.69x | 4.96x | 0.99x | 19.59x | 1.24x | 27/30 |
| snapshot | 1.00x | 1.60x | 1.19x | 2.45x | 1.19x | 1.04x | 30/30 |
| san_parse_all | 1.08x | 1.96x | N/A | 5.32x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.06x | N/A | 3.73x | N/A | N/A | 12/12 |
| apply_children_full | 1.24x | 1.67x | 1.03x | 6.69x | 1.09x | 1.21x | 27/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.07x | N/A | N/A | 5/6 |
| startpos | 1.15x | 1.11x | 71.68x | 131.90x | 198.25x | 155.28x | 5/5 |
| chess960_materialize | 0.74x | 1.34x | 1.09x | N/A | N/A | N/A | 3/4 |
| chess960_perft_d3 | 0.80x | 2.31x | 1.08x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 26.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.07x | 20.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.24x | 1.73x | 1.32x | 2.40x | 2.32x | 10/10 |
| corpus_materialize | 0.98x | 1.55x | 1.26x | 1.13x | 1.94x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.21x | 1.50x | 1.23x | 1.46x | 0.96x | 8/10 |
| replay_trusted_native | 1.18x | 0.61x | 1.19x | 1.55x | 1.95x | 2.28x | 8/10 |
| replay_checked_native | 1.09x | 2.16x | 0.98x | 7.40x | 7.35x | 8.96x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
