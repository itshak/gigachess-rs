| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.52x | 1.78x | 1.57x | 2.61x | 1.98x | 30/30 |
| materialize | 1.05x | 2.00x | 1.41x | 1.09x | 2.42x | 1.53x | 28/30 |
| perft_bulk_d4 | 1.02x | 3.89x | 1.61x | 1.37x | 2.33x | 2.14x | 30/30 |
| perft_full_d3 | 1.05x | 1.34x | 1.38x | 1.14x | 1.86x | 1.64x | 29/30 |
| fen_parse | 1.03x | 1.30x | 1.42x | 1.38x | 2.20x | 1.44x | 30/30 |
| fen_write_native_ep | 1.03x | 3.87x | 7.52x | 1.59x | 30.57x | 1.97x | 30/30 |
| snapshot | 1.02x | 1.79x | 1.25x | 4.12x | 1.25x | 1.11x | 29/30 |
| san_parse_all | 1.04x | 1.99x | N/A | 4.67x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.36x | N/A | 3.36x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.93x | 1.31x | 8.12x | 1.60x | 1.69x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 85.37x | 139.71x | 207.87x | 143.78x | 5/5 |
| chess960_materialize | 1.09x | 1.63x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.96x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.12x | 1.68x | 1.22x | 2.26x | 2.00x | 9/10 |
| corpus_materialize | 1.04x | 1.75x | 1.35x | 1.15x | 2.74x | 1.50x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.83x | 2.35x | 1.91x | 2.24x | 1.48x | 10/10 |
| replay_trusted_native | 1.03x | 0.96x | 1.73x | 2.12x | 3.26x | 4.13x | 9/10 |
| replay_checked_native | 1.08x | 2.54x | 1.10x | 6.31x | 8.37x | 8.17x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
