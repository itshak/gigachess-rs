| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 5.08x | 1.76x | 1.63x | 2.37x | 2.15x | 30/30 |
| materialize | 0.97x | 2.01x | 1.35x | 1.26x | 2.29x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.09x | 1.62x | 1.36x | 2.32x | 2.11x | 30/30 |
| perft_full_d3 | 1.00x | 1.41x | 1.52x | 1.35x | 1.99x | 1.90x | 30/30 |
| fen_parse | 1.06x | 1.37x | 1.44x | 1.39x | 2.43x | 1.54x | 30/30 |
| fen_write_native_ep | 1.00x | 4.03x | 8.02x | 1.60x | 32.25x | 2.01x | 30/30 |
| snapshot | 1.00x | 1.85x | 1.03x | 4.10x | 1.00x | 1.05x | 26/30 |
| san_parse_all | 0.98x | 2.07x | N/A | 5.46x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.17x | N/A | 3.85x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.35x | 1.48x | 9.64x | 1.57x | 1.69x | 30/30 |
| make_unmake_full | 0.85x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.25x | 78.55x | 154.98x | 251.79x | 174.14x | 5/5 |
| chess960_materialize | 0.93x | 1.46x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.80x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 29.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 21.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.40x | 1.85x | 1.36x | 2.50x | 2.44x | 10/10 |
| corpus_materialize | 0.97x | 1.52x | 1.24x | 1.14x | 1.93x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.78x | 2.20x | 1.83x | 2.17x | 1.43x | 10/10 |
| replay_trusted_native | 1.00x | 0.83x | 1.59x | 2.06x | 2.74x | 3.05x | 9/10 |
| replay_checked_native | 0.99x | 2.42x | 1.06x | 8.42x | 8.22x | 9.52x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
