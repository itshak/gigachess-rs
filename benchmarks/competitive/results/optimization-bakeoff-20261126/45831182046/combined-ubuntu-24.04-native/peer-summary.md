| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.29x | 1.55x | 1.49x | 2.32x | 1.86x | 30/30 |
| materialize | 1.00x | 1.85x | 1.60x | 1.19x | 2.18x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.04x | 3.65x | 1.47x | 1.28x | 2.22x | 1.86x | 30/30 |
| perft_full_d3 | 1.04x | 1.32x | 1.50x | 1.32x | 1.93x | 1.91x | 30/30 |
| fen_parse | 0.89x | 1.24x | 1.35x | 1.00x | 2.00x | 1.16x | 26/30 |
| fen_write_native_ep | 1.01x | 4.34x | 10.31x | 1.74x | 28.46x | 2.08x | 30/30 |
| snapshot | 1.02x | 1.99x | 1.25x | 3.52x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.03x | 2.08x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.52x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.35x | 1.34x | 4.65x | 1.49x | 1.64x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.79x | 133.49x | 234.59x | 152.50x | 5/5 |
| chess960_materialize | 1.04x | 1.53x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.73x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.92x | 25.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.93x | 1.63x | 1.25x | 2.39x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.47x | 1.12x | 1.91x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.47x | 3.23x | 2.42x | 3.15x | 1.80x | 10/10 |
| replay_trusted_native | 1.02x | 0.89x | 1.58x | 1.90x | 2.65x | 2.90x | 9/10 |
| replay_checked_native | 1.07x | 2.43x | 1.10x | 8.20x | 8.29x | 9.75x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
