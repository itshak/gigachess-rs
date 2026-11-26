| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.30x | 1.51x | 1.43x | 2.10x | 1.83x | 30/30 |
| materialize | 1.05x | 2.01x | 1.63x | 1.12x | 2.24x | 1.45x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.03x | 1.52x | 1.19x | 2.30x | 2.17x | 30/30 |
| perft_full_d3 | 1.24x | 1.36x | 1.58x | 1.19x | 2.05x | 2.19x | 30/30 |
| fen_parse | 1.19x | 1.35x | 1.50x | 1.23x | 1.98x | 1.33x | 30/30 |
| fen_write_native_ep | 1.00x | 3.86x | 8.37x | 1.51x | 25.16x | 2.05x | 30/30 |
| snapshot | 1.55x | 2.49x | 1.50x | 4.50x | 1.50x | 1.49x | 30/30 |
| san_parse_all | 0.99x | 1.77x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 11.07x | N/A | 2.33x | N/A | N/A | 12/12 |
| apply_children_full | 1.22x | 1.63x | 1.18x | 3.48x | 1.16x | 1.62x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.24x | 84.24x | 144.47x | 222.22x | 152.83x | 5/5 |
| chess960_materialize | 1.00x | 1.56x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.72x | 1.90x | 0.99x | N/A | N/A | N/A | 3/4 |
| polyglot_key | 1.04x | 37.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.86x | 23.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.17x | 1.84x | 1.21x | 2.46x | 2.28x | 10/10 |
| corpus_materialize | 1.05x | 1.77x | 1.63x | 1.19x | 2.31x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.34x | 2.99x | 2.45x | 3.00x | 1.67x | 10/10 |
| replay_trusted_native | 0.99x | 0.62x | 1.17x | 1.31x | 2.06x | 2.88x | 7/10 |
| replay_checked_native | 1.01x | 2.08x | 0.90x | 6.88x | 7.08x | 8.57x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
