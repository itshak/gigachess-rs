| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.47x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.00x | 1.81x | 1.42x | 1.25x | 2.00x | 1.37x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.73x | 1.53x | 1.37x | 1.94x | 2.00x | 30/30 |
| perft_full_d3 | 1.00x | 1.15x | 1.25x | 1.05x | 1.52x | 1.47x | 29/30 |
| fen_parse | 0.98x | 1.22x | 1.31x | 1.27x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 0.99x | 4.46x | 9.29x | 1.55x | 29.62x | 2.36x | 30/30 |
| snapshot | 1.00x | 1.12x | 0.98x | 1.44x | 0.97x | 0.90x | 12/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.75x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.10x | 1.30x | 3.52x | 1.07x | 1.05x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.19x | 70.80x | 121.84x | 156.56x | 127.58x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.61x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.75x | 1.82x | 1.47x | 2.16x | 2.37x | 10/10 |
| corpus_materialize | 0.99x | 1.57x | 1.49x | 1.20x | 1.89x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.90x | 2.07x | 2.86x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.51x | 1.31x | 1.58x | 1.76x | 1.82x | 8/10 |
| replay_checked_native | 1.00x | 2.04x | 0.99x | 6.71x | 6.00x | 7.66x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
