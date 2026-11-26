| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.75x | 1.61x | 1.65x | 2.49x | 1.90x | 30/30 |
| materialize | 0.97x | 1.98x | 1.64x | 1.25x | 2.34x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.79x | 3.28x | 1.26x | 1.11x | 1.96x | 1.55x | 29/30 |
| perft_full_d3 | 0.97x | 1.31x | 1.47x | 1.29x | 1.90x | 1.76x | 30/30 |
| fen_parse | 0.87x | 1.21x | 1.31x | 1.08x | 1.98x | 1.21x | 29/30 |
| fen_write_native_ep | 0.93x | 4.08x | 9.15x | 1.65x | 25.60x | 1.91x | 30/30 |
| snapshot | 0.90x | 1.85x | 1.42x | 2.93x | 1.42x | 1.23x | 30/30 |
| san_parse_all | 0.87x | 1.82x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 9.99x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.77x | 1.82x | 1.04x | 3.54x | 1.16x | 1.32x | 27/30 |
| make_unmake_full | 0.76x | N/A | N/A | 0.98x | N/A | N/A | 1/6 |
| startpos | 1.39x | 1.80x | 109.56x | 194.84x | 307.87x | 207.81x | 5/5 |
| chess960_materialize | 0.93x | 1.55x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 2.41x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.20x | 31.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.15x | 23.68x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.48x | 1.80x | 1.39x | 2.61x | 2.26x | 10/10 |
| corpus_materialize | 0.98x | 1.63x | 1.55x | 1.19x | 2.04x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.57x | 3.32x | 2.53x | 3.23x | 1.79x | 10/10 |
| replay_trusted_native | 0.78x | 0.70x | 1.30x | 1.53x | 2.07x | 2.31x | 8/10 |
| replay_checked_native | 0.83x | 2.14x | 0.96x | 7.11x | 7.16x | 8.32x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
