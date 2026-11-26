| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.45x | 1.88x | 1.64x | 2.56x | 1.91x | 30/30 |
| materialize | 0.99x | 2.23x | 1.69x | 1.79x | 2.69x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.82x | 1.52x | 1.20x | 2.39x | 1.86x | 29/30 |
| perft_full_d3 | 1.04x | 1.33x | 1.54x | 1.51x | 2.00x | 1.64x | 30/30 |
| fen_parse | 1.06x | 1.28x | 1.57x | 1.15x | 1.45x | 1.26x | 30/30 |
| fen_write_native_ep | 0.99x | 5.01x | 10.27x | 1.92x | 28.31x | 2.27x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 7.58x | 1.00x | 1.33x | 27/30 |
| san_parse_all | 0.95x | 1.84x | N/A | 4.14x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.92x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.10x | 1.32x | 3.99x | 1.40x | 1.61x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.80x | 1.00x | 82.37x | 148.19x | 173.03x | 158.72x | 5/5 |
| chess960_materialize | 0.99x | 1.79x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.49x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 18.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.09x | 1.87x | 1.07x | 2.35x | 2.13x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.32x | 1.09x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.30x | 2.98x | 2.24x | 2.93x | 1.58x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.59x | 1.58x | 2.22x | 2.54x | 9/10 |
| replay_checked_native | 1.01x | 2.03x | 1.10x | 8.88x | 8.23x | 9.70x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
