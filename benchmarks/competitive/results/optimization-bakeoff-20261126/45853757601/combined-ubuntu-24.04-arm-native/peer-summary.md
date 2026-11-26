| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.40x | 1.68x | 1.80x | 2.18x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.34x | 2.16x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.59x | 1.43x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.97x | 1.86x | 30/30 |
| fen_parse | 1.01x | 1.23x | 1.32x | 1.26x | 1.74x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.31x | 1.61x | 29.87x | 2.41x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.04x | 1.45x | 1.05x | 0.96x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.74x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 3.96x | 1.18x | 1.19x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.37x | 130.07x | 169.46x | 134.24x | 5/5 |
| chess960_materialize | 1.00x | 1.70x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.99x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.42x | 1.69x | 1.36x | 1.97x | 2.18x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.52x | 1.23x | 1.89x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.08x | 2.85x | 1.47x | 10/10 |
| replay_trusted_native | 0.98x | 0.61x | 1.60x | 1.93x | 2.16x | 2.20x | 8/10 |
| replay_checked_native | 1.00x | 2.15x | 1.05x | 7.12x | 6.32x | 8.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
