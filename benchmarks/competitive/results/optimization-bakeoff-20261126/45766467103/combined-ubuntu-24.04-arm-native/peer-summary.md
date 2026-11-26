| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.13x | 3.47x | 1.33x | 1.43x | 1.72x | 1.71x | 30/30 |
| materialize | 1.02x | 1.75x | 1.40x | 1.22x | 1.94x | 1.34x | 30/30 |
| perft_bulk_d4 | 1.26x | 3.07x | 1.25x | 1.13x | 1.59x | 1.63x | 30/30 |
| perft_full_d3 | 1.19x | 0.78x | 0.85x | 0.72x | 1.03x | 0.99x | 8/30 |
| fen_parse | 1.03x | 1.24x | 1.32x | 1.26x | 1.77x | 1.39x | 30/30 |
| fen_write_native_ep | 0.99x | 3.04x | 6.12x | 1.05x | 20.07x | 1.58x | 29/30 |
| snapshot | 0.97x | 0.90x | 0.79x | 1.19x | 0.77x | 0.78x | 6/30 |
| san_parse_all | 1.49x | 1.40x | N/A | 3.29x | N/A | N/A | 12/12 |
| san_render_native | 1.50x | 7.98x | N/A | 1.72x | N/A | N/A | 12/12 |
| apply_children_full | 1.14x | 0.81x | 0.94x | 2.59x | 0.77x | 0.78x | 8/30 |
| make_unmake_full | 1.14x | N/A | N/A | 1.03x | N/A | N/A | 4/6 |
| startpos | 1.07x | 0.84x | 51.69x | 87.63x | 116.52x | 93.09x | 4/5 |
| chess960_materialize | 1.12x | 1.42x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.27x | 2.12x | 1.07x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 24.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.09x | 15.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 4.38x | 1.68x | 1.35x | 1.97x | 2.17x | 10/10 |
| corpus_materialize | 1.04x | 1.53x | 1.46x | 1.19x | 1.82x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.34x | 1.10x | 1.51x | 1.10x | 1.47x | 0.76x | 8/10 |
| replay_trusted_native | 1.14x | 0.36x | 0.94x | 1.16x | 1.27x | 1.31x | 6/10 |
| replay_checked_native | 1.24x | 1.68x | 0.82x | 5.54x | 4.88x | 6.31x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
