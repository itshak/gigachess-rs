| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.32x | 1.51x | 1.47x | 2.13x | 1.88x | 30/30 |
| materialize | 1.03x | 1.90x | 1.30x | 1.18x | 2.25x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.63x | 1.44x | 1.24x | 2.07x | 1.87x | 30/30 |
| perft_full_d3 | 1.04x | 1.34x | 1.42x | 1.30x | 1.87x | 1.99x | 30/30 |
| fen_parse | 1.10x | 1.43x | 1.49x | 1.38x | 2.25x | 1.49x | 30/30 |
| fen_write_native_ep | 1.02x | 3.94x | 8.16x | 1.57x | 31.99x | 2.00x | 30/30 |
| snapshot | 3.64x | 1.80x | 0.97x | 3.97x | 0.98x | 0.97x | 22/30 |
| san_parse_all | 1.06x | 2.10x | N/A | 5.62x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 10.70x | N/A | 3.72x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 2.10x | 1.28x | 8.63x | 1.35x | 1.52x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.18x | 1.25x | 79.21x | 156.61x | 228.35x | 168.43x | 5/5 |
| chess960_materialize | 1.09x | 1.51x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.68x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 26.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.86x | 1.63x | 1.22x | 2.23x | 2.14x | 10/10 |
| corpus_materialize | 0.97x | 1.49x | 1.21x | 1.09x | 1.87x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.10x | 1.74x | 2.16x | 1.77x | 2.13x | 1.39x | 10/10 |
| replay_trusted_native | 1.16x | 0.72x | 1.38x | 1.77x | 2.22x | 2.56x | 8/10 |
| replay_checked_native | 1.15x | 2.22x | 0.99x | 7.46x | 7.62x | 8.86x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
