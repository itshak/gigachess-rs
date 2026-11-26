| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.17x | 3.96x | 1.70x | 1.44x | 2.21x | 1.73x | 30/30 |
| materialize | 0.99x | 1.92x | 1.46x | 1.61x | 2.29x | 1.76x | 30/30 |
| perft_bulk_d4 | 1.34x | 3.79x | 1.53x | 1.17x | 2.33x | 1.84x | 30/30 |
| perft_full_d3 | 2.42x | 1.42x | 1.68x | 1.58x | 2.14x | 1.74x | 30/30 |
| fen_parse | 1.13x | 1.47x | 1.30x | 1.13x | 1.45x | 1.14x | 30/30 |
| fen_write_native_ep | 0.99x | 2.85x | 6.22x | 1.25x | 18.40x | 1.59x | 30/30 |
| snapshot | 1.67x | 1.82x | 1.15x | 7.00x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 2.05x | 1.86x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 2.06x | 9.25x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.55x | 1.49x | 0.93x | 2.80x | 0.92x | 1.15x | 18/30 |
| make_unmake_full | 1.37x | N/A | N/A | 0.99x | N/A | N/A | 4/6 |
| startpos | 1.80x | 1.12x | 93.36x | 159.57x | 196.50x | 160.88x | 5/5 |
| chess960_materialize | 1.10x | 1.52x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.41x | 2.22x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 39.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 0.86x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.46x | 22.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 6.31x | 2.38x | 1.31x | 2.89x | 2.65x | 10/10 |
| corpus_materialize | 1.02x | 1.43x | 1.33x | 1.10x | 1.74x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.49x | 1.34x | 1.78x | 1.33x | 1.76x | 0.94x | 8/10 |
| replay_trusted_native | 1.25x | 0.64x | 1.15x | 1.17x | 1.62x | 1.90x | 7/10 |
| replay_checked_native | 1.27x | 1.79x | 0.99x | 7.62x | 7.26x | 8.80x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
