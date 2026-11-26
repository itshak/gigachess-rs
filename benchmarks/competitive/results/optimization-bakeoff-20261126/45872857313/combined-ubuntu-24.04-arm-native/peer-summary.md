| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.67x | 1.79x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.94x | 1.53x | 1.34x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.87x | 1.57x | 1.42x | 2.01x | 2.07x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.60x | 1.33x | 1.97x | 1.86x | 30/30 |
| fen_parse | 0.99x | 1.23x | 1.33x | 1.28x | 1.72x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.33x | 1.57x | 29.52x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.05x | 1.47x | 1.06x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.35x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.02x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.63x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.77x | 129.88x | 168.49x | 136.48x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 25.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.42x | 1.69x | 1.39x | 2.02x | 2.19x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.52x | 1.22x | 1.90x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.07x | 0.67x | 1.77x | 2.14x | 2.44x | 2.44x | 8/10 |
| replay_checked_native | 1.02x | 2.35x | 1.15x | 7.71x | 6.91x | 8.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
