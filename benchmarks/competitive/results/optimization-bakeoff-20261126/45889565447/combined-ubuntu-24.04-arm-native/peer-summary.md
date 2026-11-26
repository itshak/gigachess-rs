| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.22x | 1.61x | 1.73x | 2.09x | 2.08x | 30/30 |
| materialize | 0.99x | 1.92x | 1.50x | 1.35x | 2.13x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.77x | 1.54x | 1.40x | 1.97x | 2.01x | 30/30 |
| perft_full_d3 | 1.14x | 1.69x | 1.83x | 1.52x | 2.24x | 2.12x | 30/30 |
| fen_parse | 0.99x | 1.23x | 1.31x | 1.26x | 1.72x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 4.50x | 9.34x | 1.60x | 30.16x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.01x | 1.45x | 0.99x | 0.97x | 18/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.35x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.25x | 1.48x | 4.02x | 1.18x | 1.19x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.46x | 130.71x | 172.85x | 137.54x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.92x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 4.08x | 1.57x | 1.27x | 1.83x | 2.01x | 10/10 |
| corpus_materialize | 0.97x | 1.57x | 1.49x | 1.23x | 1.85x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.84x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.66x | 2.02x | 2.25x | 2.29x | 8/10 |
| replay_checked_native | 1.01x | 2.33x | 1.14x | 7.77x | 6.82x | 8.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
