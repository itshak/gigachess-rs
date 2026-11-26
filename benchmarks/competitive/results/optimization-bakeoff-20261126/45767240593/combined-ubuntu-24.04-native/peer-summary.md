| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.85x | 1.38x | 1.34x | 2.00x | 1.66x | 29/30 |
| materialize | 0.99x | 1.80x | 1.54x | 1.18x | 2.11x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.15x | 3.29x | 1.30x | 1.14x | 1.96x | 1.65x | 30/30 |
| perft_full_d3 | 1.26x | 0.83x | 0.90x | 0.79x | 1.16x | 1.11x | 13/30 |
| fen_parse | 1.09x | 1.41x | 1.47x | 1.16x | 2.33x | 1.34x | 30/30 |
| fen_write_native_ep | 1.01x | 2.34x | 5.10x | 0.92x | 15.19x | 1.18x | 24/30 |
| snapshot | 1.23x | 1.60x | 1.20x | 2.99x | 1.19x | 1.04x | 30/30 |
| san_parse_all | 1.91x | 1.93x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 1.77x | 10.36x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.26x | 1.61x | 0.93x | 3.18x | 1.02x | 1.17x | 22/30 |
| make_unmake_full | 1.16x | N/A | N/A | 0.98x | N/A | N/A | 3/6 |
| startpos | 1.52x | 1.11x | 67.97x | 116.32x | 203.85x | 132.11x | 5/5 |
| chess960_materialize | 1.06x | 1.39x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.20x | 2.14x | 1.14x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 25.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.32x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.08x | 1.70x | 1.28x | 2.42x | 2.25x | 10/10 |
| corpus_materialize | 1.00x | 1.59x | 1.53x | 1.16x | 1.99x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.20x | 1.09x | 1.42x | 1.08x | 1.37x | 0.78x | 8/10 |
| replay_trusted_native | 1.24x | 0.62x | 1.09x | 1.33x | 1.81x | 2.03x | 8/10 |
| replay_checked_native | 1.33x | 2.14x | 0.90x | 6.96x | 7.03x | 8.48x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
