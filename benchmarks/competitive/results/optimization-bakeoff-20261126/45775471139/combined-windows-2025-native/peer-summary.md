| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.08x | 1.42x | 1.38x | 2.00x | 1.77x | 30/30 |
| materialize | 1.03x | 1.86x | 1.27x | 1.16x | 2.15x | 1.43x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.82x | 1.50x | 1.34x | 2.14x | 1.92x | 30/30 |
| perft_full_d3 | 0.99x | 0.88x | 0.82x | 0.72x | 1.06x | 1.02x | 9/30 |
| fen_parse | 0.99x | 1.26x | 1.28x | 1.26x | 2.10x | 1.34x | 30/30 |
| fen_write_native_ep | 1.01x | 2.44x | 4.99x | 1.01x | 19.24x | 1.27x | 26/30 |
| snapshot | 1.01x | 1.55x | 1.16x | 2.37x | 1.16x | 0.91x | 24/30 |
| san_parse_all | 1.04x | 1.80x | N/A | 4.96x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.24x | N/A | 3.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.36x | 0.83x | 5.40x | 0.88x | 1.00x | 15/30 |
| make_unmake_full | 0.98x | N/A | N/A | 0.96x | N/A | N/A | 1/6 |
| startpos | 1.08x | 1.19x | 73.59x | 144.50x | 220.02x | 154.22x | 5/5 |
| chess960_materialize | 1.01x | 1.38x | 1.15x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.34x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 25.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.12x | 21.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.25x | 1.71x | 1.35x | 2.38x | 2.36x | 10/10 |
| corpus_materialize | 0.98x | 1.59x | 1.28x | 1.15x | 2.01x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.19x | 1.49x | 1.22x | 1.44x | 0.96x | 8/10 |
| replay_trusted_native | 1.10x | 0.56x | 1.06x | 1.35x | 1.71x | 2.04x | 8/10 |
| replay_checked_native | 1.00x | 1.93x | 0.86x | 6.66x | 6.35x | 7.65x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
