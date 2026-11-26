| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.88x | 1.46x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 1.01x | 1.83x | 1.43x | 1.26x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.52x | 1.39x | 1.95x | 2.00x | 30/30 |
| perft_full_d3 | 1.03x | 1.21x | 1.31x | 1.09x | 1.61x | 1.51x | 29/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.27x | 1.76x | 1.43x | 30/30 |
| fen_write_native_ep | 1.01x | 3.00x | 6.15x | 1.05x | 19.74x | 1.59x | 29/30 |
| snapshot | 0.98x | 1.17x | 1.01x | 1.46x | 1.00x | 0.94x | 20/30 |
| san_parse_all | 0.97x | 1.79x | N/A | 4.11x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.90x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.11x | 1.22x | 1.44x | 3.96x | 1.19x | 1.18x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.27x | 76.29x | 129.28x | 177.15x | 144.31x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.63x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 18.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.68x | 1.79x | 1.43x | 2.10x | 2.31x | 10/10 |
| corpus_materialize | 1.01x | 1.59x | 1.51x | 1.21x | 1.87x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.95x | 2.09x | 2.88x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.52x | 1.33x | 1.60x | 1.80x | 1.83x | 8/10 |
| replay_checked_native | 0.97x | 1.96x | 0.96x | 6.47x | 5.75x | 7.41x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
