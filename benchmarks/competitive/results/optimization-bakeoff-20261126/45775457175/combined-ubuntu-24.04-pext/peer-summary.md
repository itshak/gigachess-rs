| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.58x | 1.55x | 1.59x | 2.49x | 1.80x | 30/30 |
| materialize | 1.06x | 2.22x | 1.75x | 1.16x | 2.29x | 1.45x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.15x | 1.62x | 1.34x | 2.60x | 2.10x | 30/30 |
| perft_full_d3 | 1.02x | 0.83x | 0.87x | 0.67x | 1.22x | 0.97x | 7/30 |
| fen_parse | 0.95x | 1.29x | 1.48x | 1.11x | 1.93x | 1.28x | 30/30 |
| fen_write_native_ep | 0.99x | 2.68x | 6.57x | 1.23x | 18.94x | 1.40x | 30/30 |
| snapshot | 1.23x | 1.91x | 1.25x | 3.30x | 1.30x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.06x | N/A | 3.57x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.04x | N/A | 2.26x | N/A | N/A | 12/12 |
| apply_children_full | 0.85x | 1.41x | 0.96x | 3.05x | 1.09x | 1.14x | 25/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.97x | N/A | N/A | 2/6 |
| startpos | 0.91x | 1.00x | 80.89x | 115.74x | 183.18x | 132.37x | 4/5 |
| chess960_materialize | 1.00x | 1.74x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.89x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 17.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.70x | 1.81x | 1.40x | 2.55x | 2.07x | 10/10 |
| corpus_materialize | 1.01x | 1.93x | 1.78x | 1.29x | 2.94x | 1.59x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.38x | 1.88x | 1.35x | 1.82x | 0.97x | 8/10 |
| replay_trusted_native | 0.99x | 0.62x | 1.09x | 1.23x | 2.15x | 2.49x | 7/10 |
| replay_checked_native | 1.02x | 2.46x | 0.97x | 5.33x | 6.35x | 7.19x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
