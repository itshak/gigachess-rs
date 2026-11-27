| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.40x | 1.67x | 1.80x | 2.18x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.52x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.58x | 1.43x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.98x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.27x | 1.74x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.43x | 9.18x | 1.60x | 29.64x | 2.47x | 30/30 |
| snapshot | 1.00x | 1.11x | 1.03x | 1.47x | 1.03x | 0.95x | 23/30 |
| san_parse_all | 0.99x | 1.85x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.38x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.02x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.36x | 122.83x | 175.50x | 143.02x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.39x | 1.70x | 1.36x | 2.01x | 2.29x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.53x | 1.22x | 1.92x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.91x | 2.09x | 2.87x | 1.47x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.72x | 2.02x | 2.25x | 2.33x | 8/10 |
| replay_checked_native | 1.00x | 2.29x | 1.12x | 7.56x | 6.79x | 8.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
