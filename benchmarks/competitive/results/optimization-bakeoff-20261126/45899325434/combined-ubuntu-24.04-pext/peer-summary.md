| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.19x | 1.72x | 1.76x | 2.73x | 2.04x | 30/30 |
| materialize | 1.01x | 2.11x | 1.68x | 1.30x | 2.37x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.31x | 1.63x | 1.50x | 2.56x | 1.98x | 30/30 |
| perft_full_d3 | 1.24x | 1.67x | 1.86x | 1.64x | 2.41x | 2.24x | 30/30 |
| fen_parse | 0.87x | 1.26x | 1.34x | 1.07x | 2.01x | 1.19x | 30/30 |
| fen_write_native_ep | 0.97x | 4.25x | 9.76x | 1.66x | 27.05x | 2.18x | 30/30 |
| snapshot | 1.00x | 1.98x | 1.25x | 3.84x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.04x | 2.18x | N/A | 4.53x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.57x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.32x | 1.33x | 4.63x | 1.48x | 1.64x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.23x | 145.05x | 229.99x | 147.78x | 5/5 |
| chess960_materialize | 0.99x | 1.60x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.88x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 19.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.05x | 1.94x | 1.53x | 3.18x | 2.51x | 10/10 |
| corpus_materialize | 0.99x | 1.62x | 1.58x | 1.18x | 2.07x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.50x | 3.32x | 2.46x | 3.28x | 1.75x | 10/10 |
| replay_trusted_native | 0.97x | 0.88x | 1.57x | 1.94x | 2.55x | 2.78x | 9/10 |
| replay_checked_native | 0.97x | 2.48x | 1.08x | 8.32x | 8.44x | 9.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
