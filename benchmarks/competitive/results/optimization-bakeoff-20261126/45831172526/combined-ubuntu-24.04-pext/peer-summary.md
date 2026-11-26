| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.46x | 1.48x | 1.47x | 2.16x | 1.79x | 30/30 |
| materialize | 0.98x | 2.07x | 1.63x | 1.15x | 2.29x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.89x | 1.44x | 1.17x | 2.25x | 2.05x | 30/30 |
| perft_full_d3 | 0.94x | 1.30x | 1.55x | 1.15x | 2.00x | 1.90x | 29/30 |
| fen_parse | 0.97x | 1.38x | 1.57x | 1.18x | 2.08x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 3.84x | 8.70x | 1.47x | 25.24x | 1.93x | 30/30 |
| snapshot | 0.72x | 1.78x | 1.07x | 5.87x | 1.07x | 1.07x | 30/30 |
| san_parse_all | 0.99x | 1.79x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.90x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.91x | 1.40x | 4.13x | 1.36x | 1.93x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 91.76x | 139.90x | 221.68x | 154.10x | 5/5 |
| chess960_materialize | 1.00x | 1.64x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.69x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 35.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 27.52x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.47x | 1.61x | 1.09x | 2.10x | 1.95x | 9/10 |
| corpus_materialize | 1.01x | 1.57x | 1.49x | 1.08x | 2.08x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 2.94x | 2.38x | 2.91x | 1.67x | 10/10 |
| replay_trusted_native | 0.97x | 0.69x | 1.31x | 1.50x | 2.30x | 2.72x | 8/10 |
| replay_checked_native | 1.00x | 2.40x | 1.14x | 8.41x | 8.66x | 10.40x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
