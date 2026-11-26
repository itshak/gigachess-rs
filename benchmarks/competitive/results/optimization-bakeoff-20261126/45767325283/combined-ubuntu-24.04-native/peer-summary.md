| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.16x | 3.91x | 1.68x | 1.43x | 2.16x | 1.71x | 30/30 |
| materialize | 1.06x | 2.05x | 1.56x | 1.59x | 2.45x | 1.77x | 30/30 |
| perft_bulk_d4 | 1.34x | 3.73x | 1.54x | 1.17x | 2.30x | 1.84x | 30/30 |
| perft_full_d3 | 1.44x | 0.90x | 0.99x | 0.97x | 1.29x | 1.06x | 16/30 |
| fen_parse | 0.97x | 1.40x | 1.26x | 1.11x | 1.42x | 1.19x | 30/30 |
| fen_write_native_ep | 0.97x | 3.07x | 6.27x | 1.31x | 17.25x | 1.56x | 30/30 |
| snapshot | 1.24x | 1.80x | 1.24x | 2.81x | 1.24x | 1.15x | 30/30 |
| san_parse_all | 1.83x | 1.68x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.97x | 9.00x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.46x | 1.55x | 0.99x | 2.86x | 1.01x | 1.19x | 24/30 |
| make_unmake_full | 1.22x | N/A | N/A | 0.93x | N/A | N/A | 0/6 |
| startpos | 1.42x | 1.05x | 66.05x | 124.75x | 146.22x | 129.50x | 5/5 |
| chess960_materialize | 1.15x | 1.53x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.37x | 2.17x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 36.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.59x | 22.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 6.27x | 2.38x | 1.30x | 2.89x | 2.62x | 10/10 |
| corpus_materialize | 1.02x | 1.43x | 1.33x | 1.10x | 1.75x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.58x | 1.44x | 1.92x | 1.41x | 1.86x | 0.98x | 8/10 |
| replay_trusted_native | 1.23x | 0.61x | 1.13x | 1.11x | 1.55x | 1.83x | 7/10 |
| replay_checked_native | 1.23x | 1.73x | 0.98x | 7.45x | 7.18x | 8.55x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
