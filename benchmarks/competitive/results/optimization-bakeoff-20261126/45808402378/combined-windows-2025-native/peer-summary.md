| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.34x | 1.50x | 1.48x | 2.15x | 1.88x | 30/30 |
| materialize | 1.01x | 1.91x | 1.30x | 1.22x | 2.21x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.62x | 1.43x | 1.21x | 2.05x | 1.84x | 30/30 |
| perft_full_d3 | 0.99x | 1.35x | 1.44x | 1.26x | 1.87x | 1.80x | 30/30 |
| fen_parse | 1.04x | 1.33x | 1.42x | 1.34x | 2.25x | 1.47x | 30/30 |
| fen_write_native_ep | 1.00x | 4.26x | 8.34x | 1.61x | 32.13x | 1.96x | 30/30 |
| snapshot | 1.78x | 1.87x | 1.04x | 4.29x | 1.06x | 1.07x | 28/30 |
| san_parse_all | 0.99x | 2.03x | N/A | 5.45x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.98x | N/A | 3.46x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 2.11x | 1.28x | 8.52x | 1.35x | 1.54x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.16x | 1.25x | 78.18x | 156.37x | 235.56x | 171.60x | 5/5 |
| chess960_materialize | 1.03x | 1.52x | 1.24x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.64x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 28.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.92x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.07x | 21.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.94x | 1.68x | 1.25x | 2.27x | 2.23x | 10/10 |
| corpus_materialize | 0.93x | 1.48x | 1.20x | 1.11x | 1.87x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.77x | 2.21x | 1.86x | 2.17x | 1.40x | 10/10 |
| replay_trusted_native | 1.05x | 0.72x | 1.37x | 1.79x | 2.22x | 2.62x | 8/10 |
| replay_checked_native | 1.07x | 2.19x | 0.98x | 7.71x | 7.43x | 8.76x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
