| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.40x | 1.68x | 1.80x | 2.18x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.57x | 1.44x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.97x | 1.85x | 30/30 |
| fen_parse | 0.99x | 1.23x | 1.31x | 1.26x | 1.73x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.36x | 1.58x | 29.83x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.09x | 1.04x | 1.47x | 1.05x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.55x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.53x | 124.21x | 175.66x | 140.74x | 5/5 |
| chess960_materialize | 0.98x | 1.68x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.94x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.43x | 1.69x | 1.36x | 1.98x | 2.23x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.52x | 1.21x | 1.89x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.95x | 0.61x | 1.58x | 1.92x | 2.13x | 2.17x | 8/10 |
| replay_checked_native | 1.00x | 2.30x | 1.13x | 7.51x | 6.76x | 8.79x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
