| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 5.00x | 1.66x | 1.82x | 2.23x | 2.01x | 28/30 |
| materialize | 1.00x | 2.06x | 1.57x | 1.14x | 2.35x | 1.41x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.85x | 1.55x | 1.30x | 1.91x | 1.80x | 30/30 |
| perft_full_d3 | 1.08x | 1.57x | 1.75x | 1.36x | 1.90x | 2.00x | 29/30 |
| fen_parse | 0.87x | 1.07x | 1.12x | 1.06x | 1.38x | 1.14x | 21/30 |
| fen_write_native_ep | 0.95x | 3.95x | 8.17x | 1.61x | 27.94x | 2.10x | 30/30 |
| snapshot | 1.25x | 1.82x | 1.18x | 2.83x | 1.07x | 1.14x | 26/30 |
| san_parse_all | 0.99x | 1.88x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 10.98x | N/A | 2.76x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.18x | 1.26x | 5.67x | 1.10x | 1.29x | 29/30 |
| make_unmake_full | 0.81x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.31x | 1.18x | 75.85x | 154.85x | 180.30x | 149.11x | 5/5 |
| chess960_materialize | 1.22x | 1.55x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.26x | 3.70x | 1.62x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.31x | 49.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.79x | 0.76x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.99x | 19.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.30x | 5.93x | 1.94x | 1.03x | 2.17x | 2.23x | 9/10 |
| corpus_materialize | 1.32x | 1.57x | 1.49x | 1.11x | 1.93x | 1.25x | 9/10 |
| corpus_uci_sorted | 1.07x | 1.70x | 2.16x | 1.63x | 2.19x | 1.17x | 10/10 |
| replay_trusted_native | 1.18x | 0.61x | 1.54x | 1.55x | 1.77x | 1.77x | 8/10 |
| replay_checked_native | 1.16x | 2.24x | 1.13x | 7.88x | 6.88x | 8.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
