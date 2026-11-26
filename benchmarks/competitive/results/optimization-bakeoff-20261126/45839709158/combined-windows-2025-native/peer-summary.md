| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.85x | 1.79x | 1.70x | 2.63x | 2.16x | 30/30 |
| materialize | 1.00x | 2.04x | 1.36x | 1.19x | 2.33x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.99x | 1.64x | 1.43x | 2.45x | 2.26x | 30/30 |
| perft_full_d3 | 0.99x | 1.14x | 1.18x | 0.98x | 1.60x | 1.43x | 25/30 |
| fen_parse | 0.99x | 1.31x | 1.41x | 1.37x | 2.44x | 1.43x | 30/30 |
| fen_write_native_ep | 1.01x | 4.07x | 7.87x | 1.66x | 32.08x | 2.04x | 30/30 |
| snapshot | 0.98x | 1.80x | 1.26x | 4.05x | 1.25x | 1.10x | 29/30 |
| san_parse_all | 0.97x | 1.91x | N/A | 4.52x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.85x | N/A | 3.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.93x | 1.32x | 8.44x | 1.54x | 1.74x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.98x | 0.98x | 83.97x | 132.26x | 212.43x | 143.29x | 4/5 |
| chess960_materialize | 0.96x | 1.64x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.96x | 1.74x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.34x | 1.79x | 1.36x | 2.36x | 2.12x | 10/10 |
| corpus_materialize | 1.02x | 1.85x | 1.40x | 1.20x | 2.84x | 1.52x | 10/10 |
| corpus_uci_sorted | 0.93x | 1.75x | 2.29x | 1.88x | 2.21x | 1.48x | 10/10 |
| replay_trusted_native | 0.85x | 0.94x | 1.65x | 2.06x | 3.34x | 4.01x | 9/10 |
| replay_checked_native | 0.93x | 2.12x | 0.93x | 5.27x | 6.75x | 7.32x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
