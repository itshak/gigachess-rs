| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.30x | 1.50x | 1.45x | 2.28x | 1.83x | 30/30 |
| materialize | 1.00x | 1.99x | 1.33x | 1.12x | 2.20x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.81x | 1.54x | 1.37x | 2.41x | 2.14x | 30/30 |
| perft_full_d3 | 1.01x | 0.73x | 0.75x | 0.63x | 1.03x | 0.94x | 5/30 |
| fen_parse | 0.80x | 1.21x | 1.27x | 1.23x | 2.08x | 1.29x | 30/30 |
| fen_write_native_ep | 1.00x | 3.11x | 5.83x | 1.23x | 23.51x | 1.52x | 30/30 |
| snapshot | 0.40x | 1.22x | 1.10x | 0.95x | 1.09x | 0.99x | 19/30 |
| san_parse_all | 1.05x | 1.84x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.76x | N/A | 3.33x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.16x | 1.08x | 5.02x | 1.09x | 1.17x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 0.92x | N/A | N/A | 0/6 |
| startpos | 1.00x | 2.75x | 83.52x | 139.99x | 219.36x | 151.64x | 5/5 |
| chess960_materialize | 1.00x | 1.44x | 1.20x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 17.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.36x | 1.78x | 1.30x | 2.34x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.87x | 1.34x | 1.15x | 2.73x | 1.49x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.24x | 1.51x | 1.26x | 1.45x | 0.99x | 9/10 |
| replay_trusted_native | 1.03x | 0.52x | 0.87x | 1.06x | 1.81x | 2.36x | 5/10 |
| replay_checked_native | 1.01x | 2.11x | 0.88x | 5.02x | 6.14x | 7.19x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
