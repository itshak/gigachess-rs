| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.35x | 1.44x | 1.36x | 2.14x | 1.78x | 30/30 |
| materialize | 1.03x | 2.06x | 1.30x | 1.14x | 2.26x | 1.83x | 30/30 |
| perft_bulk_d4 | 0.89x | 3.66x | 1.31x | 1.13x | 2.09x | 1.93x | 30/30 |
| perft_full_d3 | 1.05x | 0.68x | 0.73x | 0.63x | 0.97x | 1.01x | 6/30 |
| fen_parse | 0.83x | 1.15x | 1.30x | 1.34x | 1.84x | 1.30x | 30/30 |
| fen_write_native_ep | 0.96x | 3.03x | 5.82x | 1.24x | 25.66x | 1.69x | 30/30 |
| snapshot | 2.59x | 1.76x | 1.04x | 3.01x | 1.04x | 1.01x | 28/30 |
| san_parse_all | 0.95x | 1.77x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 9.71x | N/A | 3.29x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 0.97x | 0.77x | 3.98x | 0.64x | 0.88x | 8/30 |
| make_unmake_full | 0.83x | N/A | N/A | 0.85x | N/A | N/A | 0/6 |
| startpos | 1.00x | 1.25x | 91.99x | 182.00x | 250.85x | 180.98x | 5/5 |
| chess960_materialize | 1.06x | 1.45x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 2.32x | 1.18x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.79x | 30.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.56x | 15.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.07x | 1.84x | 1.22x | 2.41x | 2.32x | 10/10 |
| corpus_materialize | 1.04x | 1.73x | 1.31x | 1.09x | 2.26x | 1.61x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.14x | 1.38x | 1.19x | 1.36x | 0.95x | 8/10 |
| replay_trusted_native | 1.05x | 0.51x | 0.87x | 1.06x | 1.55x | 1.88x | 6/10 |
| replay_checked_native | 0.71x | 1.62x | 0.70x | 5.35x | 5.59x | 7.10x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
