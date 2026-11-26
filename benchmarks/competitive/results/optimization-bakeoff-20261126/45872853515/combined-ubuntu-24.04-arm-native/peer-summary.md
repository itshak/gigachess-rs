| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.67x | 1.80x | 2.17x | 2.17x | 30/30 |
| materialize | 1.01x | 1.96x | 1.54x | 1.34x | 2.16x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.84x | 1.57x | 1.42x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.14x | 1.69x | 1.83x | 1.52x | 2.24x | 2.12x | 30/30 |
| fen_parse | 1.00x | 1.25x | 1.32x | 1.27x | 1.76x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.48x | 1.57x | 30.19x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.10x | 1.03x | 1.47x | 1.05x | 0.95x | 23/30 |
| san_parse_all | 0.99x | 1.85x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.49x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.02x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.29x | 129.89x | 170.34x | 134.25x | 5/5 |
| chess960_materialize | 1.01x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.45x | 1.72x | 1.38x | 2.03x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.53x | 1.23x | 1.93x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.09x | 2.86x | 1.45x | 10/10 |
| replay_trusted_native | 0.99x | 0.64x | 1.64x | 2.03x | 2.26x | 2.28x | 8/10 |
| replay_checked_native | 1.00x | 2.31x | 1.12x | 7.58x | 6.81x | 8.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
