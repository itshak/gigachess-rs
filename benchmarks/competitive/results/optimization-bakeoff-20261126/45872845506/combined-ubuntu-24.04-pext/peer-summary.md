| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.91x | 1.67x | 1.72x | 2.63x | 1.98x | 30/30 |
| materialize | 0.99x | 1.97x | 1.64x | 1.26x | 2.28x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.13x | 1.58x | 1.47x | 2.49x | 1.93x | 30/30 |
| perft_full_d3 | 1.00x | 1.39x | 1.49x | 1.33x | 1.94x | 1.84x | 30/30 |
| fen_parse | 0.95x | 1.32x | 1.49x | 1.19x | 2.19x | 1.34x | 30/30 |
| fen_write_native_ep | 0.94x | 4.08x | 9.33x | 1.64x | 27.12x | 1.92x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.25x | 3.26x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.01x | 2.16x | N/A | 4.48x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.47x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.39x | 1.36x | 4.77x | 1.51x | 1.69x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 77.04x | 139.94x | 237.12x | 151.48x | 5/5 |
| chess960_materialize | 0.99x | 1.60x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.87x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 27.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.67x | 1.79x | 1.42x | 2.67x | 2.33x | 10/10 |
| corpus_materialize | 0.99x | 1.63x | 1.57x | 1.19x | 2.04x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.66x | 3.28x | 2.57x | 3.22x | 1.92x | 10/10 |
| replay_trusted_native | 0.90x | 0.80x | 1.44x | 1.76x | 2.38x | 2.63x | 9/10 |
| replay_checked_native | 0.91x | 2.36x | 1.02x | 7.84x | 7.83x | 9.00x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
