| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.62x | 1.51x | 1.51x | 2.29x | 1.88x | 30/30 |
| materialize | 0.96x | 1.98x | 1.27x | 1.10x | 2.23x | 1.38x | 28/30 |
| perft_bulk_d4 | 0.99x | 3.97x | 1.43x | 1.23x | 2.24x | 2.09x | 30/30 |
| perft_full_d3 | 1.01x | 0.99x | 1.05x | 0.88x | 1.40x | 1.43x | 18/30 |
| fen_parse | 0.95x | 1.34x | 1.52x | 1.49x | 2.19x | 1.54x | 30/30 |
| fen_write_native_ep | 1.01x | 3.09x | 6.15x | 1.25x | 26.06x | 1.70x | 30/30 |
| snapshot | 0.90x | 1.95x | 1.07x | 3.00x | 1.14x | 1.00x | 28/30 |
| san_parse_all | 1.04x | 2.02x | N/A | 4.70x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.27x | N/A | 3.53x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.05x | 0.72x | 5.05x | 0.89x | 1.06x | 16/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.24x | 90.03x | 172.02x | 242.73x | 179.39x | 5/5 |
| chess960_materialize | 1.01x | 1.43x | 1.18x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.79x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 38.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 25.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.25x | 1.85x | 1.29x | 2.44x | 2.34x | 10/10 |
| corpus_materialize | 1.02x | 1.77x | 1.38x | 1.16x | 2.36x | 1.48x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.64x | 1.98x | 1.77x | 1.95x | 1.40x | 10/10 |
| replay_trusted_native | 1.00x | 0.72x | 1.22x | 1.42x | 2.22x | 2.66x | 8/10 |
| replay_checked_native | 0.97x | 1.95x | 0.93x | 6.79x | 6.73x | 8.67x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
