| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.68x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.59x | 1.44x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.14x | 1.69x | 1.83x | 1.52x | 2.24x | 2.12x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.32x | 1.27x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.50x | 9.30x | 1.58x | 29.40x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.04x | 1.00x | 1.45x | 1.01x | 0.96x | 15/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.37x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.09x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.64x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.44x | 125.07x | 172.57x | 138.00x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.98x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.44x | 1.71x | 1.38x | 1.99x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.54x | 1.22x | 1.90x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.07x | 0.69x | 1.76x | 2.15x | 2.39x | 2.45x | 8/10 |
| replay_checked_native | 0.99x | 2.30x | 1.12x | 7.55x | 6.79x | 8.70x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
