| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.49x | 1.54x | 1.56x | 2.42x | 1.82x | 30/30 |
| materialize | 0.97x | 1.92x | 1.61x | 1.23x | 2.22x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.03x | 1.55x | 1.44x | 2.44x | 1.92x | 30/30 |
| perft_full_d3 | 0.99x | 1.27x | 1.39x | 1.23x | 1.80x | 1.76x | 30/30 |
| fen_parse | 1.02x | 1.37x | 1.48x | 1.11x | 2.17x | 1.32x | 30/30 |
| fen_write_native_ep | 0.96x | 4.24x | 9.62x | 1.70x | 28.06x | 2.03x | 30/30 |
| snapshot | 1.00x | 2.01x | 1.25x | 3.27x | 1.25x | 1.26x | 30/30 |
| san_parse_all | 0.99x | 2.06x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.32x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.96x | 1.12x | 4.05x | 1.26x | 1.51x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.07x | 67.04x | 115.97x | 201.94x | 136.17x | 5/5 |
| chess960_materialize | 1.01x | 1.54x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.64x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 29.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 21.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.97x | 1.94x | 1.50x | 2.86x | 2.50x | 10/10 |
| corpus_materialize | 1.01x | 1.64x | 1.55x | 1.22x | 2.07x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.43x | 3.19x | 2.39x | 3.12x | 1.75x | 10/10 |
| replay_trusted_native | 1.00x | 0.70x | 1.26x | 1.52x | 2.06x | 2.34x | 8/10 |
| replay_checked_native | 1.02x | 2.25x | 0.99x | 7.79x | 7.55x | 8.89x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
