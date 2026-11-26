| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.96x | 1.52x | 1.64x | 2.18x | 1.88x | 30/30 |
| materialize | 1.02x | 2.11x | 1.54x | 1.08x | 2.20x | 1.26x | 27/30 |
| perft_bulk_d4 | 0.99x | 4.54x | 1.71x | 1.43x | 2.16x | 2.12x | 30/30 |
| perft_full_d3 | 0.97x | 1.55x | 1.67x | 1.28x | 1.82x | 2.01x | 29/30 |
| fen_parse | 1.03x | 1.18x | 1.33x | 1.24x | 1.60x | 1.34x | 30/30 |
| fen_write_native_ep | 1.06x | 4.17x | 9.17x | 1.87x | 31.13x | 2.50x | 30/30 |
| snapshot | 1.08x | 1.65x | 1.01x | 2.19x | 0.99x | 1.00x | 20/30 |
| san_parse_all | 1.07x | 1.91x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.16x | N/A | 2.78x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 0.97x | 1.01x | 4.83x | 0.95x | 1.03x | 16/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.36x | 73.29x | 157.72x | 186.39x | 185.75x | 5/5 |
| chess960_materialize | 1.00x | 1.60x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.84x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.79x | 32.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 6.36x | 2.12x | 1.34x | 2.44x | 2.81x | 10/10 |
| corpus_materialize | 1.02x | 1.51x | 1.30x | 1.17x | 1.83x | 1.32x | 10/10 |
| corpus_uci_sorted | 0.91x | 1.73x | 2.45x | 1.83x | 2.27x | 1.39x | 10/10 |
| replay_trusted_native | 0.91x | 0.55x | 1.50x | 1.31x | 1.44x | 1.39x | 8/10 |
| replay_checked_native | 1.07x | 2.12x | 0.99x | 6.97x | 6.26x | 7.31x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
