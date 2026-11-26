| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.68x | 1.57x | 1.49x | 2.00x | 1.75x | 30/30 |
| materialize | 1.03x | 1.85x | 1.47x | 1.01x | 2.41x | 1.24x | 26/30 |
| perft_bulk_d4 | 1.11x | 4.33x | 1.65x | 1.34x | 2.11x | 2.02x | 30/30 |
| perft_full_d3 | 0.99x | 1.54x | 1.70x | 1.34x | 2.06x | 2.01x | 30/30 |
| fen_parse | 0.97x | 1.22x | 1.30x | 1.28x | 1.60x | 1.52x | 30/30 |
| fen_write_native_ep | 1.05x | 4.18x | 9.12x | 1.81x | 29.15x | 2.29x | 30/30 |
| snapshot | 1.03x | 1.59x | 1.03x | 2.13x | 0.99x | 0.90x | 24/30 |
| san_parse_all | 1.08x | 1.88x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.10x | N/A | 2.62x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 0.93x | 1.04x | 4.73x | 0.93x | 1.10x | 19/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.18x | N/A | N/A | 5/6 |
| startpos | 0.87x | 1.17x | 67.36x | 126.38x | 147.81x | 136.44x | 5/5 |
| chess960_materialize | 1.00x | 1.54x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 3.23x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 37.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 0.92x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.07x | 21.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.23x | 2.23x | 1.30x | 2.63x | 2.65x | 10/10 |
| corpus_materialize | 0.99x | 1.43x | 1.35x | 1.18x | 1.88x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.07x | 1.96x | 2.65x | 2.08x | 2.42x | 1.45x | 10/10 |
| replay_trusted_native | 0.86x | 0.50x | 1.39x | 1.37x | 1.62x | 1.50x | 8/10 |
| replay_checked_native | 1.07x | 2.22x | 1.09x | 7.78x | 7.26x | 8.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
