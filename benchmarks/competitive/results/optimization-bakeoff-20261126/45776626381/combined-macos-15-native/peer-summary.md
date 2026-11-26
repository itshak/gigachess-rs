| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.75x | 1.48x | 1.53x | 1.99x | 1.78x | 30/30 |
| materialize | 0.89x | 1.81x | 1.41x | 0.90x | 2.01x | 1.19x | 24/30 |
| perft_bulk_d4 | 1.02x | 4.06x | 1.54x | 1.35x | 2.03x | 1.84x | 30/30 |
| perft_full_d3 | 1.74x | 1.65x | 1.77x | 1.34x | 2.01x | 2.02x | 30/30 |
| fen_parse | 1.03x | 1.29x | 1.46x | 1.45x | 1.67x | 1.39x | 30/30 |
| fen_write_native_ep | 0.97x | 3.12x | 6.61x | 1.39x | 21.17x | 1.74x | 30/30 |
| snapshot | 0.93x | 1.59x | 1.02x | 2.09x | 1.04x | 0.95x | 25/30 |
| san_parse_all | 1.15x | 1.92x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.09x | 11.70x | N/A | 2.76x | N/A | N/A | 12/12 |
| apply_children_full | 1.30x | 1.06x | 1.05x | 5.54x | 1.01x | 1.22x | 24/30 |
| make_unmake_full | 1.20x | N/A | N/A | 1.25x | N/A | N/A | 5/6 |
| startpos | 1.08x | 1.26x | 68.24x | 163.61x | 196.77x | 155.35x | 5/5 |
| chess960_materialize | 0.92x | 1.44x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.21x | 3.02x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 41.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.04x | 20.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 6.24x | 2.06x | 1.55x | 2.32x | 2.88x | 10/10 |
| corpus_materialize | 0.98x | 1.35x | 1.32x | 1.03x | 1.72x | 1.12x | 9/10 |
| corpus_uci_sorted | 1.33x | 1.80x | 2.42x | 1.76x | 2.27x | 1.31x | 10/10 |
| replay_trusted_native | 1.50x | 0.60x | 1.58x | 1.48x | 1.76x | 1.73x | 8/10 |
| replay_checked_native | 1.07x | 2.20x | 1.07x | 7.48x | 6.70x | 9.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
