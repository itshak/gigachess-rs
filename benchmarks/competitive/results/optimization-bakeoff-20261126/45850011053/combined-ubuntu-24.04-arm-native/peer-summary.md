| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.33x | 1.65x | 1.77x | 2.14x | 2.13x | 30/30 |
| materialize | 1.00x | 1.95x | 1.58x | 1.33x | 2.16x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.72x | 1.52x | 1.38x | 1.95x | 1.99x | 30/30 |
| perft_full_d3 | 0.91x | 1.35x | 1.47x | 1.22x | 1.80x | 1.70x | 30/30 |
| fen_parse | 1.02x | 1.24x | 1.32x | 1.29x | 1.74x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.46x | 9.45x | 1.58x | 30.15x | 2.40x | 30/30 |
| snapshot | 0.98x | 1.05x | 1.08x | 1.45x | 1.10x | 0.95x | 24/30 |
| san_parse_all | 0.95x | 1.75x | N/A | 4.02x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 10.77x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.30x | 1.55x | 4.19x | 1.25x | 1.25x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.46x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.24x | 71.04x | 121.85x | 158.77x | 129.53x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.83x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.37x | 1.68x | 1.35x | 1.98x | 2.18x | 10/10 |
| corpus_materialize | 0.99x | 1.57x | 1.50x | 1.21x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.95x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.93x | 0.58x | 1.52x | 1.83x | 2.08x | 2.09x | 8/10 |
| replay_checked_native | 0.94x | 2.01x | 0.98x | 6.63x | 5.95x | 7.61x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
