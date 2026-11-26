| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.32x | 1.48x | 1.46x | 2.04x | 1.78x | 30/30 |
| materialize | 0.94x | 1.84x | 1.25x | 1.15x | 2.13x | 1.52x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.75x | 1.46x | 1.37x | 2.07x | 1.90x | 30/30 |
| perft_full_d3 | 0.99x | 0.81x | 0.86x | 0.79x | 1.12x | 1.08x | 11/30 |
| fen_parse | 1.04x | 1.26x | 1.55x | 1.39x | 2.21x | 1.54x | 30/30 |
| fen_write_native_ep | 0.97x | 2.48x | 5.01x | 1.07x | 19.40x | 1.23x | 29/30 |
| snapshot | 0.98x | 2.19x | 0.99x | 3.35x | 1.23x | 1.06x | 26/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 5.07x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.15x | N/A | 3.49x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.33x | 0.85x | 5.36x | 0.89x | 0.94x | 14/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.07x | 69.02x | 132.78x | 203.34x | 153.70x | 5/5 |
| chess960_materialize | 1.01x | 1.47x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.45x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 29.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.29x | 1.75x | 1.42x | 2.56x | 2.33x | 10/10 |
| corpus_materialize | 0.96x | 1.58x | 1.24x | 1.12x | 1.95x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.20x | 1.54x | 1.25x | 1.50x | 0.98x | 9/10 |
| replay_trusted_native | 0.96x | 0.52x | 0.97x | 1.23x | 1.61x | 1.89x | 6/10 |
| replay_checked_native | 0.99x | 2.07x | 0.92x | 6.94x | 6.88x | 8.02x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
