| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.19x | 1.49x | 1.44x | 2.21x | 1.82x | 30/30 |
| materialize | 0.99x | 2.03x | 1.30x | 1.12x | 2.26x | 1.42x | 29/30 |
| perft_bulk_d4 | 0.97x | 3.69x | 1.51x | 1.35x | 2.24x | 2.04x | 30/30 |
| perft_full_d3 | 0.99x | 1.25x | 1.31x | 1.06x | 1.76x | 1.54x | 28/30 |
| fen_parse | 0.99x | 1.29x | 1.34x | 1.30x | 2.27x | 1.35x | 30/30 |
| fen_write_native_ep | 1.01x | 4.01x | 7.87x | 1.60x | 31.71x | 2.02x | 30/30 |
| snapshot | 0.97x | 1.67x | 1.17x | 4.96x | 1.19x | 1.06x | 29/30 |
| san_parse_all | 0.99x | 2.16x | N/A | 4.66x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.93x | N/A | 3.45x | N/A | N/A | 12/12 |
| apply_children_full | 0.78x | 1.20x | 0.84x | 4.94x | 0.93x | 1.09x | 18/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.08x | 98.07x | 139.74x | 223.50x | 146.73x | 5/5 |
| chess960_materialize | 1.01x | 1.50x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.54x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 17.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.62x | 1.80x | 1.32x | 2.36x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.77x | 1.37x | 1.17x | 2.78x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.87x | 2.37x | 1.97x | 2.27x | 1.52x | 10/10 |
| replay_trusted_native | 1.00x | 0.75x | 1.26x | 1.60x | 2.61x | 3.22x | 8/10 |
| replay_checked_native | 1.03x | 2.21x | 0.93x | 5.35x | 6.45x | 7.48x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
