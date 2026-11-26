| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.31x | 1.79x | 1.55x | 2.41x | 1.81x | 30/30 |
| materialize | 1.00x | 2.20x | 1.63x | 1.73x | 2.70x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.08x | 1.61x | 1.26x | 2.50x | 1.97x | 30/30 |
| perft_full_d3 | 0.98x | 1.49x | 1.75x | 1.64x | 2.22x | 1.76x | 30/30 |
| fen_parse | 1.00x | 1.29x | 1.28x | 1.07x | 1.36x | 1.15x | 30/30 |
| fen_write_native_ep | 1.00x | 2.97x | 6.52x | 1.23x | 17.80x | 1.61x | 30/30 |
| snapshot | 1.29x | 1.83x | 1.16x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.07x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.73x | 1.09x | 3.26x | 1.06x | 1.33x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.10x | 1.12x | 88.27x | 166.94x | 193.26x | 173.22x | 5/5 |
| chess960_materialize | 0.99x | 1.73x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.37x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 24.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.81x | 2.53x | 1.38x | 3.12x | 2.79x | 10/10 |
| corpus_materialize | 1.00x | 1.46x | 1.34x | 1.11x | 1.80x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.22x | 2.92x | 2.21x | 2.86x | 1.55x | 10/10 |
| replay_trusted_native | 1.01x | 0.71x | 1.31x | 1.29x | 1.80x | 2.09x | 8/10 |
| replay_checked_native | 0.99x | 1.97x | 1.09x | 8.35x | 8.11x | 9.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
