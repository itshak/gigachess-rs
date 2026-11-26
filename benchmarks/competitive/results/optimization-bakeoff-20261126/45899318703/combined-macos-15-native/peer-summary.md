| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.17x | 4.91x | 1.73x | 1.75x | 2.43x | 2.02x | 30/30 |
| materialize | 1.19x | 1.96x | 1.49x | 1.09x | 2.24x | 1.36x | 27/30 |
| perft_bulk_d4 | 1.20x | 3.72x | 1.42x | 1.16x | 1.74x | 1.66x | 30/30 |
| perft_full_d3 | 1.40x | 1.52x | 1.70x | 1.29x | 1.95x | 2.01x | 30/30 |
| fen_parse | 1.33x | 1.32x | 1.47x | 1.53x | 1.97x | 1.46x | 30/30 |
| fen_write_native_ep | 1.30x | 4.21x | 14.20x | 1.84x | 29.73x | 2.27x | 30/30 |
| snapshot | 1.08x | 1.60x | 1.16x | 2.46x | 1.06x | 1.00x | 26/30 |
| san_parse_all | 1.15x | 1.78x | N/A | 3.61x | N/A | N/A | 12/12 |
| san_render_native | 1.23x | 12.42x | N/A | 2.73x | N/A | N/A | 12/12 |
| apply_children_full | 1.19x | 1.11x | 1.15x | 5.92x | 1.16x | 1.34x | 27/30 |
| make_unmake_full | 1.20x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.60x | 1.99x | 76.05x | 138.05x | 184.23x | 126.82x | 5/5 |
| chess960_materialize | 1.27x | 1.83x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.38x | 3.20x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.20x | 34.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.31x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.15x | 18.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.16x | 5.88x | 1.84x | 1.28x | 2.25x | 2.59x | 10/10 |
| corpus_materialize | 1.15x | 1.56x | 1.45x | 1.11x | 1.83x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.21x | 1.84x | 2.46x | 1.87x | 2.38x | 1.31x | 10/10 |
| replay_trusted_native | 1.29x | 0.67x | 1.74x | 1.93x | 2.00x | 2.04x | 8/10 |
| replay_checked_native | 1.01x | 2.82x | 1.20x | 9.48x | 8.20x | 10.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
