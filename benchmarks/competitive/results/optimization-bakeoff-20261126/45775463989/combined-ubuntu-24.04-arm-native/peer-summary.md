| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.65x | 1.40x | 1.50x | 1.82x | 1.81x | 30/30 |
| materialize | 1.00x | 1.76x | 1.39x | 1.21x | 1.98x | 1.34x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.56x | 1.44x | 1.31x | 1.84x | 1.89x | 30/30 |
| perft_full_d3 | 1.00x | 0.83x | 0.90x | 0.75x | 1.10x | 1.04x | 11/30 |
| fen_parse | 1.00x | 1.25x | 1.33x | 1.29x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 3.01x | 6.17x | 1.05x | 19.69x | 1.62x | 29/30 |
| snapshot | 1.00x | 1.20x | 0.95x | 1.53x | 0.95x | 0.95x | 12/30 |
| san_parse_all | 1.00x | 1.79x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.21x | N/A | 2.43x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 0.89x | 1.04x | 2.95x | 0.86x | 0.86x | 10/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.15x | 71.18x | 122.02x | 161.05x | 129.03x | 5/5 |
| chess960_materialize | 1.00x | 1.40x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.43x | 1.23x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.44x | 1.70x | 1.38x | 1.99x | 2.22x | 10/10 |
| corpus_materialize | 0.99x | 1.53x | 1.46x | 1.16x | 1.82x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.94x | 1.39x | 1.87x | 0.97x | 8/10 |
| replay_trusted_native | 1.00x | 0.42x | 1.09x | 1.30x | 1.45x | 1.50x | 8/10 |
| replay_checked_native | 0.93x | 1.70x | 0.82x | 5.49x | 4.91x | 6.44x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
