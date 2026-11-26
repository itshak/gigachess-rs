| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.90x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.84x | 1.46x | 1.28x | 2.05x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.42x | 1.40x | 1.27x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.26x | 1.05x | 1.54x | 1.46x | 29/30 |
| fen_parse | 1.00x | 1.22x | 1.30x | 1.26x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.02x | 4.55x | 9.39x | 1.58x | 30.22x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.10x | 1.00x | 1.47x | 1.04x | 0.95x | 20/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.69x | N/A | 2.58x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.03x | 1.19x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.38x | 129.06x | 177.28x | 143.95x | 5/5 |
| chess960_materialize | 1.00x | 1.57x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.91x | 1.52x | 1.19x | 1.79x | 1.95x | 10/10 |
| corpus_materialize | 1.00x | 1.54x | 1.48x | 1.18x | 1.84x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 0.99x | 0.60x | 1.60x | 1.94x | 2.15x | 2.20x | 8/10 |
| replay_checked_native | 1.06x | 2.29x | 1.11x | 7.54x | 6.69x | 8.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
