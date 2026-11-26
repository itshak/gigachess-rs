| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.20x | 1.74x | 1.51x | 2.37x | 1.77x | 30/30 |
| materialize | 0.93x | 2.03x | 1.49x | 1.65x | 2.38x | 1.79x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.76x | 1.50x | 1.17x | 2.33x | 1.83x | 30/30 |
| perft_full_d3 | 0.99x | 0.87x | 1.01x | 0.97x | 1.26x | 1.04x | 16/30 |
| fen_parse | 0.99x | 1.42x | 1.61x | 1.17x | 1.47x | 1.25x | 30/30 |
| fen_write_native_ep | 0.96x | 3.12x | 6.33x | 1.17x | 18.14x | 1.59x | 30/30 |
| snapshot | 0.87x | 1.35x | 0.96x | 3.01x | 0.87x | 1.00x | 14/30 |
| san_parse_all | 0.93x | 1.78x | N/A | 4.02x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 8.99x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.51x | 0.96x | 2.78x | 0.97x | 1.15x | 24/30 |
| make_unmake_full | 0.93x | N/A | N/A | 0.88x | N/A | N/A | 0/6 |
| startpos | 1.12x | 1.21x | 80.93x | 151.40x | 175.24x | 158.54x | 5/5 |
| chess960_materialize | 1.01x | 1.57x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.22x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 19.34x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 6.61x | 2.48x | 1.36x | 3.08x | 2.71x | 10/10 |
| corpus_materialize | 1.00x | 1.45x | 1.32x | 1.10x | 1.78x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.41x | 1.81x | 1.38x | 1.78x | 0.95x | 8/10 |
| replay_trusted_native | 0.98x | 0.61x | 1.10x | 1.07x | 1.52x | 1.75x | 7/10 |
| replay_checked_native | 0.94x | 1.72x | 0.93x | 7.34x | 7.32x | 8.25x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
