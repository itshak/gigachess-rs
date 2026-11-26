| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.86x | 5.16x | 1.65x | 1.72x | 2.41x | 2.28x | 30/30 |
| materialize | 0.91x | 1.93x | 1.49x | 1.03x | 2.03x | 1.34x | 28/30 |
| perft_bulk_d4 | 1.03x | 3.95x | 1.56x | 1.28x | 2.00x | 1.91x | 29/30 |
| perft_full_d3 | 1.01x | 1.78x | 1.99x | 1.57x | 2.23x | 1.99x | 30/30 |
| fen_parse | 0.91x | 1.14x | 1.28x | 1.29x | 1.51x | 1.51x | 29/30 |
| fen_write_native_ep | 0.95x | 4.08x | 9.12x | 1.65x | 27.89x | 2.06x | 30/30 |
| snapshot | 1.34x | 2.07x | 1.15x | 2.91x | 1.47x | 1.17x | 27/30 |
| san_parse_all | 0.96x | 1.77x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 13.30x | N/A | 2.78x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.17x | 1.19x | 5.68x | 1.12x | 1.24x | 28/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.95x | 1.56x | 73.44x | 156.40x | 192.65x | 182.06x | 5/5 |
| chess960_materialize | 1.32x | 1.89x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.32x | 3.38x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.26x | 35.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.47x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.37x | 21.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.19x | 5.71x | 1.69x | 1.03x | 2.13x | 2.22x | 9/10 |
| corpus_materialize | 1.66x | 1.57x | 1.62x | 1.35x | 2.30x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.29x | 1.96x | 2.63x | 2.00x | 2.55x | 1.66x | 10/10 |
| replay_trusted_native | 1.29x | 0.60x | 1.78x | 1.59x | 1.73x | 1.61x | 8/10 |
| replay_checked_native | 1.24x | 2.76x | 1.20x | 9.06x | 7.48x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
