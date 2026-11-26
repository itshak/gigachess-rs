| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 3.47x | 1.33x | 1.43x | 1.73x | 1.73x | 30/30 |
| materialize | 1.02x | 1.74x | 1.37x | 1.20x | 1.92x | 1.33x | 30/30 |
| perft_bulk_d4 | 1.30x | 3.16x | 1.29x | 1.17x | 1.65x | 1.70x | 30/30 |
| perft_full_d3 | 1.26x | 0.83x | 0.90x | 0.75x | 1.09x | 1.05x | 11/30 |
| fen_parse | 1.02x | 1.23x | 1.32x | 1.27x | 1.76x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 2.98x | 6.17x | 1.05x | 19.82x | 1.59x | 29/30 |
| snapshot | 1.24x | 1.19x | 0.96x | 1.52x | 0.95x | 0.95x | 12/30 |
| san_parse_all | 1.93x | 1.80x | N/A | 4.17x | N/A | N/A | 12/12 |
| san_render_native | 2.08x | 11.10x | N/A | 2.38x | N/A | N/A | 12/12 |
| apply_children_full | 1.27x | 0.89x | 1.05x | 2.86x | 0.85x | 0.86x | 9/30 |
| make_unmake_full | 1.24x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.49x | 1.15x | 71.13x | 119.59x | 158.84x | 123.96x | 5/5 |
| chess960_materialize | 1.08x | 1.36x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.31x | 2.16x | 1.09x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 25.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.39x | 19.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 4.47x | 1.73x | 1.40x | 2.01x | 2.25x | 10/10 |
| corpus_materialize | 1.07x | 1.56x | 1.49x | 1.19x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.33x | 1.11x | 1.54x | 1.10x | 1.49x | 0.77x | 8/10 |
| replay_trusted_native | 1.28x | 0.41x | 1.06x | 1.32x | 1.44x | 1.49x | 8/10 |
| replay_checked_native | 1.34x | 1.80x | 0.88x | 5.87x | 5.28x | 6.86x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
