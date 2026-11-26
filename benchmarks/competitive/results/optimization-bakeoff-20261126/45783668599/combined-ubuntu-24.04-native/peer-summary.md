| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.99x | 1.73x | 1.44x | 2.28x | 1.75x | 30/30 |
| materialize | 1.00x | 2.12x | 1.59x | 1.63x | 2.47x | 1.82x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.87x | 1.57x | 1.20x | 2.41x | 1.89x | 30/30 |
| perft_full_d3 | 1.01x | 1.48x | 1.69x | 1.64x | 2.19x | 1.80x | 30/30 |
| fen_parse | 1.02x | 1.36x | 1.28x | 1.07x | 1.49x | 1.14x | 29/30 |
| fen_write_native_ep | 1.00x | 2.96x | 6.66x | 1.31x | 19.48x | 1.60x | 30/30 |
| snapshot | 0.91x | 1.58x | 1.25x | 2.83x | 1.25x | 1.15x | 30/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.16x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.77x | 1.16x | 3.26x | 1.16x | 1.40x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 81.21x | 147.96x | 187.95x | 155.48x | 5/5 |
| chess960_materialize | 1.00x | 1.67x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.27x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.99x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.03x | 20.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.42x | 2.43x | 1.32x | 2.97x | 2.70x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.09x | 1.73x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.23x | 3.05x | 2.24x | 2.95x | 1.54x | 10/10 |
| replay_trusted_native | 1.01x | 0.71x | 1.25x | 1.29x | 1.85x | 2.11x | 8/10 |
| replay_checked_native | 1.02x | 2.02x | 1.06x | 8.63x | 8.26x | 9.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
