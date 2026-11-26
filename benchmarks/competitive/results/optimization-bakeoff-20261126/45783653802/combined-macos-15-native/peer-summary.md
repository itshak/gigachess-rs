| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.30x | 1.45x | 1.51x | 1.96x | 1.73x | 30/30 |
| materialize | 1.00x | 1.89x | 1.50x | 1.05x | 2.29x | 1.29x | 27/30 |
| perft_bulk_d4 | 0.98x | 4.41x | 1.63x | 1.32x | 2.03x | 2.04x | 30/30 |
| perft_full_d3 | 1.00x | 1.56x | 1.80x | 1.35x | 1.97x | 2.02x | 30/30 |
| fen_parse | 1.02x | 1.29x | 1.38x | 1.36x | 1.72x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 3.12x | 6.94x | 1.31x | 21.29x | 1.72x | 30/30 |
| snapshot | 0.99x | 1.71x | 1.10x | 2.33x | 1.09x | 0.98x | 26/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.66x | N/A | 2.65x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.00x | 1.06x | 5.04x | 1.05x | 1.25x | 22/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.21x | 68.56x | 150.42x | 163.60x | 138.38x | 5/5 |
| chess960_materialize | 0.93x | 1.49x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 3.16x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 19.68x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 6.32x | 2.22x | 1.43x | 2.67x | 2.75x | 10/10 |
| corpus_materialize | 0.98x | 1.47x | 1.36x | 1.16x | 2.11x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.92x | 1.74x | 2.56x | 1.94x | 2.41x | 1.46x | 10/10 |
| replay_trusted_native | 0.90x | 0.58x | 1.45x | 1.37x | 1.54x | 1.47x | 8/10 |
| replay_checked_native | 1.00x | 2.16x | 0.99x | 8.05x | 6.98x | 8.88x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
