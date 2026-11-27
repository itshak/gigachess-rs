| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.62x | 1.88x | 1.64x | 2.58x | 1.91x | 30/30 |
| materialize | 0.99x | 2.26x | 1.67x | 1.76x | 2.72x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.89x | 1.53x | 1.20x | 2.39x | 1.88x | 29/30 |
| perft_full_d3 | 1.02x | 1.37x | 1.57x | 1.53x | 2.05x | 1.67x | 30/30 |
| fen_parse | 1.00x | 1.40x | 1.32x | 1.14x | 1.45x | 1.13x | 30/30 |
| fen_write_native_ep | 1.01x | 4.64x | 9.84x | 1.87x | 27.71x | 2.52x | 30/30 |
| snapshot | 1.00x | 2.01x | 1.00x | 4.68x | 1.01x | 1.34x | 26/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.14x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.96x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.17x | 1.36x | 4.06x | 1.44x | 1.66x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.80x | 1.00x | 78.76x | 146.10x | 176.45x | 145.09x | 4/5 |
| chess960_materialize | 1.01x | 1.77x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.50x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.96x | 22.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.36x | 2.00x | 1.12x | 2.49x | 2.22x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.30x | 1.09x | 1.75x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.26x | 3.00x | 2.22x | 2.90x | 1.55x | 10/10 |
| replay_trusted_native | 0.97x | 0.88x | 1.59x | 1.55x | 2.19x | 2.49x | 9/10 |
| replay_checked_native | 0.99x | 2.15x | 1.13x | 9.13x | 8.86x | 10.29x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
