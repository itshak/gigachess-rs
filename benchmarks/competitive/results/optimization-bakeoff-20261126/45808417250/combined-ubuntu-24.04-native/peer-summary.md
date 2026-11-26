| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.04x | 1.72x | 1.48x | 2.08x | 1.83x | 30/30 |
| materialize | 0.99x | 1.97x | 1.56x | 1.60x | 2.28x | 1.61x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.92x | 1.60x | 1.21x | 2.23x | 1.92x | 30/30 |
| perft_full_d3 | 1.00x | 1.55x | 1.88x | 1.68x | 2.19x | 1.92x | 30/30 |
| fen_parse | 1.05x | 1.18x | 1.48x | 1.22x | 1.60x | 1.27x | 30/30 |
| fen_write_native_ep | 1.02x | 5.50x | 10.64x | 2.09x | 29.00x | 2.30x | 30/30 |
| snapshot | 0.73x | 1.55x | 1.22x | 2.76x | 1.22x | 1.13x | 30/30 |
| san_parse_all | 1.11x | 1.86x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.56x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.91x | 1.26x | 3.65x | 1.25x | 1.51x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.07x | N/A | N/A | 5/6 |
| startpos | 0.81x | 1.29x | 83.59x | 150.16x | 183.63x | 148.71x | 5/5 |
| chess960_materialize | 1.01x | 1.59x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.29x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 40.06x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.65x | 14.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.55x | 2.57x | 1.31x | 2.93x | 2.81x | 10/10 |
| corpus_materialize | 0.99x | 1.42x | 1.31x | 1.07x | 1.73x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.36x | 3.05x | 2.30x | 2.95x | 1.55x | 10/10 |
| replay_trusted_native | 0.96x | 0.77x | 1.42x | 1.38x | 1.93x | 2.20x | 7/10 |
| replay_checked_native | 0.96x | 2.11x | 1.03x | 8.36x | 8.37x | 9.65x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
