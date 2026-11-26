| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 2.67x | 1.53x | 1.33x | 1.79x | 1.27x | 29/30 |
| materialize | 0.93x | 2.25x | 1.75x | 1.73x | 2.67x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.94x | 2.83x | 1.51x | 1.12x | 1.88x | 1.67x | 29/30 |
| perft_full_d3 | 0.93x | 1.27x | 1.52x | 1.31x | 1.81x | 1.51x | 30/30 |
| fen_parse | 1.00x | 1.59x | 1.29x | 1.05x | 1.35x | 1.15x | 29/30 |
| fen_write_native_ep | 0.92x | 2.77x | 8.98x | 1.75x | 25.18x | 2.14x | 30/30 |
| snapshot | 1.01x | 1.63x | 1.25x | 3.38x | 1.25x | 1.12x | 30/30 |
| san_parse_all | 0.98x | 1.87x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.35x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.15x | 1.45x | 4.14x | 1.34x | 1.56x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.39x | 1.90x | 84.97x | 147.77x | 173.73x | 157.58x | 5/5 |
| chess960_materialize | 0.85x | 1.55x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 1.95x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 43.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 20.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 3.99x | 2.04x | 1.09x | 2.04x | 2.76x | 10/10 |
| corpus_materialize | 0.97x | 1.47x | 1.37x | 1.12x | 1.80x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.98x | 3.03x | 2.26x | 2.96x | 1.57x | 10/10 |
| replay_trusted_native | 0.95x | 0.83x | 1.61x | 1.55x | 2.14x | 1.99x | 9/10 |
| replay_checked_native | 0.90x | 1.90x | 1.07x | 8.13x | 7.85x | 8.95x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
