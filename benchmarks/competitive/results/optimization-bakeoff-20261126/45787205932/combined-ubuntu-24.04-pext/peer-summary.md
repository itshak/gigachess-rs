| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.63x | 1.61x | 1.63x | 2.54x | 1.86x | 30/30 |
| materialize | 1.06x | 2.13x | 1.68x | 1.17x | 2.21x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.05x | 1.57x | 1.30x | 2.47x | 2.06x | 30/30 |
| perft_full_d3 | 0.97x | 1.23x | 1.26x | 0.97x | 1.73x | 1.81x | 26/30 |
| fen_parse | 1.00x | 1.40x | 1.48x | 1.07x | 2.16x | 1.20x | 28/30 |
| fen_write_native_ep | 1.00x | 4.10x | 9.86x | 1.84x | 27.75x | 2.10x | 30/30 |
| snapshot | 1.23x | 1.92x | 1.26x | 5.93x | 1.27x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.31x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.59x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.57x | 1.02x | 3.70x | 1.14x | 1.31x | 27/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.09x | 1.20x | 94.16x | 135.31x | 244.46x | 153.37x | 5/5 |
| chess960_materialize | 1.06x | 1.80x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.93x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.36x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.97x | 1.90x | 1.46x | 2.65x | 2.16x | 10/10 |
| corpus_materialize | 1.00x | 2.00x | 1.81x | 1.27x | 2.98x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.40x | 3.19x | 2.37x | 3.09x | 1.69x | 10/10 |
| replay_trusted_native | 1.06x | 0.76x | 1.29x | 1.48x | 2.59x | 2.98x | 8/10 |
| replay_checked_native | 0.99x | 2.55x | 0.92x | 5.36x | 6.16x | 7.09x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
