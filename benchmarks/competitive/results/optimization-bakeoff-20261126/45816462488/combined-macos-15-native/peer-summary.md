| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.22x | 1.33x | 1.58x | 1.86x | 1.71x | 29/30 |
| materialize | 1.18x | 1.91x | 1.59x | 1.01x | 2.25x | 1.40x | 27/30 |
| perft_bulk_d4 | 1.12x | 3.78x | 1.30x | 1.07x | 1.72x | 1.57x | 28/30 |
| perft_full_d3 | 1.13x | 1.51x | 1.75x | 1.46x | 2.03x | 2.01x | 29/30 |
| fen_parse | 1.08x | 1.23x | 1.32x | 1.36x | 1.63x | 1.41x | 30/30 |
| fen_write_native_ep | 1.07x | 4.28x | 9.08x | 1.76x | 29.16x | 2.20x | 30/30 |
| snapshot | 1.12x | 1.67x | 1.12x | 2.46x | 1.11x | 0.99x | 25/30 |
| san_parse_all | 1.07x | 1.85x | N/A | 3.74x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.46x | N/A | 2.47x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 0.98x | 1.06x | 5.09x | 0.98x | 1.18x | 20/30 |
| make_unmake_full | 1.12x | N/A | N/A | 1.16x | N/A | N/A | 4/6 |
| startpos | 0.97x | 1.20x | 75.32x | 177.44x | 157.08x | 145.10x | 5/5 |
| chess960_materialize | 1.15x | 1.74x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.59x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 36.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 19.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.20x | 1.67x | 1.08x | 1.84x | 2.07x | 9/10 |
| corpus_materialize | 1.04x | 1.40x | 1.33x | 0.99x | 1.89x | 1.24x | 9/10 |
| corpus_uci_sorted | 0.97x | 1.90x | 2.46x | 1.92x | 2.43x | 1.26x | 10/10 |
| replay_trusted_native | 1.13x | 0.60x | 1.71x | 1.57x | 1.89x | 1.83x | 8/10 |
| replay_checked_native | 0.99x | 2.14x | 0.91x | 7.30x | 6.18x | 7.86x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
