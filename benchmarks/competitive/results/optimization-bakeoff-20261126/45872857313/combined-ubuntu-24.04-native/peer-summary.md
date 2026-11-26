| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.31x | 1.87x | 1.55x | 2.46x | 1.88x | 30/30 |
| materialize | 1.00x | 2.19x | 1.67x | 1.70x | 2.60x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.73x | 1.52x | 1.16x | 2.30x | 1.93x | 29/30 |
| perft_full_d3 | 1.01x | 1.64x | 1.62x | 1.52x | 1.98x | 1.76x | 30/30 |
| fen_parse | 0.99x | 1.28x | 1.37x | 1.17x | 1.53x | 1.13x | 30/30 |
| fen_write_native_ep | 1.01x | 4.95x | 10.68x | 2.07x | 30.16x | 2.16x | 30/30 |
| snapshot | 1.01x | 1.99x | 1.00x | 4.62x | 0.99x | 1.32x | 21/30 |
| san_parse_all | 1.03x | 1.95x | N/A | 4.52x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 9.15x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.17x | 1.37x | 4.03x | 1.43x | 1.66x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.01x | 79.34x | 141.49x | 186.05x | 143.81x | 5/5 |
| chess960_materialize | 0.97x | 1.75x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.54x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 38.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.14x | 22.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.33x | 2.05x | 1.12x | 2.48x | 2.30x | 9/10 |
| corpus_materialize | 1.03x | 1.40x | 1.29x | 1.09x | 1.71x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.31x | 3.05x | 2.22x | 2.90x | 1.50x | 10/10 |
| replay_trusted_native | 1.03x | 0.90x | 1.55x | 1.66x | 2.28x | 2.63x | 9/10 |
| replay_checked_native | 0.98x | 2.25x | 1.15x | 9.05x | 9.04x | 10.31x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
