| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.75x | 1.67x | 1.58x | 2.31x | 2.00x | 30/30 |
| materialize | 0.99x | 2.15x | 1.69x | 1.21x | 2.43x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.30x | 1.59x | 1.25x | 2.41x | 2.25x | 30/30 |
| perft_full_d3 | 1.00x | 1.93x | 1.66x | 1.23x | 2.17x | 2.06x | 30/30 |
| fen_parse | 0.99x | 1.41x | 1.62x | 1.25x | 2.15x | 1.28x | 30/30 |
| fen_write_native_ep | 0.98x | 3.78x | 8.38x | 1.46x | 25.00x | 1.89x | 30/30 |
| snapshot | 0.99x | 2.49x | 1.50x | 4.48x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 0.99x | 1.80x | N/A | 3.98x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.75x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.75x | 1.30x | 3.77x | 1.25x | 1.75x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.17x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.25x | 83.95x | 137.74x | 224.48x | 138.58x | 5/5 |
| chess960_materialize | 1.00x | 1.75x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.90x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 35.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 27.83x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.76x | 1.73x | 1.12x | 2.20x | 2.12x | 9/10 |
| corpus_materialize | 1.01x | 1.72x | 1.55x | 1.15x | 2.21x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.31x | 2.97x | 2.40x | 3.00x | 1.70x | 10/10 |
| replay_trusted_native | 1.07x | 0.81x | 1.56x | 1.72x | 2.70x | 3.21x | 9/10 |
| replay_checked_native | 0.95x | 2.61x | 1.15x | 8.51x | 8.75x | 10.63x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
