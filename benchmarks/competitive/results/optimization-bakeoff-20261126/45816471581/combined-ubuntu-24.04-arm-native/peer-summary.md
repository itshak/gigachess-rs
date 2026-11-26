| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.89x | 1.49x | 1.64x | 1.93x | 1.93x | 30/30 |
| materialize | 1.00x | 1.83x | 1.43x | 1.26x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.40x | 1.39x | 1.27x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 1.02x | 1.21x | 1.31x | 1.09x | 1.60x | 1.52x | 29/30 |
| fen_parse | 0.99x | 1.25x | 1.34x | 1.27x | 1.76x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.56x | 9.37x | 1.58x | 30.41x | 2.46x | 30/30 |
| snapshot | 0.99x | 1.10x | 1.04x | 1.46x | 1.03x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.66x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.28x | 1.51x | 4.17x | 1.22x | 1.23x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.49x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.57x | 125.05x | 175.36x | 142.51x | 5/5 |
| chess960_materialize | 1.01x | 1.59x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.89x | 1.50x | 1.19x | 1.76x | 1.94x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.45x | 1.16x | 1.81x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.09x | 2.92x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 0.99x | 0.55x | 1.47x | 1.72x | 1.94x | 1.97x | 8/10 |
| replay_checked_native | 1.00x | 2.09x | 1.03x | 6.84x | 6.12x | 7.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
