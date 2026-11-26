| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.09x | 1.72x | 1.46x | 2.27x | 1.76x | 30/30 |
| materialize | 0.93x | 1.96x | 1.55x | 1.62x | 2.41x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.86x | 1.58x | 1.20x | 2.38x | 1.90x | 30/30 |
| perft_full_d3 | 1.01x | 1.48x | 1.71x | 1.70x | 2.12x | 1.86x | 30/30 |
| fen_parse | 1.13x | 1.38x | 1.62x | 1.08x | 1.49x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 2.92x | 6.48x | 1.38x | 17.81x | 1.72x | 30/30 |
| snapshot | 1.29x | 1.66x | 1.33x | 4.00x | 2.88x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 1.91x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.20x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.81x | 1.15x | 3.27x | 1.21x | 1.39x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.37x | 1.26x | 99.86x | 172.19x | 215.82x | 180.68x | 5/5 |
| chess960_materialize | 1.00x | 1.67x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.29x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.48x | 2.47x | 1.33x | 2.97x | 2.68x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.33x | 1.08x | 1.73x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 2.95x | 2.22x | 2.83x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.73x | 1.31x | 1.26x | 1.78x | 2.05x | 7/10 |
| replay_checked_native | 1.01x | 2.01x | 1.14x | 8.57x | 8.22x | 9.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
