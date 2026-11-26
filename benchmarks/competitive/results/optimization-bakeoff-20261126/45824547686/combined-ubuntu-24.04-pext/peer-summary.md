| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.00x | 1.66x | 1.45x | 2.21x | 1.71x | 30/30 |
| materialize | 1.00x | 2.16x | 1.67x | 1.64x | 2.66x | 1.77x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.38x | 1.35x | 1.05x | 2.06x | 1.65x | 29/30 |
| perft_full_d3 | 1.01x | 1.55x | 1.69x | 1.64x | 2.11x | 2.43x | 30/30 |
| fen_parse | 1.00x | 1.35x | 1.58x | 1.04x | 1.46x | 1.24x | 28/30 |
| fen_write_native_ep | 1.00x | 4.46x | 10.12x | 1.91x | 28.29x | 2.74x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.95x | N/A | 4.36x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.22x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.04x | 1.33x | 3.88x | 1.21x | 1.67x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.25x | 98.98x | 166.14x | 228.18x | 195.03x | 5/5 |
| chess960_materialize | 0.99x | 1.63x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.36x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 38.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.89x | 16.52x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.51x | 1.67x | 0.92x | 2.03x | 1.87x | 8/10 |
| corpus_materialize | 1.00x | 1.37x | 1.26x | 1.06x | 1.70x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.24x | 2.92x | 2.23x | 2.82x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.91x | 1.61x | 1.56x | 2.21x | 2.60x | 9/10 |
| replay_checked_native | 1.00x | 2.07x | 1.10x | 8.55x | 8.26x | 9.72x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
