| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.89x | 1.49x | 1.60x | 1.93x | 1.93x | 30/30 |
| materialize | 1.00x | 1.84x | 1.46x | 1.26x | 2.04x | 1.41x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.43x | 1.40x | 1.27x | 1.79x | 1.84x | 30/30 |
| perft_full_d3 | 1.01x | 1.17x | 1.27x | 1.06x | 1.57x | 1.48x | 29/30 |
| fen_parse | 1.01x | 1.25x | 1.32x | 1.26x | 1.76x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.37x | 1.57x | 30.41x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.03x | 1.47x | 1.02x | 0.95x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.76x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.46x | 3.98x | 1.19x | 1.19x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.19x | 129.66x | 175.95x | 142.15x | 5/5 |
| chess960_materialize | 1.01x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.64x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.88x | 1.51x | 1.19x | 1.75x | 1.97x | 10/10 |
| corpus_materialize | 1.01x | 1.53x | 1.47x | 1.17x | 1.83x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.09x | 2.90x | 2.08x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.61x | 1.63x | 1.99x | 2.20x | 2.24x | 8/10 |
| replay_checked_native | 0.99x | 2.14x | 1.04x | 6.93x | 6.23x | 8.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
