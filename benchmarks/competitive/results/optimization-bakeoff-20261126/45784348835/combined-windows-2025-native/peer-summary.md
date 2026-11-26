| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.20x | 1.47x | 1.47x | 2.17x | 1.83x | 30/30 |
| materialize | 1.03x | 1.93x | 1.33x | 1.20x | 2.24x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.53x | 1.37x | 2.20x | 1.96x | 30/30 |
| perft_full_d3 | 0.98x | 1.30x | 1.38x | 1.22x | 1.80x | 1.74x | 30/30 |
| fen_parse | 1.00x | 1.27x | 1.33x | 1.26x | 2.09x | 1.36x | 30/30 |
| fen_write_native_ep | 0.99x | 4.05x | 8.34x | 1.58x | 30.87x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.55x | 1.17x | 4.03x | 1.19x | 1.04x | 30/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 5.37x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.93x | N/A | 3.79x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 2.07x | 1.27x | 8.49x | 1.34x | 1.49x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.15x | 1.11x | 72.76x | 138.35x | 202.43x | 149.50x | 5/5 |
| chess960_materialize | 1.01x | 1.50x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.55x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 24.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 19.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.41x | 1.81x | 1.37x | 2.41x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.64x | 1.31x | 1.17x | 2.03x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.78x | 2.20x | 1.82x | 2.13x | 1.42x | 10/10 |
| replay_trusted_native | 1.06x | 0.73x | 1.39x | 1.74x | 2.25x | 2.60x | 8/10 |
| replay_checked_native | 1.00x | 2.20x | 0.99x | 7.44x | 7.39x | 8.93x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
