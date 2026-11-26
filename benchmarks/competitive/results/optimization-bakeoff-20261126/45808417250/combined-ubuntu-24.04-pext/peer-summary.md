| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.65x | 1.65x | 1.63x | 2.53x | 1.88x | 30/30 |
| materialize | 1.04x | 2.11x | 1.70x | 1.20x | 2.24x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.11x | 1.60x | 1.32x | 2.50x | 2.08x | 30/30 |
| perft_full_d3 | 0.97x | 1.26x | 1.30x | 0.99x | 1.77x | 1.44x | 26/30 |
| fen_parse | 0.95x | 1.29x | 1.50x | 1.11x | 1.92x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 4.02x | 9.64x | 1.93x | 28.72x | 2.14x | 30/30 |
| snapshot | 1.51x | 1.91x | 1.31x | 3.31x | 1.32x | 1.25x | 30/30 |
| san_parse_all | 0.98x | 2.18x | N/A | 3.76x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.54x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.72x | 1.13x | 4.08x | 1.24x | 1.47x | 28/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.20x | 1.20x | 93.40x | 135.32x | 217.22x | 161.26x | 5/5 |
| chess960_materialize | 1.04x | 1.82x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.96x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.70x | 13.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.06x | 1.91x | 1.47x | 2.65x | 2.18x | 10/10 |
| corpus_materialize | 1.00x | 2.03x | 1.78x | 1.32x | 2.92x | 1.65x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.36x | 3.18x | 2.37x | 3.00x | 1.68x | 10/10 |
| replay_trusted_native | 0.96x | 0.80x | 1.37x | 1.50x | 2.69x | 3.29x | 9/10 |
| replay_checked_native | 0.98x | 2.49x | 0.99x | 5.44x | 6.48x | 7.53x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
