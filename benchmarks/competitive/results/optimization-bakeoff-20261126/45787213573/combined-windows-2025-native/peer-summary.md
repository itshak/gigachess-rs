| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.21x | 1.49x | 1.44x | 2.20x | 1.81x | 30/30 |
| materialize | 1.05x | 1.92x | 1.32x | 1.20x | 2.25x | 1.50x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.75x | 1.49x | 1.32x | 2.12x | 1.90x | 30/30 |
| perft_full_d3 | 0.99x | 1.22x | 1.31x | 1.17x | 1.70x | 1.63x | 29/30 |
| fen_parse | 1.06x | 1.31x | 1.48x | 1.34x | 2.40x | 1.57x | 30/30 |
| fen_write_native_ep | 1.01x | 4.09x | 8.65x | 1.62x | 31.37x | 1.98x | 30/30 |
| snapshot | 1.03x | 1.61x | 1.20x | 2.57x | 1.20x | 1.04x | 30/30 |
| san_parse_all | 1.08x | 2.03x | N/A | 5.55x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.33x | N/A | 3.71x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 1.64x | 0.99x | 6.59x | 1.08x | 1.54x | 25/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.14x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.10x | 68.37x | 129.27x | 210.37x | 152.24x | 5/5 |
| chess960_materialize | 1.01x | 1.46x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.44x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 25.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 20.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.26x | 1.72x | 1.33x | 2.41x | 2.34x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.29x | 1.16x | 2.01x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.75x | 2.19x | 1.86x | 2.14x | 1.42x | 10/10 |
| replay_trusted_native | 0.90x | 0.64x | 1.17x | 1.52x | 1.96x | 2.25x | 8/10 |
| replay_checked_native | 0.94x | 2.07x | 0.92x | 6.95x | 7.01x | 8.37x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
