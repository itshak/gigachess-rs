| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.86x | 1.47x | 1.57x | 1.91x | 1.90x | 30/30 |
| materialize | 1.00x | 1.83x | 1.45x | 1.25x | 2.03x | 1.39x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.69x | 1.50x | 1.38x | 1.92x | 1.95x | 30/30 |
| perft_full_d3 | 0.97x | 1.15x | 1.24x | 1.02x | 1.51x | 1.43x | 28/30 |
| fen_parse | 1.00x | 1.25x | 1.33x | 1.28x | 1.76x | 1.42x | 30/30 |
| fen_write_native_ep | 1.01x | 4.49x | 9.24x | 1.56x | 30.21x | 2.41x | 30/30 |
| snapshot | 0.94x | 1.09x | 0.98x | 1.44x | 1.01x | 0.89x | 17/30 |
| san_parse_all | 1.01x | 1.87x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 10.61x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.10x | 1.31x | 3.65x | 1.08x | 1.07x | 28/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.28x | 76.64x | 128.97x | 172.86x | 140.36x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.60x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.80x | 1.85x | 1.50x | 2.17x | 2.37x | 10/10 |
| corpus_materialize | 1.02x | 1.60x | 1.53x | 1.21x | 1.91x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.93x | 2.09x | 2.80x | 1.46x | 10/10 |
| replay_trusted_native | 1.02x | 0.50x | 1.35x | 1.62x | 1.78x | 1.84x | 8/10 |
| replay_checked_native | 0.91x | 1.85x | 0.90x | 6.01x | 5.41x | 7.04x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
