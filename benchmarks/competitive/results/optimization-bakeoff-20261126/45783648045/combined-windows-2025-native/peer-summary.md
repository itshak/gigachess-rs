| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.39x | 1.50x | 1.44x | 2.17x | 1.84x | 30/30 |
| materialize | 0.92x | 1.74x | 1.27x | 1.15x | 2.03x | 1.44x | 29/30 |
| perft_bulk_d4 | 1.00x | 4.00x | 1.61x | 1.38x | 2.19x | 1.98x | 30/30 |
| perft_full_d3 | 0.96x | 1.31x | 1.42x | 1.31x | 1.84x | 1.81x | 30/30 |
| fen_parse | 1.01x | 1.33x | 1.42x | 1.35x | 2.15x | 1.39x | 30/30 |
| fen_write_native_ep | 0.97x | 2.55x | 5.07x | 0.99x | 19.42x | 1.25x | 27/30 |
| snapshot | 0.48x | 1.45x | 1.14x | 1.24x | 1.16x | 1.12x | 30/30 |
| san_parse_all | 1.03x | 2.08x | N/A | 5.74x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.25x | N/A | 3.79x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.85x | 1.15x | 7.71x | 1.21x | 1.36x | 29/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 2.17x | 70.17x | 151.22x | 210.77x | 144.05x | 5/5 |
| chess960_materialize | 0.95x | 1.43x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.30x | 1.25x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 23.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.39x | 1.79x | 1.36x | 2.45x | 2.36x | 10/10 |
| corpus_materialize | 0.97x | 1.67x | 1.30x | 1.21x | 2.11x | 1.46x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.76x | 2.27x | 1.83x | 2.26x | 1.53x | 10/10 |
| replay_trusted_native | 1.00x | 0.69x | 1.37x | 1.77x | 2.13x | 2.46x | 8/10 |
| replay_checked_native | 1.01x | 2.35x | 1.00x | 7.80x | 7.64x | 9.42x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
