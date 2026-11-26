| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.91x | 1.47x | 1.35x | 1.90x | 1.33x | 30/30 |
| materialize | 1.01x | 1.99x | 1.74x | 1.27x | 2.34x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.79x | 1.50x | 1.18x | 1.90x | 1.64x | 30/30 |
| perft_full_d3 | 1.01x | 1.44x | 1.66x | 1.33x | 2.01x | 1.90x | 30/30 |
| fen_parse | 0.95x | 1.51x | 1.51x | 1.32x | 2.19x | 1.31x | 30/30 |
| fen_write_native_ep | 0.99x | 2.87x | 9.42x | 1.69x | 27.79x | 2.08x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.22x | 3.06x | 1.00x | 0.97x | 23/30 |
| san_parse_all | 0.99x | 2.03x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.64x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.03x | 1.42x | 4.22x | 1.30x | 1.34x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 65.29x | 116.49x | 174.75x | 110.94x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.15x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 27.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 18.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.58x | 1.78x | 1.19x | 1.98x | 2.38x | 10/10 |
| corpus_materialize | 0.98x | 1.48x | 1.48x | 1.09x | 1.91x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.19x | 3.17x | 2.40x | 3.11x | 1.78x | 10/10 |
| replay_trusted_native | 1.03x | 0.81x | 1.61x | 1.81x | 2.37x | 2.24x | 8/10 |
| replay_checked_native | 1.00x | 2.18x | 1.07x | 7.42x | 7.58x | 8.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
