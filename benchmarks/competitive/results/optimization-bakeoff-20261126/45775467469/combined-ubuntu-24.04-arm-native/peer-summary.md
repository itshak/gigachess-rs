| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.64x | 1.40x | 1.50x | 1.82x | 1.81x | 30/30 |
| materialize | 1.00x | 1.76x | 1.39x | 1.22x | 1.96x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.54x | 1.45x | 1.32x | 1.85x | 1.90x | 30/30 |
| perft_full_d3 | 1.33x | 1.10x | 1.18x | 0.99x | 1.44x | 1.39x | 26/30 |
| fen_parse | 1.00x | 1.26x | 1.32x | 1.29x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 2.96x | 6.18x | 1.05x | 19.89x | 1.59x | 29/30 |
| snapshot | 0.97x | 1.20x | 0.96x | 1.53x | 0.95x | 0.95x | 12/30 |
| san_parse_all | 1.00x | 1.78x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.01x | N/A | 2.38x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 0.89x | 1.04x | 2.90x | 0.86x | 0.86x | 10/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.15x | 71.08x | 118.89x | 162.29x | 130.38x | 5/5 |
| chess960_materialize | 1.00x | 1.41x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.42x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.66x | 1.78x | 1.42x | 2.12x | 2.33x | 10/10 |
| corpus_materialize | 0.99x | 1.58x | 1.49x | 1.19x | 1.88x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.41x | 1.93x | 1.39x | 1.88x | 0.97x | 8/10 |
| replay_trusted_native | 1.00x | 0.41x | 1.08x | 1.31x | 1.46x | 1.50x | 8/10 |
| replay_checked_native | 1.01x | 1.85x | 0.89x | 6.01x | 5.36x | 6.97x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
