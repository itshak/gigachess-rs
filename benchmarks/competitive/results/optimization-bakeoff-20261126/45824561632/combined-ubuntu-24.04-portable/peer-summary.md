| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.70x | 1.55x | 1.34x | 1.78x | 1.29x | 29/30 |
| materialize | 1.05x | 2.10x | 1.77x | 1.71x | 2.71x | 1.86x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.67x | 1.43x | 1.06x | 1.76x | 1.59x | 29/30 |
| perft_full_d3 | 0.89x | 1.34x | 1.59x | 1.39x | 1.88x | 1.58x | 30/30 |
| fen_parse | 0.94x | 1.58x | 1.30x | 1.18x | 1.36x | 1.10x | 30/30 |
| fen_write_native_ep | 1.00x | 3.06x | 9.94x | 1.90x | 27.79x | 2.31x | 30/30 |
| snapshot | 0.90x | 1.46x | 1.12x | 4.64x | 1.12x | 1.00x | 29/30 |
| san_parse_all | 1.01x | 1.90x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.19x | 9.02x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.11x | 1.35x | 3.98x | 1.29x | 1.48x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 61.30x | 108.46x | 122.27x | 103.89x | 5/5 |
| chess960_materialize | 0.89x | 1.46x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 1.88x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.86x | 37.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.07x | 20.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 3.79x | 1.92x | 1.04x | 2.03x | 2.60x | 10/10 |
| corpus_materialize | 1.01x | 1.45x | 1.35x | 1.11x | 1.78x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.91x | 2.95x | 2.19x | 2.85x | 1.52x | 10/10 |
| replay_trusted_native | 0.95x | 0.82x | 1.59x | 1.53x | 2.09x | 1.92x | 9/10 |
| replay_checked_native | 1.00x | 1.90x | 1.06x | 8.09x | 7.75x | 9.18x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
