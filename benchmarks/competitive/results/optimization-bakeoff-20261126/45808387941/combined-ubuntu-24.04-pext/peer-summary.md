| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.30x | 1.48x | 1.52x | 2.34x | 1.74x | 30/30 |
| materialize | 1.02x | 1.89x | 1.64x | 1.21x | 2.27x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.89x | 3.61x | 1.39x | 1.30x | 2.16x | 1.69x | 30/30 |
| perft_full_d3 | 0.95x | 1.19x | 1.33x | 1.17x | 1.74x | 1.60x | 30/30 |
| fen_parse | 1.17x | 1.37x | 1.54x | 1.19x | 2.29x | 1.40x | 30/30 |
| fen_write_native_ep | 1.03x | 4.06x | 9.71x | 1.77x | 28.39x | 2.09x | 30/30 |
| snapshot | 1.76x | 1.97x | 1.25x | 3.52x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.12x | N/A | 4.58x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.41x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 2.17x | 1.24x | 4.35x | 1.38x | 1.53x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 77.99x | 139.18x | 230.72x | 156.15x | 5/5 |
| chess960_materialize | 1.04x | 1.48x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.78x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 29.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 19.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.85x | 5.10x | 1.65x | 1.28x | 2.48x | 2.08x | 10/10 |
| corpus_materialize | 0.95x | 1.54x | 1.46x | 1.11x | 1.93x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.44x | 3.20x | 2.42x | 3.09x | 1.72x | 10/10 |
| replay_trusted_native | 0.99x | 0.77x | 1.34x | 1.64x | 2.21x | 2.68x | 8/10 |
| replay_checked_native | 1.00x | 2.30x | 1.01x | 7.90x | 7.67x | 9.09x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
