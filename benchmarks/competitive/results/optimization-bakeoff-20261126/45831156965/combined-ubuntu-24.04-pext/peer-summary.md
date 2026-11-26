| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.37x | 1.48x | 1.50x | 2.30x | 1.76x | 30/30 |
| materialize | 1.00x | 1.96x | 1.57x | 1.22x | 2.22x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.66x | 1.39x | 1.28x | 2.27x | 1.75x | 30/30 |
| perft_full_d3 | 1.03x | 1.28x | 1.43x | 1.27x | 1.85x | 1.66x | 30/30 |
| fen_parse | 1.01x | 1.35x | 1.43x | 1.16x | 2.07x | 1.28x | 30/30 |
| fen_write_native_ep | 1.02x | 4.10x | 9.93x | 1.72x | 29.84x | 2.17x | 30/30 |
| snapshot | 0.78x | 1.65x | 1.02x | 2.83x | 1.24x | 0.97x | 19/30 |
| san_parse_all | 1.00x | 2.15x | N/A | 4.50x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.56x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.25x | 1.29x | 4.61x | 1.43x | 1.59x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 84.05x | 144.17x | 226.40x | 159.50x | 5/5 |
| chess960_materialize | 0.99x | 1.53x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.69x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.78x | 23.18x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 18.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.12x | 1.66x | 1.28x | 2.49x | 2.10x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.48x | 1.12x | 1.95x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.40x | 3.16x | 2.39x | 3.06x | 1.70x | 10/10 |
| replay_trusted_native | 0.99x | 0.86x | 1.57x | 1.88x | 2.48x | 2.72x | 9/10 |
| replay_checked_native | 1.00x | 2.31x | 1.02x | 8.00x | 7.87x | 9.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
