| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.29x | 1.50x | 1.46x | 2.24x | 1.75x | 30/30 |
| materialize | 0.98x | 2.03x | 1.58x | 1.12x | 2.02x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.53x | 1.26x | 2.38x | 2.08x | 30/30 |
| perft_full_d3 | 1.03x | 1.30x | 1.33x | 1.02x | 1.82x | 1.59x | 26/30 |
| fen_parse | 0.98x | 1.26x | 1.48x | 1.07x | 2.23x | 1.30x | 30/30 |
| fen_write_native_ep | 0.99x | 3.97x | 9.21x | 1.80x | 27.43x | 2.11x | 30/30 |
| snapshot | 0.82x | 1.26x | 0.83x | 2.11x | 0.89x | 0.83x | 12/30 |
| san_parse_all | 0.95x | 2.10x | N/A | 3.58x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 9.83x | N/A | 1.93x | N/A | N/A | 12/12 |
| apply_children_full | 0.82x | 1.32x | 0.88x | 3.08x | 0.96x | 1.19x | 21/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.02x | N/A | N/A | 2/6 |
| startpos | 1.10x | 1.20x | 92.74x | 139.25x | 244.51x | 162.59x | 5/5 |
| chess960_materialize | 0.99x | 1.71x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.76x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 19.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.50x | 1.81x | 1.34x | 2.51x | 2.08x | 10/10 |
| corpus_materialize | 1.00x | 1.89x | 1.72x | 1.23x | 2.82x | 1.56x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.32x | 3.14x | 2.33x | 2.92x | 1.66x | 10/10 |
| replay_trusted_native | 1.02x | 0.76x | 1.26x | 1.50x | 2.60x | 3.15x | 8/10 |
| replay_checked_native | 0.98x | 2.51x | 0.92x | 5.39x | 6.51x | 7.31x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
