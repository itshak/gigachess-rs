| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.57x | 1.59x | 1.62x | 2.50x | 1.84x | 30/30 |
| materialize | 0.99x | 2.15x | 1.69x | 1.17x | 2.20x | 1.46x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.18x | 1.61x | 1.34x | 2.58x | 2.10x | 30/30 |
| perft_full_d3 | 1.02x | 1.30x | 1.32x | 1.01x | 1.84x | 1.97x | 26/30 |
| fen_parse | 1.01x | 1.37x | 1.54x | 1.20x | 2.30x | 1.33x | 30/30 |
| fen_write_native_ep | 1.00x | 4.17x | 9.71x | 1.89x | 28.01x | 2.03x | 30/30 |
| snapshot | 0.85x | 1.36x | 0.99x | 2.06x | 1.00x | 0.84x | 16/30 |
| san_parse_all | 0.99x | 2.26x | N/A | 3.79x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.66x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.55x | 1.43x | 3.95x | 1.43x | 1.34x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.17x | 80.65x | 125.58x | 218.76x | 135.43x | 5/5 |
| chess960_materialize | 0.99x | 1.87x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.22x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.23x | 1.92x | 1.49x | 2.84x | 2.19x | 10/10 |
| corpus_materialize | 1.01x | 2.08x | 1.78x | 1.26x | 2.95x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.38x | 3.18x | 2.39x | 2.98x | 1.69x | 10/10 |
| replay_trusted_native | 1.00x | 0.74x | 1.31x | 1.48x | 2.67x | 2.93x | 8/10 |
| replay_checked_native | 0.99x | 2.45x | 0.97x | 5.42x | 6.20x | 7.12x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
