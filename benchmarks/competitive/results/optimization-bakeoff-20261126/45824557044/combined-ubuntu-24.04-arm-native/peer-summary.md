| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.89x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.82x | 1.45x | 1.26x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.41x | 1.40x | 1.27x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 0.99x | 1.17x | 1.27x | 1.06x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.26x | 1.78x | 1.41x | 30/30 |
| fen_write_native_ep | 0.98x | 4.46x | 9.17x | 1.61x | 29.07x | 2.35x | 30/30 |
| snapshot | 1.10x | 1.29x | 0.93x | 1.62x | 0.92x | 1.01x | 18/30 |
| san_parse_all | 1.01x | 1.86x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.73x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.29x | 1.49x | 4.19x | 1.24x | 1.25x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.52x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 76.22x | 130.63x | 174.49x | 140.02x | 5/5 |
| chess960_materialize | 1.00x | 1.57x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.63x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.97x | 1.51x | 1.23x | 1.79x | 1.94x | 10/10 |
| corpus_materialize | 1.01x | 1.56x | 1.46x | 1.18x | 1.84x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.09x | 2.78x | 1.45x | 10/10 |
| replay_trusted_native | 0.98x | 0.60x | 1.57x | 1.92x | 2.13x | 2.18x | 8/10 |
| replay_checked_native | 0.99x | 2.20x | 1.06x | 7.04x | 6.36x | 8.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
