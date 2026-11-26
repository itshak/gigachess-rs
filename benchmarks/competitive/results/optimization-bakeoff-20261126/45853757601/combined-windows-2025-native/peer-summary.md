| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 5.02x | 1.69x | 1.59x | 2.41x | 2.08x | 30/30 |
| materialize | 1.01x | 2.07x | 1.35x | 1.18x | 2.39x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.18x | 1.51x | 1.36x | 2.34x | 2.21x | 30/30 |
| perft_full_d3 | 0.93x | 1.33x | 1.43x | 1.22x | 1.90x | 1.97x | 30/30 |
| fen_parse | 0.96x | 1.30x | 1.48x | 1.47x | 2.08x | 1.48x | 30/30 |
| fen_write_native_ep | 0.98x | 3.70x | 7.12x | 1.52x | 31.65x | 2.06x | 30/30 |
| snapshot | 0.99x | 2.54x | 1.68x | 5.49x | 1.57x | 1.50x | 30/30 |
| san_parse_all | 1.00x | 1.97x | N/A | 4.66x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.63x | N/A | 3.81x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.83x | 1.55x | 9.19x | 1.75x | 2.16x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 0.95x | 0.99x | 67.61x | 130.31x | 179.08x | 135.20x | 4/5 |
| chess960_materialize | 1.00x | 1.74x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.92x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 36.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 27.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.91x | 1.71x | 1.14x | 2.26x | 2.14x | 9/10 |
| corpus_materialize | 0.99x | 1.63x | 1.25x | 1.06x | 2.36x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.64x | 2.02x | 1.68x | 1.93x | 1.40x | 10/10 |
| replay_trusted_native | 1.08x | 0.77x | 1.47x | 1.72x | 2.57x | 3.13x | 9/10 |
| replay_checked_native | 0.97x | 2.48x | 1.14x | 8.36x | 8.35x | 10.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
