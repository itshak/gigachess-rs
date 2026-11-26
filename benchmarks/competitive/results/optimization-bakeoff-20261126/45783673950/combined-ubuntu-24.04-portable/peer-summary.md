| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.92x | 1.49x | 1.35x | 1.89x | 1.34x | 30/30 |
| materialize | 1.00x | 1.97x | 1.77x | 1.25x | 2.34x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.89x | 1.56x | 1.21x | 1.96x | 1.69x | 30/30 |
| perft_full_d3 | 0.78x | 1.09x | 1.28x | 1.01x | 1.52x | 1.36x | 25/30 |
| fen_parse | 1.00x | 1.63x | 1.53x | 1.22x | 2.35x | 1.42x | 30/30 |
| fen_write_native_ep | 1.03x | 1.67x | 5.51x | 0.96x | 15.49x | 1.09x | 26/30 |
| snapshot | 1.00x | 1.70x | 1.20x | 3.04x | 1.01x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 2.07x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.17x | 10.97x | N/A | 2.37x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.92x | 1.32x | 3.80x | 1.20x | 1.19x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.29x | 64.54x | 105.88x | 180.61x | 112.66x | 5/5 |
| chess960_materialize | 0.99x | 1.50x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.08x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 21.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 17.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.91x | 1.95x | 1.30x | 2.15x | 2.57x | 10/10 |
| corpus_materialize | 0.98x | 1.62x | 1.62x | 1.18x | 2.13x | 1.41x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.19x | 3.27x | 2.44x | 3.13x | 1.80x | 10/10 |
| replay_trusted_native | 0.97x | 0.70x | 1.38x | 1.55x | 2.04x | 1.91x | 8/10 |
| replay_checked_native | 1.00x | 2.13x | 1.06x | 7.19x | 7.30x | 8.51x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
