| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.86x | 1.66x | 1.63x | 2.33x | 2.14x | 30/30 |
| materialize | 0.92x | 1.99x | 1.30x | 1.17x | 2.22x | 1.49x | 28/30 |
| perft_bulk_d4 | 0.98x | 4.04x | 1.61x | 1.36x | 2.29x | 2.09x | 30/30 |
| perft_full_d3 | 0.95x | 1.43x | 1.54x | 1.39x | 2.07x | 1.89x | 30/30 |
| fen_parse | 0.97x | 1.36x | 1.38x | 1.34x | 2.34x | 1.57x | 30/30 |
| fen_write_native_ep | 0.91x | 4.39x | 8.35x | 1.57x | 32.16x | 1.89x | 30/30 |
| snapshot | 0.99x | 2.07x | 1.11x | 4.30x | 1.04x | 1.17x | 27/30 |
| san_parse_all | 0.94x | 2.09x | N/A | 5.42x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.95x | N/A | 3.94x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.30x | 1.45x | 9.98x | 1.49x | 1.71x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.24x | 83.44x | 167.38x | 251.09x | 175.10x | 5/5 |
| chess960_materialize | 0.93x | 1.49x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.78x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 25.53x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 19.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.72x | 1.98x | 1.43x | 2.63x | 2.71x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.24x | 1.13x | 2.06x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.78x | 2.22x | 1.83x | 2.19x | 1.47x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.61x | 2.20x | 2.73x | 3.15x | 9/10 |
| replay_checked_native | 1.00x | 2.43x | 1.09x | 8.65x | 8.14x | 9.99x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
