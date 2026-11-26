| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.65x | 1.55x | 1.32x | 1.77x | 1.29x | 29/30 |
| materialize | 0.99x | 2.16x | 1.76x | 1.77x | 2.75x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.98x | 1.58x | 1.17x | 1.95x | 1.78x | 30/30 |
| perft_full_d3 | 0.99x | 1.53x | 1.77x | 1.57x | 2.10x | 1.83x | 30/30 |
| fen_parse | 1.01x | 1.47x | 1.35x | 1.13x | 1.50x | 1.19x | 30/30 |
| fen_write_native_ep | 1.00x | 2.99x | 9.72x | 1.83x | 26.27x | 2.55x | 30/30 |
| snapshot | 1.01x | 1.46x | 1.13x | 2.91x | 1.12x | 1.00x | 29/30 |
| san_parse_all | 1.01x | 1.80x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.24x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.82x | 1.17x | 3.39x | 1.13x | 1.27x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.79x | 111.39x | 130.59x | 112.61x | 5/5 |
| chess960_materialize | 0.94x | 1.55x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.88x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 40.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.20x | 23.68x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.61x | 2.33x | 1.26x | 2.36x | 3.22x | 10/10 |
| corpus_materialize | 1.00x | 1.54x | 1.43x | 1.17x | 1.89x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.98x | 2.89x | 2.25x | 2.81x | 1.59x | 10/10 |
| replay_trusted_native | 1.01x | 0.72x | 1.30x | 1.32x | 1.80x | 1.67x | 8/10 |
| replay_checked_native | 1.00x | 2.00x | 1.11x | 8.46x | 8.29x | 9.34x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
