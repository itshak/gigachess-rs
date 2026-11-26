| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.52x | 1.88x | 1.64x | 2.48x | 1.92x | 30/30 |
| materialize | 0.99x | 2.25x | 1.73x | 1.76x | 2.72x | 1.86x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.84x | 1.52x | 1.19x | 2.33x | 1.87x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.53x | 1.51x | 1.95x | 1.63x | 30/30 |
| fen_parse | 0.98x | 1.45x | 1.37x | 1.15x | 1.56x | 1.12x | 30/30 |
| fen_write_native_ep | 1.01x | 5.08x | 10.79x | 1.96x | 30.51x | 2.52x | 30/30 |
| snapshot | 0.99x | 1.66x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 0.91x | 1.70x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.29x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.09x | 1.34x | 4.38x | 1.26x | 1.72x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.24x | 1.24x | 98.82x | 179.08x | 228.59x | 176.67x | 5/5 |
| chess960_materialize | 0.99x | 1.77x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.55x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.10x | 1.11x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 22.56x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.42x | 2.04x | 1.13x | 2.46x | 2.28x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.10x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.33x | 3.19x | 2.28x | 3.09x | 1.58x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.50x | 1.58x | 2.24x | 2.50x | 9/10 |
| replay_checked_native | 1.00x | 2.21x | 1.12x | 9.29x | 8.96x | 10.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
