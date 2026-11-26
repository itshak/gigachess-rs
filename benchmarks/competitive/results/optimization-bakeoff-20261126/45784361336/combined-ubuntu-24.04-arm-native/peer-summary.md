| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.47x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.00x | 1.82x | 1.43x | 1.27x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.71x | 1.52x | 1.38x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.27x | 1.07x | 1.53x | 1.47x | 29/30 |
| fen_parse | 1.17x | 1.46x | 1.56x | 1.48x | 2.08x | 1.67x | 30/30 |
| fen_write_native_ep | 1.00x | 4.51x | 9.25x | 1.58x | 29.34x | 2.39x | 30/30 |
| snapshot | 0.99x | 1.18x | 0.95x | 1.50x | 0.92x | 0.96x | 12/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.80x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.12x | 1.30x | 3.59x | 1.07x | 1.08x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 73.34x | 124.26x | 165.82x | 133.73x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.61x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.81x | 1.84x | 1.45x | 2.26x | 2.37x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.50x | 1.21x | 1.91x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.50x | 1.31x | 1.59x | 1.75x | 1.82x | 8/10 |
| replay_checked_native | 1.00x | 2.03x | 0.99x | 6.74x | 6.01x | 7.73x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
