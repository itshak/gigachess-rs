| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.67x | 1.40x | 1.50x | 1.81x | 1.81x | 30/30 |
| materialize | 1.00x | 1.77x | 1.39x | 1.22x | 1.96x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.52x | 1.44x | 1.30x | 1.83x | 1.89x | 30/30 |
| perft_full_d3 | 0.99x | 0.82x | 0.90x | 0.74x | 1.08x | 1.04x | 11/30 |
| fen_parse | 0.99x | 1.24x | 1.32x | 1.26x | 1.77x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 2.95x | 6.10x | 1.04x | 19.45x | 1.58x | 28/30 |
| snapshot | 0.96x | 1.21x | 0.95x | 1.53x | 0.94x | 0.95x | 12/30 |
| san_parse_all | 1.00x | 1.77x | N/A | 4.09x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.13x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 0.88x | 1.04x | 2.92x | 0.85x | 0.86x | 10/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.15x | 71.02x | 118.75x | 166.18x | 133.73x | 5/5 |
| chess960_materialize | 1.00x | 1.41x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.42x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.68x | 1.77x | 1.43x | 2.08x | 2.30x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.48x | 1.19x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.50x | 2.11x | 2.91x | 2.15x | 2.81x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.41x | 1.08x | 1.30x | 1.45x | 1.49x | 8/10 |
| replay_checked_native | 0.99x | 1.84x | 0.90x | 5.97x | 5.36x | 6.97x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
