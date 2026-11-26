| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.30x | 1.39x | 1.45x | 1.95x | 1.72x | 30/30 |
| materialize | 0.97x | 1.97x | 1.40x | 1.03x | 2.35x | 1.24x | 28/30 |
| perft_bulk_d4 | 0.97x | 4.57x | 1.56x | 1.25x | 1.98x | 1.97x | 30/30 |
| perft_full_d3 | 0.98x | 0.92x | 0.99x | 0.84x | 1.18x | 1.25x | 13/30 |
| fen_parse | 0.87x | 1.19x | 1.29x | 1.28x | 1.55x | 1.30x | 30/30 |
| fen_write_native_ep | 0.97x | 3.02x | 6.73x | 1.27x | 20.17x | 1.65x | 30/30 |
| snapshot | 0.99x | 1.61x | 1.16x | 2.30x | 1.12x | 0.96x | 24/30 |
| san_parse_all | 0.96x | 1.62x | N/A | 3.55x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.42x | N/A | 2.58x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 0.86x | 0.96x | 4.33x | 0.91x | 1.01x | 13/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.06x | N/A | N/A | 3/6 |
| startpos | 0.98x | 1.11x | 63.08x | 129.75x | 163.61x | 141.61x | 5/5 |
| chess960_materialize | 1.21x | 1.37x | 1.15x | N/A | N/A | N/A | 3/4 |
| chess960_perft_d3 | 1.22x | 3.07x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.25x | 38.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.19x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.20x | 19.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.32x | 6.21x | 2.03x | 1.26x | 2.71x | 2.62x | 10/10 |
| corpus_materialize | 1.22x | 1.46x | 1.40x | 1.18x | 1.97x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.20x | 1.29x | 1.84x | 1.45x | 1.77x | 1.01x | 9/10 |
| replay_trusted_native | 1.29x | 0.49x | 1.20x | 1.29x | 1.46x | 1.30x | 8/10 |
| replay_checked_native | 0.96x | 2.02x | 0.80x | 6.37x | 5.95x | 7.01x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
