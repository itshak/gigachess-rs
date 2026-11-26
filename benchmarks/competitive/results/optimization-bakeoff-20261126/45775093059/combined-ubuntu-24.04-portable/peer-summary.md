| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 2.60x | 1.48x | 1.28x | 1.74x | 1.23x | 28/30 |
| materialize | 1.01x | 2.04x | 1.65x | 1.76x | 2.61x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.20x | 2.90x | 1.55x | 1.15x | 1.93x | 1.71x | 30/30 |
| perft_full_d3 | 2.30x | 1.48x | 1.72x | 1.58x | 2.14x | 1.74x | 30/30 |
| fen_parse | 1.06x | 1.48x | 1.47x | 1.19x | 1.62x | 1.22x | 30/30 |
| fen_write_native_ep | 0.98x | 1.95x | 6.14x | 1.14x | 16.65x | 1.59x | 30/30 |
| snapshot | 2.24x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 2.34x | 1.83x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 2.67x | 9.37x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 2.32x | 1.59x | 1.06x | 3.01x | 0.98x | 1.14x | 25/30 |
| make_unmake_full | 1.36x | N/A | N/A | 1.03x | N/A | N/A | 4/6 |
| startpos | 1.28x | 1.28x | 57.03x | 109.09x | 129.02x | 104.05x | 5/5 |
| chess960_materialize | 1.26x | 1.64x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.29x | 1.82x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.20x | 39.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.89x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.46x | 23.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.52x | 2.28x | 1.25x | 2.36x | 3.10x | 10/10 |
| corpus_materialize | 1.01x | 1.53x | 1.42x | 1.17x | 1.88x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.54x | 1.19x | 1.75x | 1.35x | 1.71x | 0.93x | 8/10 |
| replay_trusted_native | 1.30x | 0.64x | 1.23x | 1.24x | 1.71x | 1.55x | 7/10 |
| replay_checked_native | 1.27x | 1.77x | 0.98x | 7.61x | 7.45x | 8.48x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
