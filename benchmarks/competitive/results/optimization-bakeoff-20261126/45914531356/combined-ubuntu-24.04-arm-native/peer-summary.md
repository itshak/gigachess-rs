| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.33x | 2.16x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.85x | 1.57x | 1.43x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.96x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.28x | 1.75x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.59x | 9.27x | 1.59x | 29.89x | 2.49x | 30/30 |
| snapshot | 0.99x | 1.10x | 1.04x | 1.47x | 1.05x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.45x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.22x | 1.44x | 3.91x | 1.17x | 1.17x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.60x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.19x | 130.69x | 172.92x | 137.75x | 5/5 |
| chess960_materialize | 1.01x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.98x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.42x | 1.71x | 1.37x | 2.01x | 2.21x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.53x | 1.22x | 1.91x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.02x | 0.66x | 1.70x | 2.05x | 2.29x | 2.33x | 8/10 |
| replay_checked_native | 0.97x | 2.23x | 1.09x | 7.27x | 6.62x | 8.50x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
