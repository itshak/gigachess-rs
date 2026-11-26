| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.31x | 1.53x | 1.58x | 2.14x | 1.82x | 30/30 |
| materialize | 0.99x | 1.90x | 1.31x | 1.23x | 2.26x | 1.53x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.58x | 1.40x | 1.24x | 2.01x | 1.84x | 30/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.52x | 1.28x | 1.88x | 1.83x | 30/30 |
| fen_parse | 1.03x | 1.33x | 1.51x | 1.39x | 2.32x | 1.57x | 30/30 |
| fen_write_native_ep | 0.98x | 4.20x | 8.32x | 1.69x | 32.24x | 1.98x | 30/30 |
| snapshot | 1.29x | 1.88x | 0.98x | 4.07x | 1.04x | 1.04x | 21/30 |
| san_parse_all | 0.99x | 2.10x | N/A | 5.64x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.76x | N/A | 3.78x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 2.28x | 1.39x | 9.48x | 1.47x | 2.21x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.17x | 1.25x | 81.36x | 180.57x | 253.39x | 180.91x | 5/5 |
| chess960_materialize | 0.96x | 1.45x | 1.24x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.57x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.83x | 26.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.83x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.87x | 4.74x | 1.57x | 1.22x | 2.25x | 2.12x | 10/10 |
| corpus_materialize | 0.90x | 1.50x | 1.20x | 1.06x | 1.91x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.91x | 1.67x | 1.98x | 1.77x | 2.03x | 1.30x | 10/10 |
| replay_trusted_native | 1.12x | 0.84x | 1.58x | 2.00x | 2.60x | 2.95x | 9/10 |
| replay_checked_native | 1.01x | 2.21x | 1.03x | 7.84x | 7.51x | 8.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
