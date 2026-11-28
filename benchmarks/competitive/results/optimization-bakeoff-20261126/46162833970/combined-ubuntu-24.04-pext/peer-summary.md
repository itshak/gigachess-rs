| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 5.27x | 1.79x | 1.77x | 2.60x | 2.13x | 30/30 |
| materialize | 1.01x | 2.22x | 1.74x | 1.23x | 2.54x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.52x | 1.72x | 1.38x | 2.61x | 2.39x | 30/30 |
| perft_full_d3 | 1.33x | 1.86x | 2.18x | 1.62x | 2.77x | 2.69x | 30/30 |
| fen_parse | 0.91x | 1.26x | 1.41x | 1.10x | 1.86x | 1.28x | 30/30 |
| fen_write_native_ep | 1.05x | 4.21x | 8.81x | 1.47x | 25.64x | 1.93x | 30/30 |
| snapshot | 0.99x | 2.40x | 1.44x | 4.33x | 1.44x | 1.45x | 30/30 |
| san_parse_all | 1.02x | 1.83x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.93x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.90x | 1.41x | 4.06x | 1.35x | 1.91x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.25x | 84.35x | 141.17x | 210.53x | 151.13x | 5/5 |
| chess960_materialize | 0.97x | 1.69x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.04x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 35.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.64x | 1.58x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 25.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 6.50x | 1.94x | 1.28x | 2.54x | 2.31x | 9/10 |
| corpus_materialize | 1.01x | 1.69x | 1.58x | 1.13x | 2.24x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.37x | 2.99x | 2.48x | 3.05x | 1.68x | 10/10 |
| replay_trusted_native | 1.06x | 0.78x | 1.48x | 1.64x | 2.59x | 3.07x | 9/10 |
| replay_checked_native | 1.01x | 2.63x | 1.23x | 9.18x | 9.30x | 11.51x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
