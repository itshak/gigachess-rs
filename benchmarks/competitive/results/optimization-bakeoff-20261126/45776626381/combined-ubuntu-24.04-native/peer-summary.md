| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.04x | 1.73x | 1.46x | 2.38x | 1.77x | 30/30 |
| materialize | 1.01x | 2.10x | 1.61x | 1.64x | 2.59x | 1.90x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.83x | 1.57x | 1.19x | 2.49x | 1.88x | 30/30 |
| perft_full_d3 | 1.71x | 1.45x | 1.67x | 1.62x | 2.11x | 1.78x | 30/30 |
| fen_parse | 0.93x | 1.33x | 1.33x | 1.02x | 1.56x | 1.20x | 27/30 |
| fen_write_native_ep | 1.05x | 3.22x | 6.78x | 1.24x | 19.83x | 1.73x | 30/30 |
| snapshot | 0.91x | 1.34x | 1.08x | 2.28x | 1.08x | 1.00x | 27/30 |
| san_parse_all | 0.99x | 1.93x | N/A | 4.06x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.37x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.77x | 1.25x | 0.81x | 2.34x | 0.79x | 1.00x | 15/30 |
| make_unmake_full | 1.21x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.84x | 1.25x | 66.01x | 123.84x | 159.43x | 132.27x | 5/5 |
| chess960_materialize | 1.06x | 1.69x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.61x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 39.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 21.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 6.49x | 2.43x | 1.35x | 2.92x | 2.66x | 10/10 |
| corpus_materialize | 1.00x | 1.44x | 1.33x | 1.09x | 1.76x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.60x | 2.28x | 3.08x | 2.24x | 3.01x | 1.55x | 10/10 |
| replay_trusted_native | 1.17x | 0.72x | 1.25x | 1.27x | 1.83x | 2.12x | 8/10 |
| replay_checked_native | 1.10x | 1.95x | 1.01x | 8.26x | 8.04x | 9.55x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
