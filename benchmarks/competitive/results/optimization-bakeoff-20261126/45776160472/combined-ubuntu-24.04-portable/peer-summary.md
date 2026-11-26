| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.62x | 1.48x | 1.29x | 1.73x | 1.24x | 28/30 |
| materialize | 1.09x | 2.38x | 1.75x | 1.74x | 2.66x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.91x | 1.55x | 1.14x | 1.91x | 1.71x | 30/30 |
| perft_full_d3 | 1.00x | 1.08x | 1.17x | 1.02x | 1.41x | 1.15x | 25/30 |
| fen_parse | 0.95x | 1.43x | 1.42x | 1.15x | 1.45x | 1.13x | 30/30 |
| fen_write_native_ep | 1.51x | 3.11x | 10.16x | 1.88x | 27.75x | 2.46x | 30/30 |
| snapshot | 0.90x | 1.46x | 1.12x | 2.91x | 1.12x | 1.00x | 26/30 |
| san_parse_all | 1.05x | 1.78x | N/A | 4.11x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.01x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.62x | 1.06x | 3.05x | 1.01x | 1.15x | 25/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.97x | N/A | N/A | 2/6 |
| startpos | 1.00x | 1.36x | 57.10x | 109.20x | 122.41x | 97.08x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.81x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.07x | 23.76x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.50x | 2.26x | 1.19x | 2.31x | 3.09x | 10/10 |
| corpus_materialize | 1.01x | 1.54x | 1.44x | 1.16x | 1.86x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.21x | 1.88x | 1.38x | 1.85x | 0.97x | 8/10 |
| replay_trusted_native | 1.02x | 0.59x | 1.17x | 1.15x | 1.57x | 1.42x | 7/10 |
| replay_checked_native | 1.01x | 1.70x | 0.97x | 7.41x | 7.25x | 8.30x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
