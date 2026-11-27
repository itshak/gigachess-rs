| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 5.15x | 1.79x | 1.82x | 2.79x | 2.08x | 30/30 |
| materialize | 1.00x | 2.18x | 1.72x | 1.18x | 2.33x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.15x | 1.66x | 1.49x | 2.60x | 2.02x | 30/30 |
| perft_full_d3 | 1.31x | 1.67x | 2.10x | 1.69x | 2.63x | 2.46x | 30/30 |
| fen_parse | 1.03x | 1.40x | 1.34x | 1.08x | 1.68x | 1.28x | 30/30 |
| fen_write_native_ep | 1.02x | 4.02x | 9.80x | 1.86x | 29.12x | 2.18x | 30/30 |
| snapshot | 0.77x | 1.44x | 0.97x | 2.53x | 1.18x | 0.97x | 18/30 |
| san_parse_all | 1.01x | 1.99x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.58x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.11x | 1.32x | 5.47x | 1.43x | 1.67x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 94.47x | 137.13x | 201.96x | 156.69x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.85x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.76x | 0.77x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.99x | 18.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 4.80x | 1.92x | 1.47x | 2.64x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.86x | 1.73x | 1.24x | 2.95x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.47x | 3.26x | 2.49x | 3.07x | 1.82x | 10/10 |
| replay_trusted_native | 1.00x | 1.10x | 2.04x | 2.28x | 3.51x | 3.86x | 10/10 |
| replay_checked_native | 1.00x | 2.42x | 1.12x | 6.16x | 7.64x | 8.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
