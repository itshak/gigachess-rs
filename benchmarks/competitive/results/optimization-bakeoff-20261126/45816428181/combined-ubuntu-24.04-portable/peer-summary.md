| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.67x | 1.55x | 1.34x | 1.83x | 1.43x | 30/30 |
| materialize | 0.92x | 2.09x | 1.56x | 1.64x | 2.30x | 1.72x | 30/30 |
| perft_bulk_d4 | 0.91x | 2.71x | 1.43x | 1.06x | 1.76x | 1.64x | 28/30 |
| perft_full_d3 | 0.97x | 1.54x | 1.78x | 1.59x | 2.14x | 1.81x | 30/30 |
| fen_parse | 1.06x | 1.45x | 1.57x | 1.32x | 1.63x | 1.34x | 30/30 |
| fen_write_native_ep | 0.95x | 3.02x | 9.37x | 1.90x | 27.41x | 2.62x | 30/30 |
| snapshot | 1.01x | 1.73x | 1.21x | 3.16x | 1.22x | 0.99x | 26/30 |
| san_parse_all | 0.92x | 1.70x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.32x | 9.21x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.09x | 1.38x | 4.04x | 1.29x | 1.49x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.21x | N/A | N/A | 5/6 |
| startpos | 1.06x | 1.36x | 57.55x | 111.96x | 128.63x | 112.37x | 5/5 |
| chess960_materialize | 0.92x | 1.43x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 1.89x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 41.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.78x | 21.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.78x | 3.54x | 1.85x | 0.93x | 1.82x | 2.43x | 8/10 |
| corpus_materialize | 0.92x | 1.43x | 1.36x | 1.07x | 1.71x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.91x | 2.88x | 2.26x | 2.89x | 1.56x | 10/10 |
| replay_trusted_native | 1.07x | 0.86x | 1.57x | 1.60x | 2.17x | 1.99x | 9/10 |
| replay_checked_native | 0.97x | 1.91x | 1.06x | 8.33x | 8.45x | 9.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
