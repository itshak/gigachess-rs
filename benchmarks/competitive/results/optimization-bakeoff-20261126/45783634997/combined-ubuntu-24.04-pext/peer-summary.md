| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.25x | 1.79x | 1.54x | 2.48x | 1.81x | 30/30 |
| materialize | 0.98x | 2.10x | 1.59x | 1.70x | 2.58x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.05x | 1.61x | 1.27x | 2.58x | 1.97x | 30/30 |
| perft_full_d3 | 0.99x | 1.48x | 1.72x | 1.67x | 2.22x | 1.77x | 30/30 |
| fen_parse | 1.07x | 1.40x | 1.31x | 1.17x | 1.50x | 1.14x | 30/30 |
| fen_write_native_ep | 1.59x | 4.67x | 9.97x | 1.91x | 29.56x | 2.50x | 30/30 |
| snapshot | 0.77x | 1.55x | 1.00x | 3.48x | 1.00x | 1.15x | 24/30 |
| san_parse_all | 1.04x | 1.99x | N/A | 4.51x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.04x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.77x | 1.39x | 0.87x | 2.55x | 0.88x | 1.05x | 18/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.73x | 1.08x | 74.17x | 133.95x | 163.67x | 132.03x | 5/5 |
| chess960_materialize | 1.07x | 1.72x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.37x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.93x | 20.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.85x | 2.55x | 1.41x | 3.07x | 2.80x | 10/10 |
| corpus_materialize | 1.01x | 1.48x | 1.34x | 1.11x | 1.79x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 3.01x | 2.26x | 2.95x | 1.57x | 10/10 |
| replay_trusted_native | 0.99x | 0.72x | 1.27x | 1.26x | 1.79x | 2.07x | 8/10 |
| replay_checked_native | 1.00x | 2.02x | 1.07x | 8.38x | 8.10x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
