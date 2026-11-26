| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.70x | 1.38x | 1.25x | 1.74x | 1.24x | 27/30 |
| materialize | 1.01x | 1.97x | 1.76x | 1.25x | 2.35x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.80x | 1.51x | 1.17x | 1.93x | 1.63x | 30/30 |
| perft_full_d3 | 0.97x | 0.91x | 1.04x | 0.85x | 1.29x | 1.12x | 17/30 |
| fen_parse | 0.94x | 1.46x | 1.36x | 1.14x | 2.00x | 1.16x | 30/30 |
| fen_write_native_ep | 1.00x | 1.59x | 5.48x | 1.07x | 15.55x | 1.12x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.05x | 3.04x | 1.03x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.03x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.58x | N/A | 2.38x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.53x | 1.05x | 3.05x | 0.95x | 0.95x | 16/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.26x | 66.78x | 116.19x | 182.50x | 112.06x | 5/5 |
| chess960_materialize | 1.01x | 1.43x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.95x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 27.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 20.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.70x | 1.87x | 1.22x | 2.04x | 2.45x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.63x | 1.19x | 2.06x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.24x | 1.83x | 1.37x | 1.78x | 0.97x | 8/10 |
| replay_trusted_native | 1.00x | 0.60x | 1.19x | 1.36x | 1.74x | 1.63x | 8/10 |
| replay_checked_native | 0.88x | 1.77x | 0.88x | 6.22x | 6.17x | 7.03x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
