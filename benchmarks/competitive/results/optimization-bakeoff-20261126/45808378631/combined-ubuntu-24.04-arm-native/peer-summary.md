| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.90x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.83x | 1.44x | 1.26x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.42x | 1.40x | 1.27x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 1.01x | 1.20x | 1.29x | 1.07x | 1.58x | 1.50x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.34x | 1.28x | 1.76x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.41x | 1.62x | 29.60x | 2.39x | 30/30 |
| snapshot | 0.92x | 1.10x | 1.05x | 1.45x | 1.06x | 0.94x | 24/30 |
| san_parse_all | 0.99x | 1.83x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.72x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.28x | 1.52x | 4.13x | 1.24x | 1.24x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.29x | 76.45x | 130.77x | 169.23x | 136.12x | 5/5 |
| chess960_materialize | 1.04x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.63x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 3.93x | 1.57x | 1.22x | 1.80x | 1.96x | 10/10 |
| corpus_materialize | 0.98x | 1.54x | 1.49x | 1.17x | 1.84x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.07x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.53x | 1.38x | 1.66x | 1.86x | 1.91x | 8/10 |
| replay_checked_native | 0.99x | 2.00x | 0.96x | 6.49x | 5.81x | 7.48x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
