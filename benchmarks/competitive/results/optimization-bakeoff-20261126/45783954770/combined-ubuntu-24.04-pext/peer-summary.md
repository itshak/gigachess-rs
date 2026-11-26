| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.26x | 1.78x | 1.56x | 2.44x | 1.82x | 30/30 |
| materialize | 0.99x | 2.19x | 1.65x | 1.74x | 2.70x | 1.94x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.04x | 1.61x | 1.26x | 2.51x | 1.97x | 30/30 |
| perft_full_d3 | 1.03x | 1.57x | 1.77x | 1.70x | 2.25x | 1.86x | 30/30 |
| fen_parse | 1.04x | 1.25x | 1.33x | 1.13x | 1.45x | 1.22x | 30/30 |
| fen_write_native_ep | 0.96x | 4.36x | 9.94x | 1.82x | 27.26x | 2.15x | 30/30 |
| snapshot | 0.50x | 1.50x | 1.17x | 2.00x | 1.17x | 1.00x | 29/30 |
| san_parse_all | 0.95x | 1.82x | N/A | 4.11x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.49x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.66x | 1.10x | 3.03x | 1.08x | 1.29x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.74x | 1.33x | 65.73x | 121.94x | 150.79x | 132.09x | 5/5 |
| chess960_materialize | 1.07x | 1.74x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.36x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.14x | 21.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.80x | 2.52x | 1.41x | 3.12x | 2.83x | 10/10 |
| corpus_materialize | 0.99x | 1.46x | 1.33x | 1.10x | 1.81x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.30x | 3.02x | 2.26x | 2.99x | 1.59x | 10/10 |
| replay_trusted_native | 1.00x | 0.70x | 1.32x | 1.30x | 1.81x | 2.06x | 8/10 |
| replay_checked_native | 0.99x | 1.97x | 1.09x | 8.30x | 8.15x | 9.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
