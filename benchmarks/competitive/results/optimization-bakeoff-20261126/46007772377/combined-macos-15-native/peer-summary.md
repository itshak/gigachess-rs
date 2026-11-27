| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 5.09x | 1.73x | 1.86x | 2.35x | 2.31x | 30/30 |
| materialize | 1.00x | 1.91x | 1.46x | 1.10x | 2.30x | 1.39x | 28/30 |
| perft_bulk_d4 | 1.03x | 4.24x | 1.70x | 1.30x | 2.02x | 1.93x | 30/30 |
| perft_full_d3 | 1.11x | 1.77x | 1.96x | 1.51x | 2.26x | 2.25x | 30/30 |
| fen_parse | 0.92x | 1.21x | 1.31x | 1.29x | 1.61x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 4.10x | 9.04x | 1.75x | 27.59x | 2.21x | 30/30 |
| snapshot | 1.01x | 1.70x | 1.12x | 2.43x | 1.13x | 1.01x | 27/30 |
| san_parse_all | 0.94x | 1.69x | N/A | 3.59x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.51x | N/A | 2.84x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.15x | 1.25x | 5.78x | 1.17x | 1.33x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.21x | 74.03x | 144.42x | 175.64x | 138.95x | 5/5 |
| chess960_materialize | 1.06x | 1.78x | 2.00x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 3.27x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 39.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 6.47x | 2.03x | 1.29x | 2.42x | 2.70x | 10/10 |
| corpus_materialize | 0.98x | 1.49x | 1.48x | 1.14x | 1.96x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.90x | 2.71x | 1.91x | 2.53x | 1.36x | 10/10 |
| replay_trusted_native | 1.02x | 0.67x | 1.74x | 1.76x | 1.80x | 1.81x | 8/10 |
| replay_checked_native | 0.91x | 2.54x | 1.21x | 9.15x | 8.11x | 9.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
