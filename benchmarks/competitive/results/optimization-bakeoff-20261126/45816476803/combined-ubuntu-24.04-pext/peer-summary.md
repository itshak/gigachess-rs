| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.30x | 1.45x | 1.49x | 2.28x | 1.72x | 30/30 |
| materialize | 1.00x | 1.90x | 1.58x | 1.20x | 2.21x | 1.45x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.63x | 1.40x | 1.25x | 2.20x | 1.71x | 30/30 |
| perft_full_d3 | 1.03x | 1.28x | 1.43x | 1.25x | 1.84x | 1.84x | 30/30 |
| fen_parse | 1.14x | 1.36x | 1.47x | 1.21x | 2.25x | 1.38x | 30/30 |
| fen_write_native_ep | 0.99x | 3.99x | 11.23x | 1.72x | 27.14x | 2.05x | 30/30 |
| snapshot | 1.00x | 1.54x | 0.97x | 2.91x | 1.24x | 0.97x | 18/30 |
| san_parse_all | 1.00x | 2.15x | N/A | 4.50x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.73x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.12x | 1.21x | 4.22x | 1.36x | 1.52x | 29/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 77.97x | 146.92x | 228.38x | 154.35x | 5/5 |
| chess960_materialize | 1.00x | 1.55x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.63x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.09x | 1.64x | 1.30x | 2.46x | 2.10x | 10/10 |
| corpus_materialize | 1.01x | 1.54x | 1.47x | 1.13x | 1.96x | 1.30x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.41x | 3.18x | 2.45x | 3.07x | 1.79x | 10/10 |
| replay_trusted_native | 1.00x | 0.80x | 1.46x | 1.76x | 2.33x | 2.55x | 8/10 |
| replay_checked_native | 0.99x | 2.29x | 1.03x | 7.87x | 7.74x | 8.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
