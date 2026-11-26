| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.29x | 1.50x | 1.47x | 2.14x | 1.84x | 30/30 |
| materialize | 1.01x | 1.91x | 1.32x | 1.21x | 2.20x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.89x | 1.54x | 1.36x | 2.19x | 1.96x | 30/30 |
| perft_full_d3 | 0.90x | 1.22x | 1.30x | 1.17x | 1.69x | 1.63x | 30/30 |
| fen_parse | 0.89x | 1.14x | 1.32x | 1.16x | 2.01x | 1.38x | 30/30 |
| fen_write_native_ep | 0.99x | 4.13x | 7.99x | 1.58x | 31.71x | 1.95x | 30/30 |
| snapshot | 1.32x | 1.57x | 1.08x | 2.57x | 1.08x | 0.93x | 24/30 |
| san_parse_all | 1.02x | 2.00x | N/A | 5.41x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.67x | N/A | 3.70x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.95x | 1.19x | 8.26x | 1.26x | 1.39x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 68.03x | 128.93x | 204.19x | 153.18x | 5/5 |
| chess960_materialize | 0.99x | 1.48x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.55x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 27.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 18.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.31x | 1.76x | 1.35x | 2.46x | 2.36x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.29x | 1.15x | 2.07x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.76x | 2.22x | 1.81x | 2.15x | 1.40x | 10/10 |
| replay_trusted_native | 0.98x | 0.66x | 1.28x | 1.64x | 2.07x | 2.48x | 8/10 |
| replay_checked_native | 1.02x | 2.15x | 0.98x | 7.24x | 7.29x | 8.77x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
