| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.89x | 4.87x | 1.74x | 1.75x | 2.40x | 1.76x | 30/30 |
| materialize | 1.00x | 2.26x | 1.76x | 1.18x | 2.52x | 1.57x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.91x | 1.50x | 1.18x | 1.94x | 1.76x | 29/30 |
| perft_full_d3 | 1.00x | 1.62x | 1.83x | 1.31x | 2.02x | 2.05x | 29/30 |
| fen_parse | 1.05x | 1.28x | 1.38x | 1.40x | 1.63x | 1.55x | 30/30 |
| fen_write_native_ep | 0.96x | 4.18x | 9.10x | 1.70x | 28.03x | 2.24x | 30/30 |
| snapshot | 0.92x | 1.61x | 1.07x | 2.26x | 1.04x | 0.88x | 21/30 |
| san_parse_all | 0.97x | 1.80x | N/A | 3.74x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 12.10x | N/A | 2.73x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 1.14x | 1.23x | 5.59x | 1.08x | 1.18x | 28/30 |
| make_unmake_full | 0.85x | N/A | N/A | 1.15x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.21x | 69.78x | 160.07x | 187.02x | 150.88x | 5/5 |
| chess960_materialize | 0.99x | 1.77x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 3.54x | 1.71x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 40.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.13x | 1.10x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 21.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 5.87x | 1.75x | 1.03x | 2.21x | 2.06x | 9/10 |
| corpus_materialize | 1.06x | 1.49x | 1.48x | 1.16x | 1.84x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.15x | 1.87x | 2.45x | 1.82x | 2.34x | 1.29x | 10/10 |
| replay_trusted_native | 1.02x | 0.61x | 1.55x | 1.60x | 1.63x | 1.61x | 8/10 |
| replay_checked_native | 1.18x | 2.54x | 1.15x | 9.03x | 8.00x | 10.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
