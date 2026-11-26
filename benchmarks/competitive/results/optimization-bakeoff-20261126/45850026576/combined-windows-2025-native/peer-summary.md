| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.11x | 4.44x | 1.85x | 1.62x | 2.20x | 1.95x | 30/30 |
| materialize | 1.00x | 2.05x | 1.43x | 1.65x | 2.41x | 1.70x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.55x | 1.18x | 2.18x | 1.84x | 29/30 |
| perft_full_d3 | 0.96x | 1.75x | 1.54x | 1.41x | 1.93x | 1.58x | 30/30 |
| fen_parse | 0.89x | 1.03x | 1.28x | 1.17x | 1.54x | 1.25x | 29/30 |
| fen_write_native_ep | 1.01x | 3.94x | 8.00x | 1.69x | 30.41x | 2.25x | 30/30 |
| snapshot | 0.90x | 2.04x | 1.00x | 4.74x | 0.99x | 1.04x | 23/30 |
| san_parse_all | 0.99x | 1.77x | N/A | 5.44x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.59x | N/A | 4.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.07x | 1.33x | 10.31x | 1.37x | 1.62x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.99x | 0.99x | 79.98x | 167.79x | 211.38x | 168.58x | 4/5 |
| chess960_materialize | 1.02x | 1.63x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.60x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 40.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.18x | 32.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 4.47x | 1.74x | 0.91x | 2.07x | 1.97x | 8/10 |
| corpus_materialize | 1.00x | 1.43x | 1.26x | 1.10x | 1.76x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.85x | 2.20x | 1.90x | 2.19x | 1.66x | 10/10 |
| replay_trusted_native | 0.92x | 0.77x | 1.46x | 1.52x | 2.09x | 2.35x | 9/10 |
| replay_checked_native | 0.89x | 1.98x | 0.99x | 7.56x | 7.29x | 8.75x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
