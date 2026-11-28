| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.46x | 1.69x | 1.83x | 2.21x | 2.19x | 30/30 |
| materialize | 1.01x | 1.99x | 1.57x | 1.38x | 2.18x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 3.99x | 1.63x | 1.47x | 2.08x | 2.14x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.81x | 1.51x | 2.23x | 2.10x | 30/30 |
| fen_parse | 1.02x | 1.26x | 1.34x | 1.29x | 1.76x | 1.43x | 30/30 |
| fen_write_native_ep | 1.00x | 4.42x | 9.28x | 1.58x | 30.31x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.06x | 1.55x | 1.05x | 0.94x | 24/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.46x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.13x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 76.07x | 129.19x | 177.77x | 143.32x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.99x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.37x | 1.48x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.82x | 1.85x | 1.49x | 2.16x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.52x | 1.24x | 1.90x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.91x | 2.09x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.10x | 0.69x | 1.81x | 2.20x | 2.48x | 2.51x | 8/10 |
| replay_checked_native | 1.02x | 2.36x | 1.15x | 7.82x | 6.94x | 8.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
