| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.56x | 1.50x | 1.40x | 2.11x | 1.85x | 30/30 |
| materialize | 1.00x | 2.07x | 1.37x | 1.16x | 2.31x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.87x | 1.49x | 1.24x | 2.23x | 2.13x | 30/30 |
| perft_full_d3 | 0.90x | 0.99x | 1.16x | 0.89x | 1.50x | 1.70x | 20/30 |
| fen_parse | 0.97x | 1.19x | 1.38x | 1.36x | 1.84x | 1.33x | 30/30 |
| fen_write_native_ep | 0.99x | 3.71x | 7.53x | 1.54x | 31.89x | 2.05x | 30/30 |
| snapshot | 1.14x | 2.32x | 1.40x | 4.28x | 1.41x | 1.44x | 30/30 |
| san_parse_all | 0.99x | 1.93x | N/A | 4.75x | N/A | N/A | 12/12 |
| san_render_native | 1.18x | 11.24x | N/A | 3.92x | N/A | N/A | 12/12 |
| apply_children_full | 1.34x | 1.87x | 1.45x | 7.72x | 1.20x | 2.02x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.39x | 0.50x | 36.98x | 70.19x | 94.63x | 70.15x | 4/5 |
| chess960_materialize | 1.00x | 1.43x | 1.17x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.62x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 37.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 26.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 6.45x | 1.80x | 1.19x | 2.32x | 2.28x | 10/10 |
| corpus_materialize | 1.05x | 1.82x | 1.46x | 1.21x | 2.42x | 1.50x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.65x | 1.98x | 1.76x | 1.92x | 1.39x | 10/10 |
| replay_trusted_native | 0.97x | 0.60x | 1.20x | 1.43x | 2.24x | 2.86x | 8/10 |
| replay_checked_native | 0.99x | 1.94x | 0.93x | 6.94x | 7.14x | 8.86x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
