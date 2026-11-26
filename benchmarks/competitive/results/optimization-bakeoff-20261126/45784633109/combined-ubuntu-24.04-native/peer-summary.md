| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.18x | 1.51x | 1.45x | 2.18x | 1.80x | 30/30 |
| materialize | 1.02x | 1.88x | 1.57x | 1.22x | 2.15x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.58x | 1.43x | 1.27x | 2.11x | 1.79x | 30/30 |
| perft_full_d3 | 0.96x | 1.26x | 1.37x | 1.19x | 1.74x | 1.78x | 30/30 |
| fen_parse | 0.91x | 1.22x | 1.30x | 1.10x | 1.87x | 1.12x | 30/30 |
| fen_write_native_ep | 1.13x | 4.28x | 10.59x | 1.78x | 29.87x | 2.20x | 30/30 |
| snapshot | 1.01x | 1.33x | 1.14x | 2.04x | 1.14x | 0.86x | 24/30 |
| san_parse_all | 0.99x | 2.02x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 11.09x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.22x | 1.54x | 3.84x | 1.46x | 1.65x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.16x | 1.33x | 78.63x | 147.80x | 223.98x | 147.84x | 5/5 |
| chess960_materialize | 1.06x | 1.59x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.57x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 29.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 21.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 4.88x | 1.65x | 1.22x | 2.28x | 2.14x | 9/10 |
| corpus_materialize | 0.94x | 1.52x | 1.47x | 1.12x | 1.94x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.46x | 3.27x | 2.69x | 3.22x | 1.71x | 10/10 |
| replay_trusted_native | 1.00x | 0.70x | 1.34x | 1.59x | 2.08x | 2.35x | 8/10 |
| replay_checked_native | 1.00x | 2.26x | 1.03x | 7.65x | 7.56x | 9.23x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
