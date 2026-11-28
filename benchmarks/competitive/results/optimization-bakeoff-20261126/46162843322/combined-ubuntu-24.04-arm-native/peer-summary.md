| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.45x | 1.69x | 1.82x | 2.21x | 2.19x | 30/30 |
| materialize | 1.01x | 1.98x | 1.55x | 1.35x | 2.18x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.01x | 1.62x | 1.48x | 2.08x | 2.13x | 30/30 |
| perft_full_d3 | 1.12x | 1.66x | 1.81x | 1.50x | 2.23x | 2.09x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.33x | 1.29x | 1.76x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 4.52x | 9.30x | 1.57x | 29.91x | 2.39x | 30/30 |
| snapshot | 1.01x | 1.09x | 1.04x | 1.47x | 1.04x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.57x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.26x | 1.49x | 4.05x | 1.22x | 1.21x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.37x | 129.09x | 172.72x | 139.72x | 5/5 |
| chess960_materialize | 0.98x | 1.68x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.99x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.37x | 1.49x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.93x | 1.87x | 1.47x | 2.20x | 2.43x | 10/10 |
| corpus_materialize | 1.00x | 1.64x | 1.55x | 1.22x | 1.92x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.90x | 2.08x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 0.98x | 0.61x | 1.63x | 1.95x | 2.24x | 2.25x | 8/10 |
| replay_checked_native | 0.95x | 2.19x | 1.07x | 7.11x | 6.40x | 8.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
