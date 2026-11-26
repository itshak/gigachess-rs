| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.83x | 1.46x | 1.58x | 1.90x | 1.90x | 30/30 |
| materialize | 0.99x | 1.80x | 1.41x | 1.24x | 2.00x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.70x | 1.52x | 1.38x | 1.93x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.27x | 1.07x | 1.57x | 1.48x | 29/30 |
| fen_parse | 1.00x | 1.26x | 1.34x | 1.30x | 1.78x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.43x | 9.22x | 1.56x | 29.33x | 2.38x | 30/30 |
| snapshot | 1.01x | 1.28x | 0.92x | 1.62x | 0.91x | 1.00x | 16/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.83x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.26x | 1.45x | 4.07x | 1.20x | 1.22x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.49x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 76.51x | 128.57x | 173.20x | 138.50x | 5/5 |
| chess960_materialize | 0.99x | 1.51x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.61x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.78x | 1.81x | 1.49x | 2.11x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.61x | 1.51x | 1.22x | 1.88x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.93x | 2.09x | 2.79x | 1.46x | 10/10 |
| replay_trusted_native | 1.04x | 0.56x | 1.43x | 1.73x | 1.91x | 1.98x | 8/10 |
| replay_checked_native | 1.05x | 2.19x | 1.03x | 6.96x | 6.15x | 8.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
