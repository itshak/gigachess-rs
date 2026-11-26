| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.47x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.03x | 1.89x | 1.48x | 1.29x | 2.08x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.52x | 1.38x | 1.95x | 1.99x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.29x | 1.08x | 1.55x | 1.49x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.35x | 1.29x | 1.78x | 1.43x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.24x | 1.58x | 30.26x | 2.39x | 30/30 |
| snapshot | 1.05x | 1.18x | 1.02x | 1.57x | 1.01x | 1.02x | 27/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.88x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.13x | 1.32x | 3.64x | 1.07x | 1.08x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.17x | 71.68x | 121.67x | 158.81x | 127.00x | 5/5 |
| chess960_materialize | 0.99x | 1.50x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.59x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.78x | 1.84x | 1.50x | 2.14x | 2.37x | 10/10 |
| corpus_materialize | 1.05x | 1.65x | 1.58x | 1.26x | 1.96x | 1.43x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.14x | 2.92x | 2.09x | 2.84x | 1.47x | 10/10 |
| replay_trusted_native | 1.00x | 0.50x | 1.32x | 1.60x | 1.76x | 1.83x | 8/10 |
| replay_checked_native | 0.99x | 2.00x | 0.98x | 6.57x | 5.88x | 7.68x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
