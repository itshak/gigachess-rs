| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.43x | 1.49x | 1.53x | 2.35x | 1.77x | 30/30 |
| materialize | 1.01x | 1.97x | 1.65x | 1.24x | 2.26x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.68x | 1.40x | 1.32x | 2.22x | 1.70x | 30/30 |
| perft_full_d3 | 0.88x | 0.73x | 0.81x | 0.72x | 1.06x | 0.97x | 7/30 |
| fen_parse | 0.83x | 1.11x | 1.19x | 0.98x | 1.72x | 1.04x | 24/30 |
| fen_write_native_ep | 1.00x | 2.36x | 5.41x | 0.96x | 15.68x | 1.20x | 25/30 |
| snapshot | 0.97x | 1.27x | 1.14x | 1.93x | 1.14x | 0.86x | 24/30 |
| san_parse_all | 0.87x | 1.76x | N/A | 3.68x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 9.70x | N/A | 2.02x | N/A | N/A | 12/12 |
| apply_children_full | 0.84x | 1.37x | 0.78x | 2.82x | 0.83x | 0.97x | 14/30 |
| make_unmake_full | 0.85x | N/A | N/A | 0.86x | N/A | N/A | 0/6 |
| startpos | 0.81x | 1.00x | 61.63x | 110.71x | 170.14x | 116.33x | 5/5 |
| chess960_materialize | 0.98x | 1.51x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.32x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 25.18x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.90x | 17.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.63x | 1.78x | 1.43x | 2.71x | 2.34x | 10/10 |
| corpus_materialize | 0.99x | 1.64x | 1.56x | 1.20x | 2.05x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.41x | 1.88x | 1.40x | 1.85x | 1.02x | 10/10 |
| replay_trusted_native | 0.88x | 0.53x | 0.97x | 1.16x | 1.55x | 1.73x | 5/10 |
| replay_checked_native | 0.80x | 1.70x | 0.75x | 5.83x | 5.72x | 6.78x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
