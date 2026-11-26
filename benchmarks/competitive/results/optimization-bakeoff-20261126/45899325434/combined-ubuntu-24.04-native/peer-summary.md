| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.52x | 1.71x | 1.66x | 2.53x | 1.99x | 30/30 |
| materialize | 0.99x | 2.00x | 1.71x | 1.14x | 2.26x | 1.50x | 29/30 |
| perft_bulk_d4 | 1.00x | 3.98x | 1.61x | 1.34x | 2.45x | 1.95x | 30/30 |
| perft_full_d3 | 1.30x | 1.66x | 2.11x | 1.65x | 2.63x | 3.05x | 30/30 |
| fen_parse | 1.12x | 1.42x | 1.44x | 1.21x | 1.84x | 1.46x | 30/30 |
| fen_write_native_ep | 1.00x | 4.04x | 9.28x | 1.86x | 27.88x | 2.33x | 30/30 |
| snapshot | 0.78x | 1.44x | 0.97x | 2.53x | 1.18x | 0.97x | 18/30 |
| san_parse_all | 1.00x | 1.97x | N/A | 3.81x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.40x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.11x | 1.32x | 5.57x | 1.43x | 1.57x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 94.94x | 142.06x | 205.31x | 165.93x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.76x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.36x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.46x | 1.85x | 1.35x | 2.43x | 2.13x | 10/10 |
| corpus_materialize | 1.00x | 1.78x | 1.72x | 1.20x | 2.89x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.40x | 3.20x | 2.43x | 3.00x | 1.78x | 10/10 |
| replay_trusted_native | 0.99x | 1.10x | 2.03x | 2.33x | 3.41x | 3.99x | 10/10 |
| replay_checked_native | 0.99x | 2.43x | 1.10x | 6.14x | 7.74x | 8.27x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
